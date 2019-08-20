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

import subprocess
import logging
import logging.config
import logging.handlers
import core_pb2_grpc
import cartesi_base_pb2
import manager_high_pb2
import traceback
import grpc
import json

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

def new_cartesi_machine_server(session_id, manager_address):

    LOGGER.info("Creating a cartesi machine server with session_id '{}'".format(session_id))

    cmd_line = ["/opt/cartesi/bin/cartesi-machine-server", "-t", SOCKET_TYPE, "-s", session_id, "-m", manager_address]
    LOGGER.debug("Executing {}".format(" ".join(cmd_line)))
    proc = None
    try:
        proc = subprocess.Popen(cmd_line, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
        out, err = proc.communicate()
        LOGGER.debug("\nStdout:\n{}\nStderr:\n{}".format(out.decode("utf-8"), err.decode("utf-8")))
    except Exception as e:
        err_msg = "Cartesi machine server creation process failed for session_id '{}'".format(session_id)
        LOGGER.info(err_msg)
        if (proc):
            out, err = proc.communicate()
            LOGGER.debug("\nStdout:\n{}\nStderr:\n{}".format(out.decode("utf-8"), err.decode("utf-8")))
        raise CartesiMachineServerException(err_msg)
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

#Debugging functions

def dump_step_response_to_json(access_log):
    access_log_dict = {'accesses':[], 'notes':[], 'brackets':[]}

    for note in access_log.log.notes:
        access_log_dict['notes'].append(note)

    for bracket in access_log.log.brackets:
        access_log_dict['brackets'].append(
                {
                    'type':
                    cartesi_base_pb2._BRACKETNOTE_BRACKETNOTETYPE.values_by_number[bracket.type].name,
                    'where': bracket.where,
                    'text' : bracket.text
                })

    for access in access_log.log.accesses:
        access_dict = {
                    'read': "0x{}".format(access.read.content.hex()),
                    'written' : "0x{}".format(access.written.content.hex()),
                    'operation' : cartesi_base_pb2._ACCESSOPERATION.values_by_number[access.operation].name,
                    'proof' : {
                            'address': access.proof.address,
                            'log2_size': access.proof.log2_size,
                            'target_hash': "0x{}".format(access.proof.target_hash.content.hex()),
                            'root_hash': "0x{}".format(access.proof.root_hash.content.hex()),
                            'sibling_hashes' : []
                        }
                }

        for sibling in access.proof.sibling_hashes:
            access_dict['proof']['sibling_hashes'].append("0x{}".format(sibling.content.hex()))

        access_log_dict['accesses'].append(access_dict)

    return json.dumps(access_log_dict, indent=4, sort_keys=True)

def dump_step_response_to_file(access_log, open_dump_file):
    json_dump = dump_step_response_to_json(access_log)
    open_dump_file.write("\n\n" + '#'*80 + json_dump)

def dump_run_response_to_json(run_resp):
    resp_dict = {"summaries": [], "hashes": []}

    for val in run_resp.summaries:
        resp_dict["summaries"].append({
                                          'tohost': val.tohost,
                                          'mcycle': val.mcycle
                                      })
    for val in run_resp.hashes:
        resp_dict["hashes"].append("0x{}".format(val.content.hex()))

    return json.dumps(resp_dict, indent=4, sort_keys=True)

def dump_run_response_to_file(run_resp, open_dump_file):
    json_dump = dump_run_response_to_json(run_resp)
    open_dump_file.write("\n\n" + '#'*80 + json_dump)

#Initializing log
LOGGER = get_new_logger(__name__)
LOGGER = configure_log(LOGGER)
