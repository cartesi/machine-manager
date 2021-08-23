// Copyright (C) 2021 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate grpc_cartesi_machine;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Csr;
use grpc_cartesi_machine::{
    AccessLogType, GrpcCartesiMachineClient, MachineRuntimeConfig, RamConfig, RomConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Starting grpc cartesi test client for address {}", args[1]);

    let mut grpc_machine = GrpcCartesiMachineClient::new(args[1].clone()).await?;
    let mut default_config = grpc_machine.get_default_config().await?;
    println!(
        "I have got default cartesi machine config: {:#?}",
        default_config
    );

    let x_addr = grpc_machine.get_x_address(3).await?;
    println!("I got x address of register 3: {}", x_addr);

    let csr_addr = grpc_machine.get_csr_address(Csr::Mcycle).await?;
    println!("I got csr address of mcycle reg: {}", csr_addr);

    let dhd_addr = grpc_machine.get_dhd_h_address(3).await?;
    println!("I got dhd  address of reg index 3: {:#X}", dhd_addr);

    let semantic_version = grpc_machine.get_version().await?;
    println!("I got dhd  address of reg index 3: {:#?}", semantic_version);

    default_config.rom = RomConfig {
        bootargs: default_config.rom.bootargs,
        image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
    };
    default_config.ram = RamConfig {
        length: 1 << 20,
        image_filename: String::new(),
    };

    grpc_machine
        .create_machine(&default_config, &MachineRuntimeConfig::default())
        .await?;
    println!("I got dhd  address of reg index 3: {:#?}", semantic_version);

    let hash = grpc_machine.get_root_hash().await?;
    println!("Root hash step 0 {:?}", hash);

    let access_log = grpc_machine
        .step(
            &AccessLogType {
                annotations: true,
                proofs: true,
            },
            true,
        )
        .await?;
    println!(
        "Step performed, number of accesses: {} ",
        access_log.accesses.len()
    );

    let hash = grpc_machine.get_root_hash().await?;
    println!("Root hash step 1 {:?}", hash);

    let run_info = grpc_machine.run(100).await?;
    println!(
        "Run info: mcycle {}  tohost: {} iflags_h: {} iflags_y: {}",
        run_info.mcycle, run_info.tohost, run_info.iflags_h, run_info.iflags_y
    );

    grpc_machine.destroy().await?;
    println!("Machine destroyed");

    grpc_machine.shutdown().await?;
    println!("Server shut down");

    Ok(())
}
