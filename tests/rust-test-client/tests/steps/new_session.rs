use crate::world::{TestContext, TestWorld};
use crate::{compare_hashes, error_name_to_code};
use cucumber_rust::{t, StepContext, Steps};
use rust_test_client::stubs::cartesi_machine::Hash;
use rust_test_client::{
    generate_default_machine_config, generate_default_machine_rt_config,
    generate_hello_world_machine_config,
};
use std::boxed::Box;

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
    let session_ctx = ctx.get::<TestContext>().unwrap().clone();
    world.client_proxy.machine_config = Some(generate_default_machine_config(
        &session_ctx.image_file_root[..],
    ));
    world.client_proxy.machine_rt_config = Some(generate_default_machine_rt_config());
    open_session(world, ctx, force).await
}

pub async fn open_session_with_hello_world_config(
    world: &mut TestWorld,
    ctx: &StepContext,
    force: bool,
) -> Result<tonic::Response<Hash>, tonic::Status> {
    let session_ctx = ctx.get::<TestContext>().unwrap().clone();
    world.client_proxy.machine_config = Some(generate_hello_world_machine_config(
        &session_ctx.image_file_root[..],
    ));
    world.client_proxy.machine_rt_config = Some(generate_default_machine_rt_config());
    open_session(world, ctx, force).await
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given(
        "cartesi machine default config description",
        |mut world, ctx| {
            let new_session_ctx = ctx.get::<TestContext>().unwrap().clone();
            world.client_proxy.machine_config = Some(generate_default_machine_config(
                &new_session_ctx.image_file_root[..],
            ));
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
