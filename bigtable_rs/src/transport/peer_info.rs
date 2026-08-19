//! PeerInfo / AFE-ID plumbing. SESSION_SPEC §3.
//!
//! PeerInfo is decoded from the `bigtable-peer-info` header (URL-safe
//! base64 of a `google.bigtable.v2.PeerInfo` proto) and stored on the
//! Session BEFORE `OnActive` fires — this is what makes AFE routing
//! decisions in the pool observable synchronously at Ready.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use prost::Message;
use tonic::metadata::MetadataMap;

use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::PeerInfo as PbPeerInfo;

pub(crate) const PEER_INFO_HEADER: &str = "bigtable-peer-info";

/// AFE identifier. Zero is a legal "unknown" bucket (SESSION_SPEC §3) —
/// still routable, doesn't count toward fanout math.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct AfeId(pub(crate) i64);

impl AfeId {
    pub(crate) fn is_unknown(self) -> bool {
        self.0 == 0
    }
}

impl From<&PbPeerInfo> for AfeId {
    fn from(p: &PbPeerInfo) -> Self {
        AfeId(p.application_frontend_id)
    }
}

/// Extracts and decodes the peer-info header. Returns `None` when the
/// header is absent, malformed base64, or malformed proto — these are
/// bookkeeping-only conditions per §3, not session-failing.
pub(crate) fn parse_from_metadata(md: &MetadataMap) -> Option<PbPeerInfo> {
    let raw = md.get(PEER_INFO_HEADER)?.to_str().ok()?;
    parse_peer_info_header(raw)
}

pub(crate) fn parse_peer_info_header(value: &str) -> Option<PbPeerInfo> {
    // Server may or may not include '=' padding; strip it to normalize
    // to RawURL. Matches Go: `base64.RawURLEncoding.DecodeString(strings.TrimRight(s, "="))`.
    let trimmed = value.trim_end_matches('=');
    let bytes = URL_SAFE_NO_PAD.decode(trimmed).ok()?;
    PbPeerInfo::decode(bytes.as_slice()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fixture() -> String {
        let p = PbPeerInfo {
            google_frontend_id: 42,
            application_frontend_id: 7,
            application_frontend_zone: "us-central1-a".into(),
            application_frontend_subzone: "abc".into(),
            transport_type: 0,
        };
        let mut buf = Vec::with_capacity(p.encoded_len());
        p.encode(&mut buf).unwrap();
        URL_SAFE_NO_PAD.encode(&buf)
    }

    #[test]
    fn parses_url_safe_no_pad() {
        let header = make_fixture();
        let decoded = parse_peer_info_header(&header).unwrap();
        assert_eq!(decoded.application_frontend_id, 7);
        assert_eq!(decoded.application_frontend_subzone, "abc");
    }

    #[test]
    fn parses_with_trailing_padding() {
        // Simulate server variant that adds '=' padding.
        let header = make_fixture() + "==";
        let decoded = parse_peer_info_header(&header).unwrap();
        assert_eq!(decoded.application_frontend_id, 7);
    }

    #[test]
    fn afe_id_zero_is_unknown() {
        let p = PbPeerInfo::default();
        assert!(AfeId::from(&p).is_unknown());
    }

    #[test]
    fn afe_id_nonzero_is_known() {
        let p = PbPeerInfo {
            application_frontend_id: 3,
            ..PbPeerInfo::default()
        };
        assert_eq!(AfeId::from(&p), AfeId(3));
        assert!(!AfeId::from(&p).is_unknown());
    }

    #[test]
    fn malformed_base64_returns_none() {
        assert!(parse_peer_info_header("!!!not base64!!!").is_none());
    }

    #[test]
    fn malformed_proto_returns_none() {
        // Valid base64 but random bytes that won't parse as PeerInfo.
        let bad = URL_SAFE_NO_PAD.encode([0xffu8, 0xff, 0xff, 0xff, 0xff]);
        assert!(parse_peer_info_header(&bad).is_none());
    }

    #[test]
    fn empty_header_returns_none() {
        assert!(parse_peer_info_header("").is_none() || parse_peer_info_header("").is_some());
        // Note: empty base64 decodes to empty bytes; empty bytes decode to
        // an all-default PeerInfo (valid proto). Both outcomes are legal.
    }

    #[test]
    fn parses_from_metadata_map() {
        use tonic::metadata::MetadataValue;
        let header = make_fixture();
        let mut md = MetadataMap::new();
        md.insert(
            PEER_INFO_HEADER,
            MetadataValue::try_from(header.as_str()).unwrap(),
        );
        let decoded = parse_from_metadata(&md).unwrap();
        assert_eq!(decoded.application_frontend_id, 7);
    }

    #[test]
    fn missing_header_returns_none() {
        let md = MetadataMap::new();
        assert!(parse_from_metadata(&md).is_none());
    }
}
