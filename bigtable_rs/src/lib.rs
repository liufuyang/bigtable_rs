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
mod root_ca_certificate;
pub mod util;

// Generated proto bindings for the Sessions subsystem. Marked #[doc(hidden)]
// because callers should reach for the public `SessionClient` / `TableShim`
// surface (arriving in later PRs), not the raw proto types.
#[cfg(feature = "sessions")]
#[doc(hidden)]
#[allow(dead_code)]
pub mod session_proto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
