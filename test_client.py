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

from __future__ import print_function

import grpc
import sys
import os
import time
import datetime
import json

#So the cartesi GRPC modules are in path
import sys
sys.path.insert(0,'machine-emulator/lib/grpc-interfaces/py')

import core_pb2
import cartesi_base_pb2
import core_pb2_grpc
import manager_high_pb2
import manager_high_pb2_grpc
import manager_low_pb2
import manager_low_pb2_grpc
import traceback
import argparse
#from IPython import embed

SLEEP_TIME = 5
DEFAULT_PORT = 50051
DEFAULT_ADD = 'localhost'

TEST_SESSION_ID = "test_new_session_id"
START = "start"
BACKING = "backing"
LENGTH = "length"
SHARED = "shared"
BOOTARGS = "bootargs"
LABEL = "label"

CONTAINER_SERVER = False

TEST_ROM = {
    BOOTARGS: "console=hvc0 rootfstype=ext2 root=/dev/mtdblock0 rw {} -- /bin/echo nice && ls /mnt",
    BACKING: "rom-linux.bin"
}

TEST_RAM = {
    LENGTH: 64 << 20, #2**26 or 67108864
    BACKING: "kernel.bin"
}

CONTAINER_BASE_PATH = "/root/host/"
NATIVE_BASE_PATH = "{}/test-files/".format(os.path.dirname(os.path.realpath(__file__)))

BACKING_TEST_DRIVE_FILEPATH = "rootfs.ext2"

TEST_DRIVES = [
    {
        START: 1 << 63, #2**63 or ~ 9*10**18
        LENGTH: 46223360,
        BACKING: BACKING_TEST_DRIVE_FILEPATH,
        SHARED: False,
        LABEL: "rootfs"
    }
]

def build_mtdparts_str(drives):

    mtdparts_str = "mtdparts="

    for i,drive in enumerate(drives):
        mtdparts_str += "flash.%d:-(%s)".format(i, drive[LABEL])

    return mtdparts_str

def make_new_session_request():
    files_dir = NATIVE_BASE_PATH
    if (CONTAINER_SERVER):
        files_dir = CONTAINER_BASE_PATH

    ram_msg = cartesi_base_pb2.RAM(length=TEST_RAM[LENGTH], backing=files_dir + TEST_RAM[BACKING])
    drives_msg = []
    for drive in TEST_DRIVES:
        drive_msg = cartesi_base_pb2.Drive(start=drive[START], length=drive[LENGTH], backing=files_dir + drive[BACKING],
                                           shared=drive[SHARED])
        drives_msg.append(drive_msg)
    bootargs_str = TEST_ROM[BOOTARGS].format(build_mtdparts_str(TEST_DRIVES))
    rom_msg = cartesi_base_pb2.ROM(bootargs=bootargs_str, backing=files_dir + TEST_ROM[BACKING])

    machine_msg = cartesi_base_pb2.MachineRequest(rom=rom_msg, ram=ram_msg, flash=drives_msg)
    return manager_high_pb2.NewSessionRequest(session_id=TEST_SESSION_ID, machine=machine_msg)

def make_new_session_run_request(session_id, final_cycles):
    return manager_high_pb2.SessionRunRequest(session_id=session_id, final_cycles=final_cycles)

def make_new_session_step_request(session_id, initial_cycle):
    return manager_high_pb2.SessionStepRequest(session_id=session_id, initial_cycle=initial_cycle)

def make_new_session_get_proof_request(session_id, address, log2_size):
    proof_req = cartesi_base_pb2.GetProofRequest(address=address, log2_size=log2_size)
    return manager_high_pb2.SessionGetProofRequest(session_id=session_id, target=proof_req)

def make_new_session_read_memory_request(session_id, mem_addr, data_length):
    read_mem_req = cartesi_base_pb2.ReadMemoryRequest(address=mem_addr, length=data_length)
    return manager_high_pb2.SessionReadMemoryRequest(session_id=session_id, position=read_mem_req)

def make_new_session_write_memory_request(session_id, mem_addr, data):
    write_mem_req = cartesi_base_pb2.WriteMemoryRequest(address=mem_addr, data=data)
    return manager_high_pb2.SessionWriteMemoryRequest(session_id=session_id, position=write_mem_req)

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

def dump_proof_to_json(proof):

    proof_dict = {
        'address': proof.address,
        'log2_size': proof.log2_size,
        'target_hash': "0x{}".format(proof.target_hash.content.hex()),
        'root_hash': "0x{}".format(proof.root_hash.content.hex()),
        'sibling_hashes' : []
    }

    for sibling in proof.sibling_hashes:
        proof_dict['sibling_hashes'].append("0x{}".format(sibling.content.hex()))

    return json.dumps(proof_dict, indent=4, sort_keys=True)

def dump_read_mem_response_to_json(read_mem_resp):
    resp_dict = {"read_content": {"data": "0x{}".format(read_mem_resp.read_content.data.hex())}}

    return json.dumps(resp_dict, indent=4, sort_keys=True)

def dump_write_mem_response_to_json(write_mem_resp):
    return json.dumps("{}".format(write_mem_resp), indent=4, sort_keys=True)

def port_number(port):
    try:
        int_port = int(port)
        if not(0 <= int_port <= 65535):
            raise argparse.ArgumentTypeError("Please provide a valid port from 0 to 65535")
    except:
        raise argparse.ArgumentTypeError("Please provide a valid port from 0 to 65535")
    return port

