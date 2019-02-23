from concurrent import futures
import time
import math

import grpc

import manager_pb2_grpc
import manager_pb2

LISTENING_PORT = 50051
SLEEP_TIME = 5

class _MachineManager(manager_pb2_grpc.MachineManagerServicer):

    def __init__(self):
        pass

    def NewSession(self, request, context):
        return manager_pb2.MachineHash(
            hash='fake cartesi machine hash')

    def SessionRun(self, request, context):
        return manager_pb2.SessinoRunResponse()

    def SessionStep(self, request, context):
        return manager_pb2.SessionStepResponse()

    def CommunicateReference (self, request, context):
        return cartesi_base_pb2.Void()

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    manager_pb2_grpc.add_MachineManagerServicer_to_server(_MachineManager(),
                                                      server)
    server.add_insecure_port('[::]:{}'.format(LISTENING_PORT))
    server.start()
    print ("Server started, listening on port {}".format(LISTENING_PORT))
    try:
        while True:
            time.sleep(SLEEP_TIME)
    except KeyboardInterrupt:
        print ("\nShutting down")
        server.stop(0)


if __name__ == '__main__':
    serve()
