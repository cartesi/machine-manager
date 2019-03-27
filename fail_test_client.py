from __future__ import print_function

import grpc
import sys
import os

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
        START: 0, #This should make machine creation fail
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
    response, response2, response3, response4 = (None, None, None, None)
    srv_add, srv_port = get_args()
    conn_str = srv_add + ':' + srv_port
    print("Connecting to server in " + conn_str)
    with grpc.insecure_channel(conn_str) as channel:
        stub_low = manager_low_pb2_grpc.MachineManagerLowStub(channel)
        stub_high = manager_high_pb2_grpc.MachineManagerHighStub(channel)
        try:           
            #Test new session with problem on drive start address
            response = stub_high.NewSession(make_new_session_request())
            #embed()            
        except Exception as e:
            print("An exception occurred:")
            print(e)
            print(type(e))
        try:
            #Test communicate Address for a session that doesn't exist
            add_req_msg = manager_low_pb2.AddressRequest(session_id="test_session_id", address="localhost:50000")
            response2 = stub_low.CommunicateAddress(add_req_msg)
        except Exception as e:
            print("An exception occurred:")
            print(e)
            print(type(e))           
            
    if (response):
        print("Core manager client received: " + str(response))
    if (response2):
        print("Core manager client received: " + str(response2))
    
if __name__ == '__main__':
    run()
