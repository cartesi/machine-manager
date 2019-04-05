# Cartesi Core Manager Server Repository

## Introduction

This repository contains the server responsible for managing different sessions of Cartesi Machines. It has 3 submodule dependencies:
- The core repository, that contains the emulator GRPC server
- The cartesi-grpc repository, that contains the grpc/protobuf files that define the interfaces exported by and consumed by the server and the related auto-generated code
- The image-ci repository, that defines a Dockerfile for an image ready to compile the risc-v emulator, on which this repository contains a Dockerfile that builds upon

The easiest way of running the core-manager server + emulator and test them with a sample workload is through a docker container. To get started, follow [TL;DR;](#tldr)

You may also want to install all the dependencies in your system to compile the RISC-V emulator natively and execute the core-manager server natively

## TL;DR;

Once you have docker installed in your machine, checkout this repository with all submodules:
```console
$ git clone --recurse-submodules git@github.com:cartesi/core-manager.git
```

Build image-ci docker image:
```console
$ ./build_image_ci.sh
```

Build core-manager docker image:
```console
$ ./build_core_manager_image.sh
```

Execute an ephemeral docker container of the image just built, it will automatically start core-manager server listening on port 50051:
```console
$ ./execute_core_manager_ephemeral_container.sh
```

After this step, you should be welcomed by a log message stating that the server is up and listening on port 50051:
```console
2019-04-05 21:27:01,355 140476130260800 INFO __main__ 163 - serve: Server started, listening on address 0.0.0.0 and port 50051
```

Open another terminal to start another session on the ephemeral docker container and execute the test client:
```console
$ docker exec -it ephemeral-core-manager bash
# python3 test_client.py
```
You should see the logs on both the server and client terminals showing the steps of the tests being performed by the test client

## Installing dependencies to compile the emulator natively

To be able to compile the RISC-V emulator natively, you must install all dependencies and tools that are missing in your system. You can check the [image-ci/Dockerfile](https://github.com/cartesi/image-ci/blob/master/Dockerfile) to see what is needed in an Ubuntu 18.04 fresh environment.

## Recompiling the RISC-V emulator

To recompile the RISC-V emulator, once you have all the tools and dependencies fixed, it's a pretty straight-forward procedure. From the repository base directory go to the emulator directory:
```console
$ cd core/src/emulator
```
(Optional) Clean old artifacts:
```console
$ make clean
```

Build the RISC-V emulator + GRPC wrapper binary:
```console
$ make grpc
```

## Installing python dependencies to execute the core-manager server natively

It is highly advisable to make a separate python environment to install the dependencies for executing the core-manager server. A very popular option to do that is using virtualenv with virtualenvwrapper, on Ubuntu you can install them by executing:
```console
$ sudo apt install virtualenvwrapper
```

Install python3 in case you don't already have it
```console
$ sudo apt install python3
```

And then create a new virtual env (named "cm" in the example) that uses python3:
```console
$ mkvirtualenv -p `which python3` cm
```

Once you run this step, your terminal should exhibit the activated virtual env name right in the beginning of every line in your shell, similar to this example:
```console
(cm) carlo@parma:~/crashlabs/core-manager$ _
```

And now you may install the python dependencies from the requirements file in your virtual env:
```console
$ pip install -r requirements.txt
```

In case you don't need any additional package installed in your system to install the python modules from the step above, you are now ready to execute the core-manager server.

Once you have your virtualenv set up, you may activate it on a terminal using the command "workon":
```console
carlo@parma:~/crashlabs/core-manager$ workon cm
(cm) carlo@parma:~/crashlabs/core-manager$ _
```

And you may deactivate it and go back to using your system-wide python installed environment using the command "deactivate":
```console
(cm) carlo@parma:~/crashlabs/core-manager$ deactivate
carlo@parma:~/crashlabs/core-manager$ _
```

## Executing the core manager server

To start the server listening on localhost and port 50051, just execute it:
```console
$ python manager_server.py
```

The server has a couple of options to customize it's behavior, you can check them using the -h option:
```console
python manager_server.py -h
usage: manager_server.py [-h] [--address ADDRESS] [--port PORT] [--defective]

Instantiates a core manager server, responsible for managing and interacting
with multiple cartesi machine instances

optional arguments:
  -h, --help            show this help message and exit
  --address ADDRESS, -a ADDRESS
                        Address to listen (default: localhost)
  --port PORT, -p PORT  Port to listen (default: 50051)
  --defective, -d       Makes server behave improperly, injecting errors
                        silently in the issued commands
                        -----------------------WARNING!-----------------------
                        FOR TESTING PURPOSES ONLY!!!
                        ------------------------------------------------------
```

As stated in the help, do not use -d option in production as it will make the server misbehave silently, a useful feature only for testing purposes.

## Executing the test client

Once you have the core-manager server up and running, you may want to test it is working correctly using the included test client, if the server is running natively and locally all you have to do is execute it with no additional arguments:
```console
$ python test_client.py
```

The test client also has a couple of options to customize it's behaviour, you may check them with the -h or --help option:
```console
$ python test_client.py -h
Starting at Fri Apr  5 19:20:45 2019
usage: test_client.py [-h] [--address ADDRESS] [--port PORT] [--container]

GRPC client to the high level emulator API (core manager)

optional arguments:
  -h, --help            show this help message and exit
  --address ADDRESS, -a ADDRESS
                        Core manager GRPC server address
  --port PORT, -p PORT  Core manager GRPC server port
  --container, -c       Core manager GPRC server is running from docker
                        container
```
