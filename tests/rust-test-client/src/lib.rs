// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

pub mod stubs;
pub mod utils;
use std::{thread, time};
use stubs::cartesi_machine::*;
use stubs::cartesi_machine_manager::*;

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
        tlb: Some(TlbConfig {
            image_filename: "".to_string(),
        }),
        uarch: Some(UarchConfig {
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
        }),
        ram: Some(RamConfig {
            length: 64 << 20,
            image_filename: format!("{}/linux.bin", files_dir),
        }),
        rom: Some(RomConfig {
            bootargs: String::from(
                "console=hvc0 rootfstype=ext2 root=/dev/mtdblock0 rw \
                          mtdparts=flash.0:-(rootfs) -- for i in $(seq 0 5 1000000); \
                          do yield manual progress $i; done",
            ),
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
            rx_buffer: Some(MemoryRangeConfig {
                start: 0x60000000,
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
            input_metadata: Some(MemoryRangeConfig {
                start: 0x60400000,
                length: 4096,
                image_filename: "".to_string(),
                shared: false,
            }),
            voucher_hashes: Some(MemoryRangeConfig {
                start: 0x60600000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            notice_hashes: Some(MemoryRangeConfig {
                start: 0x60800000,
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
#[derive(Default)]
pub struct MachineManagerClientProxy {
    pub session_id: String,
    pub grpc_client:
        Option<machine_manager_client::MachineManagerClient<tonic::transport::Channel>>,
    pub machine_config: Option<MachineConfig>,
    pub machine_rt_config: Option<MachineRuntimeConfig>,
}

impl MachineManagerClientProxy {
    pub async fn connect(
        &mut self,
        server_ip: &str,
        server_port: u32,
    ) -> Result<(), tonic::transport::Error> {
        let server_address = format!("http://{}:{}", server_ip, server_port);
        self.grpc_client =
            Some(machine_manager_client::MachineManagerClient::connect(server_address).await?);
        Ok(())
    }

    pub fn build_new_session_request(&self, force: bool) -> NewSessionRequest {
        let machine = Some(MachineRequest {
            runtime: Some(self.machine_rt_config.clone().unwrap()),
            machine_oneof: Some(machine_request::MachineOneof::Config(
                self.machine_config.clone().unwrap(),
            )),
        });

        NewSessionRequest {
            machine,
            session_id: self.session_id.clone(),
            force,
        }
    }

    pub fn build_new_session_from_store_request(
        &self,
        file_root: String,
        directory: String,
    ) -> NewSessionRequest {
        let machine = Some(MachineRequest {
            runtime: Some(self.machine_rt_config.clone().unwrap()),
            machine_oneof: Some(machine_request::MachineOneof::Directory(format!(
                "{}/{}",
                file_root, directory
            ))),
        });

        NewSessionRequest {
            machine,
            session_id: self.session_id.clone(),
            force: true,
        }
    }

    pub fn build_end_session_request(&self, silent: bool) -> EndSessionRequest {
        EndSessionRequest {
            session_id: self.session_id.clone(),
            silent,
        }
    }

    pub fn build_new_session_run_request(&self, final_cycles: &Vec<u64>) -> SessionRunRequest {
        SessionRunRequest {
            session_id: self.session_id.clone(),
            final_cycles: final_cycles.clone(),
            final_ucycles: vec![],
        }
    }

    pub fn build_new_session_step_request(&self, initial_cycle: u64) -> SessionStepRequest {
        let log_type = Some(AccessLogType {
            proofs: true,
            annotations: true,
        });
        let step_request = StepUarchRequest {
            log_type,
            one_based: false,
        };

        SessionStepRequest {
            session_id: self.session_id.clone(),
            initial_cycle,
            initial_ucycle: 0,
            step_params_oneof: Some(session_step_request::StepParamsOneof::StepParams(
                step_request,
            )),
        }
    }

    pub fn build_new_session_get_proof_request(
        &self,
        cycle: u64,
        address: u64,
        log2_size: u64,
    ) -> SessionGetProofRequest {
        let proof_request = Some(GetProofRequest { address, log2_size });

        SessionGetProofRequest {
            session_id: self.session_id.clone(),
            cycle,
            ucycle: 0,
            target: proof_request,
        }
    }

    pub fn build_new_session_store_request(
        &self,
        file_root: String,
        directory: String,
    ) -> SessionStoreRequest {
        let store_request = Some(StoreRequest {
            directory: format!("{}/{}", file_root, directory),
        });

        SessionStoreRequest {
            session_id: self.session_id.clone(),
            store: store_request,
        }
    }

    pub fn build_new_session_read_memory_request(
        &self,
        cycle: u64,
        address: u64,
        data_length: u64,
    ) -> SessionReadMemoryRequest {
        let read_memory_request = Some(ReadMemoryRequest {
            address,
            length: data_length,
        });

        SessionReadMemoryRequest {
            session_id: self.session_id.clone(),
            cycle,
            position: read_memory_request,
        }
    }

    pub fn build_new_session_write_memory_request(
        &self,
        cycle: u64,
        address: u64,
        data: Vec<u8>,
    ) -> SessionWriteMemoryRequest {
        let write_memory_request = Some(WriteMemoryRequest { address, data});

        SessionWriteMemoryRequest {
            session_id: self.session_id.clone(),
            cycle,
            ucycle: 0,
            position: write_memory_request,
        }
    }

    pub async fn run_to_completion(
        &mut self,
        run_request: SessionRunRequest,
    ) -> SessionRunResponse {
        let mut grpc_client = self.grpc_client.take().unwrap();
        let mut response = grpc_client
            .session_run(run_request.clone())
            .await
            .unwrap()
            .into_inner();

        while let session_run_response::RunOneof::Progress(_) = response.run_oneof.as_ref().unwrap()
        {
            thread::sleep(time::Duration::from_secs(1));
            response = grpc_client
                .session_run(run_request.clone())
                .await
                .unwrap()
                .into_inner();
        }

        self.grpc_client = Some(grpc_client);
        response
    }
}

#[derive(Default)]
pub struct MachineClientProxy {
    pub grpc_client: Option<machine_client::MachineClient<tonic::transport::Channel>>,
}

impl MachineClientProxy {
    pub async fn connect(
        &mut self,
        server_ip: &str,
        server_port: u32,
    ) -> Result<(), tonic::transport::Error> {
        let server_address = format!("http://{}:{}", server_ip, server_port);
        self.grpc_client = Some(machine_client::MachineClient::connect(server_address).await?);
        Ok(())
    }

    pub fn build_machine_request(&self, manager_request: NewSessionRequest) -> MachineRequest {
        manager_request.machine.unwrap()
    }

    pub fn build_run_request(&self, limit: u64) -> RunRequest {
        RunRequest { limit }
    }

    pub fn build_step_request(&self, manager_request: SessionStepRequest) -> StepUarchRequest {
        let session_step_request::StepParamsOneof::StepParams(step_request) =
            manager_request.step_params_oneof.unwrap();
        return step_request;
    }

    pub fn build_get_proof_request(
        &self,
        manager_request: SessionGetProofRequest,
    ) -> GetProofRequest {
        manager_request.target.unwrap()
    }

    pub fn build_store_request(&self, manager_request: SessionStoreRequest) -> StoreRequest {
        manager_request.store.unwrap()
    }

    pub fn build_read_memory_request(
        &self,
        manager_request: SessionReadMemoryRequest,
    ) -> ReadMemoryRequest {
        manager_request.position.unwrap()
    }

    pub fn build_write_memory_request(
        &self,
        manager_request: SessionWriteMemoryRequest,
    ) -> WriteMemoryRequest {
        manager_request.position.unwrap()
    }
}
