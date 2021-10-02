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
import traceback
import argparse
import shutil

import cartesi_machine_pb2
import cartesi_machine_pb2_grpc
import machine_manager_pb2
import machine_manager_pb2_grpc

#from IPython import embed

SLEEP_TIME = 1
DEFAULT_PORT = 50051
DEFAULT_ADD = 'localhost'

TEST_SESSION_ID = "test_new_session_id"
TEST_SESSION_ID_2 = TEST_SESSION_ID +  "_2"
START = "start"
IMAGE_FILENAME = "image_filename"
LENGTH = "length"
SHARED = "shared"
BOOTARGS = "bootargs"
YIELD_MANUAL = "yield_manual"
YIELD_AUTOMATIC = "yield_automatic"
LABEL = "label"

CONTAINER_SERVER = False

TEST_ROM = {
    BOOTARGS: "console=hvc0 rootfstype=ext2 root=/dev/mtdblock0 rw {} -- for i in $(seq 0 5 1000000); do yield manual progress $i; done",
    IMAGE_FILENAME: "rom.bin"
}

TEST_RAM = {
    LENGTH: 64 << 20, #2**26 or 67108864
    IMAGE_FILENAME: "linux.bin"
}

CONTAINER_BASE_PATH = "/root/host/"
NATIVE_BASE_PATH = "{}/test-files/".format(os.path.dirname(os.path.realpath(__file__)))

IMAGE_FILENAME_TEST_DRIVE_FILEPATH = "rootfs.ext2"

TEST_DRIVES = [
    {
        START: 1 << 63, #2**63 or ~ 9*10**18
        LENGTH: 62914560,
        IMAGE_FILENAME: IMAGE_FILENAME_TEST_DRIVE_FILEPATH,
        SHARED: False,
        LABEL: "rootfs"
    }
]

HTIF_CONFIG = {
    YIELD_MANUAL: False,
    YIELD_AUTOMATIC: True
}

STORE_DIRECTORY = "store-test"

def build_mtdparts_str(drives):

    mtdparts_str = "mtdparts="

    for i,drive in enumerate(drives):
        mtdparts_str += "flash.{}:-({})".format(i, drive[LABEL])

    return mtdparts_str

def make_new_session_request(force=False):
    files_dir = NATIVE_BASE_PATH
    if (CONTAINER_SERVER):
        files_dir = CONTAINER_BASE_PATH

    ram_msg = cartesi_machine_pb2.RAMConfig(length=TEST_RAM[LENGTH], image_filename=files_dir + TEST_RAM[IMAGE_FILENAME])
    drives_msg = []
    print('='*80 + "\nFlash configs:")
    for drive in TEST_DRIVES:
        image_filename_path = files_dir + drive[IMAGE_FILENAME]
        print("New flash_drive config:\nDrive image_filename: {}\nStart: {}\nLength: {}\nShared: {}".format(image_filename_path, drive[START], drive[LENGTH], drive[SHARED]))
        drive_msg = cartesi_machine_pb2.MemoryRangeConfig(start=drive[START], length=drive[LENGTH], image_filename=image_filename_path,
                                           shared=drive[SHARED])
        drives_msg.append(drive_msg)
    bootargs_str = TEST_ROM[BOOTARGS].format(build_mtdparts_str(TEST_DRIVES))
    rom_image_filename = files_dir + TEST_ROM[IMAGE_FILENAME]
    print('='*80 + "\nRom config:\nBootargs: {}\nImage filename: {}".format(bootargs_str, rom_image_filename))
    rom_msg = cartesi_machine_pb2.ROMConfig(bootargs=bootargs_str, image_filename=rom_image_filename)
    print('='*80 + "\nHTIF config:\nYield automatic: {}\nYield manual: {}".format(HTIF_CONFIG[YIELD_AUTOMATIC], HTIF_CONFIG[YIELD_MANUAL]))
    htif_msg = cartesi_machine_pb2.HTIFConfig()
    setattr(htif_msg, "yield_automatic", HTIF_CONFIG[YIELD_AUTOMATIC])
    setattr(htif_msg, "yield_manual", HTIF_CONFIG[YIELD_MANUAL])

    machine_config = cartesi_machine_pb2.MachineConfig(rom=rom_msg, ram=ram_msg, flash_drive=drives_msg, htif=htif_msg)
    machine_msg = cartesi_machine_pb2.MachineRequest(config=machine_config)
    return machine_manager_pb2.NewSessionRequest(session_id=TEST_SESSION_ID, machine=machine_msg, force=force)

