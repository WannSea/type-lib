use std::{env, path::PathBuf};
mod helpers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../");
    let proto_files = helpers::list_files(&root, "proto").unwrap();

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }
    
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap())
    .join("proto_descriptor.bin");

    prost_build::Config::new()
        .file_descriptor_set_path(&descriptor_path)
        // Override prost-types with pbjson-types
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")

        .compile_protos(&proto_files, &[root])?;
    
        let descriptor_set = std::fs::read(descriptor_path)?;
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .build(&[".wannsea"])?;

    Ok(())
}