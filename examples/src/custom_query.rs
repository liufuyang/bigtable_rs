use bigtable_rs::bigtable;
use bigtable_rs::bigtable::read_rows::decode_read_rows_response;
use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
use bigtable_rs::google::bigtable::v2::row_range::{EndKey, StartKey};
use bigtable_rs::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowRange, RowSet};
use bigtable_rs::util::get_end_key_for_prefix;
use env_logger;
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

    let prefix_a: String = "j".to_owned();
    let prefix_b: String = "p".to_owned();

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

    // prepare a ReadRowsRequest with fully customized query
    let request = ReadRowsRequest {
        app_profile_id: "default".to_owned(),
        table_name: bigtable.get_full_table_name(table_name),
        rows_limit: 20,
        rows: Some(RowSet {
            row_keys: vec![],
            row_ranges: vec![
                RowRange {
                    start_key: Some(StartKey::StartKeyClosed(prefix_a.clone().into_bytes())),
                    end_key: get_end_key_for_prefix(prefix_a.as_ref()).map(EndKey::EndKeyOpen),
                },
                RowRange {
                    start_key: Some(StartKey::StartKeyClosed(prefix_b.clone().into_bytes())),
                    end_key: get_end_key_for_prefix(prefix_b.as_ref()).map(EndKey::EndKeyOpen),
                },
            ],
        }),
        filter: Some(RowFilter {
            filter: Some(Filter::Chain(Chain {
                filters: vec![
                    RowFilter {
                        filter: Some(Filter::FamilyNameRegexFilter("cf1".to_owned())),
                    },
                    RowFilter {
                        filter: Some(Filter::ColumnQualifierRegexFilter("c1".as_bytes().to_vec())),
                    },
                    RowFilter {
                        filter: Some(Filter::CellsPerColumnLimitFilter(1)),
                    },
                ],
            })),
        }),
        ..ReadRowsRequest::default()
    };

    // calling bigtable API to get results using inner client and customized query
    let response_stream = bigtable.get_client().read_rows(request).await?.into_inner();
    let response = decode_read_rows_response(&Some(timeout), response_stream).await?;

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
