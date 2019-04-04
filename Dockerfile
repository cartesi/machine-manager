FROM cartesi/image-ci

MAINTAINER Carlo Fragni <carlo@cartesi.io>

ENV BASE=/opt/core-manager
ENV EMU_BASE=$BASE/core/src/emulator

RUN mkdir $BASE

COPY . $BASE

# Install python and other dependencies
RUN \ 
    apt-get update && \
    apt-get install -y python3 python3-pip

RUN \
    pip3 install -r $BASE/requirements.txt

RUN \
    cd $EMU_BASE && \
    make clean && \
    make grpc

#Changing directory to base
WORKDIR $BASE
CMD python3 manager_server.py -a 0.0.0.0
