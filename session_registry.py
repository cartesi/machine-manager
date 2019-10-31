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

from threading import Lock, Event
import utils
import time

WAIT_SERVER_TIME = 2

LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

class AddressException(Exception):
    pass

class SessionIdException(Exception):
    pass

class RollbackException(Exception):
    pass

class SessionRegistryManager:

    def __init__(self, manager_address):
        self.global_lock = Lock()
        self.registry = {}
        self.shutting_down = False
        self.manager_address = manager_address;

    def new_session(self, session_id, machine_req):
        #Registering new session
        self.register_session(session_id)

        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:
            #Instantiate new cartesi machine server
            self.create_new_cartesi_machine_server(session_id)

            #Communication received, create new cartesi machine
            self.create_machine(session_id, machine_req)

            #calculate cartesi machine initial hash
            initial_hash = self.get_machine_root_hash(session_id)

            #Create snapshot
            self.snapshot_machine(session_id)

        LOGGER.debug("Released lock for session {}".format(session_id))

        return initial_hash

    def run_session(self, session_id, final_cycles):

        summaries = []
        hashes = []
        desired_cycles = [c for c in final_cycles]

        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:

            first_c = desired_cycles.pop(0)

            #Checking machine cycle is after first required cycle
            if (self.registry[session_id].cycle > first_c):
                #It is, checking if there is a snapshot image
                if (self.registry[session_id].snapshot_cycle != None):
                    #There is, checking if snapshot cycle is before or after required cycle
                    if (self.registry[session_id].snapshot_cycle <= first_c):
                        #It is, rolling back
                        self.rollback_machine(session_id)
                    else:
                        #It isn't, recreating machine from scratch
                        self.recreate_machine(session_id)
                else:
                    #There isn't, recreating machine from scratch
                    self.recreate_machine(session_id)

            #Make execute and make machine snapshot
            summaries.append(self.run_and_update_registry_cycle(session_id, first_c))
            self.snapshot_machine(session_id)

            #Getting hash
            hashes.append(self.get_machine_root_hash(session_id))

            #Executing additional runs on given final_cycles
            for c in desired_cycles:
                summaries.append(self.run_and_update_registry_cycle(session_id, c))
                hashes.append(self.get_machine_root_hash(session_id))

        run_result = utils.make_session_run_result(summaries, hashes)

        #Checking if log level is DEBUG or more detailed since building the
        #debug info is expensive
        if LOGGER.getEffectiveLevel() <= utils.logging.DEBUG:
            LOGGER.debug(utils.dump_run_response_to_json(run_result))

        #Returning SessionRunResult
        return run_result

    def step_session(self, session_id, initial_cycle):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        step_result = None

        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:

            #First, in case the machine cycle is not the desired step initial cycle, we must put the machine in desired
            #step initial cycle so we can then step and retrieve the access log of that specific cycle
            if (self.registry[session_id].cycle != initial_cycle):
                #It is different, putting machine in initial_cycle

                #Checking machine cycle is after required cycle
                if (self.registry[session_id].cycle > initial_cycle):
                    #It is, checking if there is a snapshot image
                    if (self.registry[session_id].snapshot_cycle != None):
                        #There is, checking if snapshot cycle is before or after required cycle
                        if (self.registry[session_id].snapshot_cycle <= initial_cycle):
                            #It is, rolling back
                            self.rollback_machine(session_id)
                        else:
                            #It isn't, recreating machine from scratch
                            self.recreate_machine(session_id)
                    else:
                        #There isn't, recreating machine from scratch
                        self.recreate_machine(session_id)

                #Execute up to initial_cycle if initial_cycle > 0
                if (initial_cycle > 0):
                    self.run_and_update_registry_cycle(session_id, initial_cycle)

            #The machine is in initial_cycle, stepping now
            step_result =  utils.make_session_step_result(self.step_and_update_registry_cycle(session_id))

        #Checking if log level is DEBUG or more detailed since building the
        #debug info is expensive
        if LOGGER.getEffectiveLevel() <= utils.logging.DEBUG:
            LOGGER.debug(utils.dump_step_response_to_json(step_result))

        #Returning SessionStepResult
        return step_result

    def session_read_mem(self, session_id, read_mem_req):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        read_result = None

        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:

            #Read desired memory position
            read_result =  utils.make_session_read_memory_result(utils.read_machine_memory(session_id, self.registry[session_id].address, read_mem_req))

        #Checking if log level is DEBUG or more detailed since building the
        #debug info is expensive
        if LOGGER.getEffectiveLevel() <= utils.logging.DEBUG:
            LOGGER.debug(utils.dump_read_mem_response_to_json(read_result))

        #Returning SessionReadMemoryResult
        return read_result

    def session_write_mem(self, session_id, write_mem_req):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        write_result = None

        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:

            #Write to desired memory position
            write_result =  utils.write_machine_memory(session_id, self.registry[session_id].address, write_mem_req)

        #Checking if log level is DEBUG or more detailed since building the
        #debug info is expensive
        if LOGGER.getEffectiveLevel() <= utils.logging.DEBUG:
            LOGGER.debug(utils.dump_write_mem_response_to_json(write_result))

        #Returning CartesiBase Void
        return write_result

    def session_get_proof(self, session_id, proof_req):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        proof_result = None

        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:

            #Getting required proof
            proof_result =  utils.get_machine_proof(session_id, self.registry[session_id].address, proof_req)

        #Checking if log level is DEBUG or more detailed since building the
        #debug info is expensive
        if LOGGER.getEffectiveLevel() <= utils.logging.DEBUG:
            LOGGER.debug(utils.dump_get_proof_response_to_json(proof_result))

        #Returning CartesiBase Proof
        return proof_result


    """
    Here starts the "internal" API, use the methods bellow taking the right precautions such as holding a lock a session
    """




    def register_session(self, session_id):
        #Acquiring global lock and releasing it when completed
        LOGGER.debug("Acquiring session registry global lock")
        with self.global_lock:
            LOGGER.debug("Lock acquired")
            if session_id in self.registry.keys():
                #Session id already in use
                err_msg = "Trying to register a session with a session_id that already exists: {}".format(session_id)
                LOGGER.error(err_msg)
                raise SessionIdException(err_msg)
            else:
                #Registering new session
                self.registry[session_id] = CartesiSession(session_id)
                LOGGER.info("New session registered: {}".format(session_id))

    def wait_for_session_address_communication(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id '{}'".format(session_id))

        LOGGER.debug("Waiting for session address communication for session_id '{}'".format(session_id))
        self.registry[session_id].address_set_event.wait()
        LOGGER.debug("Address for session_id '{}' communicated: {}".format(session_id, self.registry[session_id].address))
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            LOGGER.debug("Cleaning address set event")
            self.registry[session_id].address_set_event.clear()
            LOGGER.debug("Address set event cleaned")

    def register_address_for_session(self, session_id, address):

        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id '{}'".format(session_id))

        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            #Registering address and notifying that it was set
            self.registry[session_id].address = address
            self.registry[session_id].address_set_event.set()
            LOGGER.debug("Address for session '{}' set to {}".format(session_id, address))

    def create_new_cartesi_machine_server(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id '{}'".format(session_id))
        if (self.registry[session_id].address):
            raise AddressException("Address already set for server with session_id '{}'".format(session_id))

        LOGGER.debug("Creating new cartesi machine server for session_id '{}'".format(session_id))
        utils.new_cartesi_machine_server(session_id, self.manager_address)
        LOGGER.debug("Server created for session '{}'".format(session_id))

        #Wait for the new server to communicate it's listening address
        self.wait_for_session_address_communication(session_id)

    def create_machine(self, session_id, machine_req):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        LOGGER.debug("Issuing server to create a new machine for session '{}'".format(session_id))
        utils.new_machine(session_id, self.registry[session_id].address, machine_req)
        LOGGER.debug("Executed creating a new machine for session '{}'".format(session_id))
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].creation_machine_req = machine_req
            LOGGER.debug("Saved on registry machine request used to create session '{}'".format(session_id))

    def get_machine_root_hash(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        LOGGER.debug("Issuing server to get machine root hash for session '{}'".format(session_id))
        root_hash = utils.get_machine_hash(session_id, self.registry[session_id].address)
        LOGGER.debug("Executed getting machine root hash for session '{}': 0x{}".format(session_id, root_hash.content.hex()))
        return root_hash

    def snapshot_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        LOGGER.debug("Issuing server to create machine snapshot for session '{}'".format(session_id))
        utils.create_machine_snapshot(session_id, self.registry[session_id].address)
        LOGGER.debug("Executed creating machine snapshot for session '{}'".format(session_id))
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].snapshot_cycle = self.registry[session_id].cycle
            LOGGER.debug("Updated snapshot cycle of session '{}' to {}".format(session_id, self.registry[session_id].cycle))

        #Wait for the new server to communicate it's listening address
        self.wait_for_session_address_communication(session_id)

    def rollback_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
        if (self.registry[session_id].snapshot_cycle == None):
            raise RollbackException("There is no snapshot to rollback to for the cartesi machine with session_id '{}'".format(session_id))

        LOGGER.debug("Issuing server to rollback machine for session '{}'".format(session_id))
        utils.rollback_machine(session_id, self.registry[session_id].address)
        LOGGER.debug("Executed rollingback machine for session '{}'".format(session_id))
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].cycle = self.registry[session_id].snapshot_cycle
            self.registry[session_id].snapshot_cycle = None
            LOGGER.debug("Updated cycle of session '{}' to {} and cleared snapshot cycle".format(session_id, self.registry[session_id].cycle))

        #Wait for the new server to communicate it's listening address
        self.wait_for_session_address_communication(session_id)

    def recreate_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))

        #Shutting down old server if any
        if (self.registry[session_id].address):
            utils.shutdown_cartesi_machine_server(session_id, self.registry[session_id].address)

        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            #Cleaning old server session data
            self.registry[session_id].address = None
            self.registry[session_id].cycle = 0
            self.registry[session_id].snapshot_cycle = None

        LOGGER.debug("Cleaned old server session data for session '{}'".format(session_id))

        #Instantiate new cartesi machine server
        self.create_new_cartesi_machine_server(session_id)

        #Communication received, create new cartesi machine using saved parameters
        self.create_machine(session_id, self.registry[session_id].creation_machine_req)

    def run_and_update_registry_cycle(self, session_id, c):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        #Running cartesi machine
        result = utils.run_machine(session_id, self.registry[session_id].address, c)

        #Updating cartesi session cycle
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].cycle = c

        LOGGER.debug("Updated cycle of session '{}' to {}".format(session_id, self.registry[session_id].cycle))

        return result

    def step_and_update_registry_cycle(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))

        #Stepping cartesi machine
        result = utils.step_machine(session_id, self.registry[session_id].address)

        #Updating cartesi session cycle
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].cycle += 1

        LOGGER.debug("Updated cycle of session '{}' to {}".format(session_id, self.registry[session_id].cycle))

        return result


class CartesiSession:

    def __init__(self, session_id):
        self.id = session_id
        self.lock = Lock()
        self.address = None
        self.address_set_event = Event()
        self.cycle = 0
        self.snapshot_cycle = None
        self.creation_machine_req = None




