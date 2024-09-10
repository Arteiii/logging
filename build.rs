use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("logging_descriptor.bin"))
        .compile(&["proto/logging.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/logging.proto")?;
    Ok(())
}
