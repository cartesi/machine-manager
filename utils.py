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
import traceback
import grpc
import json
import time
import os

import cartesi_machine_pb2_grpc
import cartesi_machine_pb2
import machine_manager_pb2

LOG_FILENAME = "manager.log"

RUN_CYCLES_BATCH_SIZE = 10**7

MAX_CONNECTION_ATTEMPTS = 10
SLEEP_TIME = 1

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

def new_cartesi_machine_server(session_id, server_address):

    LOGGER.info("Creating a cartesi machine server with session_id '{}'".format(session_id))

    cmd_line = ["/opt/cartesi/bin/cartesi-machine-server", server_address]
    LOGGER.debug("Executing {}".format(" ".join(cmd_line)))
    proc = None
    try:
        proc = subprocess.Popen(cmd_line, stderr=subprocess.PIPE, stdout=subprocess.PIPE, env=os.environ)
        proc.poll()
    except Exception as e:
        err_msg = "Cartesi machine server creation process failed for session_id '{}'".format(session_id)
        LOGGER.info(err_msg)
        if (proc):
            out, err = proc.communicate()
            LOGGER.debug("\nStdout:\n{}\nStderr:\n{}".format(out.decode("utf-8"), err.decode("utf-8")))
        raise CartesiMachineServerException(err_msg)
    #The process should run in foreground and not terminated
    if (proc.returncode):
        err_msg = "Cartesi machine server creation process returned too soon for session_id '{}'".format(session_id)
        LOGGER.error(err_msg)
        if (proc):
            out, err = proc.communicate()
            LOGGER.debug("\nStdout:\n{}\nStderr:\n{}".format(out.decode("utf-8"), err.decode("utf-8")))
        raise CartesiMachineServerException(err_msg)

    LOGGER.info("Cartesi machine server created for session_id '{}'".format(session_id))

def new_machine(session_id, address, machine_req):

    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.Machine(machine_req)
        LOGGER.debug("Cartesi machine created for session_id '{}'".format(session_id))

