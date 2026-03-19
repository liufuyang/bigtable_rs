//! Integration tests that require a running Bigtable emulator.
//!
//! These tests are ignored by default and require:
//! 1. A running Bigtable emulator: `gcloud beta emulators bigtable start --host-port=localhost:8086`
//! 2. The BIGTABLE_EMULATOR_HOST environment variable set: `export BIGTABLE_EMULATOR_HOST=localhost:8086`
//! 3. A test table created with column families cf1 and cf2
//!
//! Run with: `BIGTABLE_EMULATOR_HOST=localhost:8086 cargo test --test tests emulator --features integration_tests`

#![cfg(feature = "integration_tests")]

use bigtable_rs::bigtable::{BigTableConnection, Error};
use bigtable_rs::google::bigtable::v2::mutation;
use bigtable_rs::google::bigtable::v2::mutation::SetCell;
use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
use bigtable_rs::google::bigtable::v2::row_range::{EndKey, StartKey};
use bigtable_rs::google::bigtable::v2::{
    MutateRowRequest, Mutation, ReadRowsRequest, RowFilter, RowRange, RowSet, SampleRowKeysRequest,
};
use futures_util::TryStreamExt;
use std::time::Duration;
use tokio::sync::OnceCell;

use bigtable_rs::google::bigtable::admin::v2::{
    bigtable_table_admin_client::BigtableTableAdminClient, // This is the raw tonic client
    ColumnFamily,
    CreateTableRequest,
    Table,
};
use std::collections::HashMap;
use tonic::transport::Channel;

const PROJECT_ID: &str = "project-1";
const INSTANCE_NAME: &str = "instance-1";
const TABLE_NAME: &str = "table-1";
const CF1: &str = "cf1";
const CF2: &str = "cf2";
const CHANNEL_SIZE: usize = 4;
const TIMEOUT_SECS: u64 = 10;

static INIT: OnceCell<()> = OnceCell::const_new();

async fn global_setup() {
    INIT.get_or_init(|| async {
        // 1. Connect to the emulator
        // Note: Emulator doesn't use TLS, so we use a plain channel
        let emulator_host = std::env::var("BIGTABLE_EMULATOR_HOST")
            .expect("BIGTABLE_EMULATOR_HOST must be set to run this test");
        let endpoint = format!("http://{}", emulator_host);

        let channel = Channel::from_shared(endpoint)
            .expect("Creating channel failed.")
            .connect()
            .await
            .expect("Connect to Bigtable emulater failed.");

        let mut client = BigtableTableAdminClient::new(channel);

        let mut column_families = HashMap::new();
        column_families.insert(CF1.to_string(), ColumnFamily::default());
        column_families.insert(CF2.to_string(), ColumnFamily::default());

        let request = CreateTableRequest {
            parent: format!("projects/{}/instances/{}", PROJECT_ID, INSTANCE_NAME),
            table_id: TABLE_NAME.to_string(),
            table: Some(Table {
                column_families,
                ..Default::default()
            }),
            initial_splits: vec![],
        };

        match client.create_table(request).await {
            Ok(_) => println!("Successfully created table: {}", TABLE_NAME),
            Err(e) if e.to_string().contains("already exists") => {
                println!("Table {} already exists in emulator, skipping.", TABLE_NAME);
            }
            Err(e) => panic!("Creating Bigtable table field. {}", e),
        };
    })
    .await;
}

async fn create_connection(read_only: bool) -> Result<BigTableConnection, Error> {
    BigTableConnection::new(
        PROJECT_ID,
        INSTANCE_NAME,
        read_only,
        CHANNEL_SIZE,
        Some(Duration::from_secs(TIMEOUT_SECS)),
    )
    .await
}

#[tokio::test]
async fn test_connection_to_emulator() {
    global_setup().await;

    let connection: Result<BigTableConnection, Error> = create_connection(true).await;
    assert!(connection.is_ok(), "Failed to connect to emulator");
}

