// Copyright 2023 Cartesi Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use cucumber_rust::{async_trait, World};
use rust_test_client::{MachineClientProxy, MachineManagerClientProxy};
use std::{
    any::Any,
    boxed::Box,
    collections::HashMap,
    convert::Infallible,
    process::Child,
    sync::mpsc::{Receiver, Sender},
};

pub const CARTESI_BIN_PATH: &str = "CARTESI_BIN_PATH";
pub const CARTESI_IMAGE_PATH: &str = "CARTESI_IMAGE_PATH";

#[derive(Default)]
pub struct TestWorld {
    pub client_proxy: MachineManagerClientProxy,
    pub machine_proxy: MachineClientProxy,
    pub response: HashMap<String, Box<dyn Any>>,
    pub image_file_root: String,
    pub manager_handler: Option<Child>,
    pub manager_receiver: Option<Receiver<String>>,
    pub manager_sender: Option<Sender<()>>,
    pub machine_handler: Option<Child>,
}

pub struct TestContext {
    pub machine_manager_ip: String,
    pub machine_manager_port: u32,
    pub machine_manager_checkin_ip: String,
    pub machine_manager_checkin_port: u32,
    pub caretsi_machine_ip: String,
    pub cartesi_machine_port: u32,
}

#[async_trait(?Send)]
impl World for TestWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let mut world = TestWorld::default();
        world.client_proxy.session_id = String::from("test_session");
        Ok(world)
    }
}

impl Drop for TestWorld {
    fn drop(&mut self) {
        if self.manager_handler.is_some() {
            self.manager_sender.as_mut().unwrap().send(()).ok();
            self.manager_handler.as_mut().unwrap().kill().ok();
        }
        if self.machine_handler.is_some() {
            self.machine_handler.as_mut().unwrap().kill().ok();
        }
    }
}
