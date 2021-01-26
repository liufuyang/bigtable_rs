//!
//! A simple Google Bigtable client
//!
//! See [`bigtable`] package for more info.
//!
//! [`bigtable`]: mod@crate::bigtable
mod access_token;
pub mod bigtable;
pub mod google;
mod root_ca_certificate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
