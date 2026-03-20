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
