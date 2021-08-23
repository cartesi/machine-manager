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

use cartesi_grpc_interfaces::grpc_stubs;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::processor_config::*;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::*;

impl From<&crate::MachineRuntimeConfig> for grpc_stubs::cartesi_machine::MachineRuntimeConfig {
    fn from(config: &crate::MachineRuntimeConfig) -> Self {
        grpc_stubs::cartesi_machine::MachineRuntimeConfig {
            dhd: Some(grpc_stubs::cartesi_machine::DhdRuntimeConfig {
                source_address: config.dhd.source_address.clone(),
            }),
            concurrency: Some(grpc_stubs::cartesi_machine::ConcurrencyConfig {
                update_merkle_tree: config.concurrency.update_merkle_tree,
            }),
        }
    }
}

impl From<&crate::Hash> for grpc_stubs::cartesi_machine::Hash {
    fn from(hash: &crate::Hash) -> Self {
        grpc_stubs::cartesi_machine::Hash {
            data: hash.0.iter().copied().collect(),
        }
    }
}

impl From<&crate::MerkleTreeProof> for grpc_stubs::cartesi_machine::MerkleTreeProof {
    fn from(proof: &crate::MerkleTreeProof) -> Self {
        grpc_stubs::cartesi_machine::MerkleTreeProof {
            target_address: proof.target_address,
            log2_root_size: proof.log2_root_size as u64,
            log2_target_size: proof.log2_target_size as u64,
            target_hash: Some(grpc_stubs::cartesi_machine::Hash::from(&proof.target_hash)),
            root_hash: Some(grpc_stubs::cartesi_machine::Hash::from(&proof.root_hash)),
            sibling_hashes: proof
                .sibling_hashes
                .iter()
                .map(grpc_stubs::cartesi_machine::Hash::from)
                .collect(),
        }
    }
}

impl From<&crate::Access> for grpc_stubs::cartesi_machine::Access {
    fn from(access: &crate::Access) -> Self {
        grpc_stubs::cartesi_machine::Access {
            address: access.address,
            r#type: access.r#type as i32,
            log2_size: access.log2_size as u64,
            read: access.read_data.clone(),
            written: access.written_data.clone(),
            proof: Some(grpc_stubs::cartesi_machine::MerkleTreeProof::from(
                &access.proof,
            )),
        }
    }
}

impl From<&crate::BracketNote> for grpc_stubs::cartesi_machine::BracketNote {
    fn from(bracket_note: &crate::BracketNote) -> Self {
        grpc_stubs::cartesi_machine::BracketNote {
            r#type: bracket_note.r#type as i32,
            r#where: bracket_note.r#where,
            text: bracket_note.text.clone(),
        }
    }
}

impl From<&crate::AccessLogType> for grpc_stubs::cartesi_machine::AccessLogType {
    fn from(log_type: &crate::AccessLogType) -> Self {
        grpc_stubs::cartesi_machine::AccessLogType {
            proofs: log_type.proofs,
            annotations: log_type.annotations,
        }
    }
}

impl From<&crate::AccessLog> for grpc_stubs::cartesi_machine::AccessLog {
    fn from(log: &crate::AccessLog) -> Self {
        grpc_stubs::cartesi_machine::AccessLog {
            log_type: Some(grpc_stubs::cartesi_machine::AccessLogType {
                proofs: log.log_type.proofs,
                annotations: log.log_type.annotations,
            }),
            accesses: log
                .accesses
                .iter()
                .map(grpc_stubs::cartesi_machine::Access::from)
                .collect(),
            brackets: log
                .brackets
                .iter()
                .map(grpc_stubs::cartesi_machine::BracketNote::from)
                .collect(),
            notes: log.notes.clone(),
        }
    }
}

