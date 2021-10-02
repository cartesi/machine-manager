"""
Copyright 2021 Cartesi Pte. Ltd.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
"""

from concurrent import futures
import grpc
import traceback
import time
import threading

import cartesi_machine_checkin_pb2_grpc
import cartesi_machine_pb2
import utils


LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

class _CheckinMachineManager(cartesi_machine_checkin_pb2_grpc.MachineCheckInServicer):
    def __init__(self, session_registry_manager):
        self.registry_manager = session_registry_manager

    def CheckIn(self, request, context):
        session_id = request.session_id
        address = request.address
        try:
            with self.registry_manager.registry[session_id].checkin_lock:
                LOGGER.info("Checkin requested for session_id {} and address {}".format(session_id, address))
                self.registry_manager.register_address_for_session(session_id, address)
                self.registry_manager.registry[session_id].checkin_cond.notify()
                return cartesi_machine_pb2.Void()

        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)


def start_checkin_server(args, registry_manager):
    checkin_address = '{}:{}'.format(args.address, args.checkin)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    cartesi_machine_checkin_pb2_grpc.add_MachineCheckInServicer_to_server(_CheckinMachineManager(registry_manager),
                                                      server)
    server.add_insecure_port(checkin_address)
    server.start()
    LOGGER.info("Checkin service started, listening on address {}".format(checkin_address))

    t = threading.currentThread()
    while getattr(t, "do_run", True):
        time.sleep(1)
    shutdown_event = server.stop(0)
    LOGGER.info("Waiting for checkin server to stop")
    shutdown_event.wait()
    LOGGER.info("Checkin server stopped")
