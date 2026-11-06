use super::*;
use std::io::Write;

#[test]
fn measured_compressed_growth_obeys_original_carrier_bounds_for_every_version() {
    for version in 0_i32..=5 {
        let frame = growth_frame(version);
        let source = carrier(&frame.stored, 421, frame.stored.len());
        let id = source.latest(421).unwrap().unwrap().id;
        let bodies = source.record_421_bodies(&id, options(&frame)).unwrap();
        let mut changed = frame.decoded.clone();
        changed[5..13].copy_from_slice(&0x123456789abcdef0_u64.to_le_bytes());
        assert!(compressed(&changed, flate2::Compression::fast()).len() > frame.stored.len());
        assert!(bodies.replace_pool_word(0, 0, 0x123456789abcdef0).is_err());
        assert_eq!(bodies.source().as_bytes(), source.as_bytes());
        assert_eq!(bodies.decoded_bytes(), frame.decoded);
    }
}

fn growth_frame(version: i32) -> PointerFrame {
    let mut decoded = version.to_le_bytes().to_vec();
    decoded.push(255);
    decoded.resize(5 + 3 * 255 * 8, 0);
    PointerFrame {
        name: format!("growth-v{version}"),
        compressed: true,
        leaf: 1,
        counts: [0; 3],
        codes: None,
        stored: compressed(&decoded, flate2::Compression::best()),
        decoded,
    }
}

fn compressed(raw: &[u8], level: flate2::Compression) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), level);
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}

#[test]
fn edited_revision_rejects_the_original_occurrence_identity() {
    let frame = growth_frame(0);
    let bodies = open(&frame.stored, options(&frame)).unwrap();
    let edit = bodies.replace_pool_word(0, 0, 3).unwrap();
    assert_invalid(
        edit.result()
            .source()
            .record_421_bodies(bodies.record(), options(&frame)),
    );
    assert_eq!(bodies.decoded_bytes(), frame.decoded);
}
