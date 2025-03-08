use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_client(true)
        //.out_dir("./src")
        .file_descriptor_set_path(out_dir.join("rest_descriptor.bin"))
        .compile_protos(&["proto/rest.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/rest.proto")?;

    Ok(())
}
