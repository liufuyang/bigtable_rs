//! Transport layer for Bigtable Sessions. SESSION_SPEC + SESSION_POOL_SPEC
//! territory.
//!
//! This PR (#2) lands per-Session lifecycle only:
//! - state machine (`state`)
//! - close-reason CAS-once (`close_reason`)
//! - lifecycle hooks (`hooks`)
//! - PeerInfo header parsing (`peer_info`)
//! - AttemptState + errors (`errors`)
//! - vRPC slot + RpcId + InFlightRpc (`vrpc`)
//! - heartbeat watchdog (`heartbeat`)
//! - Session orchestration (`session`)
//! - GOAWAY 7-step handler (`goaway`)
//! - non-blocking debug-tag ring (`debug_tag`)
//!
//! Later PRs land under this module:
//! - PR #3: retry oracle (`retrying`)
//! - PR #4: sessionList + picker + PeakEwma + PickDecision
//! - PR #5: SessionPool + PoolSizer + budget + Diverter + wire integration
//! - PR #9: session_tracer (OTel session-lifetime histograms)

pub(crate) mod close_reason;
pub(crate) mod debug_tag;
pub(crate) mod errors;
pub(crate) mod goaway;
pub(crate) mod heartbeat;
pub(crate) mod hooks;
pub(crate) mod peer_info;
pub(crate) mod session;
pub(crate) mod state;
pub(crate) mod vrpc;
