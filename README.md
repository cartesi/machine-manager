# core-manager

## Instalation

    git clone --recurse-submodules git@github.com:cartesi/core-manager.git

File `server` compiled from `core` submodule should go into `/core/src/emulator`.

    python3 -m venv vir

    source ./vir/bin/activate

## Configuring

Fix the paths to files in your `test_client.py`.

## Running

In one terminal, activate vir:

    source ./vir/bin/activate

    python manager_server.py

In another terminal, activate vir:

    source ./vir/bin/activate

    python test_client.py localhost 50051