pub fn convert_x_csr_field(
    config: &grpc_stubs::cartesi_machine::ProcessorConfig,
) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    result[0] = config.x1_oneof.as_ref().unwrap_or(&X1Oneof::X1(0)).into();
    result[1] = config.x2_oneof.as_ref().unwrap_or(&X2Oneof::X2(0)).into();
    result[2] = config.x3_oneof.as_ref().unwrap_or(&X3Oneof::X3(0)).into();
    result[3] = config.x4_oneof.as_ref().unwrap_or(&X4Oneof::X4(0)).into();
    result[4] = config.x5_oneof.as_ref().unwrap_or(&X5Oneof::X5(0)).into();
    result[5] = config.x6_oneof.as_ref().unwrap_or(&X6Oneof::X6(0)).into();
    result[6] = config.x7_oneof.as_ref().unwrap_or(&X7Oneof::X7(0)).into();
    result[7] = config.x8_oneof.as_ref().unwrap_or(&X8Oneof::X8(0)).into();
    result[8] = config.x9_oneof.as_ref().unwrap_or(&X9Oneof::X9(0)).into();
    result[9] = config
        .x10_oneof
        .as_ref()
        .unwrap_or(&X10Oneof::X10(0))
        .into();
    result[10] = config
        .x11_oneof
        .as_ref()
        .unwrap_or(&X11Oneof::X11(0))
        .into();
    result[11] = config
        .x12_oneof
        .as_ref()
        .unwrap_or(&X12Oneof::X12(0))
        .into();
    result[12] = config
        .x13_oneof
        .as_ref()
        .unwrap_or(&X13Oneof::X13(0))
        .into();
    result[13] = config
        .x14_oneof
        .as_ref()
        .unwrap_or(&X14Oneof::X14(0))
        .into();
    result[14] = config
        .x15_oneof
        .as_ref()
        .unwrap_or(&X15Oneof::X15(0))
        .into();
    result[15] = config
        .x16_oneof
        .as_ref()
        .unwrap_or(&X16Oneof::X16(0))
        .into();
    result[16] = config
        .x17_oneof
        .as_ref()
        .unwrap_or(&X17Oneof::X17(0))
        .into();
    result[17] = config
        .x18_oneof
        .as_ref()
        .unwrap_or(&X18Oneof::X18(0))
        .into();
    result[18] = config
        .x19_oneof
        .as_ref()
        .unwrap_or(&X19Oneof::X19(0))
        .into();
    result[19] = config
        .x20_oneof
        .as_ref()
        .unwrap_or(&X20Oneof::X20(0))
        .into();
    result[20] = config
        .x21_oneof
        .as_ref()
        .unwrap_or(&X21Oneof::X21(0))
        .into();
    result[21] = config
        .x22_oneof
        .as_ref()
        .unwrap_or(&X22Oneof::X22(0))
        .into();
    result[22] = config
        .x23_oneof
        .as_ref()
        .unwrap_or(&X23Oneof::X23(0))
        .into();
    result[23] = config
        .x24_oneof
        .as_ref()
        .unwrap_or(&X24Oneof::X24(0))
        .into();
    result[24] = config
        .x25_oneof
        .as_ref()
        .unwrap_or(&X25Oneof::X25(0))
        .into();
    result[25] = config
        .x26_oneof
        .as_ref()
        .unwrap_or(&X26Oneof::X26(0))
        .into();
    result[26] = config
        .x27_oneof
        .as_ref()
        .unwrap_or(&X27Oneof::X27(0))
        .into();
    result[27] = config
        .x28_oneof
        .as_ref()
        .unwrap_or(&X28Oneof::X28(0))
        .into();
    result[28] = config
        .x29_oneof
        .as_ref()
        .unwrap_or(&X29Oneof::X29(0))
        .into();
    result[29] = config
        .x30_oneof
        .as_ref()
        .unwrap_or(&X30Oneof::X30(0))
        .into();
    result[30] = config
        .x31_oneof
        .as_ref()
        .unwrap_or(&X31Oneof::X31(0))
        .into();
    result
}

pub fn convert_csr_field<'a, T>(field: &'a ::core::option::Option<T>) -> u64
where
    T: 'a,
    u64: From<&'a T>,
{
    match field {
        Some(x) => u64::from(x),
        None => 0,
    }
}

