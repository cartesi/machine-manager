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

//! Implementation of the Rust grpc client for Cartesi emulator machine server

#![allow(unused_variables, dead_code)]

pub mod conversions;

use std::convert::TryInto;
use std::fmt;

use cartesi_grpc_interfaces::grpc_stubs;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::ClintConfig;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::HtifConfig;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::machine_client::MachineClient;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::*;
use conversions::*;
use tonic::Response;

#[derive(Debug, Default)]
struct GrpcCartesiMachineError {
    message: String,
}

impl GrpcCartesiMachineError {
    fn new(message: &str) -> Self {
        GrpcCartesiMachineError {
            message: String::from(message),
        }
    }
}

impl fmt::Display for GrpcCartesiMachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grpc cartesi machine error: {}", &self.message)
    }
}

impl std::error::Error for GrpcCartesiMachineError {}

#[doc = " Server version"]
#[derive(Debug, Clone, Default)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: String,
    pub build: String,
}

impl PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        (
            self.major,
            self.minor,
            self.patch,
            &self.pre_release,
            &self.build,
        ) == (
            other.major,
            other.minor,
            other.patch,
            &other.pre_release,
            &other.build,
        )
    }
}

impl Eq for SemanticVersion {}

impl From<&grpc_stubs::versioning::SemanticVersion> for SemanticVersion {
    fn from(version: &grpc_stubs::versioning::SemanticVersion) -> Self {
        SemanticVersion {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            pre_release: version.pre_release.clone(),
            build: version.build.clone(),
        }
    }
}

#[doc = " Cartesi machine processor state configuration"]
#[derive(Debug, Copy, Clone, Default)]
pub struct ProcessorConfig {
    #[doc = "< Value of general-purpose registers"]
    pub x: [u64; 32usize],
    #[doc = "< Value of f registers"]
    pub f: [u64; 32usize],
    #[doc = "< Value of pc"]
    pub pc: u64,
    #[doc = "< Value of mvendorid CSR"]
    pub mvendorid: u64,
    #[doc = "< Value of marchid CSR"]
    pub marchid: u64,
    #[doc = "< Value of mimpid CSR"]
    pub mimpid: u64,
    #[doc = "< Value of mcycle CSR"]
    pub mcycle: u64,
    #[doc = "< Value of icycleinstret CSR"]
    pub icycleinstret: u64,
    #[doc = "< Value of mstatus CSR"]
    pub mstatus: u64,
    #[doc = "< Value of mtvec CSR"]
    pub mtvec: u64,
    #[doc = "< Value of mscratch CSR"]
    pub mscratch: u64,
    #[doc = "< Value of mepc CSR"]
    pub mepc: u64,
    #[doc = "< Value of mcause CSR"]
    pub mcause: u64,
    #[doc = "< Value of mtval CSR"]
    pub mtval: u64,
    #[doc = "< Value of misa CSR"]
    pub misa: u64,
    #[doc = "< Value of mie CSR"]
    pub mie: u64,
    #[doc = "< Value of mip CSR"]
    pub mip: u64,
    #[doc = "< Value of medeleg CSR"]
    pub medeleg: u64,
    #[doc = "< Value of mideleg CSR"]
    pub mideleg: u64,
    #[doc = "< Value of mcounteren CSR"]
    pub mcounteren: u64,
    #[doc = "< Value of stvec CSR"]
    pub stvec: u64,
    #[doc = "< Value of sscratch CSR"]
    pub sscratch: u64,
    #[doc = "< Value of sepc CSR"]
    pub sepc: u64,
    #[doc = "< Value of scause CSR"]
    pub scause: u64,
    #[doc = "< Value of stval CSR"]
    pub stval: u64,
    #[doc = "< Value of satp CSR"]
    pub satp: u64,
    #[doc = "< Value of scounteren CSR"]
    pub scounteren: u64,
    #[doc = "< Value of ilrsc CSR"]
    pub ilrsc: u64,
    #[doc = "< Value of iflags CSR"]
    pub iflags: u64,
    #[doc = "< Value of senvcfg CSR"]
    pub senvcfg: u64,
    #[doc = "< Value of menvcfg CSR"]
    pub menvcfg: u64,
    #[doc = "< Value of fcsr CSR"]
    pub fcsr: u64,
}

