# bigtable-rs

## Disclaimer - this is NOT ready for any production use

I just started trying to create a bigtable client. This library is far far away from a stage for others to use. But any
contribution or help is highly appreciated.

I had this idea and got some input from others see here
https://github.com/durch/rust-bigtable/issues/4
and basically ported many code from
https://github.com/solana-labs/solana/tree/master/storage-bigtable. Will try work on it to make it more useful.

You can use the library as follow:
```toml
[dependencies]
bigtable-rs = { path = "../bigtable-rs" }
tokio = { version = "1.0", features = ["rt-multi-thread"] }
env_logger = "0.8.2"
```

```rust
use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
use bigtable_rs::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowSet, RowRange};
use bigtable_rs::google::bigtable::v2::row_range::{StartKey, EndKey};
use env_logger;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let project_id = "project-id";
    let instance_name = "test1";
    let table_name = "my-table";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    let key_start: String = "key1".to_owned();
    let key_end: String = "key3".to_owned();

    let connection = bigtable::BigTableConnection::new(
        project_id,
        instance_name,
        true,
        channel_size,
        Some(timeout),
    )
    .await?;
    let mut bigtable = connection.client();

    let request = ReadRowsRequest {
        table_name: format!("{}{}", bigtable.table_prefix, table_name),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![], // vec![key_start.into_bytes()]
            row_ranges: vec![RowRange {
                start_key: Some(StartKey::StartKeyClosed(key_start.into_bytes())),
                end_key: Some(EndKey::EndKeyOpen(key_end.into_bytes())),
            }],
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

    let response = bigtable.read_rows(request).await?;

    response.into_iter().for_each(|(key, data)| {
        data.into_iter().for_each(|(cell_name, cell_value)| {
            println!(
                "{} -> {}:{}",
                String::from_utf8(key.clone()).unwrap(),
                String::from_utf8(cell_name).unwrap(),
                String::from_utf8(cell_value).unwrap()
            )
        })
    });

    Ok(())
}
```

To start develop or test the example above, start a test bigtable instance locally first:

```
# at one terminal, start a bigtable insatnce locally
. start_bigtable_local.sh

# at another terminal, load some data into it
. start_load_table_local.sh
```

Then run the example

```
BIGTABLE_EMULATOR_HOST=localhost:8086;RUST_LOG=bigtable_rs=trace cargo run --bin simple_read
```

If you see error `Error: AccessTokenError("GOOGLE_APPLICATION_CREDENTIALS environment variable not found")`
, then remember to set environment parameter `BIGTABLE_EMULATOR_HOST=localhost:8086`
so to let the examples connect onto the emulator.

To run it against real Bigtable instance:

```
GOOGLE_APPLICATION_CREDENTIALS=./service_account_key.json cargo run --bin simple_read
```

See `examples` folders for more examples.