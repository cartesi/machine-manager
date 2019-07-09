docker run -it --name ephemeral-defective-machine-manager -p 127.0.0.1:50052:50051 -v `pwd`/test-files:/root/host --rm cartesi/image-machine-manager python3 manager_server.py -a 0.0.0.0 -d
