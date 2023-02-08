// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use crate::world::{TestContext, TestWorld, CARTESI_BIN_PATH, CARTESI_IMAGE_PATH};
use crate::{compare_hashes, error_name_to_code};
use cucumber_rust::{t, StepContext, Steps};
use rust_test_client::stubs::cartesi_machine::{Hash, Void};
use rust_test_client::stubs::cartesi_machine_manager::NewSessionRequest;
use rust_test_client::utils::{start_listener, wait_process_output};
use rust_test_client::{generate_default_machine_config, generate_default_machine_rt_config};
use std::{
    boxed::Box,
    env,
    process::{Command, Stdio},
    sync::mpsc::channel,
};

pub async fn open_session(
    world: &mut TestWorld,
    ctx: &StepContext,
    force: bool,
) -> (
    Result<tonic::Response<Hash>, tonic::Status>,
    NewSessionRequest,
) {
    let new_session_ctx = ctx.get::<TestContext>().unwrap().clone();
    world
        .client_proxy
        .connect(&new_session_ctx.machine_manager_ip[..], new_session_ctx.machine_manager_port)
        .await
        .expect("Failed to connect to machine manager server");
    let request = world.client_proxy.build_new_session_request(force);
    let ret = world
        .client_proxy
        .grpc_client
        .as_mut()
        .unwrap()
        .new_session(request.clone())
        .await;
    (ret, request)
}

pub async fn open_session_with_default_config(
    world: &mut TestWorld,
    ctx: &StepContext,
    force: bool,
) -> (
    Result<tonic::Response<Hash>, tonic::Status>,
    NewSessionRequest,
) {
    world.client_proxy.machine_config =
        Some(generate_default_machine_config(&world.image_file_root[..]));
    world.client_proxy.machine_rt_config = Some(generate_default_machine_rt_config());
    open_session(world, ctx, force).await
}

pub async fn close_sessions(world: &mut TestWorld) {
    let request = world.client_proxy.build_end_session_request(true);
    if let Err(e) = world
        .client_proxy
        .grpc_client
        .as_mut()
        .unwrap()
        .end_session(request)
        .await
    {
        panic!("Unable to finish sessions: {}", e);
    }
}

pub async fn open_machine_grpc(world: &mut TestWorld, ctx: &StepContext) {
    let test_ctx = ctx.get::<TestContext>().unwrap().clone();
    world
        .machine_proxy
        .connect(&test_ctx.caretsi_machine_ip[..], test_ctx.cartesi_machine_port)
        .await
        .expect("Failed to connect to cartesi machine");
}

pub async fn open_verification_session(
    world: &mut TestWorld,
    ctx: &StepContext,
    manager_request: NewSessionRequest,
) {
    let machine_request = world.machine_proxy.build_machine_request(manager_request);
    open_machine_grpc(world, ctx).await;
    world
        .machine_proxy
        .grpc_client
        .as_mut()
        .unwrap()
        .machine(machine_request)
        .await
        .expect("Unable to open verification session");
}

