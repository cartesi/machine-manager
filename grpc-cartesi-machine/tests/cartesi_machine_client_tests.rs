// Copyright (C) 2021 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate grpc_cartesi_machine;
use async_mutex::Mutex;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::machine_check_in_server::MachineCheckIn;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::machine_check_in_server::MachineCheckInServer;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::CheckInRequest;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Csr;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Void;
use grpc_cartesi_machine::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rstest::*;
use std::future::Future;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Response};

static INITIAL_ROOT_HASH: Hash = Hash([
    171, 57, 119, 62, 75, 81, 39, 16, 236, 196, 74, 63, 99, 19, 28, 11, 180, 207, 215, 180, 228,
    62, 112, 76, 5, 77, 7, 167, 105, 205, 122, 79,
]);

static SECOND_STEP_HASH: Hash = Hash([
    87, 241, 30, 88, 202, 242, 149, 254, 136, 232, 94, 152, 182, 189, 0, 215, 216, 124, 2, 60, 46,
    145, 164, 254, 219, 55, 206, 141, 175, 241, 218, 98,
]);

#[allow(dead_code)]
struct Context {
    cartesi_machine_server: GrpcCartesiMachineClient,
    server_ip: String,
    port: u32,
    container_name: String,
    checkin_address: Arc<Mutex<Option<String>>>,
}

fn generate_random_name() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect()
}

fn instantiate_external_server_instance(port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("127.0.0.1:{0}", port);
    println!("Starting Cartesi remote machine on address {}", address);
    std::process::Command::new("/opt/cartesi/bin/remote-cartesi-machine")
        .arg(&address)
        .spawn()
        .expect("Unable to launch cartesi machine server");
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}

/// Instantiate local Cartesi machine server in subprocess with checking params
fn instantiate_local_server_instance_checkin(
    cartesi_bin_path: &str,
    host: &str,
    port: u32,
    session_id: &str,
    checkin_address: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let address = format!("{}:{}", host, port);
    println!(
        "Instantiating remote Cartesi machine on address {}",
        address
    );
    let cartesi_server_bin = format!("{}/remote-cartesi-machine", cartesi_bin_path);
    let output = std::process::Command::new(cartesi_server_bin)
        .arg(&format!("--server-address={}", &address))
        .arg(&format!("--session-id={}", session_id))
        .arg(&format!("--checkin-address={}", checkin_address))
        .spawn()
        .expect("Unable to launch Cartesi machine server");
    println!("Cartesi server started pid='{}'", output.id());
    Ok(output.id())
}

fn try_stop_container() {
    let result = std::process::Command::new("pkill")
        .arg("-f")
        .arg("remote-cartesi-machine")
        .status()
        .unwrap();
    if !result.success() {
        eprint!("Error stopping container");
    }
}

impl Context {
    pub fn get_server(&mut self) -> &mut GrpcCartesiMachineClient {
        &mut self.cartesi_machine_server
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Destroying container {}", &self.container_name);
        try_stop_container();
    }
}

#[allow(unused_mut)]
mod local_server {
    use super::*;

    #[fixture]
    async fn context_future() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 50051;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();

        match instantiate_external_server_instance(port) {
            Ok(_) => (),
            Err(ex) => eprint!(
                "Error instantiating cartesi machine server {}",
                ex.to_string()
            ),
        }
        println!(
            "Starting machine server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );

        Context {
            cartesi_machine_server: match GrpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
            checkin_address: Arc::new(Mutex::new(None)),
        }
    }