def shutdown_cartesi_machine_server(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.Shutdown(cartesi_machine_pb2.Void())
        LOGGER.debug("Cartesi machine server shutdown for session_id '{}'".format(session_id))

def get_machine_hash(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        LOGGER.debug("Asking for cartesi machine root hash for session_id '{}'".format(session_id))
        response = stub.GetRootHash(cartesi_machine_pb2.Void())
        LOGGER.debug("Cartesi machine root hash retrieved for session_id '{}'".format(session_id))
        return response.hash

def create_machine_snapshot(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        stub.Snapshot(cartesi_machine_pb2.Void())
        LOGGER.debug("Cartesi machine snapshot created for session_id '{}'".format(session_id))

def rollback_machine(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        stub.Rollback(cartesi_machine_pb2.Void())
        LOGGER.debug("Cartesi machine rolledback for session_id '{}'".format(session_id))

def run_machine(session_id, session_context, desired_cycle):
    ''' This function must be called only when the lock for the given session
        is held by the caller
    '''

    current_cycle = session_context.cycle
    LOGGER.debug("Current cycle: {}\nDesired cycle: {}".format(current_cycle, desired_cycle))

    if (desired_cycle < current_cycle):
        raise ValueError("The given desired_cycle must not be smaller than the current_cycle")
    response = None

    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, session_context.address))
    with grpc.insecure_channel(session_context.address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)

        #Setting cycle for run batch
        target_cycle = session_context.cycle + RUN_CYCLES_BATCH_SIZE
        #If it`s beyond the desired cycle, truncate
        if (target_cycle > desired_cycle):
            target_cycle = desired_cycle

        #Run loop
        while (True):
            #Run
            LOGGER.debug("Running cartesi machine for session id {} with target cycle of {}, current cycle is {}".format(session_id, target_cycle, session_context.cycle))
            response = stub.Run(cartesi_machine_pb2.RunRequest(limit=target_cycle))

            #Update tracked cycle and updated_at timestamp in the session context
            session_context.cycle = response.mcycle
            session_context.updated_at = time.time()

            LOGGER.debug("Updated cycle of session '{}' to {}".format(session_id, response.mcycle))

            #Checking if machine halted
            if response.iflags_h:
                #Storing the halting cycle in session context to use in progress calculations
                session_context.halt_cycle = session_context.cycle
                LOGGER.debug("Session {} halted with payload {}".format(session_id, int.from_bytes(response.tohost.to_bytes(8, 'big')[2:], byteorder='big')))
                break
            #Checking if the machine yielded
            elif response.iflags_y:
                #Parsing tohost to see if a progress command was given
                #The command is the second byte in the tohost 8bytes register
                cmd = response.tohost.to_bytes(8, 'big')[1]
                payload = int.from_bytes(response.tohost.to_bytes(8, 'big')[2:], byteorder='big')

                if (cmd==0):
                    #It was a progress command, storing the progress
                    session_context.app_progress = payload
                    LOGGER.debug("New progress for session {}: {}".format(session_id, payload))

                    #Reset IflagsY to resume
                    stub.ResetIflagsY(cartesi_machine_pb2.Void())
                else:
                    #Wasn't a progress command, just logging
                    LOGGER.debug("Session {} yielded with command {} and payload {}".format(session_id, cmd, payload))
            else:
                #The machine reached the target_cycle, setting next one if it wasn't the desired cycle
                if target_cycle == desired_cycle:
                    #It was, break the loop
                    break

                #It wasn't, set the next target cycle
                target_cycle += RUN_CYCLES_BATCH_SIZE

                #If it`s beyond the desired cycle, truncate
                if (target_cycle > desired_cycle):
                    target_cycle = desired_cycle

        LOGGER.debug("Cartesi machine ran for session_id '{}' and desired final cycle of {}, current cycle is {}".format(session_id, desired_cycle, session_context.cycle))
        return response

def step_machine(session_id, address, step_params):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.Step(step_params)
        LOGGER.debug("Cartesi machine step complete for session_id '{}'".format(session_id))
        return response.log

def store_machine(session_id, address, store_req):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.Store(store_req)
        LOGGER.debug("Stored Cartesi machine for session_id '{}', desired directory '{}'".format(session_id, store_req.directory))
        return response

def read_machine_memory(session_id, address, read_mem_req):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.ReadMemory(read_mem_req)
        LOGGER.debug("Cartesi machine memory read for session_id '{}', desired mem address {} and length {}".format(session_id, read_mem_req.address, read_mem_req.length))
        return response

def write_machine_memory(session_id, address, write_mem_req):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.WriteMemory(write_mem_req)
        LOGGER.debug("Cartesi machine memory written for session_id '{}', desired mem address {} and data {}".format(session_id, write_mem_req.address, write_mem_req.data))
        return response

def get_machine_proof(session_id, address, proof_req):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        stub = cartesi_machine_pb2_grpc.MachineStub(channel)
        response = stub.GetProof(proof_req)
        LOGGER.debug("Got Cartesi machine proof for session_id '{}', desired mem address {} and log2_size {}".format(session_id, proof_req.address, proof_req.log2_size))
        return response


def wait_for_server_availability(session_id, address):
    LOGGER.debug("Connecting to cartesi machine server from session '{}' in address '{}'".format(session_id, address))
    with grpc.insecure_channel(address) as channel:
        retry = 0
        while retry < MAX_CONNECTION_ATTEMPTS:
            try:
                stub = cartesi_machine_pb2_grpc.MachineStub(channel)
                response = stub.GetVersion(cartesi_machine_pb2.Void())
                LOGGER.debug("Cartesi machine server version for session_id '{}' is '{}'".format(session_id, response))
                break
            except Exception:
                LOGGER.warning("Cartesi machine server for session_id '{}' is not yet ready".format(session_id))
                retry += 1
                time.sleep(SLEEP_TIME)

        if retry == MAX_CONNECTION_ATTEMPTS:
            err_msg = "Cartesi machine server for session_id '{}' reached max connection attempts {}".format(session_id, MAX_CONNECTION_ATTEMPTS)
            LOGGER.error(err_msg)
            raise CartesiMachineServerException(err_msg)

def make_session_run_result(summaries, hashes):
    return machine_manager_pb2.SessionRunResponse(result=machine_manager_pb2.SessionRunResult(summaries=summaries, hashes=hashes))

def make_session_step_result(access_log):
    return machine_manager_pb2.SessionStepResponse(log=access_log)

def make_session_read_memory_result(read_mem_resp):
    return machine_manager_pb2.SessionReadMemoryResponse(read_content=read_mem_resp)

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
    access_log_dict = {'log_type': {}, 'accesses':[], 'notes':[], 'brackets':[]}

    access_log_dict['log_type']['proofs'] = access_log.log.log_type.proofs
    access_log_dict['log_type']['annotations'] = access_log.log.log_type.annotations

    for note in access_log.log.notes:
        access_log_dict['notes'].append(note)

    for bracket in access_log.log.brackets:
        access_log_dict['brackets'].append(
                {
                    'type':
                    cartesi_machine_pb2._BRACKETNOTE_BRACKETNOTETYPE.values_by_number[bracket.type].name,
                    'where': bracket.where,
                    'text' : bracket.text
                })

    for access in access_log.log.accesses:
        access_dict = {
                    'read': "0x{}".format(access.read.hex()),
                    'written' : "0x{}".format(access.written.hex()),
                    'address': access.address,
                    'log2_size': access.log2_size,
                    'operation' : cartesi_machine_pb2._ACCESSTYPE.values_by_number[access.type].name,
                    'proof' : {
                            'address': access.address,
                            'log2_size': access.log2_size,
                            'target_hash': "0x{}".format(access.proof.target_hash.data.hex()),
                            'root_hash': "0x{}".format(access.proof.root_hash.data.hex()),
                            'sibling_hashes' : []
                        }
                }

        for sibling in access.proof.sibling_hashes:
            access_dict['proof']['sibling_hashes'].append("0x{}".format(sibling.data.hex()))

        access_log_dict['accesses'].append(access_dict)

    return json.dumps(access_log_dict, indent=4, sort_keys=True)

def dump_step_response_to_file(access_log, open_dump_file):
    json_dump = dump_step_response_to_json(access_log)
    open_dump_file.write("\n\n" + '#'*80 + json_dump)

def dump_run_response_to_json(run_resp):
    resp_dict = None

    #Checking which of the oneof fields were set
    oneof_fieldname = run_resp.WhichOneof("run_oneof")

    if oneof_fieldname == "result":
        resp_dict = {"summaries": [], "hashes": []}

        for val in run_resp.result.summaries:
            resp_dict["summaries"].append({
                                          'tohost': val.tohost,
                                          'mcycle': val.mcycle
                                      })
        for val in run_resp.result.hashes:
            resp_dict["hashes"].append("0x{}".format(val.data.hex()))

    elif oneof_fieldname == "progress":
        resp_dict = {
                "progress": run_resp.progress.progress,
                "application_progress": run_resp.progress.application_progress,
                "updated_at": run_resp.progress.updated_at,
                "cycle": run_resp.progress.cycle
        }

    return json.dumps(resp_dict, indent=4, sort_keys=True)

def dump_run_response_to_file(run_resp, open_dump_file):
    json_dump = dump_run_response_to_json(run_resp)
    open_dump_file.write("\n\n" + '#'*80 + json_dump)

def dump_get_proof_response_to_json(proof_resp):

    proof = proof_resp.proof
    resp_dict = {
            'proof': {
                'address': proof.address,
                'log2_size': proof.log2_size,
                'target_hash': "0x{}".format(proof.target_hash.data.hex()),
                'root_hash': "0x{}".format(proof.root_hash.data.hex()),
                'sibling_hashes' : []
            }
    }

    for sibling in proof.sibling_hashes:
        resp_dict['proof']['sibling_hashes'].append("0x{}".format(sibling.data.hex()))

    return json.dumps(resp_dict, indent=4, sort_keys=True)

def dump_read_mem_response_to_json(read_mem_resp):
    resp_dict = {"data": "0x{}".format(read_mem_resp.read_content.data.hex())}

    return json.dumps(resp_dict, indent=4, sort_keys=True)

def dump_write_mem_response_to_json(write_mem_resp):
    return json.dumps("{}".format(write_mem_resp), indent=4, sort_keys=True)

#Initializing log
LOGGER = get_new_logger(__name__)
LOGGER = configure_log(LOGGER)
