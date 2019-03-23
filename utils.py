import subprocess
import logging
import logging.config
import logging.handlers
import core_pb2_grpc
import traceback
import grpc

LOG_FILENAME = "manager.log"

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
    
    cmd_line = ["core/src/emulator/server","unix", session_id]
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
        
#Initializing log
LOGGER = get_new_logger(__name__)
LOGGER = configure_log(LOGGER)