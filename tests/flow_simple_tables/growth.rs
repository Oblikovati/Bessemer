use super::*;
use std::io::Write;

#[test]
fn measured_growth_for_each_table_obeys_original_carrier_limits_and_source_isolation() {
    for identifier in [407, 437, 411, 413] {
        let frame = growth_frame(identifier);
        let source = carrier(&frame, frame.stored.len());
        let view = read_at(
            &source,
            &source.latest(identifier).unwrap().unwrap().id,
            &frame,
        )
        .unwrap();
        let (offset, bytes) = growth_value(identifier);
        let mut changed = frame.decoded.clone();
        changed[offset..offset + bytes.len()].copy_from_slice(&bytes);
        assert!(compressed(&changed, flate2::Compression::fast()).len() > frame.stored.len());
        assert!(apply_growth(&view).is_err());
        assert_eq!(view.decoded(), frame.decoded);
        assert_eq!(view.source().as_bytes(), source.as_bytes());
    }
}

fn growth_frame(identifier: u32) -> SimpleFrame {
    let decoded = growth_payload(identifier);
    SimpleFrame {
        name: format!("growth-{identifier}"),
        identifier,
        compressed: true,
        leaf: (identifier == 437).then_some(1024),
        legacy: None,
        stored: compressed(&decoded, flate2::Compression::best()),
        decoded,
    }
}

fn growth_payload(identifier: u32) -> Vec<u8> {
    match identifier {
        407 => parallel_growth(),
        437 => sized_payload(&[0], 16384),
        411 => sized_payload(&[4, 1024], 20480),
        413 => sized_payload(&[1, 2048], 2048),
        value => panic!("identifier {value}; expected simple table"),
    }
}

fn sized_payload(header: &[i32], bytes: usize) -> Vec<u8> {
    let mut raw: Vec<u8> = header
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    raw.resize(raw.len() + bytes, 0);
    raw
}

fn parallel_growth() -> Vec<u8> {
    let mut raw = sized_payload(&[0, 1024, 0, 1], 32768);
    raw.extend_from_slice(&0_i32.to_le_bytes());
    raw
}

fn growth_value(identifier: u32) -> (usize, Vec<u8>) {
    match identifier {
        407 => (16, 0x123456789abcdef0_u64.to_le_bytes().to_vec()),
        437 => (4, 0x123456789abcdef0_u64.to_le_bytes().to_vec()),
        411 => (8, 0x12345678_u32.to_le_bytes().to_vec()),
        413 => (8, vec![123]),
        value => panic!("identifier {value}; expected simple table"),
    }
}

fn apply_growth(view: &SimpleView) -> bessemer::Result<()> {
    match view {
        SimpleView::Parallel(table) => table.replace_word(0, 0, 0, 0x123456789abcdef0).map(|_| ()),
        SimpleView::LeafPairs(table) => table.replace_word(0, 0, 0x123456789abcdef0).map(|_| ()),
        SimpleView::Five(table) => table.replace_word(0, 0, 0x12345678).map(|_| ()),
        SimpleView::Bytes(table) => table.replace_byte(0, 123).map(|_| ()),
    }
}

fn compressed(raw: &[u8], compression: flate2::Compression) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), compression);
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
