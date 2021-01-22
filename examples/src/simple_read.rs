use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
use bigtable_rs::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowSet};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let instance_name = "test1";
    let table_name = "my-table";
    let key: String = "key1".to_owned();
    let channel_size = 4;

    let connection =
        bigtable::BigTableConnection::new("project-id", instance_name, true, None, channel_size)
            .await?;
    let mut bigtable = connection.client();

    let request = ReadRowsRequest {
        table_name: format!("{}{}", bigtable.table_prefix, table_name),
        rows_limit: 1,
        rows: Some(RowSet {
            row_keys: vec![key.into_bytes()],
            row_ranges: vec![],
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

    if let Some(response) = bigtable
        .read_rows(request)
        .await?
        .into_inner()
        .message()
        .await?
    {
        response.chunks.into_iter().for_each(|mut v| {
            println!(
                "{}, {}",
                String::from_utf8(v.qualifier.take().unwrap()).unwrap(),
                String::from_utf8(v.value).unwrap()
            )
        });
    }
    Ok(())
}