impl From<&crate::ProcessorConfig> for grpc_stubs::cartesi_machine::ProcessorConfig {
    fn from(config: &crate::ProcessorConfig) -> Self {
        grpc_stubs::cartesi_machine::ProcessorConfig {
            x1_oneof: Some(processor_config::X1Oneof::X1(config.x[0])),
            x2_oneof: Some(processor_config::X2Oneof::X2(config.x[1])),
            x3_oneof: Some(processor_config::X3Oneof::X3(config.x[2])),
            x4_oneof: Some(processor_config::X4Oneof::X4(config.x[3])),
            x5_oneof: Some(processor_config::X5Oneof::X5(config.x[4])),
            x6_oneof: Some(processor_config::X6Oneof::X6(config.x[5])),
            x7_oneof: Some(processor_config::X7Oneof::X7(config.x[6])),
            x8_oneof: Some(processor_config::X8Oneof::X8(config.x[7])),
            x9_oneof: Some(processor_config::X9Oneof::X9(config.x[8])),
            x10_oneof: Some(processor_config::X10Oneof::X10(config.x[9])),
            x11_oneof: Some(processor_config::X11Oneof::X11(config.x[10])),
            x12_oneof: Some(processor_config::X12Oneof::X12(config.x[11])),
            x13_oneof: Some(processor_config::X13Oneof::X13(config.x[12])),
            x14_oneof: Some(processor_config::X14Oneof::X14(config.x[13])),
            x15_oneof: Some(processor_config::X15Oneof::X15(config.x[14])),
            x16_oneof: Some(processor_config::X16Oneof::X16(config.x[15])),
            x17_oneof: Some(processor_config::X17Oneof::X17(config.x[16])),
            x18_oneof: Some(processor_config::X18Oneof::X18(config.x[17])),
            x19_oneof: Some(processor_config::X19Oneof::X19(config.x[18])),
            x20_oneof: Some(processor_config::X20Oneof::X20(config.x[19])),
            x21_oneof: Some(processor_config::X21Oneof::X21(config.x[20])),
            x22_oneof: Some(processor_config::X22Oneof::X22(config.x[21])),
            x23_oneof: Some(processor_config::X23Oneof::X23(config.x[22])),
            x24_oneof: Some(processor_config::X24Oneof::X24(config.x[23])),
            x25_oneof: Some(processor_config::X25Oneof::X25(config.x[24])),
            x26_oneof: Some(processor_config::X26Oneof::X26(config.x[25])),
            x27_oneof: Some(processor_config::X27Oneof::X27(config.x[26])),
            x28_oneof: Some(processor_config::X28Oneof::X28(config.x[27])),
            x29_oneof: Some(processor_config::X29Oneof::X29(config.x[28])),
            x30_oneof: Some(processor_config::X30Oneof::X30(config.x[29])),
            x31_oneof: Some(processor_config::X31Oneof::X31(config.x[30])),
            pc_oneof: Some(processor_config::PcOneof::Pc(config.pc)),
            mvendorid_oneof: Some(processor_config::MvendoridOneof::Mvendorid(
                config.mvendorid,
            )),
            marchid_oneof: Some(processor_config::MarchidOneof::Marchid(config.marchid)),
            mimpid_oneof: Some(processor_config::MimpidOneof::Mimpid(config.mimpid)),
            mcycle_oneof: Some(processor_config::McycleOneof::Mcycle(config.mcycle)),
            minstret_oneof: Some(processor_config::MinstretOneof::Minstret(config.minstret)),
            mstatus_oneof: Some(processor_config::MstatusOneof::Mstatus(config.mstatus)),
            mtvec_oneof: Some(processor_config::MtvecOneof::Mtvec(config.mtvec)),
            mscratch_oneof: Some(processor_config::MscratchOneof::Mscratch(config.mscratch)),
            mepc_oneof: Some(processor_config::MepcOneof::Mepc(config.mepc)),
            mcause_oneof: Some(processor_config::McauseOneof::Mcause(config.mcause)),
            mtval_oneof: Some(processor_config::MtvalOneof::Mtval(config.mtval)),
            misa_oneof: Some(processor_config::MisaOneof::Misa(config.misa)),
            mie_oneof: Some(processor_config::MieOneof::Mie(config.mie)),
            mip_oneof: Some(processor_config::MipOneof::Mip(config.mip)),
            medeleg_oneof: Some(processor_config::MedelegOneof::Medeleg(config.medeleg)),
            mideleg_oneof: Some(processor_config::MidelegOneof::Mideleg(config.mideleg)),
            mcounteren_oneof: Some(processor_config::McounterenOneof::Mcounteren(
                config.mcounteren,
            )),
            stvec_oneof: Some(processor_config::StvecOneof::Stvec(config.stvec)),
            sscratch_oneof: Some(processor_config::SscratchOneof::Sscratch(config.sscratch)),
            sepc_oneof: Some(processor_config::SepcOneof::Sepc(config.sepc)),
            scause_oneof: Some(processor_config::ScauseOneof::Scause(config.scause)),
            stval_oneof: Some(processor_config::StvalOneof::Stval(config.stval)),
            satp_oneof: Some(processor_config::SatpOneof::Satp(config.satp)),
            scounteren_oneof: Some(processor_config::ScounterenOneof::Scounteren(
                config.scounteren,
            )),
            ilrsc_oneof: Some(processor_config::IlrscOneof::Ilrsc(config.ilrsc)),
            iflags_oneof: Some(processor_config::IflagsOneof::Iflags(config.iflags)),
        }
    }
}

impl From<&crate::RamConfig> for grpc_stubs::cartesi_machine::RamConfig {
    fn from(config: &crate::RamConfig) -> Self {
        grpc_stubs::cartesi_machine::RamConfig {
            length: config.length,
            image_filename: config.image_filename.clone(),
        }
    }
}

