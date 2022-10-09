# bigtable_rs

A simple Rust library for working
with [Google Bigtable](https://cloud.google.com/bigtable/docs/) [Data API V2](https://cloud.google.com/bigtable/docs/reference/data/rpc/google.bigtable.v2)
.

![ci_badge](https://github.com/liufuyang/bigtable_rs/workflows/bigtable_rs%20CI/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/bigtable_rs)](https://crates.io/crates/bigtable_rs)
[![Documentation](https://docs.rs/bigtable_rs/badge.svg)](https://docs.rs/bigtable_rs)
[![Crates.io](https://img.shields.io/crates/l/bigtable_rs)](LICENSE)

## Disclaimer - might be ready for production use

This library might be production ready. Any contribution or help is highly appreciated.

I had the idea to build a client like this and got some input from others
see [here](https://github.com/durch/rust-bigtable/issues/4)
and porting code from [here](
https://github.com/solana-labs/solana/tree/master/storage-bigtable)
(thanks to [@mvines](https://github.com/mvines)).

Code for [read_rows parsing](https://github.com/liufuyang/bigtable_rs/blob/main/bigtable_rs/src/bigtable/read_rows.rs#L36-L212) logic
is [tested](https://github.com/liufuyang/bigtable_rs/blob/main/bigtable_rs/tests/read_rows/read_rows_test.rs)
by [porting](https://github.com/liufuyang/bigtable_rs/blob/main/bigtable_rs/tests/read_rows/read_rows_test.json)
Google's Java client
[tests code](https://github.com/googleapis/java-bigtable/blob/main/google-cloud-bigtable/src/test/java/com/google/cloud/bigtable/data/v2/stub/readrows/ReadRowsMergingAcceptanceTest.java)
with [json test cases](https://github.com/googleapis/conformance-tests/blob/main/bigtable/v2/readrows.json) as raw
input.

## Introduction

Current idea is to make this library very light weighted, and you assemble requests based
on [Google Bigtable V2 protobuf schema](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto)
and send the requests via [tonic gRPC over HTTP/2](https://github.com/hyperium/tonic). So the user have the flexibility
of creating any type of Bigtable request and use this client to talk to Bigtable service.

Compiled Bigtable API proto as Rust code is also included in the repo here so users don't need to compile from proto
again.

The returned row values from Bigtable is parsed by this library.

Supported interfaces towards Bigtable:

* [ReadRows](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto#L55)
* [SampleRowKeys](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto#L68)
* [MutateRow](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto#L78)
* [MutateRows](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto#L90)

For other gRPC APIs/methods, one should be able to use the gRCP client directly and assemble the request you need to
interact with Bigtable service via building the Protobuf messages (already complied as rs files and included here).

[gcp_auth](https://github.com/hrvolapeter/gcp_auth) is used, which
supports:

* Application Default Credentials
* Connection authenticated via Google service account key `json` file
  (by setting `GOOGLE_APPLICATION_CREDENTIALS=path/to/key.json` environment parameter)
* Default service account by retrieving a token from the gcloud metadata server

You can use the library as follow:

```toml
[dependencies]
bigtable_rs = "0.2.0"
tokio = { version = "1.0", features = ["rt-multi-thread"] }
env_logger = "0.9.1"
```

Documentation is on [crate.io](https://docs.rs/bigtable_rs/0.1.3/bigtable_rs/).
See [examples](./examples) folders for more examples. The following example showing how to do a key range scan

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
        table_name: bigtable.get_full_table_name(table_name),
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

```

---

## Play with example code

To start develop or test the example code above,
install [cbt](https://cloud.google.com/bigtable/docs/cbt-overview) tool if you haven't,
then start a test Bigtable instance locally and insert some test data like this:

```
# at one terminal, start a bigtable insatnce locally
. start_bigtable_local.sh

# at another terminal, load some data into it
. start_load_table_local.sh
```

Then run an example with this command:

```
BIGTABLE_EMULATOR_HOST=localhost:8086 RUST_LOG=bigtable_rs=trace cargo run --bin simple_read
```

If you see error related to google authentication or gcp project not found,
then remember to set environment parameter `BIGTABLE_EMULATOR_HOST=localhost:8086`
so to let the example's client connect onto the local emulator.

To run it against real Bigtable instance:

```
GOOGLE_APPLICATION_CREDENTIALS=<path_to_key>/service_account_key.json cargo run --bin simple_read
```
Or if you want to use your `gcloud auth login` auth then simply run the client without any special env settings:
```
cargo run --bin simple_read
```

See `examples` folders for more examples.

## Development

Clone this repo, then checkout the submodules if necesary

```
cd googleapis
git submodule init
git submodule update
```

Then checkout `bigtable_rs/src/build.rs` to update Google protos

Running tests:

```
cargo test -- --nocapture
```
