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
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use rust_test_client::stubs::cartesi_machine::{Hash, Void};
use rust_test_client::stubs::cartesi_machine_manager::*;
use rust_test_client::MachineManagerClientProxy;
use std::boxed::Box;

pub async fn run_machine(
    cycles: &Vec<u64>,
    ucycles: &Vec<u64>,
    client: &mut MachineManagerClientProxy,
) -> SessionRunResponse {
    let run_request = client.build_new_session_run_request(cycles, ucycles);
    client.run_to_completion(run_request).await
}

pub fn str_to_uints(s: &String) -> Vec<u64> {
    s
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

pub async fn get_verification_hashes(world: &mut TestWorld, cycles: &Vec<u64>, ucycles: &Vec<u64>) {
    let mut verification_hashes: Vec<Hash> = vec![];
    let mut current_cycle = 0;
    let mut current_ucycle = 0;    
    for cycle_idx in 0..cycles.len() {
        let cycle = cycles[cycle_idx];
        let ucycle = ucycles[cycle_idx];
        match world.machine_proxy.run_to(current_cycle, cycle, current_ucycle, ucycle).await {
            Ok(result) => (current_cycle, current_ucycle) = result,
            Err(err) => panic!("Unable to make verification run: {}", err),
        }
        let response = world
            .machine_proxy
            .grpc_client
            .as_mut()
            .unwrap()
            .get_root_hash(Void {})
            .await;
        let hash = match response {
            Ok(val) => val.into_inner().hash.unwrap(),
            Err(e) => panic!("Unable to get verification hash: {}", e),
        };
        verification_hashes.push(hash);
    }

    world.response.insert(
        String::from("verification_hashes"),
        Box::new(verification_hashes),
    );
    world.response.insert(
        String::from("machine_cycle"),
        Box::new(current_cycle),
    );
    world.response.insert(
        String::from("machine_ucycle"),
        Box::new(current_ucycle),
    );
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given_async(
        "a pristine machine manager server session",
        t!(|mut world, ctx| {
            let (ret, manager_request) =
                open_session_with_default_config(&mut world, &ctx, true).await;
            if let Err(e) = ret {
                panic!("New session request failed: {}", e);
            }
            open_verification_session(&mut world, &ctx, manager_request).await;
            world
        }),
    );
    steps.given_regex_async(
        r#"the machine executed with cycles ((\d+,)*\d+) and ucycles ((\d+,)*\d+)"#,
        t!(|mut world, ctx| {
            let cycles = str_to_uints(&ctx.matches[1]);
            let ucycles = str_to_uints(&ctx.matches[3]);
            let ret = run_machine(&cycles, &ucycles, &mut world.client_proxy).await;
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
                Box::new(str_to_uints(&ctx.matches[1])),
            );
            world
        },
    );
    steps.given_regex(
        r#"the ucycles array ((\d+,)*\d+) to run the machine"#,
        |mut world, ctx| {
            world.response.insert(
                String::from("exec_ucycles"),
                Box::new(str_to_uints(&ctx.matches[1])),
            );
            world
        },
    );
    steps.when_async(
        "client asks server to run session",
        t!(|mut world, _ctx| {
            let cycles = world
                .response
                .get(&String::from("exec_cycles"))
                .and_then(|x| x.downcast_ref::<Vec<u64>>())
                .take()
                .expect("No Vec<u64> type in the result")
                .clone();
            let ucycles = world
                .response
                .get(&String::from("exec_ucycles"))
                .and_then(|x| x.downcast_ref::<Vec<u64>>())
                .take()
                .expect("No Vec<u64> type in the result")
                .clone();
            get_verification_hashes(&mut world, &cycles.to_vec(), &ucycles.to_vec()).await;
            let ret = run_machine(&cycles.to_vec(), &ucycles.to_vec(), &mut world.client_proxy).await;
            if let session_run_response::RunOneof::Result(result) = ret.run_oneof.as_ref().unwrap()
            {
                let result_hashes: Vec<Hash> = result.hashes.clone();
                world
                    .response
                    .insert(String::from("hashes"), Box::new(result_hashes));
                world
                    .response
                    .insert(String::from("manager_cycle"), Box::new(result.cycle));
                world
                    .response
                    .insert(String::from("manager_ucycle"), Box::new(result.ucycle));
                world
            } else {
                panic!("Invalid state: server job didn't finish");
            }
        }),
    );
    steps.then_regex(
        r#"server returns correct session cycle (\d+) and ucycle (\d+)"#, 
        |world, ctx| {
            let expected_cycle = ctx.matches[1].parse::<u64>().unwrap();
            let expected_ucycle = ctx.matches[2].parse::<u64>().unwrap();
            let machine_cycle = world
                .response
                .get(&String::from("machine_cycle"))
                .and_then(|x| x.downcast_ref::<u64>())
                .take()
                .expect("No u64 type in the result");
            let machine_ucycle = world
                .response
                .get(&String::from("machine_ucycle"))
                .and_then(|x| x.downcast_ref::<u64>())
                .take()
                .expect("No u64 type in the result");
            let manager_cycle = world
                .response
                .get(&String::from("manager_cycle"))
                .and_then(|x| x.downcast_ref::<u64>())
                .take()
                .expect("No u64 type in the result");
            let manager_ucycle = world
                .response
                .get(&String::from("manager_ucycle"))
                .and_then(|x| x.downcast_ref::<u64>())
                .take()
                .expect("No u64 type in the result");
            assert!(*machine_cycle == expected_cycle);
            assert!(*machine_ucycle == expected_ucycle);
            assert!(*manager_cycle == expected_cycle);
            assert!(*manager_ucycle == expected_ucycle);
            world
        }
    );
    steps.then_async("server returns correct machine hashes", 
        t!(|mut world, _ctx| {
        let result_hashes = world
            .response
            .get(&String::from("hashes"))
            .and_then(|x| x.downcast_ref::<Vec<Hash>>())
            .take()
            .expect("No Vec<Hash> type in the result");
        let verification_hashes = world
            .response
            .get(&String::from("verification_hashes"))
            .and_then(|x| x.downcast_ref::<Vec<Hash>>())
            .take()
            .expect("No verification hashes in the result");
        assert!(verification_hashes
            .iter()
            .zip(result_hashes)
            .all(|(a, b)| compare_hashes(&b.data, &a.data)));
        close_sessions(&mut world).await;
        world
    }));

    steps
}
