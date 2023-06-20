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

use cartesi_jsonrpc_interfaces::index::*;
impl From<&crate::MachineRuntimeConfig> for ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6 {
    fn from(rc: &crate::MachineRuntimeConfig) -> Self {
        ObjectOfObjectOfInteger7Bd9WOt2INmaA59KDcXbowU6 {
            concurrency: Some(ObjectOfInteger7Bd9WOt2INmaA59K {
                update_merkle_tree: Some(rc.concurrency.update_merkle_tree),
            }),
        }
    }
}
impl From<&crate::MerkleTreeProof> for ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQ {
    fn from(proof: &crate::MerkleTreeProof) -> Self {
        ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQ {
            target_address: proof.target_address,
            log_2_target_size: proof.log2_target_size as u64,
            log_2_root_size: proof.log2_root_size as u64,
            target_hash: proof.target_hash.clone(),
            root_hash: proof.root_hash.clone(),
            sibling_hashes: proof
                .sibling_hashes.clone()
        }
    }
}

impl From<&crate::Access> for ObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85Fz {
    fn from(access: &crate::Access) -> Self {
        let mut read = base64::encode(access.read_data.clone());
        let mut written = base64::encode(access.written_data.clone());

        if read.ends_with("=") {
            read.push('\n');

        }
        if written.ends_with("=") {
            written.push('\n');
        }

        ObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85Fz {
            r#type: match access.r#type {
                crate::AccessType::Read => serde_json::json!("read"),
                crate::AccessType::Write => serde_json::json!("write"),
            },
            read: read,
            written: Some(written),
            proof: Some(ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQ::from(&access.proof)),
            address: access.address,
            log_2_size: access.log2_size as u64,
        }
    }
}

impl std::convert::From<&crate::BracketNote> for ObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0 {
    fn from(bracket_note: &crate::BracketNote) -> Self {
        ObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0 {
            _type: match bracket_note.r#type {
                crate::BracketType::Begin => serde_json::json!("begin"),
                crate::BracketType::End => serde_json::json!("end"),
            },
            r#where: bracket_note.r#where,
            text: bracket_note.text.clone(),
        }
    }
}

impl From<&crate::AccessLogType> for ObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2 {
    fn from(log_type: &crate::AccessLogType) -> Self {
        ObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2 {
            has_proofs: log_type.proofs,
            has_annotations: log_type.annotations,
        }
    }
}
impl From<&crate::AccessLog> for ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD {
    fn from(log: &crate::AccessLog) -> Self {
        let log_type = ObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2 {
                has_proofs: log.log_type.proofs,
                has_annotations: log.log_type.annotations
        };
        ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaObjectOfBooleanVyG3AEThBooleanVyG3AEThBLOCJyD2UnorderedSetOfObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0XfimEAp1UnorderedSetOfObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85FzJF6PfWW6LwdUyvkD {
            log_type,
            accesses: log.accesses.iter().map(|e| ObjectOfStringBlBqGDJ5AnySqN23SiJStringBlBqGDJ5ObjectOfStringLtCQe6EUInteger7Bd9WOt2UnorderedSetOfStringLtCQe6EUR3HDjcx3StringLtCQe6EUInteger7Bd9WOt2Integer7Bd9WOt27JClvfPQInteger7Bd9WOt2Integer7Bd9WOt2GCdT85Fz::from(e)).collect(),
            brackets: Some(log.brackets.iter().map(|e| ObjectOfInteger7Bd9WOt2AnyIiyrobPFStringDoaGddGAMDICVpd0::from(e)).collect()),
            notes: Some(log.notes.clone()),
        }
    }
}