pub async fn get_verification_hash(world: &mut TestWorld) {
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
    world
        .response
        .insert(String::from("verification_hash"), Box::new(hash));
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given("machine manager server is up", |mut world, ctx| {
        let cartesi_image_path = env::var(&CARTESI_IMAGE_PATH).unwrap_or_else(|_| {
            panic!(
                "{} that points to folder with Cartesi images is not set",
                &CARTESI_IMAGE_PATH
            )
        });
        world.image_file_root = cartesi_image_path.clone();
        let cartesi_bin_path = env::var(&CARTESI_BIN_PATH).unwrap_or_else(|_| {
            panic!(
                "{} that points to folder with Cartesi executables is not set",
                &CARTESI_BIN_PATH
            )
        });

        eprintln!(
            "Starting verification cartesi machine: {}/remote-cartesi-machine",
            cartesi_bin_path
        );
        let test_ctx = ctx.get::<TestContext>().unwrap().clone();
        world.machine_handler = Some(
            Command::new(format!(
                "{}/remote-cartesi-machine",
                cartesi_bin_path.clone()
            ))
            .arg(format!(
                "--server-address={}:{}",
                test_ctx.caretsi_machine_ip, test_ctx.cartesi_machine_port
            ))
            .env(CARTESI_IMAGE_PATH, cartesi_image_path.clone())
            .spawn()
            .expect("Unable to launch verification cartesi machine"),
        );

        eprintln!("Starting machine manager: {}/machine-manager", cartesi_bin_path);
        world.manager_handler = Some(
            Command::new(&format!("{}/machine-manager", cartesi_bin_path.clone()))
                .env(CARTESI_BIN_PATH, cartesi_bin_path.clone())
                .env(CARTESI_IMAGE_PATH, cartesi_image_path)
                .stderr(Stdio::piped())
                .spawn()
                .expect("Unable to launch machine manager server"),
        );
        let (data_sender, data_receiver) = channel();
        let (cmd_sender, cmd_receiver) = channel();
        world.manager_sender = Some(cmd_sender);
        world.manager_receiver = Some(data_receiver);
        start_listener(
            data_sender,
            cmd_receiver,
            world
                .manager_handler
                .as_mut()
                .unwrap()
                .stderr
                .take()
                .unwrap(),
        );
        if let Err(e) = wait_process_output(
            world.manager_receiver.as_ref().unwrap(),
            vec![
                (format!("Starting check in service on address {}:{}", test_ctx.machine_manager_checkin_ip, test_ctx.machine_manager_checkin_port), 1),
                (format!("Starting machine manager service on address {}:{}", test_ctx.caretsi_machine_ip, test_ctx.machine_manager_port), 1)
            ],
        ) {
            panic!("{}", e);
        }
        world
    });
    steps.given(
        "cartesi machine default config description",
        |mut world, _ctx| {
            world.client_proxy.machine_config =
                Some(generate_default_machine_config(&world.image_file_root[..]));
            world.client_proxy.machine_rt_config = Some(generate_default_machine_rt_config());

            world
        },
    );
    steps.when_async(
        "client asks machine manager server to create a new session",
        t!(|mut world, ctx| {
            let (ret, manager_request) = open_session(&mut world, &ctx, false).await;
            match ret {
                Ok(val) => {
                    open_verification_session(&mut world, &ctx, manager_request).await;
                    get_verification_hash(&mut world).await;
                    world.response.insert(String::from("hash"), Box::new(val))
                }
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };

            world
        }),
    );
    steps.then_async("server returns correct machine hash",
        t!(|mut world, _ctx| {
            let manager_hash = world
                .response
                .get(&String::from("hash"))
                .and_then(|x| x.downcast_ref::<tonic::Response<Hash>>())
                .take()
                .expect("No tonic::Response<Hash> type in the result");
            let verification_hash = world
                .response
                .get(&String::from("verification_hash"))
                .and_then(|x| x.downcast_ref::<Hash>())
                .take()
                .expect("No tonic::Response<Hash> type in the result");
            assert!(compare_hashes(
                &manager_hash.get_ref().data,
                &verification_hash.data,
            ));
            close_sessions(&mut world).await;
            world
    }));
    steps.given_async(
        "some session exists",
        t!(|mut world, ctx| {
            let (ret, _) = open_session(&mut world, &ctx, false).await;
            if let Err(e) = ret {
                panic!("New session request failed: {}", e);
            }
            world
        }),
    );
    steps.when_regex_async(
        r#"client asks machine manager server to create a new session with the same session id when forcing is (.*)"#,
        t!(|mut world, ctx| {
            let (ret, manager_request) = open_session(&mut world, &ctx, ctx.matches[1] == String::from("enabled")).await;
            match ret {
                Ok(val) => {
                    open_verification_session(&mut world, &ctx, manager_request).await;
                    get_verification_hash(&mut world).await;
                    world.response.insert(String::from("hash"), Box::new(val))
                },
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };
            world
        }),
    );
    steps.then_regex_async(
        r#"machine manager server returns an ([[:alpha:]]+) error"#,
        t!(|mut world, ctx| {
            let response = world
                .response
                .get(&String::from("error"))
                .and_then(|x| x.downcast_ref::<tonic::Status>())
                .take()
                .expect("No tonic::Status type in the result");
            assert_eq!(response.code(), error_name_to_code(&ctx.matches[1]));
            close_sessions(&mut world).await;
            world
        }),
    );

    steps
}
