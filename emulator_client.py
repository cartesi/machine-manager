from __future__ import print_function

import grpc

import core_pb2
import cartesi_base_pb2
import core_pb2_grpc
import traceback

class Client():

    srv_conn_str = ""

    def __init__ (self, connection_string):
        self.srv_conn_str = connection_string

    def get_stub (self):
        with grpc.insecure_channel(srv_conn_str) as channel:
            return core_pb2_grpc.MachineStub(channel)

    def create_machine(machine_req):
        stub = get_stub()
        return stub.Machine(machine_req)

    def run_machine(limit):
        stub = get_stub()
        return stub.Run(cartesi_base_pb2.RunRequest(limit=limit))

    def step_machine():
        stub = get_stub()
        return stub.Step(cartesi_base_pb2.Void())
