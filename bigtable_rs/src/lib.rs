//!
//! A simple Google Bigtable client.
//!
//! See [`bigtable`] package for more info.
//!
//! [[github repo]]
//!
//! [`bigtable`]: mod@crate::bigtable
//! [github repo]: https://github.com/liufuyang/bigtable_rs
mod access_token;
pub mod bigtable;
pub mod google;
mod root_ca_certificate;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