impl From<&crate::RomConfig> for grpc_stubs::cartesi_machine::RomConfig {
    fn from(config: &crate::RomConfig) -> Self {
        grpc_stubs::cartesi_machine::RomConfig {
            bootargs: config.bootargs.clone(),
            image_filename: config.image_filename.clone(),
        }
    }
}

impl From<&crate::MemoryRangeConfig> for grpc_stubs::cartesi_machine::MemoryRangeConfig {
    fn from(config: &crate::MemoryRangeConfig) -> Self {
        grpc_stubs::cartesi_machine::MemoryRangeConfig {
            start: config.start,
            length: config.length,
            image_filename: config.image_filename.clone(),
            shared: config.shared,
        }
    }
}

impl From<&crate::DhdConfig> for grpc_stubs::cartesi_machine::DhdConfig {
    fn from(config: &crate::DhdConfig) -> Self {
        grpc_stubs::cartesi_machine::DhdConfig {
            tstart: config.tstart,
            tlength: config.tlength,
            image_filename: config.image_filename.clone(),
            dlength: config.dlength,
            hlength: config.hlength,
            h: config.h.to_vec(),
        }
    }
}

impl From<&crate::ClintConfig> for grpc_stubs::cartesi_machine::ClintConfig {
    fn from(config: &crate::ClintConfig) -> Self {
        grpc_stubs::cartesi_machine::ClintConfig {
            mtimecmp_oneof: Some(
                grpc_stubs::cartesi_machine::clint_config::MtimecmpOneof::Mtimecmp(config.mtimecmp),
            ),
        }
    }
}

impl From<&crate::HtifConfig> for grpc_stubs::cartesi_machine::HtifConfig {
    fn from(config: &crate::HtifConfig) -> Self {
        grpc_stubs::cartesi_machine::HtifConfig {
            console_getchar: config.console_getchar,
            yield_manual: config.yield_manual,
            yield_automatic: config.yield_automatic,
            fromhost_oneof: Some(
                grpc_stubs::cartesi_machine::htif_config::FromhostOneof::Fromhost(config.fromhost),
            ),
            tohost_oneof: Some(
                grpc_stubs::cartesi_machine::htif_config::TohostOneof::Tohost(config.fromhost),
            ),
        }
    }
}

impl From<&crate::RollupConfig> for grpc_stubs::cartesi_machine::RollupConfig {
    fn from(config: &crate::RollupConfig) -> Self {
        grpc_stubs::cartesi_machine::RollupConfig {
            input_metadata: match &config.input_metadata {
                Some(config) => Some(grpc_stubs::cartesi_machine::MemoryRangeConfig::from(config)),
                None => None,
            },
            tx_buffer: match &config.tx_buffer {
                Some(config) => Some(grpc_stubs::cartesi_machine::MemoryRangeConfig::from(config)),
                None => None,
            },
            voucher_hashes: match &config.voucher_hashes {
                Some(config) => Some(grpc_stubs::cartesi_machine::MemoryRangeConfig::from(config)),
                None => None,
            },
            rx_buffer: match &config.rx_buffer {
                Some(config) => Some(grpc_stubs::cartesi_machine::MemoryRangeConfig::from(config)),
                None => None,
            },
            notice_hashes: match &config.notice_hashes {
                Some(config) => Some(grpc_stubs::cartesi_machine::MemoryRangeConfig::from(config)),
                None => None,
            },
        }
    }
}

impl From<&crate::MachineConfig> for grpc_stubs::cartesi_machine::MachineConfig {
    fn from(config: &crate::MachineConfig) -> Self {
        grpc_stubs::cartesi_machine::MachineConfig {
            processor: Some(grpc_stubs::cartesi_machine::ProcessorConfig::from(
                &config.processor,
            )),
            ram: Some(grpc_stubs::cartesi_machine::RamConfig::from(&config.ram)),
            rom: Some(grpc_stubs::cartesi_machine::RomConfig::from(&config.rom)),
            flash_drive: config
                .flash_drives
                .iter()
                .map(grpc_stubs::cartesi_machine::MemoryRangeConfig::from)
                .collect(),
            clint: Some(grpc_stubs::cartesi_machine::ClintConfig::from(
                &config.clint,
            )),
            htif: Some(grpc_stubs::cartesi_machine::HtifConfig::from(&config.htif)),
            dhd: Some(grpc_stubs::cartesi_machine::DhdConfig::from(&config.dhd)),
            rollup: Some(grpc_stubs::cartesi_machine::RollupConfig::from(
                &config.rollup,
            )),
        }
    }
}
