/// Reference:
/// https://github.com/fdeantoni/prost-wkt
/// https://github.com/hyperium/tonic/tree/master/tonic-build
/// https://github.com/tokio-rs/prost/issues/672
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conditionally run this build.rs so we do not need protoc dependency
    // for normal build process.
    // Set up env parameter BUILD_BIGTABLE_RS_GOOGLE_PROTO=true
    // whenever we need to update the generated google proto files
    match std::env::var("BUILD_BIGTABLE_RS_GOOGLE_PROTO") {
        Ok(var) => {
            if var != "true" {
                return Ok(());
            }
        }
        Err(_) => return Ok(()),
    }

    println!(
        "cargo:warning=Running build.rs to generate and format Google API Bigtable proto rs files."
    );

    use prost_wkt_build::{FileDescriptorSet, Message};
    use std::{env, path::PathBuf};

    // Build test protos
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");
    tonic_prost_build::configure()
        .build_server(false)
        .out_dir("tests/read_rows/google")
        .compile_well_known_types(true)
        .type_attribute(".", "#[serde_with::serde_as]")
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .type_attribute(
            ".google.cloud.conformance.bigtable.v2.ReadRowsTest",
            "#[serde(default)]",
        )
        .field_attribute(
            ".google.cloud.conformance.bigtable.v2.ReadRowsTest.Result.timestamp_micros",
            "#[serde_as(as = \"serde_with::DisplayFromStr\")]",
        )
        .extern_path(
            ".google.protobuf.BytesValue",
            "::prost::alloc::vec::Vec<u8>",
        )
        .extern_path(
            ".google.protobuf.StringValue",
            "::prost::alloc::string::String",
        )
        .extern_path(
            ".google.cloud.conformance.bigtable.v2.ReadRowsTest.CellChunkWithSerde",
            "crate::read_rows::types_with_serde::CellChunkWithSerde",
        )
        // todo type change
        .extern_path(".google.protobuf", "::prost_wkt_types")
        .file_descriptor_set_path(&descriptor_file)
        .compile_protos(&["./tests/bigtable_test.proto"], &["tests"])?;

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();
    prost_wkt_build::add_serde(out, descriptor);

    std::process::Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("Running `cargo fmt` failed");

    Ok(())
}
