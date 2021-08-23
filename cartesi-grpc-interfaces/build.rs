fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build machine manager server and client stubs
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .out_dir("src/grpc_stubs")
        .compile(
            &[
                "../lib/grpc-interfaces/machine-manager.proto",
                "../lib/grpc-interfaces/cartesi-machine-checkin.proto",
            ],
            &["../lib/grpc-interfaces"],
        )?;
    Ok(())
}
