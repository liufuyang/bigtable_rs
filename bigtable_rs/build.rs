/// Reference:
/// https://github.com/fdeantoni/prost-wkt
/// https://github.com/hyperium/tonic/tree/master/tonic-build
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Uncomment the code below and cargo build again, when updating google protos

    // use prost_wkt_build::{FileDescriptorSet, Message};
    // use std::{env, path::PathBuf};
    // let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    // let descriptor_file = out.join("descriptors.bin");
    //
    // tonic_build::configure()
    //     .build_server(false)
    //     .out_dir("src/google")
    //     .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
    //     .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
    //     .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
    //     .file_descriptor_set_path(&descriptor_file)
    //     .compile(
    //         &[
    //             "../googleapis/google/bigtable/v2/bigtable.proto",
    //             "../googleapis/test/bigtable_test.proto", // only works with fork https://github.com/liufuyang/googleapis
    //         ],
    //         &["../googleapis"],
    //     )?;
    //
    // let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    // let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();
    // prost_wkt_build::add_serde(out, descriptor);

    Ok(())
}
