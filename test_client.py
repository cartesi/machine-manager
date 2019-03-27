from __future__ import print_function

import grpc
import sys
import os
import time
import datetime

#So the cartesi GRPC modules are in path
import sys
sys.path.insert(0,'cartesi-grpc/py')

import core_pb2
import cartesi_base_pb2
import core_pb2_grpc
import manager_high_pb2
import manager_high_pb2_grpc
import manager_low_pb2
import manager_low_pb2_grpc
import traceback
import argparse
from IPython import embed

SLEEP_TIME = 5

TEST_SESSION_ID = "test_new_session_id"
START = "start" 
BACKING = "backing"
LENGTH = "length"
SHARED = "shared"
LABEL = "label"
BOOTARGS = "bootargs"

TEST_ROM = {
    BOOTARGS: "console=hvc0 rootfstype=ext2 root=/dev/mtdblock0 rw -- /bin/echo nice"
}

TEST_RAM = {
    LENGTH: 64 << 20, #2**26 or 67108864
    BACKING: "/home/carlo/crashlabs/core/src/emulator/kernel.bin"
    
}

BACKING_TEST_DRIVE_FILEPATH = "/home/carlo/crashlabs/core/src/emulator/rootfs.ext2"

TEST_DRIVES = [
    {
        START: 1 << 63, #2**63 or ~ 9*10**18
        LENGTH: os.path.getsize(BACKING_TEST_DRIVE_FILEPATH),
        BACKING: BACKING_TEST_DRIVE_FILEPATH,
        SHARED: False,
        LABEL: "root filesystem"
    }
]

def make_new_session_request():
    rom_msg = cartesi_base_pb2.ROM(bootargs=TEST_ROM[BOOTARGS])
    ram_msg = cartesi_base_pb2.RAM(length=TEST_RAM[LENGTH], backing=TEST_RAM[BACKING])
    drives_msg = []
    for drive in TEST_DRIVES:
        drive_msg = cartesi_base_pb2.Drive(start=drive[START], length=drive[LENGTH], backing=drive[BACKING], 
                                           shared=drive[SHARED], label=drive[LABEL])
        drives_msg.append(drive_msg)
    machine_msg = cartesi_base_pb2.MachineRequest(rom=rom_msg, ram=ram_msg, flash=drives_msg)
    return manager_high_pb2.NewSessionRequest(session_id=TEST_SESSION_ID, machine=machine_msg)

def make_new_session_run_request(session_id, times):
    return manager_high_pb2.SessionRunRequest(session_id=session_id, times=times)

def make_new_session_step_request(session_id, time):
    return manager_high_pb2.SessionStepRequest(session_id=session_id, time=time)

def address(add):
    #TODO: validate address
    return add

def port_number(port):
    try:
        int_port = int(port)
        if not(0 <= int_port <= 65535):
            raise argparse.ArgumentTypeError("Please provide a valid port from 0 to 65535")
    except:
        raise argparse.ArgumentTypeError("Please provide a valid port from 0 to 65535")
    return port
   
def get_args():
    parser = argparse.ArgumentParser(description='GRPC client to the high level emulator API (core manager)')
    parser.add_argument('server_add', type=address, help="Core manager GRPC server address")
    parser.add_argument('server_port', type=port_number, help="Core manager GRPC server port")
    args = parser.parse_args()

    srv_add = "localhost"
    srv_port = "50051"
    
    if args.server_add:
        srv_add = args.server_add

    if args.server_port:
        srv_port = args.server_port

    return (srv_add, srv_port) 

def run():
    responses = []
    srv_add, srv_port = get_args()
    conn_str = srv_add + ':' + srv_port
    print("Connecting to server in " + conn_str)
    with grpc.insecure_channel(conn_str) as channel:
        stub_low = manager_low_pb2_grpc.MachineManagerLowStub(channel)
        stub_high = manager_high_pb2_grpc.MachineManagerHighStub(channel)
        try:
            #NEW SESSION
            print("\n\n\nNEW SESSION TESTS\n\n\n")
            
            #Test new session
            print("Asking to create a new session")
            print("Server response:\n{}".format(stub_high.NewSession(make_new_session_request())))
            
            #RUN SESSION
            print("\n\n\nRUN SESSION TESTS\n\n\n")
            
            #Test run from pristine machine
            times = [1, 15, 30, 45, 60]
            print("Asking to run the machine for {} time(s) ({})".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            #Test run with first time < machine time and first time > snapshot time to force rollback
            times = [30, 35, 40, 45]
            print("Asking to run the machine for {} time(s) ({}), the 1st time forces a rollback".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            #Test run with first time < machine time and first time < snapshot time to force recreating machine
            times = [1, 5, 10]
            print("Asking to run the machine for {} time(s) ({}), the 1st time forces a recreating the machine".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            #Test run with first time > machine time so no special effort should be needed
            times = [15]
            print("Asking to run the machine for {} time(s) ({}), no special effort needed".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            
            #STEP SESSION
            print("\n\n\nSTEP SESSION TESTS\n\n\n")
            
            #Test step with time < machine time and time > snapshot time to force rollback
            print("Test step with time < machine time and time > snapshot time to force rollback")
            times = [15,30]
            print("Asking to run the machine for {} time(s) ({}), to prepare machine for that scenario".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            time = 16 
            print("Asking to step the machine on time ({})".format(time))
            step_req = make_new_session_step_request(TEST_SESSION_ID, time)
            print("Server response:\n{}".format(stub_high.SessionStep(step_req)))
            #Test step with time < machine time and time < snapshot time to force recreating machine
            print("Test step with time < machine time and time < snapshot time to force recreating machine")
            times = [20,30]
            print("Asking to run the machine for {} time(s) ({}), to prepare machine for that scenario".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            time = 1 
            print("Asking to step the machine on time ({})".format(time))
            step_req = make_new_session_step_request(TEST_SESSION_ID, time)
            print("Server response:\n{}".format(stub_high.SessionStep(step_req)))
            #Test step with time > machine time so no special effort should be needed
            print("Test step with time > machine time so no special effort should be needed")
            times = [20,30]
            print("Asking to run the machine for {} time(s) ({}), to prepare machine for that scenario".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            time = 35 
            print("Asking to step the machine on time ({})".format(time))
            step_req = make_new_session_step_request(TEST_SESSION_ID, time)
            print("Server response:\n{}".format(stub_high.SessionStep(step_req)))
            #Test step with time = machine time - 1, so step doesn't even have to make a dummy run to get into machine time = time - 1
            print("Test step with time = machine time - 1, so step doesn't even have to make a dummy run to get into machine time = time - 1")
            times = [20,30]
            print("Asking to run the machine for {} time(s) ({}), to prepare machine for that scenario".format(len(times),times))
            run_req = make_new_session_run_request(TEST_SESSION_ID, times)
            print("Server response:\n{}".format(stub_high.SessionRun(run_req)))
            time = 31 
            print("Asking to step the machine on time ({})".format(time))
            step_req = make_new_session_step_request(TEST_SESSION_ID, time)
            print("Server response:\n{}".format(stub_high.SessionStep(step_req)))
            
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
          