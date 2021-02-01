use bigtable_rs::bigtable::{BigTable, BigTableConnection};
use bigtable_rs::google::bigtable::v2::mutation::SetCell;
use bigtable_rs::google::bigtable::v2::row_filter::Chain;
use bigtable_rs::google::bigtable::v2::row_filter::Filter;
use bigtable_rs::google::bigtable::v2::{
    mutation, MutateRowRequest, Mutation, ReadRowsRequest, RowFilter, RowSet,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use warp::http::Response;
use warp::http::StatusCode;
use warp::reply::with_status;
use warp::{Filter as WarpFilter, Rejection, Reply};

const TABLE_NAME: &str = "table-1";

#[derive(Deserialize, Serialize, Clone)]
struct Value {
    value: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let port = 3030;

    let project_id = "project-id";
    let instance_name = "instance-1";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    // make a bigtable client
    let connection: BigTableConnection =
        BigTableConnection::new(project_id, instance_name, true, channel_size, Some(timeout))
            .await?;
    let bigtable = connection.client();

    let bigtable_clone = bigtable.clone();
    let post = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        //.and(warp::any().map(move || bigtable_clone.clone()))
        .and_then(move |key: String, value: Value| {
            post_handler(key, value, bigtable_clone.clone())
        });

    let bigtable_clone = bigtable.clone();
    let get = warp::get()
        .and(warp::path::param::<String>())
        //.and(warp::any().map(move || bigtable_clone.clone()))
        .and_then(move |key: String| get_handler(key, bigtable_clone.clone()));

    // View access logs by setting `RUST_LOG=http`.
    let routes = get.or(post).with(warp::log("http"));
    // Start up the server...
    info!("Starting http server on port: {}", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}

async fn post_handler(
    key: String,
    value: Value,
    mut bigtable: BigTable,
) -> Result<Box<dyn Reply>, Rejection> {
    let request = MutateRowRequest {
        table_name: bigtable.get_full_table_name(&TABLE_NAME),
        row_key: key.into_bytes(),
        mutations: vec![Mutation {
            mutation: Some(mutation::Mutation::SetCell(SetCell {
                family_name: "cf1".to_owned(),
                column_qualifier: "c1".to_owned().into_bytes(),
                timestamp_micros: -1, // IMPORTANT: Don't leave it empty. Use -1 for current Bigtable server time.
                value: value.value.into_bytes(),
            })),
        }],
        ..MutateRowRequest::default()
    };

    // write to table
    let response = bigtable.mutate_row(request).await;

    match response {
        Ok(_result) => Ok(Box::new("Done")),
        Err(error) => Ok(Box::new(with_status(
            Response::new(format!("{:?}", error)),
            StatusCode::BAD_REQUEST,
        ))),
    }
}

async fn get_handler(key: String, mut bigtable: BigTable) -> Result<Box<dyn Reply>, Rejection> {
    // read from table again
    // prepare a ReadRowsRequest
    let request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(&TABLE_NAME),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![key.clone().into_bytes()],
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

    // calling bigtable API to get results
    let mut response = match bigtable.read_rows(request).await {
        Ok(r) => r,
        Err(error) => {
            return Ok(Box::new(with_status(
                Response::new(format!("{:?}", error)),
                StatusCode::BAD_REQUEST,
            )))
        }
    };

    // simply print results for example usage
    match response.pop() {
        Some((_key, mut vec)) => Ok(Box::new(
            warp::reply::json(
                &vec.pop() // we mostly will have only 1 cell
                    .map(|row_cell| String::from_utf8(row_cell.value).unwrap())
                    .unwrap_or("".to_owned()),
            )
            .into_response(),
        )),
        None => Ok(Box::new(StatusCode::NOT_FOUND)),
    }
}
