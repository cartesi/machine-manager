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
    result[0] = convert_csr_field(config.x1);
    result[1] = convert_csr_field(config.x2);
    result[2] = convert_csr_field(config.x3);
    result[3] = convert_csr_field(config.x4);
    result[4] = convert_csr_field(config.x5);
    result[5] = convert_csr_field(config.x6);
    result[6] = convert_csr_field(config.x7);
    result[7] = convert_csr_field(config.x8);
    result[8] = convert_csr_field(config.x9);
    result[9] = convert_csr_field(config.x10);
    result[10] = convert_csr_field(config.x11);
    result[11] = convert_csr_field(config.x12);
    result[12] = convert_csr_field(config.x13);
    result[13] = convert_csr_field(config.x14);
    result[14] = convert_csr_field(config.x15);
    result[15] = convert_csr_field(config.x16);
    result[16] = convert_csr_field(config.x17);
    result[17] = convert_csr_field(config.x18);
    result[18] = convert_csr_field(config.x19);
    result[19] = convert_csr_field(config.x20);
    result[20] = convert_csr_field(config.x21);
    result[21] = convert_csr_field(config.x22);
    result[22] = convert_csr_field(config.x23);
    result[23] = convert_csr_field(config.x24);
    result[24] = convert_csr_field(config.x25);
    result[25] = convert_csr_field(config.x26);
    result[26] = convert_csr_field(config.x27);
    result[27] = convert_csr_field(config.x28);
    result[28] = convert_csr_field(config.x29);
    result[29] = convert_csr_field(config.x30);
    result[30] = convert_csr_field(config.x31);
    result
}

pub fn convert_csr_field(field: ::core::option::Option<u64>) -> u64
where
{
    match field {
        Some(x) => u64::from(x),
        None => 0,
    }
}

impl From<&crate::ProcessorConfig> for grpc_stubs::cartesi_machine::ProcessorConfig {
    fn from(config: &crate::ProcessorConfig) -> Self {
        grpc_stubs::cartesi_machine::ProcessorConfig {
            x1: Some(config.x[0]),
            x2: Some(config.x[1]),
            x3: Some(config.x[2]),
            x4: Some(config.x[3]),
            x5: Some(config.x[4]),
            x6: Some(config.x[5]),
            x7: Some(config.x[6]),
            x8: Some(config.x[7]),
            x9: Some(config.x[8]),
            x10: Some(config.x[9]),
            x11: Some(config.x[10]),
            x12: Some(config.x[11]),
            x13: Some(config.x[12]),
            x14: Some(config.x[13]),
            x15: Some(config.x[14]),
            x16: Some(config.x[15]),
            x17: Some(config.x[16]),
            x18: Some(config.x[17]),
            x19: Some(config.x[18]),
            x20: Some(config.x[19]),
            x21: Some(config.x[20]),
            x22: Some(config.x[21]),
            x23: Some(config.x[22]),
            x24: Some(config.x[23]),
            x25: Some(config.x[24]),
            x26: Some(config.x[25]),
            x27: Some(config.x[26]),
            x28: Some(config.x[27]),
            x29: Some(config.x[28]),
            x30: Some(config.x[29]),
            x31: Some(config.x[30]),
            pc: Some(config.pc),
            mvendorid: Some(config.mvendorid),
            marchid: Some(config.marchid),
            mimpid: Some(config.mimpid),
            mcycle: Some(config.mcycle),
            minstret: Some(config.minstret),
            mstatus: Some(config.mstatus),
            mtvec: Some(config.mtvec),
            mscratch: Some(config.mscratch),
            mepc: Some(config.mepc),
            mcause: Some(config.mcause),
            mtval: Some(config.mtval),
            misa: Some(config.misa),
            mie: Some(config.mie),
            mip: Some(config.mip),
            medeleg: Some(config.medeleg),
            mideleg: Some(config.mideleg),
            mcounteren: Some(config.mcounteren),
            stvec: Some(config.stvec),
            sscratch: Some(config.sscratch),
            sepc: Some(config.sepc),
            scause: Some(config.scause),
            stval: Some(config.stval),
            satp: Some(config.satp),
            scounteren: Some(config.scounteren),
            ilrsc: Some(config.ilrsc),
            iflags: Some(config.iflags)
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
                config.clint.clone(),
            )),
            htif: Some(grpc_stubs::cartesi_machine::HtifConfig::from(config.htif.clone())),
            dhd: Some(grpc_stubs::cartesi_machine::DhdConfig::from(&config.dhd)),
            rollup: Some(grpc_stubs::cartesi_machine::RollupConfig::from(
                &config.rollup,
            )),
        }
    }
}
