#[path = "google"]
pub mod bigtable {
    #[path = "google.bigtable.v2.rs"]
    pub mod v2;
}

#[path = "google/google.rpc.rs"]
mod rpc;
