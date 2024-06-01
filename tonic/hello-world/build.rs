fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello_world.proto")?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/hello_world.proto"], &["protos"])?;
    Ok(())
}