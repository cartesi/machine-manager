// Copyright 2023 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::{
    ClintConfig, HtifConfig, machine_request, ProcessorConfig,
    MemoryRangeConfig, MachineConfig, MachineRequest, MachineRuntimeConfig,
     RamConfig, RollupConfig, RomConfig, UarchConfig,
};
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::machine_manager_client::MachineManagerClient;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::*;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::UarchProcessorConfig;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::UarchRamConfig;
pub const CARTESI_BIN_PATH: &str = "CARTESI_BIN_PATH";
pub const CARTESI_IMAGE_PATH: &str = "CARTESI_IMAGE_PATH";

pub fn generate_default_machine_config(files_dir: &str) -> MachineConfig {
    MachineConfig {
        processor: Some(ProcessorConfig {
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
            f0: Some(0),
            f1: Some(0),
            f2: Some(0),
            f3: Some(0),
            f4: Some(0),
            f5: Some(0),
            f6: Some(0),
            f7: Some(0),
            f8: Some(0),
            f9: Some(0),
            f10: Some(0),
            f11: Some(0),
            f12: Some(0),
            f13: Some(0),
            f14: Some(0),
            f15: Some(0),
            f16: Some(0),
            f17: Some(0),
            f18: Some(0),
            f19: Some(0),
            f20: Some(0),
            f21: Some(0),
            f22: Some(0),
            f23: Some(0),
            f24: Some(0),
            f25: Some(0),
            f26: Some(0),
            f27: Some(0),
            f28: Some(0),
            f29: Some(0),
            f30: Some(0),
            f31: Some(0),
            fcsr: Some(0),
            menvcfg: Some(0),
            senvcfg: Some(0),
            pc: Some(0x1000),
            mvendorid: Some(0x6361727465736920),
            marchid: Some(0xf),
            mimpid: Some(1),
            mcycle: Some(0),
            icycleinstret: Some(0),
            mstatus: Some(0),
            mtvec: Some(0),
            mscratch: Some(0),
            mepc: Some(0),
            mcause: Some(0),
            mtval: Some(0),
            misa: Some(0x800000000014112d),
            mie: Some(0),
            mip: Some(0),
            medeleg: Some(0),
            mideleg: Some(0),
            mcounteren: Some(0),
            stvec: Some(0),
            sscratch: Some(0),
            sepc: Some(0),
            scause: Some(0),
            stval: Some(0),
            satp: Some(0),
            scounteren: Some(0),
            ilrsc: Some(u64::MAX),
            iflags: Some(0x0),
        }),
        tlb: None,
        uarch: Some(UarchConfig {
            processor: Some(UarchProcessorConfig::default()),
            ram: Some(UarchRamConfig::default())
        }),
        ram: Some(RamConfig {
            length: 64 << 20,
            image_filename: format!("{}/linux.bin", files_dir),
        }),
        rom: Some(RomConfig {
            bootargs: String::from("console=hvc0 rootfstype=ext2 root=/dev/mtdblock0 rwmtdparts=flash.0:-(rootfs) -- for i in $(seq 0 5 1000); do yield progress $i; done"),
            image_filename: format!("{}/rom.bin", files_dir),
        }),
        flash_drive: vec![MemoryRangeConfig {
            start: 1 << 55,
            length: 71303168,
            image_filename: format!("{}/rootfs.ext2", files_dir),
            shared: false,
        }],
        clint: Some(ClintConfig {
            mtimecmp: Some(0),
        }),
        htif: Some(HtifConfig {
            console_getchar: false,
            yield_manual: true,
            yield_automatic: false,
            fromhost: Some(0),
            tohost: Some(0),
        }),
        rollup: Some(RollupConfig {
            input_metadata: Some(MemoryRangeConfig{
                start: 0x60400000,
                length: 4096,
                image_filename: "".to_string(),
                shared: false,
            }),
            notice_hashes: Some(MemoryRangeConfig{
                start: 0x60800000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            rx_buffer: Some(MemoryRangeConfig{
                start: 0x60000000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            voucher_hashes: Some(MemoryRangeConfig{
                start: 0x60600000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            tx_buffer: Some(MemoryRangeConfig{
                start: 0x60200000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
        }),
    }
}

pub fn generate_default_machine_rt_config() -> MachineRuntimeConfig {
    MachineRuntimeConfig {
        concurrency: None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting demo machine client");
    let session_id = "mysession";
    let image_file_root = std::env::var(&CARTESI_IMAGE_PATH).unwrap_or_else(|_| {
        panic!(
            "{} that points to folder with Cartesi images is not set",
            &CARTESI_IMAGE_PATH
        )
    });
    // Instantiate client
    let mut client = MachineManagerClient::connect("http://127.0.0.1:50051").await?;
    // Create new session
    let machine = Some(MachineRequest {
        runtime: Some(generate_default_machine_rt_config()),
        machine_oneof: Some(machine_request::MachineOneof::Config(
            generate_default_machine_config(&image_file_root),
        )),
    });

    let request = tonic::Request::new(NewSessionRequest {
        machine,
        session_id: session_id.to_string(),
        force: false,
    });

    let response = client.new_session(request).await?;
    println!("Session created\n{:?}", response.into_inner().data);

    loop {
        //Run to 20 cycle
        let request = tonic::Request::new(SessionRunRequest {
            session_id: session_id.to_string(),
            final_cycles: vec![20],
        });
        let response = client.session_run(request).await?;
        if let Some(one_of) = response.into_inner().run_oneof {
            match one_of {
                session_run_response::RunOneof::Progress(progress) => {
                    println!(
                        "Running session, progress {}, cycle {}\n",
                        progress.progress, progress.cycle
                    );
                }
                session_run_response::RunOneof::Result(result) => {
                    println!(
                        "Job executed, resulting hash {:?}\n",
                        &result.hashes[0].data
                    );
                    break;
                }
            }
        }
    }

    // End session
    let request = tonic::Request::new(EndSessionRequest {
        session_id: session_id.to_string(),
        silent: false,
    });
    let _response = client.end_session(request).await?;
    println!("Session ended\n");
    Ok(())
}