    #[fixture]
    async fn context_with_machine_future() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 50051;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();
        match instantiate_external_server_instance(port) {
            Ok(_) => (),
            Err(err) => eprint!(
                "Error instantiating cartesi machine server {}",
                err.to_string()
            ),
        }
        println!(
            "Starting cartesi server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );
        let mut context = Context {
            cartesi_machine_server: match GrpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
            checkin_address: Arc::new(Mutex::new(None)),
        };
        //Modify default configuration
        let mut default_config = match context.get_server().get_default_config().await {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to get default config: {}", err.to_string())
            }
        };
        default_config.rom = RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        match context
            .get_server()
            .create_machine(&default_config, &MachineRuntimeConfig::default())
            .await
        {
            Ok(_) => context,
            Err(err) => {
                panic!("Unable to instantiate cartesi machine: {}", err.to_string())
            }
        }
    }

    #[fixture]
    async fn context_with_machine_with_flash_future() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 50051;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();
        match instantiate_external_server_instance(port) {
            Ok(_) => (),
            Err(err) => eprint!(
                "Error instantiating cartesi machine server {}",
                err.to_string()
            ),
        }
        println!(
            "Starting cartesi server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );
        let mut context = Context {
            cartesi_machine_server: match GrpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
            checkin_address: Arc::new(Mutex::new(None)),
        };
        //Modify default configuration
        let mut default_config = match context.get_server().get_default_config().await {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to get default config: {}", err.to_string())
            }
        };
        default_config.rom = RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        //Create flash image and add to flash configuration
        match std::fs::write("/tmp/input_root.raw", b"Root data in flash") {
            Ok(_) => (),
            Err(err) => panic!(
                "Unable to create temporary flash image: {}",
                err.to_string()
            ),
        }
        std::process::Command::new("truncate")
            .args(&["-s", "62914560", "/tmp/input_root.raw"])
            .output()
            .expect("Unable to create flash image file");
        default_config.flash_drives = vec![MemoryRangeConfig {
            start: 0x8000000000000000,
            image_filename: "/tmp/input_root.raw".to_string(),
            length: 0x3c00000,
            shared: false,
        }];
        //Create machine
        match context
            .get_server()
            .create_machine(&default_config, &MachineRuntimeConfig::default())
            .await
        {
            Ok(_) => context,
            Err(err) => {
                panic!("Unable to instantiate cartesi machine: {}", err.to_string())
            }
        }
    }

    pub struct TestManagerCheckinService {
        address: Arc<Mutex<Option<String>>>,
    }

    impl TestManagerCheckinService {
        pub fn new(address: Arc<Mutex<Option<String>>>) -> Self {
            TestManagerCheckinService { address }
        }
    }

    #[tonic::async_trait]
    impl MachineCheckIn for TestManagerCheckinService {
        /// Check in grpc api implementation
        async fn check_in(
            &self,
            request: Request<CheckInRequest>,
        ) -> Result<tonic::Response<Void>, tonic::Status> {
            let request = request.into_inner();
            println!(
                "Got a check in request for session id: {} and address {} ",
                &request.session_id, &request.address
            );
            let mut address = self.address.lock().await;
            println!("Setting address to: {}", &request.address);
            *address = Some(request.address.clone());
            Ok(Response::new(Void {}))
        }
    }

    async fn run_checkin_service(
        address: Arc<Mutex<Option<String>>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting checkin service on address 127.0.0.1:50052");
        let checkin_service = TestManagerCheckinService::new(address);
        let addr_checkin = format!("{}:{}", "127.0.0.1", "50052").parse()?;
        match Server::builder()
            .add_service(MachineCheckInServer::new(checkin_service))
            .serve(addr_checkin)
            .await
        {
            Ok(_x) => Ok(()),
            Err(x) => Err(Box::new(x)),
        }
    }

    async fn wait_for_checkin(address_mut: &Arc<Mutex<Option<String>>>) -> String {
        let mut address = address_mut.lock().await;
        *address = None;
        drop(address);
        let mut result = String::new();
        for i in 1..=20 {
            println!("Waiting for snapshot check in for retry={}", i);
            std::thread::sleep(std::time::Duration::from_millis(100));
            if let Some(address) = address_mut.lock().await.as_ref() {
                result = address.clone();
                break;
            }

            if i == 20 {
                panic!("Checkin not performed");
            }
        }
        result
    }

    #[fixture]
    async fn context_with_machine_future_checkin() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 0;
        let container_name = generate_random_name();
        //Start checkin service
        let address: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let address_temp = Arc::clone(&address);
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .thread_name(&format!("Run jo"))
                .enable_io()
                .thread_stack_size(1024 * 1024)
                .build()
                .unwrap();
            runtime.block_on(async {
                let temp = address_temp;
                run_checkin_service(Arc::clone(&temp)).await.unwrap();
            });
        });
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!(
            "Starting cartesi server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );
        match instantiate_local_server_instance_checkin(
            "/opt/cartesi/bin",
            "127.0.0.1",
            0,
            "Session123455",
            "127.0.0.1:50052",
        ) {
            Ok(_) => (),
            Err(err) => eprint!(
                "Error instantiating cartesi machine server {}",
                err.to_string()
            ),
        }
        //Wait for checkin
        let uri = wait_for_checkin(&address).await;
        println!("GOT URI: {}", &uri);
        let uri = format!("http://{}", &uri);
        let mut context = Context {
            cartesi_machine_server: match GrpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
            checkin_address: address,
        };
        //Modify default configuration
        let mut default_config = match context.get_server().get_default_config().await {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to get default config: {}", err.to_string())
            }
        };
        default_config.rom = RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        match context
            .get_server()
            .create_machine(&default_config, &MachineRuntimeConfig::default())
            .await
        {
            Ok(_) => context,
            Err(err) => {
                panic!("Unable to instantiate cartesi machine: {}", err.to_string())
            }
        }
    }

    #[rstest]
    #[tokio::test]
    #[should_panic]
    async fn test_invalid_server_address() -> () {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 12345;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();

        let _context = Context {
            cartesi_machine_server: match GrpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
            checkin_address: Arc::new(Mutex::new(None)),
        };
        ()
    }

    #[rstest]
    #[tokio::test]
    async fn test_cartesi_server_instance(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        println!(
            "Sleeping in the test... context container name: {}",
            context.container_name
        );
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("End sleeping");
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_version(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let semantic_version = context.get_server().get_version().await?;
        println!("Acquired semantic version: {:?} ", semantic_version);
        assert_eq!(
            semantic_version,
            SemanticVersion {
                major: 0,
                minor: 5,
                patch: 0,
                pre_release: "".to_string(),
                build: "".to_string()
            }
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_default_config(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let default_config = context.get_server().get_default_config().await?;
        println!("Acquired default config {:?}", default_config);
        assert_eq!(default_config.processor.pc, 4096);
        assert_eq!(default_config.processor.mvendorid, 7161130726739634464);
        assert_eq!(default_config.processor.marchid, 9);
        assert_eq!(default_config.ram.length, 0);
        assert_eq!(default_config.rom.image_filename, "");
        assert_eq!(default_config.flash_drives.len(), 0);
        assert_eq!(default_config.htif.fromhost, 0);
        assert_eq!(default_config.htif.tohost, 0);
        assert_eq!(default_config.dhd.dlength, 0);
        assert_eq!(default_config.clint.mtimecmp, 0);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_machine_create(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let mut default_config = context.get_server().get_default_config().await?;
        default_config.rom = RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        context
            .get_server()
            .create_machine(&default_config, &MachineRuntimeConfig::default())
            .await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_machine_create_already_created(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let mut default_config = context.get_server().get_default_config().await?;
        default_config.rom = RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        let ret = context
            .get_server()
            .create_machine(&default_config, &MachineRuntimeConfig::default())
            .await;
        match ret {
            Ok(_) => panic!("Creating existing machine should fail"),
            Err(err) => assert_eq!(
                err.to_string(),
                r##"status: FailedPrecondition, message: "machine already exists", details: [], metadata: MetadataMap { headers: {"content-type": "application/grpc"} }"##
            ),
        }
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_run(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let run_response = context.get_server().run(1000).await?;
        assert_eq!(run_response.mcycle, 1000);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_store(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context
            .get_server()
            .store(&format!("/tmp/cartesi_{}", generate_random_name()))
            .await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_store_nomachine(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let ret = context.get_server().store("/tmp/cartesi_store").await;
        assert!(ret.is_err());
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_destroy(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().destroy().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_snapshot(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().snapshot().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_rollback(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().rollback().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_step(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let log = context
            .get_server()
            .step(
                &AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        //println!("Acquired log for step: {:?} ", log);
        assert!(log.accesses.len() > 0);
        assert!(log.accesses[0].r#type == AccessType::Read);
        assert!(log.brackets.len() > 0);
        assert!(log.log_type.proofs == true && log.log_type.annotations == true);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_snapshot_step_rollback(
        context_with_machine_future_checkin: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future_checkin.await;
        context.get_server().snapshot().await?;
        let new_address = wait_for_checkin(&context.checkin_address).await;
        let new_address = format!("http://{}", new_address);
        context.cartesi_machine_server = GrpcCartesiMachineClient::new(new_address).await.unwrap();
        let _log = context
            .get_server()
            .step(
                &AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        context.get_server().rollback().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_shutdown(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().shutdown().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_double_shutdown(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().shutdown().await?;
        let ret = context.get_server().shutdown().await;
        assert!(ret.is_err());
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_memory(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().read_memory(0x1000, 16).await?;
        assert_eq!(
            ret,
            vec![151, 2, 0, 0, 147, 130, 66, 6, 115, 144, 82, 48, 55, 21, 0, 0]
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_write_memory(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context
            .get_server()
            .write_memory(0x8000000F, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
            .await?;
        let ret = context.get_server().read_memory(0x8000000F, 12).await?;
        assert_eq!(ret, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_word(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().read_word(0x100).await?;
        assert_eq!(ret, 4096);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_root_hash(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().get_root_hash().await?;
        assert_eq!(ret, INITIAL_ROOT_HASH);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_root_hash_after_step(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().get_root_hash().await?;
        assert_eq!(ret, INITIAL_ROOT_HASH);
        let _log = context
            .get_server()
            .step(
                &AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        let ret = context.get_server().get_root_hash().await?;
        assert_eq!(ret, SECOND_STEP_HASH);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_proof(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let proof = context.get_server().get_proof(0x0, 10).await?;
        assert_eq!(proof.log2_target_size, 10);
        assert_eq!(
            proof.target_hash,
            Hash([
                190, 202, 43, 74, 46, 123, 22, 69, 193, 136, 9, 144, 96, 180, 233, 7, 36, 184, 154,
                226, 168, 75, 72, 242, 83, 134, 219, 40, 64, 110, 201, 10
            ])
        );
        assert_eq!(proof.sibling_hashes.len(), 54);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_replace_flash_drive(
        context_with_machine_with_flash_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_with_flash_future.await;
        std::fs::write("/tmp/input.raw", b"test data 1234567890")?;
        std::process::Command::new("truncate")
            .args(&["-s", "62914560", "/tmp/input.raw"])
            .output()
            .expect("Unable to create flash image file");

        let memory_range_config = MemoryRangeConfig {
            start: 0x8000000000000000,
            image_filename: "/tmp/input.raw".to_string(),
            length: 0x3c00000,
            shared: true,
        };
        context
            .get_server()
            .replace_flash_drive(&memory_range_config)
            .await?;
        let ret = context
            .get_server()
            .read_memory(0x8000000000000000, 12)
            .await?;
        assert_eq!(
            ret,
            vec![116, 101, 115, 116, 32, 100, 97, 116, 97, 32, 49, 50]
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_x_address(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let x_address = context.get_server().get_x_address(2).await?;
        assert_eq!(x_address, 0x10);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_write_x(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let x_value = context.get_server().read_x(2).await?;
        assert_eq!(x_value, 0x0);
        context.get_server().write_x(2, 0x1234).await?;
        let x_value = context.get_server().read_x(2).await?;
        assert_eq!(x_value, 0x1234);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_reset_i_flags_y(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().reset_iflags_y().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_dhd_h_address(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let dhd_h_address = context.get_server().get_dhd_h_address(1).await?;
        assert_eq!(dhd_h_address, 1073938480);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_write_dhd_h(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let x_value = context.get_server().read_dhd_h(2).await?;
        assert_eq!(x_value, 0x0);
        context.get_server().write_dhd_h(2, 0x1234).await?;
        let x_value = context.get_server().read_dhd_h(2).await?;
        assert_eq!(x_value, 0x1234);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_csr_address(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let address = context.get_server().get_csr_address(Csr::Pc).await?;
        println!("Got address: {}", address);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_write_csr(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let x_value = context.get_server().read_csr(Csr::Sscratch).await?;
        assert_eq!(x_value, 0x0);
        context
            .get_server()
            .write_csr(Csr::Sscratch, 0x12345)
            .await?;
        let x_value = context.get_server().read_csr(Csr::Sscratch).await?;
        assert_eq!(x_value, 0x12345);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_initial_config(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let initial_config = context.get_server().get_initial_config().await?;
        println!("Acquired initial config {:?}", initial_config);
        assert_eq!(initial_config.processor.pc, 4096);
        assert_eq!(initial_config.processor.mvendorid, 7161130726739634464);
        assert_eq!(initial_config.processor.marchid, 9);
        assert_eq!(initial_config.ram.length, 1048576);
        assert_eq!(
            initial_config.rom.image_filename,
            "/opt/cartesi/share/images/rom.bin"
        );
        assert_eq!(initial_config.flash_drives.len(), 0);
        assert_eq!(initial_config.htif.fromhost, 0);
        assert_eq!(initial_config.htif.tohost, 0);
        assert_eq!(initial_config.dhd.dlength, 0);
        assert_eq!(initial_config.clint.mtimecmp, 0);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_merkle_tree(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().update_merkle_tree().await?;
        assert!(ret);
        let ret = context.get_server().verify_merkle_tree().await?;
        assert!(ret);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_merkle_tree(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().update_merkle_tree().await?;
        assert!(ret);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_dirty_page_maps(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().verify_dirty_page_maps().await?;
        assert!(ret);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_dump_pmas(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().dump_pmas().await?;
        std::thread::sleep(std::time::Duration::from_secs(3));
        std::process::Command::new("rm")
            .args(&[
                "0000000000000000--0000000000001000.bin",
                "0000000000001000--000000000000f000.bin",
                "0000000002000000--00000000000c0000.bin",
                "0000000040008000--0000000000001000.bin",
                "0000000080000000--0000000000100000.bin",
            ])
            .status()
            .expect("Failed to cleanup dump pmas test");
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_access_log(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().update_merkle_tree().await?;
        let log = context
            .get_server()
            .step(
                &AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        context
            .get_server()
            .verify_access_log(&log, &MachineRuntimeConfig::default(), false)
            .await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_state_transition(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().update_merkle_tree().await?;
        let root_hash_before = context.get_server().get_root_hash().await?;
        let log = context
            .get_server()
            .step(
                &AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        let root_hash_after = context.get_server().get_root_hash().await?;
        context
            .get_server()
            .verify_state_transition(
                &root_hash_before,
                &log,
                &root_hash_after,
                false,
                &MachineRuntimeConfig::default(),
            )
            .await?;
        Ok(())
    }
}
