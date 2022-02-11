mod steps;
mod world;
use cucumber_rust::{Context, Cucumber};
use world::{TestContext, TestWorld};

pub fn hash_to_string(hash: &[u8]) -> String {
    format!("{:02X?}", hash)
        .replace(" ", "")
        .replace(",", "")
        .replace("[", "")
        .replace("]", "")
}

pub fn compare_hashes(ha: &[u8], hb: &[u8]) -> bool {
    let ha_s = hash_to_string(ha);
    let hb_s = hash_to_string(hb);
    eprintln!("{} || {}", ha_s, hb_s);
    ha_s == hb_s
}

pub fn error_name_to_code(name: &str) -> tonic::Code {
    match name {
        "InvalidArgument" => tonic::Code::InvalidArgument,
        "Internal" => tonic::Code::Internal,
        _ => panic!("Unknown error code was requested in test scenario"),
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<TestWorld>::new()
        .features(&["./features"])
        .steps(steps::new_session::steps())
        .steps(steps::session_run::steps())
        .steps(steps::session_step::steps())
        .steps(steps::session_get_proof::steps())
        .steps(steps::session_store_load::steps())
        .steps(steps::session_read_write_memory::steps())
        .steps(steps::end_session::steps())
        .enable_capture(false)
        .context(Context::new().add(TestContext {
            machine_manager_ip: String::from("127.0.0.1"),
            machine_manager_port: 50051,
            machine_manager_checkin_ip: String::from("127.0.0.1"),
            machine_manager_checkin_port: 50052,
            caretsi_machine_ip: String::from("127.0.0.1"),
            cartesi_machine_port: 50055,
        }))
        .run_and_exit()
        .await
}
