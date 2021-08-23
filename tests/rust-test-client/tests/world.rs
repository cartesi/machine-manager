use cucumber_rust::{async_trait, World};
use rust_test_client::MachineManagerClientProxy;
use std::{any::Any, boxed::Box, collections::HashMap, convert::Infallible};

pub const CARTESI_BIN_PATH: &'static str = "CARTESI_BIN_PATH";
pub const CARTESI_IMAGE_PATH: &'static str = "CARTESI_IMAGE_PATH";
pub const CARTESI_EXTERNAL_MACHINE_MANAGER: &'static str = "CARTESI_EXTERNAL_MACHINE_MANAGER";

#[derive(Default)]
pub struct TestWorld {
    pub client_proxy: MachineManagerClientProxy,
    pub response: HashMap<String, Box<dyn Any>>,
}

pub struct TestContext {
    pub server_ip: String,
    pub server_port: u32,
    pub image_file_root: String,
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
