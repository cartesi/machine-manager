FROM cartesi/machine-emulator:0.1.1-pre

LABEL maintainer="Carlo Fragni <carlo@cartesi.io>"

ENV BASE=/opt/cartesi/machine-manager

COPY requirements.txt $BASE/

# Install python and other dependencies
RUN \
    apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install -r $BASE/requirements.txt

COPY . $BASE

# Making grpc/protobuf autogenerated python code files
RUN \
    cd $BASE/lib/grpc-interfaces && \
    bash generate_python_grpc_code.sh

# Changing directory to base
WORKDIR $BASE
CMD [ "bash", "-c", "python3 manager_server.py -a 0.0.0.0" ]
