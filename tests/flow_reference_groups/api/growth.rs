use super::*;
use std::io::Write;

#[test]
fn measured_zlib_growth_in_every_version_obeys_original_carrier_limits() {
    for version in 0..18 {
        let frame = growth_frame(version);
        let source = carrier(&frame.stored, 416, frame.stored.len());
        let id = source.latest(416).unwrap().unwrap().id;
        let view = source.record_416_references(&id, options(&frame)).unwrap();
        let field = if version < 6 {
            FlowReferenceHeaderField::Discarded
        } else {
            FlowReferenceHeaderField::Stored
        };
        let mut changed = frame.decoded.clone();
        changed[4..8].copy_from_slice(&0x12345678_i32.to_le_bytes());
        assert!(compressed(&changed, flate2::Compression::fast()).len() > frame.stored.len());
        assert!(view.replace_header(field, 0x12345678).is_err());
        assert_eq!(view.decoded_bytes(), frame.decoded);
        assert_eq!(view.source().as_bytes(), source.as_bytes());
    }
}

fn growth_frame(version: i32) -> ReferenceFrame {
    let mut decoded = version.to_le_bytes().to_vec();
    if version < 6 {
        decoded.extend_from_slice(&0_i32.to_le_bytes());
    }
    if version >= 3 {
        decoded.extend_from_slice(&0_i32.to_le_bytes());
    }
    decoded.extend_from_slice(&[0; 8]);
    ReferenceFrame {
        name: format!("growth-v{version}"),
        compressed: true,
        mapping: false,
        stored: compressed(&decoded, flate2::Compression::best()),
        decoded,
    }
}

fn compressed(raw: &[u8], level: flate2::Compression) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), level);
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