def make_new_session_from_store_request(session_id, directory):
    base_load_dir = NATIVE_BASE_PATH
    if (CONTAINER_SERVER):
        base_load_dir = CONTAINER_BASE_PATH

    machine_msg = cartesi_machine_pb2.MachineRequest(directory=base_load_dir + directory)
    return machine_manager_pb2.NewSessionRequest(session_id=session_id, machine=machine_msg)

def make_new_session_run_request(session_id, final_cycles):
    return machine_manager_pb2.SessionRunRequest(session_id=session_id, final_cycles=final_cycles)

def make_new_session_step_request(session_id, initial_cycle):
    log_type = cartesi_machine_pb2.AccessLogType(proofs=True, annotations=True)
    step_req = cartesi_machine_pb2.StepRequest(log_type=log_type)
    return machine_manager_pb2.SessionStepRequest(session_id=session_id, initial_cycle=initial_cycle, step_params=step_req)

def make_new_session_get_proof_request(session_id, cycle, address, log2_size):
    proof_req = cartesi_machine_pb2.GetProofRequest(address=address, log2_size=log2_size)
    return machine_manager_pb2.SessionGetProofRequest(session_id=session_id, cycle=cycle, target=proof_req)

def make_new_session_store_request(session_id, directory):
    base_store_dir = NATIVE_BASE_PATH
    if (CONTAINER_SERVER):
        base_store_dir = CONTAINER_BASE_PATH
    store_req = cartesi_machine_pb2.StoreRequest(directory=base_store_dir + directory)
    return machine_manager_pb2.SessionStoreRequest(session_id=session_id, store=store_req)

def make_new_session_read_memory_request(session_id, cycle, mem_addr, data_length):
    read_mem_req = cartesi_machine_pb2.ReadMemoryRequest(address=mem_addr, length=data_length)
    return machine_manager_pb2.SessionReadMemoryRequest(session_id=session_id, cycle=cycle, position=read_mem_req)

def make_new_session_write_memory_request(session_id, cycle, mem_addr, data):
    write_mem_req = cartesi_machine_pb2.WriteMemoryRequest(address=mem_addr, data=data)
    return machine_manager_pb2.SessionWriteMemoryRequest(session_id=session_id, cycle=cycle, position=write_mem_req)

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

def dump_proof_to_json(proof):

    proof_dict = {
        'target_address': proof.target_address,
        'log2_target_size': proof.log2_target_size,
        'target_hash': "0x{}".format(proof.target_hash.data.hex()),
        'log2_root_size': proof.log2_root_size,
        'root_hash': "0x{}".format(proof.root_hash.data.hex()),
        'sibling_hashes' : []
    }

    for sibling in proof.sibling_hashes:
        proof_dict['sibling_hashes'].append("0x{}".format(sibling.data.hex()))

    return json.dumps(proof_dict, indent=4, sort_keys=True)

def dump_store_response_to_json(store_resp):
    return json.dumps("{}".format(store_resp), indent=4, sort_keys=True)

def dump_read_mem_response_to_json(read_mem_resp):
    resp_dict = {"read_content": {"data": "0x{}".format(read_mem_resp.read_content.data.hex())}}

    return json.dumps(resp_dict, indent=4, sort_keys=True)

def dump_write_mem_response_to_json(write_mem_resp):
    return json.dumps("{}".format(write_mem_resp), indent=4, sort_keys=True)

def run_to_completion(stub, run_req):

    resp = stub.SessionRun(run_req)

    while (resp.WhichOneof("run_oneof") != "result"):
        print("Server progress response:\n{}\n".format(dump_run_response_to_json(resp)))
        time.sleep(SLEEP_TIME)
        resp = stub.SessionRun(run_req)

    return resp

def port_number(port):
    try:
        int_port = int(port)
        if not(0 <= int_port <= 65535):
            raise argparse.ArgumentTypeError("Please provide a valid port from 0 to 65535")
    except:
        raise argparse.ArgumentTypeError("Please provide a valid port from 0 to 65535")
    return port

