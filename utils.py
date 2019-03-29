import subprocess
import logging
import logging.config
import logging.handlers
import core_pb2_grpc
import cartesi_base_pb2
import manager_high_pb2
import traceback
import grpc

LOG_FILENAME = "manager.log"
UNIX = "unix"
TCP = "tcp"
SOCKET_TYPE = UNIX

def get_new_logger(name):
    return logging.getLogger(name)

def configure_log(logger):
    
    logger.setLevel(logging.DEBUG)
    
    #Setting format
    formatter = logging.Formatter('%(asctime)s %(thread)d %(levelname)-s %(name)s %(lineno)s - %(funcName)s: %(message)s')
    
    #File rotation log handler
    rotating_file_handler = logging.handlers.RotatingFileHandler(
              LOG_FILENAME, maxBytes=2**20, backupCount=5)
    rotating_file_handler.setFormatter(formatter)
    rotating_file_handler.setLevel(logging.DEBUG)
    
    #Stream log handler
    stream_handler = logging.StreamHandler()
    stream_handler.setLevel(logging.DEBUG)
    stream_handler.setFormatter(formatter)

    logger.addHandler(rotating_file_handler)
    logger.addHandler(stream_handler)
    
    return logger

def new_cartesi_machine_server(session_id):
    
    LOGGER.info("Creating a cartesi machine server with session_id '{}'".format(session_id))
    
    cmd_line = ["core/src/emulator/server",SOCKET_TYPE, session_id]
    LOGGER.debug("Executing {}".format(" ".join(cmd_line)))
    proc = subprocess.Popen(cmd_line, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    out, err = proc.communicate()
    if (proc.returncode == 0):
        LOGGER.info("Cartesi machine server creation process returned for session_id '{}'".format(session_id))
        LOGGER.debug("\nStdout:\n{}\nStderr:\n{}".format(out.decode("utf-8"), err.decode("utf-8")))
    else:
        err_msg = "Cartesi machine server creation process returned non-zero code for session_id '{}'".format(session_id)
        LOGGER.error(err_msg)
        LOGGER.error("\nStdout:\n{}\nStderr:\n{}".format(out.decode("utf-8"), err.decode("utf-8")))
        raise CartesiMachineServerException(err_msg)

def new_machine(session_id, address, machine_req):
    
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        response = stub.Machine(machine_req)
        LOGGER.debug("Cartesi machine created for session_id '{}'".format(session_id))
        
def shutdown_cartesi_machine_server(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        response = stub.Shutdown(cartesi_base_pb2.Void())
        LOGGER.debug("Cartesi machine server shutdown for session_id '{}'".format(session_id))
        
def get_machine_hash(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        LOGGER.debug("Asking for cartesi machine root hash for session_id '{}'".format(session_id))
        response = stub.GetRootHash(cartesi_base_pb2.Void())
        LOGGER.debug("Cartesi machine root hash retrieved for session_id '{}'".format(session_id))
        return response
    
def create_machine_snapshot(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        stub.Snapshot(cartesi_base_pb2.Void())
        LOGGER.debug("Cartesi machine snapshot created for session_id '{}'".format(session_id))        

def rollback_machine(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        stub.Rollback(cartesi_base_pb2.Void())
        LOGGER.debug("Cartesi machine rolledback for session_id '{}'".format(session_id))
    
def run_machine(session_id, address, c):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        response = stub.Run(cartesi_base_pb2.RunRequest(limit=c))
        LOGGER.debug("Cartesi machine ran for session_id '{}' and desired final cycle of {}".format(session_id, c))
        return response
    
def step_machine(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = core_pb2_grpc.MachineStub(channel)
        response = stub.Step(cartesi_base_pb2.Void())
        LOGGER.debug("Cartesi machine step complete for session_id '{}'".format(session_id))
        return response    
    
def make_session_run_result(summaries, hashes):
    return manager_high_pb2.SessionRunResult(summaries=summaries, hashes=hashes)

def make_session_step_result(access_log):
    return manager_high_pb2.SessionStepResult(log=access_log)

class CycleException(Exception):
    pass

class CartesiMachineServerException(Exception):
    pass

def validate_cycles(values):
    last_value = None
    
    #Checking if at least one value was passed
    if values:
        for value in values:
            if (value < 0):
                raise CycleException("Positive values expected, first offending value: {}".format(value))
            if last_value:
                if value < last_value:
                    raise CycleException("Provide cycle values in crescent order, received {} after {}".format(value, last_value)) 
            last_value = value
    else:
        raise CycleException("Provide a cycle value") 
          
#Initializing log
LOGGER = get_new_logger(__name__)
LOGGER = configure_log(LOGGER)