impl ProcessorConfig {
    pub fn new() -> Self {
        ProcessorConfig {
            mvendorid: 0x6361727465736920,
            marchid: 0x7,
            mimpid: 0x1,
            ..Default::default()
        }
    }
}

impl From<&grpc_stubs::cartesi_machine::ProcessorConfig> for ProcessorConfig {
    fn from(config: &grpc_stubs::cartesi_machine::ProcessorConfig) -> Self {
        ProcessorConfig {
            x: convert_x_csr_field(config),
            f: convert_f_csr_field(config),
            pc: convert_csr_field(config.pc),
            mvendorid: convert_csr_field(config.mvendorid),
            marchid: convert_csr_field(config.marchid),
            mimpid: convert_csr_field(config.mimpid),
            mcycle: convert_csr_field(config.mcycle),
            icycleinstret: convert_csr_field(config.icycleinstret),
            mstatus: convert_csr_field(config.mstatus),
            mtvec: convert_csr_field(config.mtvec),
            mscratch: convert_csr_field(config.mscratch),
            mepc: convert_csr_field(config.mepc),
            mcause: convert_csr_field(config.mcause),
            mtval: convert_csr_field(config.mtval),
            misa: convert_csr_field(config.misa),
            mie: convert_csr_field(config.mie),
            mip: convert_csr_field(config.mip),
            medeleg: convert_csr_field(config.medeleg),
            mideleg: convert_csr_field(config.mideleg),
            mcounteren: convert_csr_field(config.mcounteren),
            stvec: convert_csr_field(config.stvec),
            sscratch: convert_csr_field(config.sscratch),
            sepc: convert_csr_field(config.sepc),
            scause: convert_csr_field(config.scause),
            stval: convert_csr_field(config.stval),
            satp: convert_csr_field(config.satp),
            scounteren: convert_csr_field(config.scounteren),
            ilrsc: convert_csr_field(config.ilrsc),
            iflags: convert_csr_field(config.iflags),
            senvcfg: convert_csr_field(config.senvcfg),
            menvcfg: convert_csr_field(config.menvcfg),
            fcsr: convert_csr_field(config.fcsr)
        }
    }
}

#[doc = " Cartesi machine RAM state configuration"]
#[derive(Debug, Clone, Default)]
pub struct RamConfig {
    #[doc = "< RAM length"]
    pub length: u64,
    #[doc = "< RAM image file name"]
    pub image_filename: String,
}

impl RamConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&grpc_stubs::cartesi_machine::RamConfig> for RamConfig {
    fn from(config: &grpc_stubs::cartesi_machine::RamConfig) -> Self {
        RamConfig {
            length: config.length,
            image_filename: config.image_filename.clone(),
        }
    }
}

#[doc = " Cartesi machine Tlb"]
#[derive(Debug, Clone, Default)]
pub struct TlbConfig {
    #[doc = "< Tlb image file name"]
    pub image_filename: String,
}

impl TlbConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&grpc_stubs::cartesi_machine::TlbConfig> for TlbConfig {
    fn from(config: &grpc_stubs::cartesi_machine::TlbConfig) -> Self {
        TlbConfig {
            image_filename: config.image_filename.clone(),
        }
    }
}

#[doc = " Cartesi machine Uarch"]
#[derive(Debug, Clone, Default)]
pub struct UarchConfig {
    #[doc = "< Uarch processor"]
    pub processor: ::core::option::Option<UarchProcessorConfig>,
    #[doc = "< Uarch ram"]
    pub ram: ::core::option::Option<UarchRamConfig>,}

impl UarchConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&grpc_stubs::cartesi_machine::UarchConfig> for UarchConfig {
    fn from(config: &grpc_stubs::cartesi_machine::UarchConfig) -> Self {
        UarchConfig {
            processor: config.processor.clone(),
            ram: config.ram.clone(),
        }
    }
}

#[doc = " Cartesi machine ROM state configuration"]
#[derive(Debug, Clone, Default)]
pub struct RomConfig {
    #[doc = "< Bootargs to pass to kernel"]
    pub bootargs: String,
    #[doc = "< ROM image file"]
    pub image_filename: String,
}

impl RomConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&grpc_stubs::cartesi_machine::RomConfig> for RomConfig {
    fn from(config: &grpc_stubs::cartesi_machine::RomConfig) -> Self {
        RomConfig {
            image_filename: config.image_filename.clone(),
            bootargs: config.bootargs.clone(),
        }
    }
}

