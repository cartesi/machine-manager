FROM ubuntu:20.04 as build-image

# Install python and other dependencies
RUN apt-get update && apt-get install -y python3 python3-pip

COPY requirements.txt /root/

RUN GRPC_PYTHON_BUILD_EXT_COMPILER_JOBS=$(nproc) pip3 install --user -r /root/requirements.txt

# Generating python grpc code
COPY ./lib/grpc-interfaces /root/grpc-interfaces
RUN \
    mkdir -p /root/grpc-interfaces/out \
    && cd /root/grpc-interfaces \
    && python3 -m grpc_tools.protoc -I. \
        --python_out=./out --grpc_python_out=./out \
        cartesi-machine.proto cartesi-machine-checkin.proto \
        machine-manager.proto versioning.proto

# Container final image
# ----------------------------------------------------
# NOTE: the proper machine-emulator image is not released yet
# so using image from the private repo. Should be changed prior to
# releasing.

FROM cartesicorp/machine-emulator:0.8.0

LABEL maintainer="Carlo Fragni <carlo@cartesi.io>"

ENV BASE /opt/cartesi
ENV MANAGER_PATH $BASE/share/machine-manager

# Install python and other dependencies
RUN \
    apt-get update \
    && apt-get install -y python3 libstdc++6 \
    && rm -rf /var/lib/apt/lists/*

# Copy python packages and make sure scripts in .local are usable:
COPY --from=build-image /root/.local /root/.local
ENV PATH=/root/.local/bin:$PATH

RUN mkdir -p $BASE/bin $MANAGER_PATH/proto $MANAGER_PATH/src

COPY --from=build-image /root/grpc-interfaces/out/*.py $MANAGER_PATH/proto/
COPY ./src/*.py $MANAGER_PATH/src/
COPY ./*.py $MANAGER_PATH/
COPY ./machine-manager $BASE/bin/machine-manager

EXPOSE 50051

# Changing directory to base
WORKDIR $BASE
CMD [ "./bin/machine-manager", "-a", "0.0.0.0" ]
