//! Smoke test: assert every Sessions-critical proto type is generated and
//! links against the extern crates without a compilation error.
//!
//! Grows over the stack — later PRs add integration tests that actually
//! use these types; this file just guards the proto surface.

#![cfg(feature = "sessions")]

use bigtable_rs::session_proto as pb;

#[test]
fn session_envelopes_compile() {
    // Envelope types the SessionClient/Session/pool layers depend on.
    let _ = pb::OpenSessionRequest::default();
    let _ = pb::OpenSessionResponse::default();
    let _ = pb::CloseSessionRequest::default();
    let _ = pb::GoAwayResponse::default();
    let _ = pb::SessionRefreshConfig::default();

    // Per-resource open envelopes (SESSION_CLIENT_SPEC #22).
    let _ = pb::OpenTableRequest::default();
    let _ = pb::OpenAuthorizedViewRequest::default();
    let _ = pb::OpenMaterializedViewRequest::default();

    // Server-config surface (SESSION_CLIENT_SPEC #14).
    let _ = pb::GetClientConfigurationRequest::default();
    let _ = pb::ClientConfiguration::default();

    // vRPC surface (SESSION_SPEC §2).
    let _ = pb::VirtualRpcRequest::default();
    let _ = pb::VirtualRpcResponse::default();
    let _ = pb::HeartbeatResponse::default();
}

#[test]
fn extern_types_are_reused_from_googleapis_crate() {
    // If extern_path bindings drift, this fails to compile: the field's type
    // is the extern crate's Row, so assigning a locally-constructed default
    // from the same path proves they're the same type.
    use googleapis_tonic_google_bigtable_v2::google::bigtable::v2 as btpb;

    let mut resp = pb::SessionReadRowResponse::default();
    resp.row = Some(btpb::Row::default());

    let mut req = pb::SessionMutateRowRequest::default();
    req.mutations = vec![btpb::Mutation::default()];
}
