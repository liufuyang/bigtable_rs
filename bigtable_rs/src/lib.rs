//!
//! A simple Google Bigtable client.
//!
//! See [`bigtable`] package for more info.
//!
//! [[github repo]]
//!
//! [`bigtable`]: mod@crate::bigtable
//! [github repo]: https://github.com/liufuyang/bigtable_rs
mod auth_service;
pub mod bigtable;
pub mod google {
    pub mod bigtable {
        pub use googleapis_tonic_google_bigtable_v2::google::bigtable::v2;
    }
}
mod root_ca_certificate;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