pub fn convert_x_csr_field(
    config: &ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q,
) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    result[0] = convert_csr_field(Some(config.x.clone().unwrap()[0]));
    result[1] = convert_csr_field(Some(config.x.clone().unwrap()[1]));
    result[2] = convert_csr_field(Some(config.x.clone().unwrap()[2]));
    result[3] = convert_csr_field(Some(config.x.clone().unwrap()[3]));
    result[4] = convert_csr_field(Some(config.x.clone().unwrap()[4]));
    result[5] = convert_csr_field(Some(config.x.clone().unwrap()[5]));
    result[6] = convert_csr_field(Some(config.x.clone().unwrap()[6]));
    result[7] = convert_csr_field(Some(config.x.clone().unwrap()[7]));
    result[8] = convert_csr_field(Some(config.x.clone().unwrap()[8]));
    result[9] = convert_csr_field(Some(config.x.clone().unwrap()[9]));
    result[10] = convert_csr_field(Some(config.x.clone().unwrap()[10]));
    result[11] = convert_csr_field(Some(config.x.clone().unwrap()[11]));
    result[12] = convert_csr_field(Some(config.x.clone().unwrap()[12]));
    result[13] = convert_csr_field(Some(config.x.clone().unwrap()[13]));
    result[14] = convert_csr_field(Some(config.x.clone().unwrap()[14]));
    result[15] = convert_csr_field(Some(config.x.clone().unwrap()[15]));
    result[16] = convert_csr_field(Some(config.x.clone().unwrap()[16]));
    result[17] = convert_csr_field(Some(config.x.clone().unwrap()[17]));
    result[18] = convert_csr_field(Some(config.x.clone().unwrap()[18]));
    result[19] = convert_csr_field(Some(config.x.clone().unwrap()[19]));
    result[20] = convert_csr_field(Some(config.x.clone().unwrap()[20]));
    result[21] = convert_csr_field(Some(config.x.clone().unwrap()[21]));
    result[22] = convert_csr_field(Some(config.x.clone().unwrap()[22]));
    result[23] = convert_csr_field(Some(config.x.clone().unwrap()[23]));
    result[24] = convert_csr_field(Some(config.x.clone().unwrap()[24]));
    result[25] = convert_csr_field(Some(config.x.clone().unwrap()[25]));
    result[26] = convert_csr_field(Some(config.x.clone().unwrap()[26]));
    result[27] = convert_csr_field(Some(config.x.clone().unwrap()[27]));
    result[28] = convert_csr_field(Some(config.x.clone().unwrap()[28]));
    result[29] = convert_csr_field(Some(config.x.clone().unwrap()[29]));
    result[30] = convert_csr_field(Some(config.x.clone().unwrap()[30]));
    result
}

pub fn convert_f_csr_field(
    config: &ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q,
) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    result[0] = convert_csr_field(Some(config.f.clone().unwrap()[0]));
    result[1] = convert_csr_field(Some(config.f.clone().unwrap()[1]));
    result[2] = convert_csr_field(Some(config.f.clone().unwrap()[2]));
    result[3] = convert_csr_field(Some(config.f.clone().unwrap()[3]));
    result[4] = convert_csr_field(Some(config.f.clone().unwrap()[4]));
    result[5] = convert_csr_field(Some(config.f.clone().unwrap()[5]));
    result[6] = convert_csr_field(Some(config.f.clone().unwrap()[6]));
    result[7] = convert_csr_field(Some(config.f.clone().unwrap()[7]));
    result[8] = convert_csr_field(Some(config.f.clone().unwrap()[8]));
    result[9] = convert_csr_field(Some(config.f.clone().unwrap()[9]));
    result[10] = convert_csr_field(Some(config.f.clone().unwrap()[10]));
    result[11] = convert_csr_field(Some(config.f.clone().unwrap()[11]));
    result[12] = convert_csr_field(Some(config.f.clone().unwrap()[12]));
    result[13] = convert_csr_field(Some(config.f.clone().unwrap()[13]));
    result[14] = convert_csr_field(Some(config.f.clone().unwrap()[14]));
    result[15] = convert_csr_field(Some(config.f.clone().unwrap()[15]));
    result[16] = convert_csr_field(Some(config.f.clone().unwrap()[16]));
    result[17] = convert_csr_field(Some(config.f.clone().unwrap()[17]));
    result[18] = convert_csr_field(Some(config.f.clone().unwrap()[18]));
    result[19] = convert_csr_field(Some(config.f.clone().unwrap()[19]));
    result[20] = convert_csr_field(Some(config.f.clone().unwrap()[20]));
    result[21] = convert_csr_field(Some(config.f.clone().unwrap()[21]));
    result[22] = convert_csr_field(Some(config.f.clone().unwrap()[22]));
    result[23] = convert_csr_field(Some(config.f.clone().unwrap()[23]));
    result[24] = convert_csr_field(Some(config.f.clone().unwrap()[24]));
    result[25] = convert_csr_field(Some(config.f.clone().unwrap()[25]));
    result[26] = convert_csr_field(Some(config.f.clone().unwrap()[26]));
    result[27] = convert_csr_field(Some(config.f.clone().unwrap()[27]));
    result[28] = convert_csr_field(Some(config.f.clone().unwrap()[28]));
    result[29] = convert_csr_field(Some(config.f.clone().unwrap()[29]));
    result[30] = convert_csr_field(Some(config.f.clone().unwrap()[30]));
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

