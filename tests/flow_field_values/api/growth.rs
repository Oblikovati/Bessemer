use super::*;
use std::io::Write;

#[test]
fn measured_body_growth_preserves_raw_version_and_obeys_original_carrier_limits() {
    for version in [5, 6, 7, 8, 99, i32::MAX] {
        let frame = growth_frame(version);
        let source = carrier(&frame.stored, 108, frame.stored.len());
        let id = source.latest(108).unwrap().unwrap().id;
        let view = source.record_108_values(&id, options(&frame)).unwrap();
        let offset = if version < 7 { 0 } else { 8 };
        let mut changed = frame.decoded.clone();
        changed[offset..offset + 8].copy_from_slice(&0x123456789abcdef0_u64.to_le_bytes());
        assert!(4 + compressed(&changed, flate2::Compression::fast()).len() > frame.stored.len());
        let result = if version < 7 {
            view.replace_object_word(0, 0, 0x123456789abcdef0)
        } else {
            view.replace_region_word(0, 0, 0x123456789abcdef0)
        };
        assert!(result.is_err());
        assert_eq!(view.body_bytes(), frame.decoded);
        assert_eq!(view.source().as_bytes(), source.as_bytes());
    }
}

fn growth_frame(version: i32) -> FieldFrame {
    let (context, mut decoded) = growth_context(version);
    decoded.resize(decoded.len() + 8192, 0);
    let mut stored = version.to_le_bytes().to_vec();
    stored.extend_from_slice(&compressed(&decoded, flate2::Compression::best()));
    FieldFrame {
        name: format!("growth-v{version}"),
        compressed: true,
        restriction: false,
        context,
        stored,
        decoded,
    }
}

fn compressed(body: &[u8], level: flate2::Compression) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), level);
    encoder.write_all(body).unwrap();
    encoder.finish().unwrap()
}

fn growth_context(version: i32) -> (FieldContext, Vec<u8>) {
    if version < 7 {
        (
            FieldContext::Legacy {
                table_count: 0,
                object_widths: vec![1024],
                expected: None,
            },
            vec![],
        )
    } else {
        (
            FieldContext::Regions {
                rows: vec![1024],
                widths: vec![1],
                active: vec![true],
            },
            [1_i32, 1].into_iter().flat_map(i32::to_le_bytes).collect(),
        )
    }
}
