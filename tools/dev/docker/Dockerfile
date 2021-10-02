FROM cartesi/machine-emulator:0.8.0

MAINTAINER Diego Nehab <diego@cartesi.io>

ENV DEBIAN_FRONTEND=noninteractive

ENV BASE="/opt/emulator"

RUN \
    mkdir -p $BASE

RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y \
        build-essential vim wget git libreadline-dev libboost-container-dev \
        libboost-program-options-dev libboost-serialization-dev \
        ca-certificates automake libtool patchelf python3 python3-pip \
        libstdc++6 virtualenvwrapper

ENV PATH="${PATH}:${BASE}/build/Linux_x86_64/bin"

WORKDIR $BASE

CMD ["/bin/bash", "-l"]
