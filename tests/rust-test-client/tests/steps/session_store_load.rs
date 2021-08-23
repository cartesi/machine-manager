use crate::steps::session_run::run_machine;
use crate::world::{TestContext, TestWorld, CARTESI_IMAGE_PATH};
use cucumber_rust::{t, Steps};
use rust_test_client::stubs::cartesi_machine::Hash;
use rust_test_client::stubs::cartesi_machine_manager::*;
use std::{boxed::Box, env, fs::remove_dir_all};

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.when_regex_async(
        r#"asking machine manager server to store the machine in a directory (.+)"#,
        t!(|mut world, ctx| {
            let session_ctx = ctx.get::<TestContext>().unwrap().clone();
            let request = world.client_proxy.build_new_session_store_request(
                session_ctx.image_file_root.clone(),
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
        t!(|mut world, ctx| {
            let session_ctx = ctx.get::<TestContext>().unwrap().clone();
            let dir = world
                .response
                .get(&String::from("dir"))
                .and_then(|x| x.downcast_ref::<String>())
                .take()
                .expect("No String type in the result");
            let request = world.client_proxy.build_new_session_from_store_request(
                session_ctx.image_file_root.clone(),
                dir.to_string(),
            );
            world
                .client_proxy
                .grpc_client
                .as_mut()
                .unwrap()
                .new_session(request)
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
        r#"machine manager server is able to execute this machine for (\d+) cycles"#,
        t!(|mut world, ctx| {
            let ret = run_machine(
                vec![ctx.matches[1].parse::<u64>().unwrap()],
                &mut world.client_proxy,
            )
            .await;
            if let session_run_response::RunOneof::Result(result) = ret.run_oneof.as_ref().unwrap()
            {
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
