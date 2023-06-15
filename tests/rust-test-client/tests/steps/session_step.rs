// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use crate::compare_hashes;
use crate::steps::new_session::{open_session_with_default_config, open_verification_session, close_sessions};
use crate::steps::session_get_proof::proof_to_json;
use crate::steps::session_run::{run_machine, strs_to_uints};
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use json::object;
use rust_test_client::stubs::cartesi_machine::{AccessLog, StepUarchResponse};
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
        r#"a machine manager server with a machine executed for ((\d+,)*\d+) final cycles and ((\d+,)*\d+) final ucycles"#,
        t!(|mut world, ctx| {
            let (ret, manager_request) =
                open_session_with_default_config(&mut world, &ctx, true).await;
            if let Err(e) = ret {
                panic!("New session request failed: {}", e);
            }

            let cycles = strs_to_uints(&ctx.matches[1]);
            let ucycles = strs_to_uints(&ctx.matches[3]);
            let ret = run_machine(cycles.clone(), ucycles.clone(), &mut world.client_proxy).await;
            if let session_run_response::RunOneof::Progress(_) = ret.run_oneof.as_ref().unwrap() {
                panic!("Invalid state: server job didn't finish");
            }

            open_verification_session(&mut world, &ctx, manager_request).await;
            for idx in 1..cycles.len() {
                let request = world.machine_proxy.build_run_request(cycles[idx]);
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

                let uarch_request = world.machine_proxy.build_run_uarch_request(ucycles[idx]);
                if let Err(e) = world
                    .machine_proxy
                    .grpc_client
                    .as_mut()
                    .unwrap()
                    .run_uarch(uarch_request)
                    .await
                {
                    panic!("Unable to make verification run uarch: {}", e);
                }
            }

            world
        }),
    );

    steps.when_regex_async(
        r#"the machine manager server asks machine to step on initial cycle (\d+) and ucycle (\d+)"#,
        t!(|mut world, ctx| {
            let request = world
                .client_proxy
                .build_new_session_step_request(
                    ctx.matches[1].parse::<u64>().unwrap(),
                    ctx.matches[2].parse::<u64>().unwrap()
                );
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
                        .step_uarch(verification_request)
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
                .and_then(|x| x.downcast_ref::<StepUarchResponse>())
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
