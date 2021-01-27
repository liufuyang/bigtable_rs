use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::SampleRowKeysRequest;
use env_logger;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let project_id = "project-id";
    let instance_name = "instance-1";
    let table_name = "table-1";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    // make a bigtable client
    let connection = bigtable::BigTableConnection::new(
        project_id,
        instance_name,
        true,
        channel_size,
        Some(timeout),
    )
    .await?;
    let mut bigtable = connection.client();

    let request = SampleRowKeysRequest {
        table_name: bigtable.get_full_table_name(table_name),
        ..SampleRowKeysRequest::default()
    };
    let mut response = bigtable.sample_row_keys(request).await?;

    while let Some(res) = response.message().await? {
        println!(
            "key: {}, offset_bytes: {}",
            String::from_utf8(res.row_key).unwrap(),
            res.offset_bytes
        );
    }

    Ok(())
}
