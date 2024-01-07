// ToDo: move to build script
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.compile_protos(&["../protos/wannsea.proto"], &["../protos/"])?;

    Ok(())
}