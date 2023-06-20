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

use cartesi_jsonrpc_interfaces::index::*;
use cartesi_jsonrpc_interfaces::*;

use conversions::*;
use tonic::Response;

#[derive(Debug, Default)]
struct JsonrpcCartesiMachineError {
    message: String,
}

impl JsonrpcCartesiMachineError {
    fn new(message: &str) -> Self {
        JsonrpcCartesiMachineError {
            message: String::from(message),
        }
    }
}

impl fmt::Display for JsonrpcCartesiMachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Jsonrpc cartesi machine error: {}", &self.message)
    }
}

impl std::error::Error for JsonrpcCartesiMachineError {}

#[doc = " Server version"]
#[derive(Debug, Clone, Default)]
pub struct SemanticVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
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

impl
    From<&ObjectOfStringDoaGddGAInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2StringDoaGddGAUXIipOfa>
    for SemanticVersion
{
    fn from(
        version: &ObjectOfStringDoaGddGAInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2StringDoaGddGAUXIipOfa,
    ) -> Self {
        SemanticVersion {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            pre_release: version.pre_release.clone().unwrap(),
            build: version.build.clone().unwrap(),
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

impl From<&ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q> for ProcessorConfig {
    fn from(config: &ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q) -> Self {
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

impl From<&ObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0> for RamConfig {
    fn from(config: &ObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0) -> Self {
        RamConfig {
            length: config.length,
            image_filename: config.image_filename.clone().unwrap(),
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

impl From<&ObjectOfStringDoaGddGAMbKkyjX7> for TlbConfig {
    fn from(config: &ObjectOfStringDoaGddGAMbKkyjX7) -> Self {
        TlbConfig {
            image_filename: config.image_filename.clone().unwrap(),
        }
    }
}

#[doc = " Cartesi machine Uarch"]
#[derive(Debug, Clone, Default)]
pub struct UarchConfig {
    #[doc = "< Uarch processor"]
    pub processor: ::core::option::Option<
        ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CW,
    >,
    #[doc = "< Uarch ram"]
    pub ram: ::core::option::Option<ObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebB>,
}

impl UarchConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&ObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5A> for UarchConfig {
    fn from(config: &ObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5A) -> Self {
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

impl From<&ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6O> for RomConfig {
    fn from(config: &ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6O) -> Self {
        RomConfig {
            image_filename: config.image_filename.clone().unwrap(),
            bootargs: config.bootargs.clone().unwrap(),
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

impl From<&ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F>
    for MemoryRangeConfig
{
    fn from(
        config: &ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F,
    ) -> Self {
        MemoryRangeConfig {
            start: config.start.unwrap(),
            length: config.length.unwrap(),
            shared: config.shared.unwrap(),
            image_filename: config.image_filename.clone().unwrap(),
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

impl From<&ObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGD> for RollupConfig {
    fn from(config: &ObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGD) -> Self {
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
    pub clint: ObjectOfInteger7Bd9WOt2KmrDcohf,
    pub htif:
        ObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQei,
    pub rollup: RollupConfig,
    pub tlb: TlbConfig,
    pub uarch: UarchConfig,
}

impl From<&ObjectOfObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5AObjectOfStringDoaGddGAMbKkyjX7ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6OObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGDObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4QObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQeiUnorderedSetOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6Fr01Y9HDOObjectOfInteger7Bd9WOt2KmrDcohfM5JY4BqN> for MachineConfig {
    fn from(mc: &ObjectOfObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5AObjectOfStringDoaGddGAMbKkyjX7ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6OObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGDObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4QObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQeiUnorderedSetOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6Fr01Y9HDOObjectOfInteger7Bd9WOt2KmrDcohfM5JY4BqN) -> Self {
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
            flash_drives: { mc.flash_drive.clone().unwrap().iter().map(MemoryRangeConfig::from).collect() },
            clint: match &mc.clint {
                Some(clint_config) => ObjectOfInteger7Bd9WOt2KmrDcohf::from(clint_config.clone()),
                None => Default::default(),
            },
            htif: match &mc.htif {
                Some(htif_config) => ObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQei::from(htif_config.clone()),
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

impl From<&ObjectOfInteger7Bd9WOt2INmaA59K> for ConcurrencyConfig {
    fn from(config: &ObjectOfInteger7Bd9WOt2INmaA59K) -> Self {
        ConcurrencyConfig {
            update_merkle_tree: config.update_merkle_tree.unwrap(),
        }
    }
}

#[doc = " Machine runtime configuration"]
#[derive(Debug, Clone, Default)]
pub struct MachineRuntimeConfig {
    pub concurrency: ConcurrencyConfig,
}

impl From<&ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6> for MachineRuntimeConfig {
    fn from(rc: &ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6) -> Self {
        MachineRuntimeConfig {
            concurrency: ConcurrencyConfig::from(
                rc.concurrency
                    .as_ref()
                    .unwrap_or(&ObjectOfInteger7Bd9WOt2INmaA59K::default()),
            ),
        }
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
    pub target_hash: String,
    pub log2_root_size: usize,
    pub root_hash: String,
    pub sibling_hashes: Vec<String>,
}

impl From<&ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQ> for MerkleTreeProof {
    fn from(proof: &ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQ) -> Self {
        MerkleTreeProof {
            target_address: proof.target_address,
            log2_target_size: proof.log_2_target_size as usize,
            log2_root_size: proof.log_2_root_size as usize,
            target_hash: proof.target_hash.clone(),
            root_hash: proof.root_hash.clone(),
            sibling_hashes: proof.sibling_hashes.clone(),
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

impl From<&ObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2> for AccessLogType {
    fn from(log_type: &ObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2) -> Self {
        AccessLogType {
            proofs: log_type.has_proofs,
            annotations: log_type.has_annotations,
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

impl From<&ObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85Fz> for Access {
    fn from(access: &ObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85Fz) -> Self {
        let mut read_data = access.read.clone();
        let mut written_data: String = match access.written.clone() {
            Some(written_data) => written_data.clone(),
            None => Default::default(),
        };

        if written_data.ends_with('\n') {
            written_data.pop(); 
        }

        if read_data.ends_with('\n') {
            read_data.pop(); 
        }

        Access {
            r#type: match access.r#type.to_string().as_str() {
                "\"read\"" => AccessType::Read,
                "\"write\"" => AccessType::Write,
                _ => AccessType::Read,
            },
            read_data: base64::decode(read_data).unwrap(),
            written_data: base64::decode(written_data).unwrap(),
            proof: match &access.proof {
                Some(x) => MerkleTreeProof::from(x),
                None => Default::default(),
            },
            address: access.address,
            log2_size: access.log_2_size as i32,
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
impl std::convert::From<&cartesi_jsonrpc_interfaces::index::ObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0> for BracketNote {
    fn from(bracket_note: &ObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0) -> Self {
        BracketNote {
            r#type: match bracket_note._type.to_string().as_str() {
                "begin" => BracketType::Begin,
                "end" => BracketType::End,
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

impl From<&ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD> for AccessLog {
    fn from(log: &ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD) -> Self {
        let log_type = AccessLogType {
                proofs: log.log_type.has_proofs,
                annotations: log.log_type.has_annotations
        };
        AccessLog {
            log_type,
            accesses: log.accesses.iter().map(Access::from).collect(),
            brackets: log.brackets.clone().unwrap().iter().map(|e| BracketNote::from(e)).collect(),
            notes: log.notes.clone().unwrap(),
        }
    }
}

#[doc = "Client for Cartesi emulator machine server"]
pub struct JsonRpcCartesiMachineClient {
    server_address: String,
    client: RemoteCartesiMachine<jsonrpc_client_http::HttpHandle>,
}

impl JsonRpcCartesiMachineClient {
    /// Create new client instance. Connect to the server as part of client instantiation
    pub async fn new(server_address: String) -> Result<Self, jsonrpc_client_core::Error> {

        let transport = jsonrpc_client_http::HttpTransport::new()
            .standalone()
            .unwrap();
        let transport_handle = transport.handle(&server_address).unwrap();

        let mut remote_machine = RemoteCartesiMachine::new(transport_handle);
        match remote_machine.CheckConnection().call().err().unwrap().kind() {
            jsonrpc_client_core::ErrorKind::TransportError => {
                return Err(remote_machine.CheckConnection().call().err().unwrap())
            },
            _ => {},
        }
        Ok(JsonRpcCartesiMachineClient {
            server_address,
            client: remote_machine,
        })
    }

    /// Create new client instance. Connect to the server as part of client instantiation
    pub fn get_address(&self) -> &String {
        &self.server_address
    }

    /// Get Cartesi machine server version
    pub async fn get_version(&mut self) -> Result<SemanticVersion, Box<dyn std::error::Error>> {
        let response = self.client.GetVersion().call();
        match response {
            Ok(stub_semantic_version) => Ok(SemanticVersion::from(&stub_semantic_version)),
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(
                "Cound get retrieve semantic version",
            ))),
        }
    }

    // /// Create machine instance on remote Cartesi machine server
    pub async fn create_machine(
        &mut self,
        machine_config: &MachineConfig,
        machine_runtime_config: &MachineRuntimeConfig,
    ) -> Result<(), Box<jsonrpc_client_core::Error>> {
        let runtime = ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6::from(machine_runtime_config);
        let machine_oneof = ObjectOfObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5AObjectOfStringDoaGddGAMbKkyjX7ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6OObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGDObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4QObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQeiUnorderedSetOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6Fr01Y9HDOObjectOfInteger7Bd9WOt2KmrDcohfM5JY4BqN::from(machine_config);
        let response = self
            .client
            .MachineMachineConfig(machine_oneof, runtime)
            .call();
        if response.is_err() {
            return Err(Box::new(response.err().unwrap()))
        }
        Ok(())
    }

    /// Create machine from storage on remote Cartesi machine server
    pub async fn load_machine(
        &mut self,
        directory: &str,
        machine_runtime_config: &MachineRuntimeConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6::from(machine_runtime_config);
        let response = self
            .client
            .MachineMachineDirectory(directory.to_string(), runtime)
            .call()
            .unwrap();
        Ok(())
    }

    /// Run remote machine to maximum limit cycle
    pub async fn run(
        &mut self,
        limit: u64,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let response = self.client.MachineRun(limit).call().unwrap();
        Ok(response)
    }

    /// Serialize entire remote machine state to directory on cartesi machine server host
    pub async fn store(&mut self, directory: &str) -> Result<(), Box<jsonrpc_client_core::Error>> {
        let response = self
            .client
            .MachineStore(directory.to_string())
            .call();
        if response.is_err() {
            return Err(Box::new(response.err().unwrap()))
        }
        Ok(())
    }

    /// Destroy remote machine instance
    pub async fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.MachineDestroy().call();
        Ok(())
    }

    /// Fork remote machine
    pub async fn fork(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client.Fork().call().unwrap();
        Ok(response)
    }

    /// Shutdown the server
    pub async fn shutdown(&mut self) -> Result<(), Box<jsonrpc_client_core::Error>> {
        let response = self.client.Shutdown().call();
        if response.is_err() {
            return Err(Box::new(response.err().unwrap()))
        }
        Ok(())
    }

    /// Runs the remote machine for one cycle logging all accesses to the state
    pub async fn step(
        &mut self,
        log_type: &AccessLogType,
        one_based: bool,
    ) -> Result<AccessLog, Box<dyn std::error::Error>> {
        let log_type = ObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2 {
            has_proofs: log_type.proofs,
            has_annotations: log_type.annotations,
        };
        let response = self.client.MachineStepUarch(log_type, one_based).call();
        match response {
            Ok(log) => Ok(AccessLog::from(&log)),
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(
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
        let mut response = self
            .client
            .MachineReadMemory(address, length)
            .call()
            .unwrap();

        if response.ends_with('\n') {
            response.pop(); 
        }

        Ok(base64::decode(response).unwrap())
    }

    /// Writes a chunk of data to the remote machine memory
    pub async fn write_memory(
        &mut self,
        address: u64,
        data: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .MachineWriteMemory(address, data)
            .call()
            .unwrap();
        Ok(())
    }

    /// Read the value of a word in the remote machine state
    pub async fn read_word(&mut self, address: u64) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self.client.MachineReadWord(address).call();
        match response {
            Ok(read_word_response) => Ok(read_word_response),
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(
                "Failed to read word from required address",
            ))),
        }
    }

    /// Obtains the root hash of the Merkle tree for the remote machine
    pub async fn get_root_hash(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut response = self.client.MachineGetRootHash().call();
        match response {
            Ok(mut hash) => {
                if hash.ends_with('\n') {
                    hash.pop(); 
                }
                Ok(base64::decode(hash).unwrap())
            },
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(
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
        let response = self.client.MachineGetProof(address, log2_size).call();
        match response {
            Ok(proof) => Ok(MerkleTreeProof::from(&proof)),
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(&format!(
                "Error acquiring proof for address {} and log2_size {}",
                address, log2_size
            )))),
        }
    }

    /// Replaces a flash drive on a remote machine
    pub async fn replace_memory_range(
        &mut self,
        config: ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .MachineReplaceMemoryRange(config)
            .call()
            .unwrap();
        Ok(())
    }

    /// Gets the address of a general-purpose register
    pub async fn get_x_address(&mut self, index: u64) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self.client.MachineGetXAddress(index).call().unwrap();
        Ok(response)
    }

    /// Reads the value of a general-purpose register from the remote machine
    pub async fn read_x(&mut self, index: u64) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self.client.MachineReadX(index).call().unwrap();
        Ok(response)
    }

    /// Writes the value of a general-purpose register for the remote machine
    pub async fn write_x(
        &mut self,
        index: u64,
        value: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.MachineWriteX(index, value).call().unwrap();
        Ok(())
    }

    /// Resets the value of the iflags_Y flag on the remote machine
    pub async fn reset_iflags_y(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.MachineResetIflagsY().call().unwrap();
        Ok(())
    }

    /// Gets the address of any CSR
    pub async fn get_csr_address(&mut self, csr: String) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self
            .client
            .MachineGetCsrAddress(csr)
            .call()
            .unwrap();
        Ok(response)
    }

    /// Read the value of any CSR from remote machine
    pub async fn read_csr(&mut self, csr: String) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self
            .client
            .MachineReadCsr(csr)
            .call()
            .unwrap();
        Ok(response)
    }

    /// Writes the value of any CSR on remote machine
    pub async fn write_csr(
        &mut self,
        csr: String,
        value: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .MachineWriteCsr(csr, value)
            .call()
            .unwrap();
        Ok(())
    }

    /// Returns copy of initialization config of the remote machine
    pub async fn get_initial_config(
        &mut self,
    ) -> Result<MachineConfig, Box<dyn std::error::Error>> {
        let response = self.client.MachineGetInitialConfig().call();
        match response {
            Ok(def_config) => Ok(MachineConfig::from(&def_config)),
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(
                "Error acquiring initial configuration",
            ))),
        }
    }

    /// Verifies integrity of Merkle tree on the remote machine
    pub async fn verify_merkle_tree(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.client.MachineVerifyMerkleTree().call().unwrap();
        Ok(response)
    }

    /// Verify if dirty page maps are consistent on the remote machine
    pub async fn verify_dirty_page_maps(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.client.MachineVerifyDirtyPageMaps().call().unwrap();
        Ok(response)
    }

    /// Dump all memory ranges to files in current working directory on the server (for debugging purporses)
    pub async fn dump_pmas(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.MachineDumpPmas().call().unwrap();
        Ok(())
    }

    /// Returns copy of default system config from remote Cartesi machine server
    pub async fn get_default_config(
        &mut self,
    ) -> Result<MachineConfig, Box<dyn std::error::Error>> {
        let response = self.client.MachineGetDefaultConfig().call();
        match response {
            Ok(def_config) => Ok(MachineConfig::from(&def_config)),
            Err(_) => Err(Box::new(JsonrpcCartesiMachineError::new(
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let log = ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD::from(log);
        let runtime = ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6::from(runtime);

        let response = self
            .client
            .MachineVerifyAccessLog(log, runtime, one_based)
            .call()
            .unwrap();
        Ok(())
    }

    /// Checks the validity of a state transition
    pub async fn verify_state_transition(
        &mut self,
        root_hash_before: Vec<u8>,
        log: &AccessLog,
        root_hash_after: Vec<u8>,
        one_based: bool,
        runtime: &MachineRuntimeConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut root_hash_before = base64::encode(root_hash_before.clone());
        let mut root_hash_after = base64::encode(root_hash_after.clone());

        if root_hash_before.ends_with("=") {
            root_hash_before.push('\n');

        }
        if root_hash_after.ends_with("=") {
            root_hash_after.push('\n');
        }
        let log = ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD::from(log);
        //let root_hash_before = ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD::from(root_hash_before);
        //let root_hash_after = ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD::from(root_hash_after);
        let runtime = ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6::from(runtime);

        let response = self
            .client
            .MachineVerifyStateTransition(
                root_hash_before,
                log,
                root_hash_after,
                runtime,
                one_based
            )
            .call()
            .unwrap();
        Ok(())
    }
}
