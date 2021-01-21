# bigtable-rs

## Disclaimer - this is NOT ready for any production use
I just started trying to create a bigtable client. This library is 
far far away from a stage for others to use. But any contribution or 
help is highly appreciated.

I had this idea and got some input from others see here
https://github.com/durch/rust-bigtable/issues/4
and basically ported many code from 
https://github.com/solana-labs/solana/tree/master/storage-bigtable.
Will try work on it to make it more useful.

You can use the library as follow:

```rust
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

    let connection = bigtable::BigTableConnection::new(instance_name, true, None, channel_size).await?;
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
# export BIGTABLE_EMULATOR_HOST=localhost:8086
cargo run --bin simple_read
```
If you see error `Error: AccessTokenError("GOOGLE_APPLICATION_CREDENTIALS environment variable not found")`
, then remember to set environment parameter `BIGTABLE_EMULATOR_HOST=localhost:8086` 
so to let the examples connect onto the emulator.

See `examples` folders for more examples.