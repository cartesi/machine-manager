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
wget -O test-files/linux.bin -c https://github.com/cartesi-corp/image-kernel/releases/download/v0.8.0/linux-5.5.19-ctsi-2.bin
wget -O test-files/rom.bin -c https://github.com/cartesi-corp/machine-emulator-rom/releases/download/v0.7.0/rom.bin
wget -O test-files/rootfs.ext2 -c https://github.com/cartesi-corp/image-rootfs/releases/download/v0.7.0/rootfs.ext2
