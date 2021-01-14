> :warning: The Cartesi team keeps working internally on the next version of this repository, following its regular development roadmap. Whenever there's a new version ready or important fix, these are published to the public source tree as new releases.

# Machine Manager Server Repository

## Introduction

This repository contains the server responsible for managing different sessions of Cartesi Machines. It has a submodule dependency, the gRPC interface definitions repository, which contains all the interfaces for the communication between this higher level manager and the emulator server.

The easiest way of running the machine manager server + emulator and test them with a sample workload is through a docker container. To get started, follow [TL;DR;](#tldr)

You may also want to install all the dependencies in your system to compile the emulator natively and execute the machine manager server natively.

## Getting Started

### Requirements

- Python >= 3.6
- Python modules described in the requirements.txt file
- [Machine Emulator](https://github.com/cartesi/machine-emulator)

## TL;DR;

Once you have docker installed in your machine, checkout this repository with all submodules:
```console
$ git clone --recurse-submodules git@github.com:cartesi/machine-manager.git
```

The machine emulator has pre-built Docker images published at [Docker Hub](https://hub.docker.com/repository/docker/cartesi/machine-emulator). That will be used as the base image for the machine manager image.

Build machine manager Docker image:
```console
% docker build . -t cartesi/machine-manager
```

Download the test image files:
```console
%./download-test-files
```

Execute a Docker container of the image just built, it will automatically start the machine manager server listening on port 50051:
```console
% docker run -p 50051:50051 -v $(pwd)/test-files:/root/host cartesi/machine-manager
```

After this step, you should be welcomed by a log message stating that the server is up and listening on port 50051:
```console
% INFO __main__ 338 - serve: Server started, listening on address 0.0.0.0 and port 50051
```

Open another terminal to start another session on the ephemeral docker container and execute the test client:
```console
% ./generate-cartesi-gprc
% ./test_client -c
```
You should see the logs on both the server and client terminals showing the steps of the tests being performed by the test client

## Installing dependencies to compile the emulator natively

Please follow the instructions from the [machine emulator repository](https://github.com/cartesi/machine-emulator/blob/master/README.md)

## Installing python dependencies to execute the machine manager server natively

It is highly advisable to make a separate python environment to install the dependencies for executing the machine manager server. A very popular option to do that is using virtualenv with virtualenvwrapper, on Ubuntu you can install them by executing:
```console
% sudo apt install virtualenvwrapper
```

Install python3 in case you don't already have it
```console
% sudo apt install python3
```

And then create a new virtual env (named "mm" in the example) that uses python3:
```console
% mkvirtualenv -p `which python3` mm
```

And now you may install the python dependencies from the requirements file in your virtual env:
```console
$ pip install -r requirements.txt
```

## Executing the machine manager server

To start the server listening on localhost and port 50051, just execute it:
```console
$ ./machine-manager
```

The server has a couple of options to customize it's behavior, you can check them using the -h option:
```console
./machine-manager -h
usage: ./machine-manager [-h] [--address ADDRESS] [--port PORT] [--defective]

Instantiates a machine manager server, responsible for managing and interacting
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

Once you have the machine manager server up and running, you may want to test it is working correctly using the included test client, if the server is running natively and locally all you have to do is execute it with no additional arguments:
```console
$ ./test_client
```

The test client also has a couple of options to customize it's behavior, you may check them with the -h or --help option:
```console
$ ./test_client -h
Starting at Fri Apr  5 19:20:45 2019
usage: ./test_client [-h] [--address ADDRESS] [--port PORT] [--container]

GRPC test client to the machine manager server

optional arguments:
  -h, --help            show this help message and exit
  --address ADDRESS, -a ADDRESS
                        Machine manager server address
  --port PORT, -p PORT  Machine manager server port
  --container, -c       Fixes file references for when machine manager server
                        is running from docker container
```

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Authors

- *Carlo Fragni*

## License

The machine-manager repository and all contributions are licensed under
[APACHE 2.0](https://www.apache.org/licenses/LICENSE-2.0). Please review our [LICENSE](LICENSE) file.

## Acknowledgments

- Original work
