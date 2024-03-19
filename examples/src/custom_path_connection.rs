use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::mutation;
use bigtable_rs::google::bigtable::v2::mutation::SetCell;
use bigtable_rs::google::bigtable::v2::row_filter::Filter;
use bigtable_rs::google::bigtable::v2::{
    MutateRowRequest, Mutation, ReadRowsRequest, RowFilter, RowSet,
};
use env_logger;
use gcp_auth::{AuthenticationManager, CustomServiceAccount};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let project_id = "project-1";
    let instance_name = "instance-1";
    let table_name = "table-1";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    let key: String = "key3".to_owned();

    let json_path: &str = "examples/src/custom_path_fake_key.json";
    // make a bigtable client
    let connection = bigtable::BigTableConnection::new_with_auth_manager(
        project_id,
        instance_name,
        false,
        channel_size,
        Some(timeout),
        AuthenticationManager::from(CustomServiceAccount::from_file(json_path).unwrap()),
    )?;
    let mut bigtable = connection.client();

    let request = MutateRowRequest {
        table_name: bigtable.get_full_table_name(table_name),
        row_key: key.clone().into_bytes(),
        mutations: vec![Mutation {
            mutation: Some(mutation::Mutation::SetCell(SetCell {
                family_name: "cf1".to_owned(),
                column_qualifier: "c1".to_owned().into_bytes(),
                timestamp_micros: -1, // IMPORTANT: Don't leave it empty. Use -1 for current Bigtable server time.
                value: "a new write value".to_owned().into_bytes(),
            })),
        }],
        ..MutateRowRequest::default()
    };

    // write to table
    let _response = bigtable.mutate_row(request).await?;

    // read from table again
    // prepare a ReadRowsRequest
    let request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(table_name),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![key.clone().into_bytes()],
            row_ranges: vec![],
        }),
        filter: Some(RowFilter {
            filter: Some(Filter::CellsPerColumnLimitFilter(1)),
        }),
        ..ReadRowsRequest::default()
    };

    // calling bigtable API to get results
    let response = bigtable.read_rows(request).await?;

    // simply print results for example usage
    response.into_iter().for_each(|(key, data)| {
        println!("------------\n{}", String::from_utf8(key.clone()).unwrap());
        data.into_iter().for_each(|row_cell| {
            println!(
                "    [{}:{}] \"{}\" @ {}",
                row_cell.family_name,
                String::from_utf8(row_cell.qualifier).unwrap(),
                String::from_utf8(row_cell.value).unwrap(),
                row_cell.timestamp_micros
            )
        })
    });

    Ok(())
}
