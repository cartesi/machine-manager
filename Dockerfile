FROM ubuntu:20.04 as build-image

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

# Check cargo is visible
RUN cargo --version

# Build cartesi grpc interfaces code
COPY ./lib/grpc-interfaces /root/lib/grpc-interfaces
COPY ./cartesi-grpc-interfaces /root/cartesi-grpc-interfaces
RUN cd /root/cartesi-grpc-interfaces && cargo build --release

# Build grpc cartesi machine client
COPY ./grpc-cartesi-machine /root/grpc-cartesi-machine
RUN cd /root/grpc-cartesi-machine && cargo build --release

# Build machine manager server
COPY ./machine-manager-server /root/machine-manager-server
RUN cd /root/machine-manager-server && cargo build --release && cargo install --force --path . --root /root/cargo

# Container final image
# ----------------------------------------------------
FROM cartesicorp/machine-emulator:develop as machine-manager-rust

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
