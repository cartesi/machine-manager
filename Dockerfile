ARG EMULATOR_REPOSITORY=cartesi/machine-emulator
ARG EMULATOR_VERSION=latest
FROM ubuntu:20.04 as build-image

# Install python and other dependencies
RUN apt-get update && apt-get install -y python3 python3-pip

COPY requirements.txt /root/

ENV PYTHON_INSTALL /opt/cartesi/share/python
RUN GRPC_PYTHON_BUILD_EXT_COMPILER_JOBS=$(nproc) pip3 install --target=$PYTHON_INSTALL -r /root/requirements.txt

COPY . /root
ENV PYTHONPATH=$PYTHONPATH:$PYTHON_INSTALL
RUN \
    mkdir -p /root/proto \
    && cd /root/lib/grpc-interfaces \
    && python3 -m grpc_tools.protoc -I. \
        --python_out=/root/proto --grpc_python_out=/root/proto \
        cartesi-machine.proto cartesi-machine-checkin.proto \
        machine-manager.proto versioning.proto

RUN cd /root && ./install.sh

# Container final image
# ----------------------------------------------------
ARG EMULATOR_REPOSITORY
ARG EMULATOR_VERSION
FROM ${EMULATOR_REPOSITORY}:${EMULATOR_VERSION}

LABEL maintainer="Carlo Fragni <carlo@cartesi.io>"

ENV BASE /opt/cartesi
ENV MANAGER_PATH $BASE/bin

# Install python and other dependencies
RUN \
    apt-get update \
    && apt-get install -y python3 libstdc++6 \
    && rm -rf /var/lib/apt/lists/*

ENV PYTHON_INSTALL /opt/cartesi/share/python
ENV PATH=$PYTHON_INSTALL:$PATH
COPY --from=build-image /opt/cartesi /opt/cartesi
EXPOSE 50051

# Changing directory to base
WORKDIR $BASE
CMD [ "./bin/machine-manager", "-a", "0.0.0.0" ]
