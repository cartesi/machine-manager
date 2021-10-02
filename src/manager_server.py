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

import signal
import time
import threading

import utils
from machine_manager import start_manager_server
from checkin_manager import start_checkin_server

# docker graceful shutdown, raise a KeyboardInterrupt in case of SIGTERM
def handle_sigterm(*args):
    raise KeyboardInterrupt()

signal.signal(signal.SIGTERM, handle_sigterm)

LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

LISTENING_ADDRESS = '127.0.0.1'
LISTENING_PORT = 50051
CHECKIN_PORT = 50052
SLEEP_TIME = 1


def shutdown_servers(registry_manager, manager_server, checkin_server):
    LOGGER.info("\nIssued to shut down")

    LOGGER.debug("Acquiring session registry global lock")
    #Acquiring lock to write on session registry
    with registry_manager.global_lock:
        LOGGER.debug("Session registry global lock acquired")
        registry_manager.shutting_down = True

    #Shutdown all active sessions servers
    for session_id in registry_manager.registry.keys():
        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with registry_manager.registry[session_id].lock:
            LOGGER.debug("Lock for session {} acquired".format(session_id))
            if (registry_manager.registry[session_id].address):
                utils.shutdown_cartesi_machine_server(session_id, registry_manager.registry[session_id].address)
                registry_manager.kill_session(session_id)

    manager_server.do_run = False
    checkin_server.do_run = False


def serve(args):
    #Importing the defective session registry if defective flag is set
    if args.defective:
        from defective_session_registry import SessionRegistryManager
    else:
        from session_registry import SessionRegistryManager
    session_registry_manager = SessionRegistryManager(
            '{}:0'.format(args.address),
            '{}:{}'.format(args.address, args.checkin))

    checkin_server = threading.Thread(target=start_checkin_server, args=(args, session_registry_manager, ))
    manager_server = threading.Thread(target=start_manager_server, args=(args, session_registry_manager, ))

    checkin_server.start()
    manager_server.start()

    try:
        while True:
            time.sleep(SLEEP_TIME)
    except KeyboardInterrupt:
        shutdown_servers(session_registry_manager, manager_server, checkin_server)
