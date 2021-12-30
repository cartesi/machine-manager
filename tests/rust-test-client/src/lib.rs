pub mod stubs;
pub mod utils;
use std::{thread, time};
use stubs::cartesi_machine::*;
use stubs::cartesi_machine_manager::*;

pub fn generate_default_machine_config(files_dir: &str) -> MachineConfig {
    MachineConfig {
        processor: Some(ProcessorConfig {
            x1_oneof: Some(processor_config::X1Oneof::X1(0)),
            x2_oneof: Some(processor_config::X2Oneof::X2(0)),
            x3_oneof: Some(processor_config::X3Oneof::X3(0)),
            x4_oneof: Some(processor_config::X4Oneof::X4(0)),
            x5_oneof: Some(processor_config::X5Oneof::X5(0)),
            x6_oneof: Some(processor_config::X6Oneof::X6(0)),
            x7_oneof: Some(processor_config::X7Oneof::X7(0)),
            x8_oneof: Some(processor_config::X8Oneof::X8(0)),
            x9_oneof: Some(processor_config::X9Oneof::X9(0)),
            x10_oneof: Some(processor_config::X10Oneof::X10(0)),
            x11_oneof: Some(processor_config::X11Oneof::X11(0)),
            x12_oneof: Some(processor_config::X12Oneof::X12(0)),
            x13_oneof: Some(processor_config::X13Oneof::X13(0)),
            x14_oneof: Some(processor_config::X14Oneof::X14(0)),
            x15_oneof: Some(processor_config::X15Oneof::X15(0)),
            x16_oneof: Some(processor_config::X16Oneof::X16(0)),
            x17_oneof: Some(processor_config::X17Oneof::X17(0)),
            x18_oneof: Some(processor_config::X18Oneof::X18(0)),
            x19_oneof: Some(processor_config::X19Oneof::X19(0)),
            x20_oneof: Some(processor_config::X20Oneof::X20(0)),
            x21_oneof: Some(processor_config::X21Oneof::X21(0)),
            x22_oneof: Some(processor_config::X22Oneof::X22(0)),
            x23_oneof: Some(processor_config::X23Oneof::X23(0)),
            x24_oneof: Some(processor_config::X24Oneof::X24(0)),
            x25_oneof: Some(processor_config::X25Oneof::X25(0)),
            x26_oneof: Some(processor_config::X26Oneof::X26(0)),
            x27_oneof: Some(processor_config::X27Oneof::X27(0)),
            x28_oneof: Some(processor_config::X28Oneof::X28(0)),
            x29_oneof: Some(processor_config::X29Oneof::X29(0)),
            x30_oneof: Some(processor_config::X30Oneof::X30(0)),
            x31_oneof: Some(processor_config::X31Oneof::X31(0)),
            pc_oneof: Some(processor_config::PcOneof::Pc(0x1000)),
            mvendorid_oneof: Some(processor_config::MvendoridOneof::Mvendorid(
                0x6361727465736920,
            )),
            marchid_oneof: Some(processor_config::MarchidOneof::Marchid(0x9)),
            mimpid_oneof: Some(processor_config::MimpidOneof::Mimpid(1)),
            mcycle_oneof: Some(processor_config::McycleOneof::Mcycle(0)),
            minstret_oneof: Some(processor_config::MinstretOneof::Minstret(0)),
            mstatus_oneof: Some(processor_config::MstatusOneof::Mstatus(0)),
            mtvec_oneof: Some(processor_config::MtvecOneof::Mtvec(0)),
            mscratch_oneof: Some(processor_config::MscratchOneof::Mscratch(0)),
            mepc_oneof: Some(processor_config::MepcOneof::Mepc(0)),
            mcause_oneof: Some(processor_config::McauseOneof::Mcause(0)),
            mtval_oneof: Some(processor_config::MtvalOneof::Mtval(0)),
            misa_oneof: Some(processor_config::MisaOneof::Misa(0x141101)),
            mie_oneof: Some(processor_config::MieOneof::Mie(0)),
            mip_oneof: Some(processor_config::MipOneof::Mip(0)),
            medeleg_oneof: Some(processor_config::MedelegOneof::Medeleg(0)),
            mideleg_oneof: Some(processor_config::MidelegOneof::Mideleg(0)),
            mcounteren_oneof: Some(processor_config::McounterenOneof::Mcounteren(0)),
            stvec_oneof: Some(processor_config::StvecOneof::Stvec(0)),
            sscratch_oneof: Some(processor_config::SscratchOneof::Sscratch(0)),
            sepc_oneof: Some(processor_config::SepcOneof::Sepc(0)),
            scause_oneof: Some(processor_config::ScauseOneof::Scause(0)),
            stval_oneof: Some(processor_config::StvalOneof::Stval(0)),
            satp_oneof: Some(processor_config::SatpOneof::Satp(0)),
            scounteren_oneof: Some(processor_config::ScounterenOneof::Scounteren(0)),
            ilrsc_oneof: Some(processor_config::IlrscOneof::Ilrsc(u64::MAX)),
            iflags_oneof: Some(processor_config::IflagsOneof::Iflags(0x0)),
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
            start: 1 << 63,
            length: 62914560,
            image_filename: format!("{}/rootfs.ext2", files_dir),
            shared: false,
        }],
        clint: Some(ClintConfig {
            mtimecmp_oneof: Some(clint_config::MtimecmpOneof::Mtimecmp(0)),
        }),
        htif: Some(HtifConfig {
            console_getchar: false,
            yield_manual: true,
            yield_automatic: false,
            fromhost_oneof: Some(htif_config::FromhostOneof::Fromhost(0)),
            tohost_oneof: Some(htif_config::TohostOneof::Tohost(0)),
        }),
        dhd: Some(DhdConfig {
            tstart: 0,
            tlength: 0,
            image_filename: String::new(),
            dlength: 0,
            hlength: 0,
            h: vec![0; 4],
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
        dhd: None,
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
        }
    }

    pub fn build_new_session_step_request(&self, initial_cycle: u64) -> SessionStepRequest {
        let log_type = Some(AccessLogType {
            proofs: true,
            annotations: true,
        });
        let step_request = StepRequest {
            log_type,
            one_based: false,
        };

        SessionStepRequest {
            session_id: self.session_id.clone(),
            initial_cycle,
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
        let write_memory_request = Some(WriteMemoryRequest { address, data });

        SessionWriteMemoryRequest {
            session_id: self.session_id.clone(),
            cycle,
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

    pub fn build_step_request(&self, manager_request: SessionStepRequest) -> StepRequest {
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