impl From<&crate::ProcessorConfig> for ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q {
    fn from(config: &crate::ProcessorConfig) -> Self {
        ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q {
            x: Some(config.x.to_vec()),
            f: Some(config.f.to_vec()),
            pc: Some(config.pc),
            mvendorid: Some(config.mvendorid),
            marchid: Some(config.marchid),
            mimpid: Some(config.mimpid),
            mcycle: Some(config.mcycle),
            icycleinstret: Some(config.icycleinstret),
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
            iflags: Some(config.iflags),
            menvcfg: Some(config.menvcfg),
            senvcfg: Some(config.senvcfg),
            fcsr: Some(config.fcsr),
        }
    }
}

impl From<&crate::RamConfig> for ObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0 {
    fn from(config: &crate::RamConfig) -> Self {
        ObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0 {
            length: config.length,
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&crate::RomConfig> for ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6O {
    fn from(config: &crate::RomConfig) -> Self {
        ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6O {
            bootargs: Some(config.bootargs.clone()),
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&crate::TlbConfig> for ObjectOfStringDoaGddGAMbKkyjX7 {
    fn from(config: &crate::TlbConfig) -> Self {
        ObjectOfStringDoaGddGAMbKkyjX7 {
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&crate::UarchConfig> for ObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5A {
    fn from(config: &crate::UarchConfig) -> Self {
        ObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5A {
            processor: config.processor.clone(),
            ram: config.ram.clone(),
        }
    }
}

impl From<&crate::MemoryRangeConfig> for ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F {
    fn from(config: &crate::MemoryRangeConfig) -> Self {
        ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F {
            start: Some(config.start),
            length: Some(config.length),
            image_filename: Some(config.image_filename.clone()),
            shared: Some(config.shared),
        }
    }
}

impl From<&crate::RollupConfig> for ObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGD {
    fn from(config: &crate::RollupConfig) -> Self {
        ObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGD {
            input_metadata: match &config.input_metadata {
                Some(config) => Some(ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F::from(config)),
                None => None,
            },
            tx_buffer: match &config.tx_buffer {
                Some(config) => Some(ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F::from(config)),
                None => None,
            },
            voucher_hashes: match &config.voucher_hashes {
                Some(config) => Some(ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F::from(config)),
                None => None,
            },
            rx_buffer: match &config.rx_buffer {
                Some(config) => Some(ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F::from(config)),
                None => None,
            },
            notice_hashes: match &config.notice_hashes {
                Some(config) => Some(ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F::from(config)),
                None => None,
            },
        }
    }
}

impl From<&crate::MachineConfig> for ObjectOfObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5AObjectOfStringDoaGddGAMbKkyjX7ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6OObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGDObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4QObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQeiUnorderedSetOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6Fr01Y9HDOObjectOfInteger7Bd9WOt2KmrDcohfM5JY4BqN {
    fn from(config: &crate::MachineConfig) -> Self {
        ObjectOfObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5AObjectOfStringDoaGddGAMbKkyjX7ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6OObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGDObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4QObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQeiUnorderedSetOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6Fr01Y9HDOObjectOfInteger7Bd9WOt2KmrDcohfM5JY4BqN {
            processor: Some(ObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2Integer7Bd9WOt2UnorderedSetOfInteger7Bd9WOt2MMEUfR9YBevRvl4Q::from(
                &config.processor,
            )),
            ram: Some(ObjectOfInteger7Bd9WOt2StringDoaGddGAChHEqCz0::from(&config.ram)),
            rom: Some(ObjectOfStringDoaGddGAStringDoaGddGAGb16ED6O::from(&config.rom)),
            tlb: Some(ObjectOfStringDoaGddGAMbKkyjX7::from(&config.tlb)),
            uarch: Some(ObjectOfObjectOfInteger7Bd9WOt2StringDoaGddGAJbt7HebBObjectOfUnorderedSetOfInteger7Bd9WOt2MMEUfR9YInteger7Bd9WOt2Integer7Bd9WOt2RuFwo0CWPjNtUa5A::from(
                &config.uarch,
            )),
            flash_drive: Some(config
                .flash_drives
                .iter()
                .map(|e| ObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6F::from(e))
                .collect()),
            clint: Some(ObjectOfInteger7Bd9WOt2KmrDcohf::from(
                config.clint.clone(),
            )),
            htif: Some(ObjectOfBooleanVyG3AEThBooleanVyG3AEThInteger7Bd9WOt2Integer7Bd9WOt2BooleanVyG3AEThUydnhQei::from(
                config.htif.clone(),
            )),
            rollup: Some(ObjectOfObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FObjectOfInteger7Bd9WOt2BooleanVyG3AEThInteger7Bd9WOt2StringDoaGddGAZZfVcS6FHGjLuuGD::from(
                &config.rollup,
            )),
        }
    }
}
