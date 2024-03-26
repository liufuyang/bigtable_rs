//! `google` module having the Rust code generated from Google Bigtable V2 gPRC API proto file.
#[path = "google"]
pub mod bigtable {
    #[path = "google.bigtable.v2.rs"]
    pub mod v2;
}

#[path = "google"]
pub mod cloud {
    #[path = "./"]
    pub mod conformace {
        #[path = "./"]
        pub mod bigtable {
            #[path = "google.cloud.conformance.bigtable.v2.rs"]
            pub mod v2;
        }
    }
}

#[path = "google/google.rpc.rs"]
pub mod rpc;
