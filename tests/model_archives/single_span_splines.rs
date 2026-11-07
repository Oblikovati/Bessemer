use super::lightweight_section_tests::next_case;
use super::sketch_spline_tests::{decode, decode_context};
use super::{ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/single-span-splines/cases.bin");
type SplineCase = (u32, &'static [u8], &'static str);

fn records() -> Vec<SplineCase> {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut result = Vec::new();
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        assert_eq!(mode, 0);
        result.push((version, bytes, oracle));
    }
    assert_eq!(result.len(), 800);
    result
}

#[test]
fn single_span_graphs_match_independent_fields_and_bytes() {
    let mut observed = 0;
    for (version, bytes, oracle) in records() {
        let fields = decode(0, version, bytes).unwrap();
        observed += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        assert!(!fields.iter().any(|f| f.path.contains("single_span_mode")));
    }
    assert_eq!(observed, 75720);
}

#[test]
fn single_span_graphs_reject_every_prefix_extra_byte_and_old_context() {
    let context = super::kernel_frame_tests::context();
    for (version, bytes, _) in records() {
        for length in 0..bytes.len() {
            assert!(decode_context(0, version, &bytes[..length], &context).is_err());
        }
        assert!(decode(0, 0x27cb, bytes).is_err());
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(decode_context(0, version, &extra, &context).is_err());
    }
}

#[test]
fn single_span_zero_controls_still_read_present_secondary_blocks() {
    let original = include_bytes!("../fixtures/single-span-splines/original.bin");
    let fields = decode(0, 19000, original).unwrap();
    let find = |path: &str| {
        &fields
            .iter()
            .find(|f| f.path.as_ref() == path)
            .unwrap()
            .value
    };
    assert_eq!(
        find("root.splines[0].control_point_count"),
        &ModelFieldValue::I32(0)
    );
    assert_eq!(
        find("root.splines[0].secondary_interpolation.count"),
        &ModelFieldValue::U32(3)
    );
    assert_eq!(
        find("root.splines[0].unknown_190"),
        &ModelFieldValue::I32(-31)
    );
}

#[test]
fn single_span_edit_matches_independent_golden_and_isolates_source() {
    let original = include_bytes!("../fixtures/single-span-splines/original.bin");
    let expected = include_bytes!("../fixtures/single-span-splines/edited.bin");
    let fields = decode(0, 19000, original).unwrap();
    let mut changed = fields.clone();
    changed
        .iter_mut()
        .find(|f| f.path.ends_with("unknown_190"))
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), expected);
    assert_eq!(decode(0, 19000, expected).unwrap(), changed);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), original);
}

#[test]
fn single_span_unproven_modes_stop_before_the_spline_payload() {
    for mask in [0x81_u16, 0x100, 0x180, 0x280, 0x480, 0x880, 0xffff] {
        let mut raw = 1_i32.to_le_bytes().to_vec();
        raw.extend_from_slice(&mask.to_le_bytes());
        let Err(error) = decode(0, 19000, &raw) else {
            panic!("unproven mode was accepted")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}
