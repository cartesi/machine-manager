// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use crate::steps::session_run::{get_verification_hashes, run_machine};
use crate::world::{TestWorld, CARTESI_IMAGE_PATH};
use cucumber_rust::{t, Steps};
use rust_test_client::stubs::cartesi_machine::Hash;
use rust_test_client::stubs::cartesi_machine_manager::*;
use std::{boxed::Box, env, fs::remove_dir_all};

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.when_regex_async(
        r#"asking machine manager server to store the machine in a directory (.+)"#,
        t!(|mut world, ctx| {
            let request = world.client_proxy.build_new_session_store_request(
                world.image_file_root.clone(),
                ctx.matches[1].clone(),
            );
            match world
                .client_proxy
                .grpc_client
                .as_mut()
                .unwrap()
                .session_store(request)
                .await
            {
                Ok(_) => world
                    .response
                    .insert(String::from("dir"), Box::new(ctx.matches[1].clone())),
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };
            world
        }),
    );

    steps.then_async(
        "machine manager server is able to load machine from this directory correctly",
        t!(|mut world, _ctx| {
            let dir = world
                .response
                .get(&String::from("dir"))
                .and_then(|x| x.downcast_ref::<String>())
                .take()
                .expect("No String type in the result");
            let request = world.client_proxy.build_new_session_from_store_request(
                world.image_file_root.clone(),
                dir.to_string(),
            );
            world
                .client_proxy
                .grpc_client
                .as_mut()
                .unwrap()
                .new_session(request.clone())
                .await
                .expect("Unable to perform restore request");

            // now remove the saved machine
            let cartesi_image_path = env::var(&CARTESI_IMAGE_PATH).unwrap();
            let dir_to_remove = format!("{}/{}", cartesi_image_path, dir);
            if let Err(e) = remove_dir_all(dir_to_remove) {
                panic!("Unable to remove serialized machine: {}", e);
            }

            world
        }),
    );

    steps.then_regex_async(
        r#"machine manager server is able to execute this machine for (\d+) cycles and (\d+) ucycles"#,
        t!(|mut world, ctx| {
            let cycles = vec![ctx.matches[1].parse::<u64>().unwrap()];
            let ucycles = vec![ctx.matches[2].parse::<u64>().unwrap()];
            let ret = run_machine(
                &cycles,
                &ucycles,
                &mut world.client_proxy,
            )
            .await;
            if let session_run_response::RunOneof::Result(result) = ret.run_oneof.as_ref().unwrap()
            {
                get_verification_hashes(&mut world, &cycles, &ucycles)
                    .await;
                let result_hashes: Vec<Hash> = result.hashes.clone();
                world
                    .response
                    .insert(String::from("hashes"), Box::new(result_hashes));
                world
            } else {
                panic!("Invalid state: server job didn't finish");
            }
        }),
    );

    steps
}
