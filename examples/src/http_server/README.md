# http_server

This example showing how to use `bigtable_rs` as a client in a http server like application.

## Run example:

Make sure Bigtable emulator is running:

```
. ./start_bigtable_local.sh
```

Start the service

```
BIGTABLE_EMULATOR_HOST=localhost:8086 RUST_LOG=bigtable_rs,http=trace cargo run --bin http_server
```

Then from another terminal, test out the service:

```
# read a key
curl localhost:3030/key1

# write a key
curl -H "Content-Type: application/json" -X POST localhost:3030/key1 -d '{ "value": "value1 is awesome"}'
```
