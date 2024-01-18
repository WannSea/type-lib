use std::{path::PathBuf, env};

// ToDo: move to build script
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../").join("protos");
    let proto_files = vec![root.join("wannsea.proto")];
    
    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }
    
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile_protos(&proto_files, &[root])?;
    
    Ok(())
}