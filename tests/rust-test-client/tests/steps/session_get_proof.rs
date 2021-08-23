use crate::compare_hashes;
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use json::object;
use rust_test_client::stubs::cartesi_machine::MerkleTreeProof;
use sha2::Digest;
use std::boxed::Box;

pub fn proof_to_json(input: &MerkleTreeProof) -> String {
    let out = object! {
        target_address: input.target_address.clone(),
        log2_target_size: input.log2_target_size.clone(),
        target_hash: input.target_hash.as_ref().unwrap().data.clone(),
        log2_root_size: input.log2_root_size.clone(),
        root_hash: input.root_hash.as_ref().unwrap().data.clone(),
        sibling_hashes: input.sibling_hashes.iter().map(|x| x.data.clone()).collect::<Vec<Vec<u8>>>(),
    };

    out.dump()
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.when_regex_async(
        r#"the machine manager server asks machine for proof on cycle (\d+) for address (\d+) with log2_size (\d+)"#,
    t!(|mut world, ctx| {
        let request = world.client_proxy.build_new_session_get_proof_request(
            ctx.matches[1].parse::<u64>().unwrap(),
            ctx.matches[2].parse::<u64>().unwrap(),
            ctx.matches[3].parse::<u64>().unwrap());
        match world.client_proxy.grpc_client.as_mut().unwrap().session_get_proof(request).await {
            Ok(val) => world.response.insert(String::from("response"), Box::new(val.into_inner())),
            Err(e) => panic!("Unable to perform get_proof request: {}", e),
        };
        world
    }));

    steps.then_regex_async(
        r#"server returns proof which SHA256 sum is ((\d|[A-Z]){64})"#,
        t!(|mut world, ctx| {
            let response = world
                .response
                .get(&String::from("response"))
                .and_then(|x| x.downcast_ref::<MerkleTreeProof>())
                .take()
                .expect("No MerkleTreeProof in the result");
            let proof_string = proof_to_json(response);
            assert!(compare_hashes(
                &sha2::Sha256::digest(proof_string.as_bytes()),
                &ctx.matches[1][..]
            ));
            world
        }),
    );

    steps
}