def get_args():
    parser = argparse.ArgumentParser(description='gRPC test client to the machine manager server')
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

    #Rename the previous store directory if any to the same name + .old
    store_dir = NATIVE_BASE_PATH  + STORE_DIRECTORY

    if os.path.exists(store_dir):
        if os.path.isdir(store_dir):
            print("Moving old store directory to {}".format(store_dir + ".old"))
            #Removing old dir if exists, ignore_errors set to True not to raise exceptions in case it doesn't
            shutil.rmtree(store_dir + ".old", ignore_errors=True)
            shutil.move(store_dir, store_dir + ".old")
        else:
            print("The configured directory path for store already exists but it's a file: {}".format(store_dir))
            sys.exit(1)

    conn_str = "{}:{}".format(srv_add, srv_port)
    print("Connecting to server in " + conn_str)
    with grpc.insecure_channel(conn_str) as channel:
        stub_machine_man = machine_manager_pb2_grpc.MachineManagerStub(channel)
        try:
            #NEW SESSION
            print("\n\n\nNEW SESSION TESTS\n\n\n")

            #Test new session
            print("Asking to create a new session")
            print("Server response:\n{}".format(stub_machine_man.NewSession(make_new_session_request()).data.hex()))

            print("\nAsking to force create a new session with the same id as the previous one")
            print("Server response:\n{}".format(stub_machine_man.NewSession(make_new_session_request(force=True)).data.hex()))

            #RUN SESSION
            print("\n\n\nRUN SESSION TESTS\n\n\n")

            #Test run from pristine machine
            final_cycles = [0, 15, 30, 45, 60]
            print("Asking to run the machine for {} final cycle(s) ({})".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            #Test run with first final_cycle < machine cycle and first final_cycle > snapshot cycle to force rollback
            final_cycles = [30, 35, 40, 45]
            print("Asking to run the machine for {} final cycles(s) ({}), the 1st final cycle forces a rollback".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            #Test run with first final cycle < machine cycle and first final cycle < snapshot cycle to force recreating machine
            final_cycles = [1, 5, 10]
            print("Asking to run the machine for {} final cycle(s) ({}), the 1st final cycle forces recreating the machine".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            #Test run with first final cycle > machine cycle so no special effort should be needed
            final_cycles = [15]
            print("Asking to run the machine for {} final cycle(s) ({}), no special effort needed".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            #Test a long run
            final_cycles=[500000000]
            print("Asking to run the machine for {} final cycle(s) ({}), long run".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            #STEP SESSION
            print("\n\n\nSTEP SESSION TESTS\n\n\n")

            #Test step with initial cycle = 0, so step should happen on a new machine
            print("Test step with initial cycle = 0, so step should happen on a new machine")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            initial_cycle = 0
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_machine_man.SessionStep(step_req))))

            #Test step with initial cycle < machine cycle and initial cycle > snapshot cycle to force rollback

            print("Test step with initial cycle < machine cycle and initial cycle > snapshot cycle to force rollback")
            final_cycles = [15,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            initial_cycle = 16
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_machine_man.SessionStep(step_req))))

            #Test step with initial cycle < machine cycle and initial cycle < snapshot cycle to force recreating machine

            print("Test step with initial cycle < machine cycle and initial cycle < snapshot cycle to force recreating machine")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            initial_cycle = 1
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_machine_man.SessionStep(step_req))))

            #Test step with initial cycle > machine cycle so no special effort should be needed

            print("Test step with initial cycle > machine cycle so no special effort should be needed")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            initial_cycle = 35
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_machine_man.SessionStep(step_req))))

            #Test step with initial cycle = machine cycle, so step doesn't even have to make a dummy run to get into machine cycle = initial cycle

            print("Test step with initial cycle = machine cycle, so step doesn't even have to make a dummy run to get into machine cycle = initial cycle")
            final_cycles = [20,30]
            print("Asking to run the machine for {} final cycle(s) ({}), to prepare machine for that scenario".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))
            initial_cycle = 30
            print("Asking to step the machine on initial cycle ({})".format(initial_cycle))
            step_req = make_new_session_step_request(TEST_SESSION_ID, initial_cycle)
            print("Server response:\n{}".format(dump_step_response_to_json(stub_machine_man.SessionStep(step_req))))

            #Test get proof

            cycle, addr, log2_size = (30, 288, 3)
            print("Asking for proof on cyle {} for address {} with log2_size {}".format(cycle, addr, log2_size))
            proof_req = make_new_session_get_proof_request(TEST_SESSION_ID, cycle, addr, log2_size)
            print("Server response:\n{}".format(dump_proof_to_json(stub_machine_man.SessionGetProof(proof_req))))

            cyle, addr, log2_size = (30, 288, 4)
            print("Asking for proof on cyle {} for address {} with log2_size {}".format(cycle, addr, log2_size))
            proof_req = make_new_session_get_proof_request(TEST_SESSION_ID, cycle, addr, log2_size)
            print("Server response:\n{}".format(dump_proof_to_json(stub_machine_man.SessionGetProof(proof_req))))

            cyle, addr, log2_size = (0, 1<<63, 3)
            print("Asking for proof on cyle {} for address {} with log2_size {}".format(cycle, addr, log2_size))
            proof_req = make_new_session_get_proof_request(TEST_SESSION_ID, cycle, addr, log2_size)
            print("Server response:\n{}".format(dump_proof_to_json(stub_machine_man.SessionGetProof(proof_req))))

            #Test store and loading in new session
            print("Asking to store machine in directory {}".format(STORE_DIRECTORY))
            store_req = make_new_session_store_request(TEST_SESSION_ID, STORE_DIRECTORY)
            print("Server response:\n{}".format(dump_store_response_to_json(stub_machine_man.SessionStore(store_req))))

            print("Asking to create a new session from previous store")
            print("Server response:\n{}".format(stub_machine_man.NewSession(make_new_session_from_store_request(TEST_SESSION_ID_2, STORE_DIRECTORY)).data.hex()))

            final_cycles = [60]
            print("Asking to run the new machine for {} final cycle(s) ({})".format(len(final_cycles),final_cycles))
            run_req = make_new_session_run_request(TEST_SESSION_ID_2, final_cycles)
            print("Server response:\n{}".format(dump_run_response_to_json(run_to_completion(stub_machine_man, run_req))))

            #Test read and write mem
            cycle, mem_addr, data_length = (30, 1<<63, 16)
            print("Asking to read memory on cycle {} starting on address {} for lenght {}".format(cycle, mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, cycle, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_machine_man.SessionReadMemory(read_mem_req))))

            cycle, mem_addr, data_length = (0, 1<<63, 16)
            print("Asking to read memory on cycle {} starting on address {} for lenght {}".format(cycle, mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, cycle, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_machine_man.SessionReadMemory(read_mem_req))))

            cycle, mem_addr, data = (30, 1<<63, bytes.fromhex('aeafacaacaba'))
            print("Asking to write memory on cycle {} starting on address {} with data {}".format(cycle, mem_addr, data))
            write_mem_req = make_new_session_write_memory_request(TEST_SESSION_ID, cycle, mem_addr, data)
            print("Server response:\n{}".format(dump_write_mem_response_to_json(stub_machine_man.SessionWriteMemory(write_mem_req))))

            cycle, mem_addr, data_length = (30, 1<<63, 16)
            print("Asking to read memory on cycle {} starting on address {} for lenght {}".format(cycle, mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, cycle, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_machine_man.SessionReadMemory(read_mem_req))))

            cycle, mem_addr, data_length = (0, 1<<63, 16)
            print("Asking to read memory on cycle {} starting on address {} for lenght {}".format(cycle, mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, cycle, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_machine_man.SessionReadMemory(read_mem_req))))

            cycle, mem_addr, data_length = (30, 1<<63, 16)
            print("Asking to read memory on cycle {} starting on address {} for lenght {}".format(cycle, mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, cycle, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_machine_man.SessionReadMemory(read_mem_req))))

            cycle, mem_addr, data_length = (30, 1<<63, 16)
            print("Asking to read memory on cycle {} starting on address {} for lenght {}".format(cycle, mem_addr, data_length))
            read_mem_req = make_new_session_read_memory_request(TEST_SESSION_ID, cycle, mem_addr, data_length)
            print("Server response:\n{}".format(dump_read_mem_response_to_json(stub_machine_man.SessionReadMemory(read_mem_req))))

            #Recreating the session once more
            print("Asking to force create a new session with the same id as the previous one")
            print("Server response:\n{}".format(stub_machine_man.NewSession(make_new_session_request(force=True)).data.hex()))

            #Eventually used for debugging: hook a ipython session in this point
            #embed()

        except Exception as e:
            print("An exception occurred:")
            print(e)
            print(type(e))
            sys.exit(1)

if __name__ == '__main__':
    start = time.time()
    print("Starting at {}".format(time.ctime()))
    run()
    print("Ending at {}".format(time.ctime()))
    delta = time.time() - start
    print("Took {} seconds to execute".format(datetime.timedelta(seconds=delta)))