#[doc = " Cartesi machine memory range state configuration"]
#[derive(Debug, Clone, Default)]
pub struct MemoryRangeConfig {
    #[doc = "< Memory range start position"]
    pub start: u64,
    #[doc = "< Memory range length"]
    pub length: u64,
    #[doc = "< Target changes to drive affect image file?"]
    pub shared: bool,
    #[doc = "< Memory range image file name"]
    pub image_filename: String,
}

impl MemoryRangeConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&grpc_stubs::cartesi_machine::MemoryRangeConfig> for MemoryRangeConfig {
    fn from(config: &grpc_stubs::cartesi_machine::MemoryRangeConfig) -> Self {
        MemoryRangeConfig {
            start: config.start,
            length: config.length,
            shared: config.shared,
            image_filename: config.image_filename.clone(),
        }
    }
}

#[doc = " Cartesi machine rollup configuration"]
#[derive(Debug, Clone, Default)]
pub struct RollupConfig {
    pub rx_buffer: Option<MemoryRangeConfig>,
    pub tx_buffer: Option<MemoryRangeConfig>,
    pub input_metadata: Option<MemoryRangeConfig>,
    pub voucher_hashes: Option<MemoryRangeConfig>,
    pub notice_hashes: Option<MemoryRangeConfig>,
}

impl RollupConfig {
    pub fn new() -> Self {
        Default::default()
    }
}


