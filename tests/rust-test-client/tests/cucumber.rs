mod steps;
mod world;
use cucumber_rust::{criteria::feature, criteria::scenario, Context, Cucumber};
use regex::Regex;
use std::{env, process::Command, thread, time};
use world::{
    TestContext, TestWorld, CARTESI_BIN_PATH, CARTESI_EXTERNAL_MACHINE_MANAGER, CARTESI_IMAGE_PATH,
};

pub fn compare_hashes(ha: &[u8], hb_s: &str) -> bool {
    let ha_s = format!("{:02X?}", ha)
        .replace(" ", "")
        .replace(",", "")
        .replace("[", "")
        .replace("]", "");
    println!("{} -> {}", hb_s, ha_s);
    ha_s == hb_s
}

pub fn error_name_to_code(name: &str) -> tonic::Code {
    match name {
        "InvalidArgument" => tonic::Code::InvalidArgument,
        "Internal" => tonic::Code::Internal,
        _ => panic!("Unknown error code was requested in test scenario"),
    }
}

fn kill_machine_manager() {
    let use_external_machine_manager = match env::var(&CARTESI_EXTERNAL_MACHINE_MANAGER) {
        Ok(val) => {
            if val == "1" {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    if use_external_machine_manager {
        //do nothing
        return;
    } else {
        Command::new("pkill")
            .arg("machine-manager")
            .spawn()
            .expect("Machine manager server is not launched");
        // we also need to kill all instantiated cartesi machines
        Command::new("pkill")
            .arg("cartesi-machine")
            .spawn()
            .expect("There is no instantiated cartesi machines");
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn rerun_machine_manager() {
    let use_external_machine_manager = match env::var(&CARTESI_EXTERNAL_MACHINE_MANAGER) {
        Ok(val) => {
            if val == "1" {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    if use_external_machine_manager {
        //do nothing
        return;
    } else {
        kill_machine_manager();
        let cartesi_image_path = match env::var(&CARTESI_IMAGE_PATH) {
            Ok(val) => val,
            Err(e) => panic!(
                "Please, set {0}. Could not interpret {0}: {1}",
                &CARTESI_IMAGE_PATH, e
            ),
        };

        let cartesi_bin_path = match env::var(&CARTESI_BIN_PATH) {
            Ok(path) => path,
            Err(e) => panic!(
                "Please, set {0}. Could not interpret {0}: {1}",
                &CARTESI_BIN_PATH, e
            ),
        };
        println!(
            "Starting machine manager: {}/machine-manager",
            cartesi_bin_path
        );
        Command::new(format!("{}/machine-manager", cartesi_bin_path))
            .env(CARTESI_BIN_PATH, cartesi_bin_path)
            .env(CARTESI_IMAGE_PATH, cartesi_image_path)
            .spawn()
            .expect("Unable to launch machine manager server");
        thread::sleep(time::Duration::from_secs(1)); // we need to wait a bit to give server some time to start
    }
}

#[tokio::main]
async fn main() {
    let image_file_root = env::var(&CARTESI_IMAGE_PATH).unwrap_or_else(|_| {
        panic!(
            "{} that points to folder with Cartesi images is not set",
            &CARTESI_IMAGE_PATH
        )
    });

    let args = env::args().collect::<Vec<String>>();
    let scenario_filter = "".to_string();
    let scenario_filter: &str = args.iter().skip(1).next().unwrap_or(&scenario_filter);
    println!("Using scenario filter={}", &scenario_filter);

    Cucumber::<TestWorld>::new()
        .features(&["./features"])
        .steps(steps::new_session::steps())
        .steps(steps::session_run::steps())
        .steps(steps::session_run_hello_world::steps())
        .steps(steps::session_step::steps())
        .steps(steps::session_get_proof::steps())
        .steps(steps::session_store_load::steps())
        .steps(steps::session_read_write_memory::steps())
        .steps(steps::end_session::steps())
        .context(Context::new().add(TestContext {
            server_ip: String::from("127.0.0.1"),
            server_port: 50051,
            image_file_root,
        }))
        .scenario_regex(&scenario_filter)
        .before(
            scenario(Regex::new(r#"asking server to create.*session.*"#).unwrap()),
            |_ctx| {
                rerun_machine_manager();
                Box::pin(async {})
            },
        )
        .before(
            feature(Regex::new(r#"^Session.* feature$"#).unwrap()),
            |_ctx| {
                rerun_machine_manager();
                Box::pin(async {})
            },
        )
        .after(
            feature(Regex::new(r#"^[[:alpha:]]*Session.* feature$"#).unwrap()),
            |_ctx| {
                kill_machine_manager();
                Box::pin(async {})
            },
        )
        .run_and_exit()
        .await
}