def get_args():
    parser = argparse.ArgumentParser(description='GRPC test client to the machine manager server')
    parser.add_argument('--address', '-a', dest='address', default=DEFAULT_ADD, help="Machine manager server address")
    parser.add_argument('--port', '-p', type=port_number, dest='port', default=DEFAULT_PORT, help="Machine manager server port")
    parser.add_argument('--container', '-c', action="store_true", dest="container_server", help="Fixes file references for when machine manager server is running from docker container")
    args = parser.parse_args()

    global CONTAINER_SERVER
    CONTAINER_SERVER = args.container_server

    return (args.address, args.port)

def run():
    responses = []
    srv_add, srv_port = get_args()
    conn_str = "{}:{}".format(srv_add, srv_port)
    print("Connecting to server in " + conn_str)
    with grpc.insecure_channel(conn_str) as channel:
        stub_low = manager_low_pb2_grpc.MachineManagerLowStub(channel)
        stub_high = manager_high_pb2_grpc.MachineManagerHighStub(channel)
        try:
            #NEW SESSION
            print("\n\n\nNEW SESSION TESTS\n\n\n")

            #Test new session
            print("Asking to create a new session")
            print("Server response:\n{}".format(stub_high.NewSession(make_new_session_request()).content.hex()))

            #RUN SESSION
            print("\n\n\nRUN SESSION TESTS\n\n\n")

            #Test run from pristine machine
            final_cycles = [0, 15, 30, 45, 60]
            print("Asking to run the machine for {} final cycle(s) ({})".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            #Test run with first final_cycle < machine cycle and first final_cycle > snapshot cycle to force rollback
            final_cycles = [30, 35, 40, 45]
            print("Asking to run the machine for {} final cycles(s) ({}), the 1st final cycle forces a rollback".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            #Test run with first final cycle < machine cycle and first final cycle < snapshot cycle to force recreating machine
            final_cycles = [1, 5, 10]
            print("Asking to run the machine for {} final cycle(s) ({}), the 1st final cycle forces recreating the machine".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            #Test run with first final cycle > machine cycle so no special effort should be needed
            final_cycles = [15]
            print("Asking to run the machine for {} final cycle(s) ({}), no special effort needed".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))

            #STEP SESSION
            print("\n\n\nSTEP SESSION TESTS\n\n\n")

            #Test step with initial cycle = 0, so step should happen on a new machine
            print("Test step with initial cycle = 0, so step should happen on a new machine")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            initial_cycle = 0
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_high.SessionStep(step_req))))

            #Test step with initial cycle < machine cycle and initial cycle > snapshot cycle to force rollback

            print("Test step with initial cycle < machine cycle and initial cycle > snapshot cycle to force rollback")
            final_cycles = [15,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            initial_cycle = 16
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_high.SessionStep(step_req))))

            #Test step with initial cycle < machine cycle and initial cycle < snapshot cycle to force recreating machine

            print("Test step with initial cycle < machine cycle and initial cycle < snapshot cycle to force recreating machine")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            initial_cycle = 1
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_high.SessionStep(step_req))))

            #Test step with initial cycle > machine cycle so no special effort should be needed

            print("Test step with initial cycle > machine cycle so no special effort should be needed")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            initial_cycle = 35
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_high.SessionStep(step_req))))

            #Test step with initial cycle = machine cycle, so step doesn't even have to make a dummy run to get into machine cycle = initial cycle

            print("Test step with initial cycle = machine cycle, so step doesn't even have to make a dummy run to get into machine cycle = initial cycle")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(stub_high.SessionRun(run_req))))
            initial_cycle = 30
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_high.SessionStep(step_req))))

            #Test get proof

            addr, log2_size = (288, 3)
            print("Asking for proof on address {} with log2_size {}".format(addr, log2_size))
            proof_req = make_new_session_get_proof_request(TEST_SESSION_ID, addr, log2_size)
            print("Server response:\n{}".format(dump_proof_to_json(stub_high.SessionGetProof(proof_req))))

            addr, log2_size = (288, 4)
            print("Asking for proof on address {} with log2_size {}".format(addr, log2_size))
            proof_req = make_new_session_get_proof_request(TEST_SESSION_ID, addr, log2_size)
            print("Server response:\n{}".format(dump_proof_to_json(stub_high.SessionGetProof(proof_req))))

            addr, log2_size = (1<<63, 3)
            print("Asking for proof on address {} with log2_size {}".format(addr, log2_size))
            proof_req = make_new_session_get_proof_request(TEST_SESSION_ID, addr, log2_size)
            print("Server response:\n{}".format(dump_proof_to_json(stub_high.SessionGetProof(proof_req))))

            #Test read and write mem
            mem_addr, data_length = (1<<63, 16)
            print("Asking to read memory starting on address {} for lenght {}".format(mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_high.SessionReadMemory(read_mem_req))))

            mem_addr, data = (1<<63, bytes.fromhex('aeafacaacaba')) #b'Hello')
            print("Asking to write memory starting on address {} with data {}".format(mem_addr, data))
            write_mem_req = make_new_session_write_memory_request(TEST_SESSION_ID, mem_addr, data)
            print("Server response:\n{}".format(dump_write_mem_response_to_json(stub_high.SessionWriteMemory(write_mem_req))))

            mem_addr, data_length = (1<<63, 16)
            print("Asking to read memory starting on address {} for lenght {}".format(mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_high.SessionReadMemory(read_mem_req))))

            #Eventually used for debugging: hook a ipython session in this point
            #embed()

        except Exception as e:
            print("An exception occurred:")
            print(e)
            print(type(e))

if __name__ == '__main__':
    start = time.time()
    print("Starting at {}".format(time.ctime()))
    run()
    print("Ending at {}".format(time.ctime()))
    delta = time.time() - start
    print("Took {} seconds to execute".format(datetime.timedelta(seconds=delta)))
