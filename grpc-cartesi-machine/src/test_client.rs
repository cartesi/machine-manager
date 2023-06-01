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
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::{
    Csr, UarchProcessorConfig, UarchRamConfig,
};

use grpc_cartesi_machine::{
    AccessLogType, GrpcCartesiMachineClient, MachineRuntimeConfig, MemoryRangeConfig, RamConfig,
    RollupConfig, RomConfig, UarchConfig,
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

    //let semantic_version = grpc_machine.get_version().await?;
    //println!("I got dhd  address of reg index 3: {:#?}", semantic_version);

    default_config.rom = RomConfig {
        bootargs: default_config.rom.bootargs,
        image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
    };
    default_config.ram = RamConfig {
        length: 1 << 20,
        image_filename: String::new(),
    };

    default_config.uarch = UarchConfig {
        processor: Some(UarchProcessorConfig {
            x1: Some(0),
            x2: Some(0),
            x3: Some(0),
            x4: Some(0),
            x5: Some(0),
            x6: Some(0),
            x7: Some(0),
            x8: Some(0),
            x9: Some(0),
            x10: Some(0),
            x11: Some(0),
            x12: Some(0),
            x13: Some(0),
            x14: Some(0),
            x15: Some(0),
            x16: Some(0),
            x17: Some(0),
            x18: Some(0),
            x19: Some(0),
            x20: Some(0),
            x21: Some(0),
            x22: Some(0),
            x23: Some(0),
            x24: Some(0),
            x25: Some(0),
            x26: Some(0),
            x27: Some(0),
            x28: Some(0),
            x29: Some(0),
            x30: Some(0),
            x31: Some(0),
            pc: Some(0x70000000),
            cycle: Some(0),
        }),
        ram: Some(UarchRamConfig {
            length: 77128,
            image_filename: String::from("/opt/cartesi/share/images/uarch-ram.bin"),
        }),
    };

    default_config.rollup = RollupConfig {
        input_metadata: Some(MemoryRangeConfig {
            start: 0x60400000,
            length: 4096,
            image_filename: "".to_string(),
            shared: false,
        }),
        notice_hashes: Some(MemoryRangeConfig {
            start: 0x60800000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
        rx_buffer: Some(MemoryRangeConfig {
            start: 0x60000000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
        voucher_hashes: Some(MemoryRangeConfig {
            start: 0x60600000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
        tx_buffer: Some(MemoryRangeConfig {
            start: 0x60200000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
    };

    grpc_machine
        .create_machine(&default_config, &MachineRuntimeConfig::default())
        .await?;
    //println!("I got dhd  address of reg index 3: {:#?}", semantic_version);

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
