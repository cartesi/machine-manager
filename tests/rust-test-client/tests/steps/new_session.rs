use crate::world::{TestContext, TestWorld, CARTESI_BIN_PATH, CARTESI_IMAGE_PATH};
use crate::{compare_hashes, error_name_to_code};
use cucumber_rust::{t, StepContext, Steps};
use rust_test_client::stubs::cartesi_machine::Hash;
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
) -> Result<tonic::Response<Hash>, tonic::Status> {
    let new_session_ctx = ctx.get::<TestContext>().unwrap().clone();
    world
        .client_proxy
        .connect(&new_session_ctx.server_ip[..], new_session_ctx.server_port)
        .await
        .expect("Failed to connect to machine manager server");
    let request = world.client_proxy.build_new_session_request(force);
    world
        .client_proxy
        .grpc_client
        .as_mut()
        .unwrap()
        .new_session(request)
        .await
}

pub async fn open_session_with_default_config(
    world: &mut TestWorld,
    ctx: &StepContext,
    force: bool,
) -> Result<tonic::Response<Hash>, tonic::Status> {
    world.client_proxy.machine_config =
        Some(generate_default_machine_config(&world.image_file_root[..]));
    world.client_proxy.machine_rt_config = Some(generate_default_machine_rt_config());
    open_session(world, ctx, force).await
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given("machine manager server is up", |mut world, _ctx| {
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
        eprintln!("Starting machine manager: {}/manager.py", cartesi_bin_path);
        world.manager_handler = Some(
            Command::new("python3")
                .arg(format!("{}/manager.py", cartesi_bin_path.clone()))
                .env(CARTESI_BIN_PATH, cartesi_bin_path.clone())
                .env(CARTESI_IMAGE_PATH, cartesi_image_path)
                .env("PYTHONPATH", format!("{0}/proto:{0}/src", cartesi_bin_path))
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
                ("start_manager_server: Server started".to_string(), 1),
                (
                    "start_checkin_server: Checkin service started".to_string(),
                    1,
                ),
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
            let ret = open_session(&mut world, &ctx, false).await;
            match ret {
                Ok(val) => world.response.insert(String::from("hash"), Box::new(val)),
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };

            world
        }),
    );
    steps.then_regex(
        r#"server returns machine hash ((\d|[A-Z]){64})"#,
        |world, ctx| {
            let response = world
                .response
                .get(&String::from("hash"))
                .and_then(|x| x.downcast_ref::<tonic::Response<Hash>>())
                .take()
                .expect("No tonic::Response<Hash> type in the result");
            assert!(compare_hashes(
                &response.get_ref().data,
                &ctx.matches[1][..]
            ));
            world
        },
    );
    steps.given_async(
        "some session exists",
        t!(|mut world, ctx| {
            if let Err(e) = open_session(&mut world, &ctx, false).await {
                panic!("New session request failed: {}", e);
            }
            world
        }),
    );
    steps.when_regex_async(
        r#"client asks machine manager server to create a new session with the same session id when forcing is (.*)"#,
        t!(|mut world, ctx| {
            let ret = open_session(&mut world, &ctx, ctx.matches[1] == String::from("enabled")).await;
            match ret {
                Ok(val) => world.response.insert(String::from("hash"), Box::new(val)),
                Err(e) => world.response.insert(String::from("error"), Box::new(e)),
            };
            world
        }),
    );
    steps.then_regex(
        r#"machine manager server returns an ([[:alpha:]]+) error"#,
        |world, ctx| {
            let response = world
                .response
                .get(&String::from("error"))
                .and_then(|x| x.downcast_ref::<tonic::Status>())
                .take()
                .expect("No tonic::Status type in the result");
            assert_eq!(response.code(), error_name_to_code(&ctx.matches[1]));
            world
        },
    );

    steps
}
