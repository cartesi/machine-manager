// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

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
