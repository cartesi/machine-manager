
#!/bin/sh

set -e

mkdir -p test-files
wget -O test-files/linux.bin -c https://github.com/cartesi-corp/image-kernel/releases/download/v0.8.0/linux-5.5.19-ctsi-2.bin
wget -O test-files/rom.bin -c https://github.com/cartesi-corp/machine-emulator-rom/releases/download/v0.7.0/rom.bin
wget -O test-files/rootfs.ext2 -c https://github.com/cartesi-corp/image-rootfs/releases/download/v0.7.0/rootfs.ext2
