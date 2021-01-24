# bigtable_rs

## Disclaimer - this is NOT ready for any production use

I just started trying to create a bigtable client. This library is far far away from a stage for others to use. But any
contribution or help is highly appreciated.

I had this idea and got some input from others see [here](https://github.com/durch/rust-bigtable/issues/4)
and basically ported many code from [here](
https://github.com/solana-labs/solana/tree/master/storage-bigtable). Will try work on it to make it more useful.

## Introduction 
Current idea is to make this library very light weighted and you assemble
requests based on [Google Bigtable V2 protobuf schema](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto) and issue request via 
tonic GRPC. So the user have the flexibility of creating any type of Bigtable
request and use this client to talk to Bigtable service.

Supported interfaces towards Bigtable:
* [ReadRows](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto#L55)
* [More will come...]

Also support connection authenticated via Google service account key `json` file 
(by setting `GOOGLE_APPLICATION_CREDENTIALS=path/to/key.json` environment parameter)

You can use the library as follow:
```toml
[dependencies]
bigtable_rs = { path = "../bigtable_rs" }
tokio = { version = "1.0", features = ["rt-multi-thread"] }
env_logger = "0.8.2"
```

The following example showing how to do a key range scan
```rust
use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
use bigtable_rs::google::bigtable::v2::row_range::{EndKey, StartKey};
use bigtable_rs::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowRange, RowSet};
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

    let key_start: String = "key1".to_owned();
    let key_end: String = "key4".to_owned();

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

    // prepare a ReadRowsRequest
    let request = ReadRowsRequest {
        table_name: format!("{}{}", bigtable.table_prefix, table_name),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![], // use this field to put keys for reading specific rows
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
                        filter: Some(Filter::CellsPerColumnLimitFilter(2)),
                    },
                ],
            })),
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
                "    [{}:{}] \"{}\" at {}",
                row_cell.family_name,
                String::from_utf8(row_cell.qualifier).unwrap(),
                String::from_utf8(row_cell.value).unwrap(),
                row_cell.timestamp_micros
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