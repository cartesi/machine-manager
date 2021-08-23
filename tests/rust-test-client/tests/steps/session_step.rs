use crate::compare_hashes;
use crate::steps::new_session::open_session_with_default_config;
use crate::steps::session_get_proof::proof_to_json;
use crate::steps::session_run::{run_machine, strs_to_uints};
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use json::object;
use rust_test_client::stubs::cartesi_machine::AccessLog;
use rust_test_client::stubs::cartesi_machine_manager::*;
use sha2::Digest;
use std::boxed::Box;

fn access_log_to_json(input: &AccessLog) -> String {
    let mut out = object! {
        log_type: {
            proofs: input.log_type.as_ref().unwrap().proofs,
            annotations: input.log_type.as_ref().unwrap().annotations
        },
        accesses: [],
        brackets: input.brackets.iter().map(|x| x.text.clone()).collect::<Vec<String>>(),
        notes: input.notes.clone()
    };

    for access in &input.accesses {
        let access_json = object! {
            read: access.read.clone(),
            write: access.written.clone(),
            proof: proof_to_json(access.proof.as_ref().unwrap()),
            address: access.address.clone(),
            access: access.log2_size.clone(),
        };
        out["accesses"]
            .push(access_json)
            .expect("Unexpected error while building AccessLog JSON");
    }

    out.dump()
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given_regex_async(
        r#"a machine manager server with a machine executed for ((\d+,)*\d+) final cycles"#,
        t!(|mut world, ctx| {
            if let Err(e) = open_session_with_default_config(&mut world, &ctx, true).await {
                panic!("New session request failed: {}", e);
            }

            let ret = run_machine(strs_to_uints(&ctx.matches), &mut world.client_proxy).await;
            if let session_run_response::RunOneof::Progress(_) = ret.run_oneof.as_ref().unwrap() {
                panic!("Invalid state: server job didn't finish");
            }
            world
        }),
    );

    steps.when_regex_async(
        r#"the machine manager server asks machine to step on initial cycle (\d+)"#,
        t!(|mut world, ctx| {
            let request = world
                .client_proxy
                .build_new_session_step_request(ctx.matches[1].parse::<u64>().unwrap());
            match world
                .client_proxy
                .grpc_client
                .as_mut()
                .unwrap()
                .session_step(request)
                .await
            {
                Ok(val) => world
                    .response
                    .insert(String::from("response"), Box::new(val.into_inner())),
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };
            world
        }),
    );

    steps.then_regex_async(
        r#"server returns access log which SHA256 sum is ((\d|[A-Z]){64})"#,
        t!(|mut world, ctx| {
            let response = world
                .response
                .get(&String::from("response"))
                .and_then(|x| x.downcast_ref::<SessionStepResponse>())
                .take()
                .expect("No SessionStepResponse type in the result");
            let log_string = access_log_to_json(&response.log.as_ref().unwrap());
            assert!(compare_hashes(
                &sha2::Sha256::digest(log_string.as_bytes()),
                &ctx.matches[1][..]
            ));
            world
        }),
    );

    steps
}
