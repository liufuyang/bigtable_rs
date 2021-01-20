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
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let instance_name = "test1";
    let table_name = "my-table";
    let key: String = "key1".to_owned();

    println!("hello_world");
    let connection = bigtable::BigTableConnection::new(instance_name, true, None).await?;
    let mut bigtable = connection.client();
    let row_data = bigtable.get_single_row_data(table_name, key).await?;
    if let Some((cell, value)) = row_data.get(0) {
        println!("{}, {}", cell, String::from_utf8(value.to_vec())?);
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