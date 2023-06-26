# Machine Manager Server Repository

## Introduction

This repository contains the server responsible for managing different sessions of Cartesi Machines. It has a submodule dependency, the gRPC interface definitions repository, which contains all the interfaces for the communication between this higher level manager and the cartesi machine emulator server. Repository contains three creates:

- `cartesi-grpc-interfaces` is Rust grpc interface stub for Cartesi emulator and Machine Manager
- `grpc-cartesi-machine` is Cartesi emulator machine grpc client (depends on `cartesi-grpc-interfaces`)
- `machine-manager-server` is Rust implementation of the Cartesi machine manager service (depends on `cartesi-grpc-interfaces` and `grpc-cartesi-machine`)


The easiest way of running the machine manager server + emulator and test them with a sample workload is through a docker container. To get started, follow [TL;DR;](#tldr)

You may also want to install all the dependencies in your system to compile the emulator natively and execute the machine manager server natively.

## Getting Started

### Requirements

- Rust >= 1.51
- Docker
- [Machine Emulator](https://github.com/cartesi/machine-emulator)

#### Installing Rust dependencies
```console
% apt update && apt upgrade
% curl https://sh.rustup.rs -sSf | sh
```

## TL;DR;

Once you have docker and Rust installed in your machine, checkout this repository with all submodules:
```console
$ git clone --recurse-submodules git@github.com:cartesi/machine-manager.git
```

The machine emulator has pre-built Docker images published at [Docker Hub](https://hub.docker.com/repository/docker/cartesi/machine-emulator). Latest version will be used as the base image for the machine manager image.

> :warning: latest version of Cartesi machine server that uses check in mechanism is not
> yet released on public Cartesi repository, it needs to be generated locally from [cartesi-corp/machine-emulator](https://github.com/cartesi-corp/machine-emulator) develop branch
> Dockerfile uses cartesicorp/machine-emulator:develop image to build machine manager image


Download the test image files:
> :warning: some image files are not yet released and are only available in the Cartesi corp repositories
```console
$ cd machine-manager
$ ./tests/download-test-images.sh
$ export CARTESI_IMAGE_PATH=`pwd`/test-files
```

Build machine manager Rust Docker image:
```console
$ docker build . -t cartesi/machine-manager-rust
```

Execute a Docker container of the image just built, it will automatically start the machine manager server listening on port 50051:
```console
$ docker run -p 50051:50051 -v $(pwd)/test-files:/opt/cartesi/share/images cartesi/machine-manager-rust 
```

After this step, you should be welcomed by a log message stating that the server is up and listening on port 50051:
```console
Starting check in service on address 0.0.0.0:50052
Starting machine manager service on address 0.0.0.0:50051
```

To test Machine Manager server open another terminal session on host computer and demo client:
> :warning: TODO IMPLMENT demo-machine-manager-client
```console
$ cd tests/demo-machine-manager-client
$ cargo run
```
You should see the logs on both the server and client terminals showing the steps of the tests being performed by the test client

### Run Cartesi Machine Manager tests in Docker

Build and run machine-manager-rust-test image
```console
$ docker build . -t cartesi/machine-manager-rust-test -f Dockerfile-test
$ docker run -v $(pwd)/test-files:/opt/cartesi/share/images cartesi/machine-manager-rust-test
```

## Build from source code

### Installing dependencies to compile the emulator natively

Please follow the instructions from the [machine emulator repository](https://github.com/cartesi/machine-emulator/blob/master/README.md)

### Build 

Build `cartesi-grpc-interfaces` from [Cartesi gRPC Interfaces](https://github.com/cartesi/grpc-interfaces) Protobuf definitions:

```console
$ cd cartesi-grpc-interfaces
$ cargo build
```

Build Cartesi machine client:
```console
$ cd grpc-cartesi-machine
$ cargo build
```

Build Machine Manager server:
```console
$ cd machine-manager-server
$ cargo build
```

### Run Machine Manager service

Specify console environment variables that point to the Cartesi images folder and Cartesi machine server.
```console
$ export CARTESI_IMAGE_PATH=`pwd`/test-files
$ export CARTESI_BIN_PATH=<path to folder with cartesi-machine-server>
```

To start the server listening on localhost and port 50051, just execute it:
```console
$ cd cartesi-machine-server
$ cargo run --
```

The server has a couple of options to customize it's behavior, you can check them using the -h option:
```console
$ cargo run -- -h
Usage: target/debug/machine-manager [-h] [--address ADDRESS] [--port PORT]
CARTESI_BIN_PATH and CARTESI_IMAGE_PATH environment variables must be set prior to running

Options:
        --address       Address to listen (default: localhost)
    -p, --port          Port to listen (default: 50051)
        --port-checkin  Port to listen for cartesi server manager checkin
                        (default: 50052)
    -h, --help          show this help message and exit

```

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Authors

### Rust Machine Manager
- *Marko Atanasievski*

### Python Machine Manager
- *Carlo Fragni*

## License

The machine-manager repository and all contributions are licensed under
[APACHE 2.0](https://www.apache.org/licenses/LICENSE-2.0). Please review our [LICENSE](LICENSE) file.