#[tokio::test]
async fn test_write_and_read_row() {
    global_setup().await;

    let connection: BigTableConnection = create_connection(false).await.expect("Failed to connect");
    let mut bigtable = connection.client();

    let test_key = format!("integration_test_key_{}", std::process::id());
    let test_value = "integration_test_value";

    // Write a row
    let write_request = MutateRowRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        row_key: test_key.clone().into_bytes(),
        mutations: vec![Mutation {
            mutation: Some(mutation::Mutation::SetCell(SetCell {
                family_name: "cf1".to_owned(),
                column_qualifier: "c1".to_owned().into_bytes(),
                timestamp_micros: -1,
                value: test_value.to_owned().into_bytes(),
            })),
        }],
        ..MutateRowRequest::default()
    };

    let write_result = bigtable.mutate_row(write_request).await;
    assert!(
        write_result.is_ok(),
        "Failed to write row: {:?}",
        write_result.err()
    );

    // Read the row back
    let read_request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        rows_limit: 1,
        rows: Some(RowSet {
            row_keys: vec![test_key.clone().into_bytes()],
            row_ranges: vec![],
        }),
        filter: Some(RowFilter {
            filter: Some(Filter::CellsPerColumnLimitFilter(1)),
        }),
        ..ReadRowsRequest::default()
    };

    let response = bigtable.read_rows(read_request).await;
    assert!(response.is_ok(), "Failed to read row: {:?}", response.err());

    let rows = response.unwrap();
    assert_eq!(rows.len(), 1, "Expected 1 row, got {}", rows.len());

    let (key, cells) = &rows[0];
    assert_eq!(
        String::from_utf8(key.clone()).unwrap(),
        test_key,
        "Row key mismatch"
    );
    assert!(!cells.is_empty(), "Expected at least one cell");

    let cell = &cells[0];
    assert_eq!(cell.family_name, "cf1");
    assert_eq!(String::from_utf8(cell.qualifier.clone()).unwrap(), "c1");
    assert_eq!(String::from_utf8(cell.value.clone()).unwrap(), test_value);
}

#[tokio::test]
async fn test_read_rows_with_range() {
    global_setup().await;

    let connection: BigTableConnection = create_connection(false).await.expect("Failed to connect");
    let mut bigtable = connection.client();

    let prefix = format!("range_test_{}_", std::process::id());

    // Write multiple rows
    for i in 1..=5 {
        let key = format!("{}{}", prefix, i);
        let write_request = MutateRowRequest {
            table_name: bigtable.get_full_table_name(TABLE_NAME),
            row_key: key.into_bytes(),
            mutations: vec![Mutation {
                mutation: Some(mutation::Mutation::SetCell(SetCell {
                    family_name: "cf1".to_owned(),
                    column_qualifier: "c1".to_owned().into_bytes(),
                    timestamp_micros: -1,
                    value: format!("value{}", i).into_bytes(),
                })),
            }],
            ..MutateRowRequest::default()
        };
        bigtable
            .mutate_row(write_request)
            .await
            .expect("Failed to write row");
    }

    // Read with range (keys 1-3)
    let read_request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![],
            row_ranges: vec![RowRange {
                start_key: Some(StartKey::StartKeyClosed(
                    format!("{}1", prefix).into_bytes(),
                )),
                end_key: Some(EndKey::EndKeyOpen(format!("{}4", prefix).into_bytes())),
            }],
        }),
        filter: Some(RowFilter {
            filter: Some(Filter::CellsPerColumnLimitFilter(1)),
        }),
        ..ReadRowsRequest::default()
    };

    let response = bigtable.read_rows(read_request).await;
    assert!(
        response.is_ok(),
        "Failed to read rows: {:?}",
        response.err()
    );

    let rows = response.unwrap();
    assert_eq!(
        rows.len(),
        3,
        "Expected 3 rows in range, got {}",
        rows.len()
    );
}

#[tokio::test]
async fn test_read_rows_with_filter_chain() {
    global_setup().await;

    let connection: BigTableConnection = create_connection(false).await.expect("Failed to connect");
    let mut bigtable = connection.client();

    let test_key = format!("filter_test_{}", std::process::id());

    // Write a row with multiple column families and qualifiers
    let mutations = vec![
        Mutation {
            mutation: Some(mutation::Mutation::SetCell(SetCell {
                family_name: "cf1".to_owned(),
                column_qualifier: "c1".to_owned().into_bytes(),
                timestamp_micros: -1,
                value: "cf1_c1_value".to_owned().into_bytes(),
            })),
        },
        Mutation {
            mutation: Some(mutation::Mutation::SetCell(SetCell {
                family_name: "cf1".to_owned(),
                column_qualifier: "c2".to_owned().into_bytes(),
                timestamp_micros: -1,
                value: "cf1_c2_value".to_owned().into_bytes(),
            })),
        },
        Mutation {
            mutation: Some(mutation::Mutation::SetCell(SetCell {
                family_name: "cf2".to_owned(),
                column_qualifier: "c1".to_owned().into_bytes(),
                timestamp_micros: -1,
                value: "cf2_c1_value".to_owned().into_bytes(),
            })),
        },
    ];

    let write_request = MutateRowRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        row_key: test_key.clone().into_bytes(),
        mutations,
        ..MutateRowRequest::default()
    };

    bigtable
        .mutate_row(write_request)
        .await
        .expect("Failed to write row");

    // Read with filter chain: cf1 family, c1 qualifier only
    let read_request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        rows_limit: 1,
        rows: Some(RowSet {
            row_keys: vec![test_key.clone().into_bytes()],
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
                ],
            })),
        }),
        ..ReadRowsRequest::default()
    };

    let response = bigtable.read_rows(read_request).await;
    assert!(
        response.is_ok(),
        "Failed to read rows: {:?}",
        response.err()
    );

    let rows = response.unwrap();
    assert_eq!(rows.len(), 1, "Expected 1 row");

    let (_, cells) = &rows[0];
    assert_eq!(
        cells.len(),
        1,
        "Expected 1 cell after filtering, got {}",
        cells.len()
    );
    assert_eq!(cells[0].family_name, "cf1");
    assert_eq!(String::from_utf8(cells[0].qualifier.clone()).unwrap(), "c1");
}

