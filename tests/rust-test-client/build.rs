fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/stubs")
        .compile(
            &[
                "../../lib/grpc-interfaces/cartesi-machine.proto",
                "../../lib/grpc-interfaces/machine-manager.proto",
            ],
            &["../../lib/grpc-interfaces"],
        )?;
    Ok(())
}
