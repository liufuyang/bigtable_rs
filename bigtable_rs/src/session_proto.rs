//! Rust bindings generated from `google/bigtable/v2/session.proto`.
//!
//! Types imported from other .proto files that are already published by the
//! `googleapis-tonic-google-bigtable-v2` and `googleapis-tonic-google-rpc`
//! crates are redirected there via `build.rs`'s `extern_path`; only the
//! session-family types (`OpenSessionRequest`, `CloseSessionRequest`,
//! `GetClientConfigurationRequest`, `GoAwayResponse`, `SessionRefreshConfig`,
//! and their inner messages) are generated here.
//!
//! Kept at `pub(crate)` visibility for now — the public Sessions surface
//! (`SessionClient`, `TableShim`, `Table`) will land in later PRs and expose
//! only the caller-facing types.

pub mod google {
    pub mod bigtable {
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/google.bigtable.v2.rs"));
        }
    }
}

#[allow(unused_imports)]
pub use google::bigtable::v2::*;
