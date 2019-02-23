#!/bin/bash

python -m grpc_tools.protoc -I. --python_out=. --grpc_python_out=. manager.proto
python -m grpc_tools.protoc -Icore/src/emulator --python_out=. core/src/emulator/cartesi-base.proto
python -m grpc_tools.protoc -Icore/src/emulator --python_out=. --grpc_python_out=. core/src/emulator/core.proto


