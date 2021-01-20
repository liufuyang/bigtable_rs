fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Uncomment the code below and cargo build again, when updating google protos

    // tonic_build::configure()
    //     .build_server(false)
    //     .out_dir("src/google")
    //     .compile(
    //         &["googleapis/google/bigtable/v2/bigtable.proto"],
    //         &["googleapis"],
    //     )?;
    Ok(())
}