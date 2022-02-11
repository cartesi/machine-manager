use crate::compare_hashes;
use crate::steps::new_session::{open_session_with_default_config, open_verification_session, close_sessions};
use crate::steps::session_get_proof::proof_to_json;
use crate::steps::session_run::{run_machine, strs_to_uints};
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use json::object;
use rust_test_client::stubs::cartesi_machine::{AccessLog, StepResponse};
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
            let (ret, manager_request) =
                open_session_with_default_config(&mut world, &ctx, true).await;
            if let Err(e) = ret {
                panic!("New session request failed: {}", e);
            }

            let ret = run_machine(strs_to_uints(&ctx.matches), &mut world.client_proxy).await;
            if let session_run_response::RunOneof::Progress(_) = ret.run_oneof.as_ref().unwrap() {
                panic!("Invalid state: server job didn't finish");
            }

            open_verification_session(&mut world, &ctx, manager_request).await;
            for cycle in strs_to_uints(&ctx.matches) {
                let request = world.machine_proxy.build_run_request(cycle);
                if let Err(e) = world
                    .machine_proxy
                    .grpc_client
                    .as_mut()
                    .unwrap()
                    .run(request)
                    .await
                {
                    panic!("Unable to make verification run: {}", e);
                }
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
                .session_step(request.clone())
                .await
            {
                Ok(val) => {
                    let verification_request = world.machine_proxy.build_step_request(request);
                    let verification_response = world
                        .machine_proxy
                        .grpc_client
                        .as_mut()
                        .unwrap()
                        .step(verification_request)
                        .await;
                    if let Err(e) = verification_response {
                        panic!("Unable to make verification step: {}", e);
                    }

                    world.response.insert(
                        String::from("verification_response"),
                        Box::new(verification_response.unwrap().into_inner()),
                    );
                    world
                        .response
                        .insert(String::from("response"), Box::new(val.into_inner()))
                }
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };
            world
        }),
    );

    steps.then_async(
        "server returns correct access log",
        t!(|mut world, _ctx| {
            let response = world
                .response
                .get(&String::from("response"))
                .and_then(|x| x.downcast_ref::<SessionStepResponse>())
                .take()
                .expect("No SessionStepResponse type in the result");
            let verification_response = world
                .response
                .get(&String::from("verification_response"))
                .and_then(|x| x.downcast_ref::<StepResponse>())
                .take()
                .expect("No verification StepResponse type in the result");
            let log_string = access_log_to_json(&response.log.as_ref().unwrap());
            let verification_log_string =
                access_log_to_json(&verification_response.log.as_ref().unwrap());
            assert!(compare_hashes(
                &sha2::Sha256::digest(log_string.as_bytes()),
                &sha2::Sha256::digest(verification_log_string.as_bytes()),
            ));
            close_sessions(&mut world).await;
            world
        }),
    );

    steps
}
