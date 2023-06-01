# Copyright 2023 Cartesi Pte. Ltd.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use
# this file except in compliance with the License. You may obtain a copy of the
# License at http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed
# under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
# CONDITIONS OF ANY KIND, either express or implied. See the License for the
# specific language governing permissions and limitations under the License.

FROM ubuntu:22.04 as build-image

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"
RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y cmake unzip && \
    rm -rf /var/lib/apt/lists/*

RUN export ARCH=$(uname -m | sed 's/aarch64/aarch_64/') && \
   curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.20.1/protoc-3.20.1-linux-$ARCH.zip && \
   unzip protoc-3.20.1-linux-$ARCH.zip -d $HOME/.local

# Check cargo is visible
RUN cargo --version

# Build cartesi grpc interfaces code
COPY ./Cargo.toml /root/
COPY ./lib/grpc-interfaces /root/lib/grpc-interfaces
COPY ./cartesi-grpc-interfaces /root/cartesi-grpc-interfaces
COPY ./grpc-cartesi-machine /root/grpc-cartesi-machine
COPY ./machine-manager-server /root/machine-manager-server
COPY ./tests /root/tests
RUN cd /root/cartesi-grpc-interfaces && PATH="$PATH:$HOME/.local/bin" cargo build --release

# Build grpc cartesi machine client
RUN cd /root/grpc-cartesi-machine && PATH="$PATH:$HOME/.local/bin" cargo build --release

# Build machine manager server
RUN cd /root/machine-manager-server && PATH="$PATH:$HOME/.local/bin" cargo build --release && PATH="$PATH:$HOME/.local/bin" cargo install --force --path . --root /root/cargo

# Container final image
# ----------------------------------------------------
FROM cartesi/machine-emulator:0.11.1 as machine-manager-rust

LABEL maintainer="Marko Atanasievski <marko.atanasievski@cartesi.io>"

ENV BASE /opt/cartesi
ENV CARTESI_IMAGE_PATH $BASE/share/images
ENV CARTESI_BIN_PATH $BASE/bin

# Install Rust and other dependencies
RUN \
    apt-get update \
    && apt-get install -y build-essential curl libstdc++6 \
    && rm -rf /var/lib/apt/lists/*

# Copy machine manager
COPY --from=build-image /root/cargo/bin/machine-manager $CARTESI_BIN_PATH/machine-manager
ENV PATH=$CARTESI_BIN_PATH:$PATH

EXPOSE 50051

## Changing directory to base
WORKDIR $BASE
CMD [ "./bin/machine-manager", "--address", "0.0.0.0", "--port", "50051","--port-checkin","50052"]
