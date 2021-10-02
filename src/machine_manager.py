"""
Copyright 2019 Cartesi Pte. Ltd.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
"""

from concurrent import futures
from threading import Lock, currentThread
import pickle
import time
import traceback
import grpc
from grpc_reflection.v1alpha import reflection

import machine_manager_pb2_grpc
import machine_manager_pb2
import cartesi_machine_pb2
import cartesi_machine_pb2_grpc
import utils
from session_registry import SessionIdException, AddressException, RollbackException, CheckinException


LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

class NotReadyException(Exception):
    pass

class SessionJob:
    def __init__(self, session_id):
        self.id = session_id
        self.job_hash = None
        self.job_future = None

class _MachineManager(machine_manager_pb2_grpc.MachineManagerServicer):
    def __init__(self, session_registry_manager):
        self.executor = futures.ThreadPoolExecutor(max_workers=10)
        self.session_registry_manager = session_registry_manager
        self.global_lock = Lock()
        self.job_cache = {}
        self.job = {}

    def __set_job_cache__(self, request, future):
        LOGGER.debug("Setting job cache")
        result = future.result()
        request_hash = pickle.dumps(request)
        #Cache the job only if no exception raised
        self.job_cache[request_hash] = future
        return result

    def __set_job_future__(self, session_id, future):
        self.job[session_id].job_future = future

    def __set_job_hash__(self, session_id, request):
        self.job[session_id].job_hash = request

    def __reset_job__(self, session_id):
        self.job[session_id].job_future = None
        self.job[session_id].job_hash = None

    def __get_job__(self, session_id, request, err_msg, fn, *args):
        LOGGER.debug("Acquiring manager global lock")
        with self.global_lock:
            LOGGER.debug("Lock acquired")
            request_hash = pickle.dumps(request)

            if request_hash in self.job_cache.keys():
                LOGGER.debug("Job found in cache")
                return self.job_cache[request_hash]

            if session_id in self.job.keys():
                if self.job[session_id].job_future is not None:
                    if self.job[session_id].job_future.done():
                        LOGGER.debug("Job is done")
                        if request_hash == self.job[session_id].job_hash:
                            LOGGER.debug("Request hash matches, return job")
                            job = self.job[session_id].job_future
                            self.__reset_job__(session_id)
                            return job
                        else:
                            LOGGER.debug("Request hash not match, dump result and start fresh")
                    else:
                        LOGGER.debug("Job is not done")
                        raise NotReadyException(err_msg)
            else:
                LOGGER.debug("First SessionJob creation")
                self.job[session_id] = SessionJob(session_id)

            self.__set_job_hash__(session_id, request_hash)
            self.__set_job_future__(session_id, self.executor.submit(fn, *args))
            raise NotReadyException(err_msg)


    def ServerShuttingDown(self, context):
        if self.session_registry_manager.shutting_down:
            context.set_details("Server is shutting down, not accepting new requests")
            context.set_code(grpc.StatusCode.UNAVAILABLE)
            return True
        else:
            return False

    def NewSession(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            machine_req = request.machine
            force = request.force

            LOGGER.info("New session requested with session_id: {}".format(session_id))

            return self.session_registry_manager.new_session(session_id, machine_req, force)

        #No session with provided id or address issue
        except (SessionIdException, AddressException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Checkin request failed
        except CheckinException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.DEADLINE_EXCEEDED)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)

    def SessionRun(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            final_cycles = request.final_cycles
            LOGGER.info("New session run requested for session_id {} with final cycles {}".format(session_id, final_cycles))

            #Validate cycle values
            utils.validate_cycles(final_cycles)

            err_msg = "Result is not yet ready for SessionRun: " + session_id
            job = self.__get_job__(session_id, request, err_msg, self.session_registry_manager.run_session, session_id, final_cycles)
            return self.__set_job_cache__(request, job)

        #If the session result is not ready yet, return progress
        except NotReadyException as e:
            LOGGER.debug("Not ready yet, getting progress")
            session_context = self.session_registry_manager.registry[session_id]

            #Calculating cycles related progress
            last_cycle = request.final_cycles[-1]
            if session_context.halt_cycle != None:
                if last_cycle > session_context.halt_cycle:
                    last_cycle = session_context.halt_cycle

            cycle_progress = 0
            #Calcuting percentage progress with 2 decimal places, if machine already in a cycle
            #that alows it to run to the desired cycle
            if (session_context.cycle <= last_cycle):
                cycle_progress = int(int(session_context.cycle/last_cycle * 10000) / 100)

            #Build a status object to return
            session_run_progress = machine_manager_pb2.SessionRunProgress(
                    progress=cycle_progress,
                    application_progress=session_context.app_progress,
                    updated_at=int(session_context.updated_at),
                    cycle=session_context.cycle
            )
            return machine_manager_pb2.SessionRunResponse(progress=session_run_progress)

        #No session with provided id, address issue, bad final cycles provided or problem during rollback
        except (SessionIdException, AddressException, utils.CycleException, RollbackException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Checkin request failed
        except CheckinException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.DEADLINE_EXCEEDED)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)

    def SessionStep(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            initial_cycle = request.initial_cycle
            step_params = None

            #Setting step_params if provided
            if (request.WhichOneof("step_params_oneof") is not None):
                if (request.WhichOneof("step_params_oneof") == "step_params"):
                    step_params = request.step_params
                    LOGGER.info("Step parameters received on request")

            #Setting default step parameters if none were provided
            if (step_params == None):
                log_type = cartesi_machine_pb2.AccessLogType(proofs=True, annotations=False)
                step_params = cartesi_machine_pb2.StepRequest(log_type=log_type)
                LOGGER.info("Step parameters set to default")

            LOGGER.info("New session step requested for session_id {} with initial cycle {}\nLog proofs: {}\nLog annotations: {}".format(session_id, initial_cycle, step_params.log_type.proofs, step_params.log_type.annotations))

            #Validate cycle value
            utils.validate_cycles([initial_cycle])
            return self.session_registry_manager.step_session(session_id, initial_cycle, step_params)

        #No session with provided id, address issue, bad initial cycle provided or problem during rollback
        except (SessionIdException, AddressException, utils.CycleException, RollbackException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Checkin request failed
        except CheckinException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.DEADLINE_EXCEEDED)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)

    def SessionStore(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            store_req = request.store

            LOGGER.info("New session store requested for session_id {} on directory {}".format(session_id, store_req.directory))

            return self.session_registry_manager.session_store(session_id, store_req)

        #No session with provided id or address issue
        except (SessionIdException, AddressException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)

    def SessionReadMemory(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            read_mem_req = request.position
            cycle = request.cycle
            LOGGER.info("New session memory read requested for session_id {} on cycle {} for address {} with length {}".format(session_id, cycle, read_mem_req.address, read_mem_req.length))

            return self.session_registry_manager.session_read_mem(session_id, cycle, read_mem_req)

        #No session with provided id or address issue
        except (SessionIdException, AddressException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Checkin request failed
        except CheckinException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.DEADLINE_EXCEEDED)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)

    def SessionWriteMemory(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            write_mem_req = request.position
            cycle = request.cycle
            LOGGER.info("New session memory write requested for session_id {} on cycle {} for address {} with data {}".format(session_id, cycle, write_mem_req.address, write_mem_req.data))

            return self.session_registry_manager.session_write_mem(session_id, cycle, write_mem_req)

        #No session with provided id or address issue
        except (SessionIdException, AddressException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Checkin request failed
        except CheckinException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.DEADLINE_EXCEEDED)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)

    def SessionGetProof(self, request, context):
        try:
            if self.ServerShuttingDown(context):
                return

            session_id = request.session_id
            proof_req = request.target
            cycle = request.cycle

            LOGGER.info("New session proof requested for session_id {} on cycle {} for address {} with log2_size {}".format(session_id, cycle, proof_req.address, proof_req.log2_size))

            return self.session_registry_manager.session_get_proof(session_id, cycle, proof_req)

        #No session with provided id or address issue
        except (SessionIdException, AddressException) as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Checkin request failed
        except CheckinException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.DEADLINE_EXCEEDED)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)


def start_manager_server(args, registry_manager):
    manager_address = '{}:{}'.format(args.address, args.port)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    machine_manager_pb2_grpc.add_MachineManagerServicer_to_server(_MachineManager(registry_manager),
                                                      server)
    SERVICE_NAMES = (
        machine_manager_pb2.DESCRIPTOR.services_by_name['MachineManager'].full_name,
        reflection.SERVICE_NAME,
    )
    reflection.enable_server_reflection(SERVICE_NAMES, server)
    server.add_insecure_port(manager_address)
    server.start()
    LOGGER.info("Server started, listening on address {}".format(manager_address))

    t = currentThread()
    while getattr(t, "do_run", True):
        time.sleep(1)
    shutdown_event = server.stop(0)
    LOGGER.info("Waiting for manager server to stop")
    shutdown_event.wait()
    LOGGER.info("Manager server stopped")
