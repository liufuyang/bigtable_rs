mod access_token;
pub mod bigtable;
mod root_ca_certificate;

mod google {
    mod rpc {
        include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        concat!("/src/google/google.rpc.rs")
        ));
    }

    pub mod bigtable {
        pub mod v2 {
            include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            concat!("/src/google/google.bigtable.v2.rs")
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
