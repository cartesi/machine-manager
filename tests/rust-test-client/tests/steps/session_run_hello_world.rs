// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use crate::compare_hashes;
use crate::steps::new_session::open_session_with_hello_world_config;
use crate::steps::session_run::{run_machine, strs_to_uints};
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use rust_test_client::stubs::cartesi_machine::{Hash, RunResponse};
use rust_test_client::stubs::cartesi_machine_manager::*;
use std::boxed::Box;

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given_async(
        "a pristine machine manager hello world server session",
        t!(|mut world, ctx| {
            if let Err(e) = open_session_with_hello_world_config(&mut world, &ctx, true).await {
                panic!("New session request failed: {}", e);
            }
            world
        }),
    );
    steps.given_regex_async(
        r#"the machine executed with cycles ((\d+,)*\d+)"#,
        t!(|mut world, ctx| {
            let ret = run_machine(strs_to_uints(&ctx.matches), &mut world.client_proxy).await;
            if let session_run_response::RunOneof::Progress(_) = ret.run_oneof.as_ref().unwrap() {
                panic!("Invalid state: server job didn't finish");
            }
            world
        }),
    );
    steps.given_regex(
        r#"the cycles array ((\d+,)*\d+) to run the machine"#,
        |mut world, ctx| {
            world.response.insert(
                String::from("exec_cycles"),
                Box::new(strs_to_uints(&ctx.matches)),
            );
            world
        },
    );
    steps.when_async(
        "client asks server to run hello world session",
        t!(|mut world, _ctx| {
            let cycles = world
                .response
                .get(&String::from("exec_cycles"))
                .and_then(|x| x.downcast_ref::<Vec<u64>>())
                .take()
                .expect("No Vec<u64> type in the result");
            let ret = run_machine(cycles.to_vec(), &mut world.client_proxy).await;
            if let session_run_response::RunOneof::Result(result) = ret.run_oneof.as_ref().unwrap()
            {
                let result_hashes: Vec<Hash> = result.hashes.clone();
                let result_responses: Vec<RunResponse> = result.summaries.clone();
                world
                    .response
                    .insert(String::from("hashes"), Box::new(result_hashes));
                world
                    .response
                    .insert(String::from("summaries"), Box::new(result_responses));
                world
            } else {
                panic!("Invalid state: server job didn't finish");
            }
        }),
    );
    steps.then(
        "server returns machine hashes of hello world machine:",
        |world, ctx| {
            let result_hashes = world
                .response
                .get(&String::from("hashes"))
                .and_then(|x| x.downcast_ref::<Vec<Hash>>())
                .take()
                .expect("No Vec<Hash> type in the result");
            let result_summaries = world
                .response
                .get(&String::from("summaries"))
                .and_then(|x| x.downcast_ref::<Vec<RunResponse>>())
                .take()
                .expect("No Vec<RunResponse> type in the result");
            let control_cycles = &ctx.step.table.as_ref().unwrap().rows;
            eprintln!(
                "control hash/cycle {:?}; received hashes {:?} cycles: {:?}",
                &control_cycles,
                result_hashes
                    .iter()
                    .map(|x| &x.data)
                    .map(|hsh| hsh
                        .iter()
                        .map(|b| format!("{:02X}", *b))
                        .collect::<Vec::<_>>()
                        .join(""))
                    .collect::<Vec::<_>>(),
                result_summaries
                    .iter()
                    .map(|x| x.mcycle)
                    .collect::<Vec::<_>>()
            );
            assert!(control_cycles
                .iter()
                .skip(1)
                .zip(result_summaries)
                .all(|(a, b)| {
                    let cycle = a[2].parse::<u64>().unwrap();
                    b.mcycle == cycle
                }));
            let control_hashes = &ctx.step.table.as_ref().unwrap().rows;
            assert!(control_hashes
                .iter()
                .skip(1)
                .zip(result_hashes)
                .all(|(a, b)| compare_hashes(&b.data, &a[1])));
            world
        },
    );

    steps
}
