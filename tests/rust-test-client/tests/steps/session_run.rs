use crate::compare_hashes;
use crate::steps::new_session::open_session_with_default_config;
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use rust_test_client::stubs::cartesi_machine::Hash;
use rust_test_client::stubs::cartesi_machine_manager::*;
use rust_test_client::MachineManagerClientProxy;
use std::boxed::Box;

pub async fn run_machine(
    cycles: Vec<u64>,
    client: &mut MachineManagerClientProxy,
) -> SessionRunResponse {
    let run_request = client.build_new_session_run_request(&cycles);
    client.run_to_completion(run_request).await
}

pub fn strs_to_uints(matches: &Vec<String>) -> Vec<u64> {
    matches[1]
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given_async(
        "a pristine machine manager server session",
        t!(|mut world, ctx| {
            if let Err(e) = open_session_with_default_config(&mut world, &ctx, true).await {
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
        "client asks server to run session",
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
                world
                    .response
                    .insert(String::from("hashes"), Box::new(result_hashes));
                world
            } else {
                panic!("Invalid state: server job didn't finish");
            }
        }),
    );
    steps.then("server returns machine hashes:", |world, ctx| {
        let result_hashes = world
            .response
            .get(&String::from("hashes"))
            .and_then(|x| x.downcast_ref::<Vec<Hash>>())
            .take()
            .expect("No Vec<Hash> type in the result");
        let control_hashes = &ctx.step.table.as_ref().unwrap().rows;
        // skipping the first row because of the table headings
        assert!(control_hashes
            .iter()
            .skip(1)
            .zip(result_hashes)
            .all(|(a, b)| compare_hashes(&b.data, &a[1])));

        world
    });

    steps
}