#[tokio::test]
async fn test_stream_rows() {
    global_setup().await;

    let connection: BigTableConnection = create_connection(false).await.expect("Failed to connect");
    let mut bigtable = connection.client();

    let prefix = format!("stream_test_{}_", std::process::id());

    // Write multiple rows
    for i in 1..=3 {
        let key = format!("{}{}", prefix, i);
        let write_request = MutateRowRequest {
            table_name: bigtable.get_full_table_name(TABLE_NAME),
            row_key: key.into_bytes(),
            mutations: vec![Mutation {
                mutation: Some(mutation::Mutation::SetCell(SetCell {
                    family_name: "cf1".to_owned(),
                    column_qualifier: "c1".to_owned().into_bytes(),
                    timestamp_micros: -1,
                    value: format!("stream_value{}", i).into_bytes(),
                })),
            }],
            ..MutateRowRequest::default()
        };
        bigtable
            .mutate_row(write_request)
            .await
            .expect("Failed to write row");
    }

    // Read using stream_rows
    let read_request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![],
            row_ranges: vec![RowRange {
                start_key: Some(StartKey::StartKeyClosed(
                    format!("{}1", prefix).into_bytes(),
                )),
                end_key: Some(EndKey::EndKeyOpen(format!("{}9", prefix).into_bytes())),
            }],
        }),
        filter: Some(RowFilter {
            filter: Some(Filter::CellsPerColumnLimitFilter(1)),
        }),
        ..ReadRowsRequest::default()
    };

    let mut stream = bigtable
        .stream_rows(read_request)
        .await
        .expect("Failed to create stream");

    let mut count = 0;
    while let Some((key, cells)) = stream.try_next().await.expect("Stream error") {
        assert!(
            String::from_utf8(key).unwrap().starts_with(&prefix),
            "Unexpected key prefix"
        );
        assert!(!cells.is_empty(), "Expected at least one cell");
        count += 1;
    }

    assert_eq!(count, 3, "Expected 3 rows from stream, got {}", count);
}

#[tokio::test]
async fn test_sample_row_keys() {
    global_setup().await;

    let connection: BigTableConnection = create_connection(true).await.expect("Failed to connect");
    let mut bigtable = connection.client();

    let request = SampleRowKeysRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        ..SampleRowKeysRequest::default()
    };

    let response = bigtable.sample_row_keys(request).await;
    assert!(
        response.is_ok(),
        "Failed to sample row keys: {:?}",
        response.err()
    );

    // The emulator should return at least one sample (or empty for small tables)
    // We just verify the call succeeds
    let mut stream = response.unwrap();
    let mut count = 0;
    while let Ok(Some(_)) = stream.message().await {
        count += 1;
    }
    // Sample row keys may return 0 results for small tables, which is valid
    println!("sample_row_keys returned {} samples", count);
}

#[tokio::test]
async fn test_read_nonexistent_row() {
    global_setup().await;

    let connection: BigTableConnection = create_connection(true).await.expect("Failed to connect");
    let mut bigtable = connection.client();

    let read_request = ReadRowsRequest {
        table_name: bigtable.get_full_table_name(TABLE_NAME),
        rows_limit: 1,
        rows: Some(RowSet {
            row_keys: vec!["nonexistent_key_12345".to_owned().into_bytes()],
            row_ranges: vec![],
        }),
        ..ReadRowsRequest::default()
    };

    let response = bigtable.read_rows(read_request).await;
    assert!(response.is_ok(), "Failed to read: {:?}", response.err());

    let rows = response.unwrap();
    assert!(rows.is_empty(), "Expected no rows for nonexistent key");
}
