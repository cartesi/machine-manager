use crate::steps::new_session::open_session_with_default_config;
use crate::world::TestWorld;
use cucumber_rust::{t, Steps};

pub fn steps() -> Steps<TestWorld> {
    let mut steps: Steps<TestWorld> = Steps::new();

    steps.given_async(
        "machine manager server with terminated session",
        t!(|mut world, ctx| {
            let (ret, _) = open_session_with_default_config(&mut world, &ctx, false).await;
            if let Err(e) = ret {
                panic!("New session request failed: {}", e);
            }

            let request = world.client_proxy.build_end_session_request(true);
            if let Err(e) = world
                .client_proxy
                .grpc_client
                .as_mut()
                .unwrap()
                .end_session(request)
                .await
            {
                panic!("Unable to perform EndSession request: {}", e);
            }

            world
        }),
    );

    steps
}
