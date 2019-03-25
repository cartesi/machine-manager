from concurrent import futures
import time
import math
import grpc
import sys
import traceback

#So the cartesi GRPC modules are in path
import sys
sys.path.insert(0,'cartesi-grpc/py')

import manager_low_pb2_grpc
import manager_low_pb2
import manager_high_pb2_grpc
import manager_high_pb2
import cartesi_base_pb2
import utils
from session_registry import SessionRegistryManager, SessionIdException, AddressException

LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

LISTENING_PORT = 50051
SLEEP_TIME = 5

class _MachineManagerHigh(manager_high_pb2_grpc.MachineManagerHighServicer):

    def __init__(self, session_registry_manager):
        self.session_registry_manager = session_registry_manager
    
    def NewSession(self, request, context):
        try:            
            session_id = request.session_id
            machine_req = request.machine
            LOGGER.debug("New session requested with session_id: {}".format(session_id))

            #Registering new session
            self.session_registry_manager.register_session(session_id)
            
            #Instantiate new cartesi machine server
            self.session_registry_manager.create_new_cartesi_machine_server(session_id)

            #Wait for the new server to communicate it's listening address
            self.session_registry_manager.wait_for_session_address_communication(session_id)

            #Communication received, create new cartesi machine
            self.session_registry_manager.create_machine(session_id, machine_req)

            #calculate cartesi machine initial hash
            initial_hash = self.session_registry_manager.get_machine_root_hash(session_id)
            
            #Create snapshot
            self.session_registry_manager.snapshot_machine(session_id)

            #Return the initial hash
            return initial_hash
    
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

    def SessionRun(self, request, context):
        return manager_high_pb2.SessinoRunResult()

    def SessionStep(self, request, context):
        return manager_high_pb2.SessionStepResult()

class _MachineManagerLow(manager_low_pb2_grpc.MachineManagerLowServicer):

    def __init__(self, session_registry_manager):
        self.session_registry_manager = session_registry_manager

    def CommunicateAddress (self, request, context):
        try:
            address = request.address
            session_id = request.session_id
            
            LOGGER.info("Received a CommunicateAddress request for session_id {} and address {}".format(session_id, address))
            
            self.session_registry_manager.register_address_for_session(session_id, address)
            
            #Returning
            return cartesi_base_pb2.Void()
    
        #No session with provided id
        except SessionIdException as e:
            LOGGER.error(e)
            context.set_details("{}".format(e))
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
        #Generic error catch
        except Exception as e:
            LOGGER.error("An exception occurred: {}\nTraceback: {}".format(e, traceback.format_exc()))            
            context.set_details('An exception with message "{}" was raised!'.format(e))
            context.set_code(grpc.StatusCode.UNKNOWN)        

def serve():
    session_registry_manager = SessionRegistryManager()
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    manager_high_pb2_grpc.add_MachineManagerHighServicer_to_server(_MachineManagerHigh(session_registry_manager),
                                                      server)
    manager_low_pb2_grpc.add_MachineManagerLowServicer_to_server(_MachineManagerLow(session_registry_manager),
                                                      server)

    server.add_insecure_port('[::]:{}'.format(LISTENING_PORT))
    server.start()
    LOGGER.info("Server started, listening on port {}".format(LISTENING_PORT))
    try:
        while True:
            time.sleep(SLEEP_TIME)
    except KeyboardInterrupt:
        LOGGER.info("\nShutting down")
        server.stop(0)


if __name__ == '__main__':
    serve()