impl From<&grpc_stubs::cartesi_machine::RollupConfig> for RollupConfig {
    fn from(config: &grpc_stubs::cartesi_machine::RollupConfig) -> Self {
        RollupConfig {
            input_metadata: match &config.input_metadata {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
            notice_hashes: match &config.notice_hashes {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
            rx_buffer: match &config.rx_buffer {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
            voucher_hashes: match &config.voucher_hashes {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
            tx_buffer: match &config.tx_buffer {
                Some(config) => Some(MemoryRangeConfig::from(config)),
                None => None,
            },
        }
    }
}

#[doc = " Machine state configuration"]
#[derive(Debug, Clone)]
pub struct MachineConfig {
    pub processor: ProcessorConfig,
    pub ram: RamConfig,
    pub rom: RomConfig,
    pub flash_drives: Vec<MemoryRangeConfig>,
    pub clint: ClintConfig,
    pub htif: HtifConfig,
    pub rollup: RollupConfig,
    pub tlb: TlbConfig,
    pub uarch: UarchConfig,
}

impl From<&grpc_stubs::cartesi_machine::MachineConfig> for MachineConfig {
    fn from(mc: &grpc_stubs::cartesi_machine::MachineConfig) -> Self {
        MachineConfig {
            processor: match &mc.processor {
                Some(proc_config) => ProcessorConfig::from(proc_config),
                None => ProcessorConfig::new(),
            },
            ram: match &mc.ram {
                Some(ram_config) => RamConfig::from(ram_config),
                None => RamConfig::new(),
            },
            rom: match &mc.rom {
                Some(rom_config) => RomConfig::from(rom_config),
                None => RomConfig::new(),
            },
            tlb: match &mc.tlb {
                Some(tlb_config) => TlbConfig::from(tlb_config),
                None => TlbConfig::new(),
            },
            uarch: match &mc.uarch {
                Some(uarch_config) => UarchConfig::from(uarch_config),
                None => UarchConfig::new(),
            },
            flash_drives: { mc.flash_drive.iter().map(MemoryRangeConfig::from).collect() },
            clint: match &mc.clint {
                Some(clint_config) => ClintConfig::from(clint_config.clone()),
                None => Default::default(),
            },
            htif: match &mc.htif {
                Some(htif_config) => HtifConfig::from(htif_config.clone()),
                None => Default::default(),
            },
            rollup: match &mc.rollup {
                Some(rollup_config) => RollupConfig::from(rollup_config),
                None => RollupConfig::new(),
            },
        }
    }
}

#[doc = " Concurrency runtime configuration"]
#[derive(Debug, Clone, Default)]
pub struct ConcurrencyConfig {
    pub update_merkle_tree: u64,
}

impl From<&grpc_stubs::cartesi_machine::ConcurrencyConfig> for ConcurrencyConfig {
    fn from(config: &grpc_stubs::cartesi_machine::ConcurrencyConfig) -> Self {
        ConcurrencyConfig {
            update_merkle_tree: config.update_merkle_tree,
        }
    }
}

#[doc = " Machine runtime configuration"]
#[derive(Debug, Clone, Default)]
pub struct MachineRuntimeConfig {
    pub concurrency: ConcurrencyConfig,
}

impl From<&grpc_stubs::cartesi_machine::MachineRuntimeConfig> for MachineRuntimeConfig {
    fn from(rc: &grpc_stubs::cartesi_machine::MachineRuntimeConfig) -> Self {
        MachineRuntimeConfig {
            concurrency: ConcurrencyConfig::from(
                rc.concurrency
                    .as_ref()
                    .unwrap_or(&grpc_stubs::cartesi_machine::ConcurrencyConfig::default()),
            ),
        }
    }
}

#[doc = " Hash data of 32 bytes size"]
#[derive(Debug, Clone, Default)]
pub struct Hash(pub [u8; 32usize]);

impl From<&grpc_stubs::cartesi_machine::Hash> for Hash {
    fn from(hash: &grpc_stubs::cartesi_machine::Hash) -> Self {
        Self(hash.data.clone().try_into().unwrap_or_default())
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[doc = " Merkle tree proof structure"]
#[doc = " \\details"]
#[doc = " This structure holds a proof that the node spanning a log2_target_size"]
#[doc = " at a given address in the tree has a certain hash."]
#[derive(Debug, Clone, Default)]
pub struct MerkleTreeProof {
    pub target_address: u64,
    pub log2_target_size: usize,
    pub target_hash: Hash,
    pub log2_root_size: usize,
    pub root_hash: Hash,
    pub sibling_hashes: Vec<Hash>,
}

impl From<&grpc_stubs::cartesi_machine::MerkleTreeProof> for MerkleTreeProof {
    fn from(proof: &grpc_stubs::cartesi_machine::MerkleTreeProof) -> Self {
        MerkleTreeProof {
            target_address: proof.target_address,
            log2_target_size: proof.log2_target_size as usize,
            log2_root_size: proof.log2_root_size as usize,
            target_hash: match &proof.target_hash {
                Some(x) => Hash::from(x),
                None => Default::default(),
            },
            root_hash: match &proof.root_hash {
                Some(x) => Hash::from(x),
                None => Default::default(),
            },
            sibling_hashes: proof.sibling_hashes.iter().map(Hash::from).collect(),
        }
    }
}

#[doc = " Type of state access"]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessType {
    Read = 0,
    Write,
}

#[doc = " Access log type"]
#[derive(Debug, Clone, Copy, Default)]
pub struct AccessLogType {
    pub proofs: bool,
    pub annotations: bool,
}

impl From<&grpc_stubs::cartesi_machine::AccessLogType> for AccessLogType {
    fn from(log_type: &grpc_stubs::cartesi_machine::AccessLogType) -> Self {
        AccessLogType {
            proofs: log_type.proofs,
            annotations: log_type.annotations,
        }
    }
}

#[doc = " Records an access to the machine state"]
#[derive(Debug, Clone)]
pub struct Access {
    #[doc = "< Type of access"]
    pub r#type: AccessType,
    #[doc = "< Address of access"]
    pub address: u64,
    #[doc = "< Log2 of size of access"]
    pub log2_size: i32,
    #[doc = "< Data before access"]
    pub read_data: Vec<u8>,
    #[doc = "< Data after access (if writing)"]
    pub written_data: Vec<u8>,
    #[doc = "< Proof of data before access"]
    pub proof: MerkleTreeProof,
}

impl From<&grpc_stubs::cartesi_machine::Access> for Access {
    fn from(access: &grpc_stubs::cartesi_machine::Access) -> Self {
        Access {
            r#type: match access.r#type {
                0 => AccessType::Read,
                1 => AccessType::Write,
                _ => AccessType::Read,
            },
            read_data: access.read.iter().copied().collect(),
            written_data: access.written.iter().copied().collect(),
            proof: match &access.proof {
                Some(x) => MerkleTreeProof::from(x),
                None => Default::default(),
            },
            address: access.address,
            log2_size: access.log2_size as i32,
        }
    }
}

#[doc = " Bracket type"]
#[derive(Debug, Clone, Copy)]
pub enum BracketType {
    Begin = 0,
    End,
}

#[doc = " Bracket note"]
#[derive(Debug, Clone)]
pub struct BracketNote {
    #[doc = "< Bracket type"]
    pub r#type: BracketType,
    #[doc = "< Where it points to in the log"]
    pub r#where: u64,
    #[doc = "< Note text"]
    pub text: String,
}

impl From<&grpc_stubs::cartesi_machine::BracketNote> for BracketNote {
    fn from(bracket_note: &grpc_stubs::cartesi_machine::BracketNote) -> Self {
        BracketNote {
            r#type: match bracket_note.r#type {
                0 => BracketType::Begin,
                1 => BracketType::End,
                _ => BracketType::Begin,
            },
            r#where: bracket_note.r#where,
            text: bracket_note.text.clone(),
        }
    }
}

#[doc = " Access log"]
#[derive(Debug, Clone)]
pub struct AccessLog {
    pub accesses: Vec<Access>,
    pub brackets: Vec<BracketNote>,
    pub notes: Vec<String>,
    pub log_type: AccessLogType,
}

impl From<&grpc_stubs::cartesi_machine::AccessLog> for AccessLog {
    fn from(log: &grpc_stubs::cartesi_machine::AccessLog) -> Self {
        let log_type = match &log.log_type {
            Some(ltype) => AccessLogType {
                proofs: ltype.proofs,
                annotations: ltype.annotations,
            },
            None => Default::default(),
        };
        AccessLog {
            log_type,
            accesses: log.accesses.iter().map(Access::from).collect(),
            brackets: log.brackets.iter().map(BracketNote::from).collect(),
            notes: log.notes.clone(),
        }
    }
}

#[doc = "Client for Cartesi emulator machine server"]
#[derive(Debug, Clone)]
pub struct GrpcCartesiMachineClient {
    server_address: String,
    client: MachineClient<tonic::transport::Channel>,
}

impl GrpcCartesiMachineClient {
    /// Create new client instance. Connect to the server as part of client instantiation
    pub async fn new(server_address: String) -> Result<Self, tonic::transport::Error> {
        let client = MachineClient::connect(server_address.clone()).await?;
        Ok(GrpcCartesiMachineClient {
            server_address,
            client,
        })
    }

    /// Create new client instance. Connect to the server as part of client instantiation
    pub fn get_address(&self) -> &String {
        &self.server_address
    }

    /// Get Cartesi machine server version
    pub async fn get_version(&mut self) -> Result<SemanticVersion, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response: Response<grpc_stubs::versioning::GetVersionResponse> =
            self.client.get_version(request).await?;
        match response.into_inner().version {
            Some(stub_semantic_version) => Ok(SemanticVersion::from(&stub_semantic_version)),
            None => Err(Box::new(GrpcCartesiMachineError::new(
                "Cound get retrieve semantic version",
            ))),
        }
    }

    /// Create machine instance on remote Cartesi machine server
    pub async fn create_machine(
        &mut self,
        machine_config: &MachineConfig,
        machine_runtime_config: &MachineRuntimeConfig,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = MachineRequest {
            runtime: Some(grpc_stubs::cartesi_machine::MachineRuntimeConfig::from(
                machine_runtime_config,
            )),
            machine_oneof: Some(machine_request::MachineOneof::Config(
                grpc_stubs::cartesi_machine::MachineConfig::from(machine_config),
            )),
        };
        let response = self.client.machine(request).await?;
        Ok(Void {})
    }

    /// Create machine from storage on remote Cartesi machine server
    pub async fn load_machine(
        &mut self,
        directory: &str,
        machine_runtime_config: &MachineRuntimeConfig,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = MachineRequest {
            runtime: Some(grpc_stubs::cartesi_machine::MachineRuntimeConfig::from(
                machine_runtime_config,
            )),
            machine_oneof: Some(machine_request::MachineOneof::Directory(String::from(
                directory,
            ))),
        };
        let response = self.client.machine(request).await?;
        Ok(Void {})
    }

    /// Run remote machine to maximum limit cycle
    pub async fn run(&mut self, limit: u64) -> Result<RunResponse, Box<dyn std::error::Error>> {
        let request = RunRequest { limit };
        let response = self.client.run(request).await?;
        Ok(response.into_inner())
    }

    /// Serialize entire remote machine state to directory on cartesi machine server host
    pub async fn store(&mut self, directory: &str) -> Result<Void, Box<dyn std::error::Error>> {
        let request = StoreRequest {
            directory: directory.to_string(),
        };
        let response = self.client.store(request).await?;
        Ok(Void {})
    }

    /// Destroy remote machine instance
    pub async fn destroy(&mut self) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.destroy(request).await?;
        Ok(Void {})
    }

    /// Do a snapshot of the remote machine
    pub async fn snapshot(&mut self) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.snapshot(request).await?;
        Ok(Void {})
    }

