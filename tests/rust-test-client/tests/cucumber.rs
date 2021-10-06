mod steps;
mod world;
use cucumber_rust::{Context, Cucumber};
use world::{TestContext, TestWorld};

pub fn compare_hashes(ha: &[u8], hb_s: &str) -> bool {
    let ha_s = format!("{:02X?}", ha)
        .replace(" ", "")
        .replace(",", "")
        .replace("[", "")
        .replace("]", "");
    eprintln!("{} -> {}", hb_s, ha_s);
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
            server_ip: String::from("127.0.0.1"),
            server_port: 50051,
        }))
        .run_and_exit()
        .await
}
