use crate::compare_hashes;
use crate::steps::new_session::{open_session_with_default_config, open_verification_session, close_sessions};
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};
use rust_test_client::stubs::cartesi_machine::{Hash, Void};
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

pub async fn get_verification_hashes(world: &mut TestWorld, cycles: Vec<u64>) {
    let mut verification_hashes: Vec<Hash> = vec![];
    for cycle in cycles {
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
                .expect("No Vec<u64> type in the result")
                .clone();
            get_verification_hashes(&mut world, cycles.clone().to_vec()).await;
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