    /// Perform an rollback on the remote machine
    pub async fn rollback(&mut self) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.rollback(request).await?;
        Ok(Void {})
    }

    /// Shutdown the server
    pub async fn shutdown(&mut self) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.shutdown(request).await?;
        Ok(Void {})
    }

    /// Runs the remote machine for one cycle logging all accesses to the state
    pub async fn step(
        &mut self,
        log_type: &AccessLogType,
        one_based: bool,
    ) -> Result<AccessLog, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(StepUarchRequest {
            log_type: Some(grpc_stubs::cartesi_machine::AccessLogType {
                proofs: log_type.proofs,
                annotations: log_type.annotations,
            }),
            one_based,
        });
        let response: Response<StepUarchResponse> = self.client.step_uarch(request).await?;
        match response.into_inner().log {
            Some(log) => Ok(AccessLog::from(&log)),
            None => Err(Box::new(GrpcCartesiMachineError::new(
                "Error acquiring access log, unknown step result",
            ))),
        }
    }

    /// Reads a chunk of data from the remote machine memory
    pub async fn read_memory(
        &mut self,
        address: u64,
        length: u64,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ReadMemoryRequest { address, length });
        let response = self.client.read_memory(request).await?;
        Ok(response.into_inner().data)
    }

    /// Writes a chunk of data to the remote machine memory
    pub async fn write_memory(
        &mut self,
        address: u64,
        data: Vec<u8>,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(WriteMemoryRequest { address, data });
        let response = self.client.write_memory(request).await?;
        Ok(Void {})
    }

    /// Read the value of a word in the remote machine state
    pub async fn read_word(&mut self, address: u64) -> Result<u64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ReadWordRequest { address });
        let response = self.client.read_word(request).await?;
        let read_word_response = response.into_inner();
        match read_word_response.success {
            true => Ok(read_word_response.value),
            false => Err(Box::new(GrpcCartesiMachineError::new(
                "Failed to read word from required address",
            ))),
        }
    }

    /// Obtains the root hash of the Merkle tree for the remote machine
    pub async fn get_root_hash(&mut self) -> Result<Hash, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.get_root_hash(request).await?;
        match response.into_inner().hash {
            Some(hash) => Ok(Hash::from(&hash)),
            None => Err(Box::new(GrpcCartesiMachineError::new(
                "Error acquiring root hash from cartesi machine",
            ))),
        }
    }

    /// Obtains the proof for a node in the Merkle tree from remote machine
    pub async fn get_proof(
        &mut self,
        address: u64,
        log2_size: u64,
    ) -> Result<MerkleTreeProof, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetProofRequest { address, log2_size });
        let response = self.client.get_proof(request).await?;
        match response.into_inner().proof {
            Some(proof) => Ok(MerkleTreeProof::from(&proof)),
            None => Err(Box::new(GrpcCartesiMachineError::new(&format!(
                "Error acquiring proof for address {} and log2_size {}",
                address, log2_size
            )))),
        }
    }

    /// Replaces a flash drive on a remote machine
    pub async fn replace_memory_range(
        &mut self,
        config: &cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::MemoryRangeConfig,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ReplaceMemoryRangeRequest {
            config: Some(config.clone()),
        });
        let response = self.client.replace_memory_range(request).await?;
        Ok(Void {})
    }

    /// Gets the address of a general-purpose register
    pub async fn get_x_address(&mut self, index: u32) -> Result<u64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetXAddressRequest { index });
        let response = self.client.get_x_address(request).await?;
        Ok(response.into_inner().address)
    }

    /// Reads the value of a general-purpose register from the remote machine
    pub async fn read_x(&mut self, index: u32) -> Result<u64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ReadXRequest { index });
        let response = self.client.read_x(request).await?;
        Ok(response.into_inner().value)
    }

    /// Writes the value of a general-purpose register for the remote machine
    pub async fn write_x(
        &mut self,
        index: u32,
        value: u64,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(WriteXRequest { index, value });
        let response = self.client.write_x(request).await?;
        Ok(Void {})
    }

    /// Resets the value of the iflags_Y flag on the remote machine
    pub async fn reset_iflags_y(&mut self) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.reset_iflags_y(request).await?;
        Ok(Void {})
    }

    /// Gets the address of any CSR
    pub async fn get_csr_address(&mut self, csr: Csr) -> Result<u64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetCsrAddressRequest { csr: csr as i32 });
        let response = self.client.get_csr_address(request).await?;
        Ok(response.into_inner().address)
    }

    /// Read the value of any CSR from remote machine
    pub async fn read_csr(&mut self, csr: Csr) -> Result<u64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ReadCsrRequest { csr: csr as i32 });
        let response = self.client.read_csr(request).await?;
        Ok(response.into_inner().value)
    }

    /// Writes the value of any CSR on remote machine
    pub async fn write_csr(
        &mut self,
        csr: Csr,
        value: u64,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(WriteCsrRequest {
            csr: csr as i32,
            value,
        });
        let response = self.client.write_csr(request).await?;
        Ok(Void {})
    }

    /// Returns copy of initialization config of the remote machine
    pub async fn get_initial_config(
        &mut self,
    ) -> Result<MachineConfig, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.get_initial_config(request).await?;
        match response.into_inner().config {
            Some(def_config) => Ok(MachineConfig::from(&def_config)),
            None => Err(Box::new(GrpcCartesiMachineError::new(
                "Error acquiring initial configuration",
            ))),
        }
    }

    /// Verifies integrity of Merkle tree on the remote machine
    pub async fn verify_merkle_tree(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.verify_merkle_tree(request).await?;
        Ok(response.into_inner().success)
    }

    /// Verify if dirty page maps are consistent on the remote machine
    pub async fn verify_dirty_page_maps(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.verify_dirty_page_maps(request).await?;
        Ok(response.into_inner().success)
    }

    /// Dump all memory ranges to files in current working directory on the server (for debugging purporses)
    pub async fn dump_pmas(&mut self) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.dump_pmas(request).await?;
        Ok(Void {})
    }

    /// Returns copy of default system config from remote Cartesi machine server
    pub async fn get_default_config(
        &mut self,
    ) -> Result<MachineConfig, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Void {});
        let response = self.client.get_default_config(request).await?;
        match response.into_inner().config {
            Some(def_config) => Ok(MachineConfig::from(&def_config)),
            None => Err(Box::new(GrpcCartesiMachineError::new(
                "Error acquiring default configuration",
            ))),
        }
    }

    /// Checks the internal consistency of an access log
    pub async fn verify_access_log(
        &mut self,
        log: &AccessLog,
        runtime: &MachineRuntimeConfig,
        one_based: bool,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(VerifyAccessLogRequest {
            log: Some(grpc_stubs::cartesi_machine::AccessLog::from(log)),
            runtime: Some(grpc_stubs::cartesi_machine::MachineRuntimeConfig::from(
                runtime,
            )),
            one_based,
        });
        let response = self.client.verify_access_log(request).await?;
        Ok(Void {})
    }

    /// Checks the validity of a state transition
    pub async fn verify_state_transition(
        &mut self,
        root_hash_before: &Hash,
        log: &AccessLog,
        root_hash_after: &Hash,
        one_based: bool,
        runtime: &MachineRuntimeConfig,
    ) -> Result<Void, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(VerifyStateTransitionRequest {
            root_hash_before: Some(grpc_stubs::cartesi_machine::Hash::from(root_hash_before)),
            log: Some(grpc_stubs::cartesi_machine::AccessLog::from(log)),
            root_hash_after: Some(grpc_stubs::cartesi_machine::Hash::from(root_hash_after)),
            one_based,
            runtime: Some(grpc_stubs::cartesi_machine::MachineRuntimeConfig::from(
                runtime,
            )),
        });
        let response = self.client.verify_state_transition(request).await?;
        Ok(Void {})
    }
}
