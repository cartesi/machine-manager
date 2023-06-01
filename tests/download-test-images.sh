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

#!/bin/sh

set -e

mkdir -p test-files
wget -O test-files/linux.bin -c https://github.com/cartesi/image-kernel/releases/download/v0.16.0/linux-5.15.63-ctsi-2.bin
wget -O test-files/rom.bin -c https://github.com/cartesi/machine-emulator-rom/releases/download/v0.16.0/rom-v0.16.0.bin
wget -O test-files/rootfs.ext2 -c https://github.com/cartesi/image-rootfs/releases/download/v0.17.0/rootfs-v0.17.0.ext2
wget -O test-files/machine-emulator-Linux-v0.14.0.tar.gz -c https://github.com/cartesi/machine-emulator/releases/download/v0.14.0/machine-emulator-Linux-v0.14.0.tar.gz
tar -vxC test-files -f test-files/machine-emulator-Linux-v0.14.0.tar.gz --strip-components=3 ./share/images/uarch-ram.bin
