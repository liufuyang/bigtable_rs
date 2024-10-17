use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::mutation;
use bigtable_rs::google::bigtable::v2::mutation::SetCell;
use bigtable_rs::google::bigtable::v2::row_filter::Filter;
use bigtable_rs::google::bigtable::v2::{
    MutateRowRequest, Mutation, ReadRowsRequest, RowFilter, RowSet,
};
use env_logger;
use gcp_auth::CustomServiceAccount;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use std::usize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let project_id = "project-1";
    let instance_name = "instance-1";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    let json_path: &str = "examples/src/custom_path_fake_key.json";
    // make a bigtable client
    let mut connection = bigtable::BigTableConnection::new_with_token_provider(
        project_id,
        instance_name,
        false,
        channel_size,
        Some(timeout),
        Arc::new(CustomServiceAccount::from_file(json_path).unwrap()),
    )?;

    // update the config for the inner client of the connection
    connection
        .configure_inner_client(|inner_client| inner_client.max_decoding_message_size(usize::MAX));

    let mut client = connection.client();

    // or update the config one step later
    client.configure_inner_client(|inner_client| {
        inner_client.max_decoding_message_size(2 * 1024 * 1024)
    });

    Ok(())
}
