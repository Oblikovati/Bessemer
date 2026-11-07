use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/reference-dimensions/cases.bin");
type ReferenceRecord = (u8, u32, &'static [u8], &'static str);

fn records() -> Vec<ReferenceRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    let mut result = Vec::new();
    while cursor.remaining() != 0 {
        result.push(next_case(&mut cursor));
    }
    assert_eq!(result.len(), 250);
    result
}

#[test]
fn complete_reference_dimensions_handles_and_plane_tags_match_independent_bytes_and_fields() {
    let mut observations = 0;
    for (_, version, bytes, oracle) in records() {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        observations += oracle.lines().count();
    }
    assert!(observations > 10000);
}

#[test]
fn reference_dimensions_reject_all_incomplete_prefixes_extra_bytes_and_old_context() {
    for (kind, version, bytes, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], None, LIMITS).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), None, LIMITS).is_err());
        assert!(decode_graph(0x27cb, bytes, None, LIMITS).is_err());
    }
}

#[test]
fn plane_tags_preserve_unknown_null_kinds_and_reject_unknown_nonnull_or_wrong_face() {
    let records = records();
    let null = records
        .iter()
        .find(|record| record.0 == 4 && record.1 == 19000 && record.2.ends_with(&[255, 255, 0, 0]))
        .unwrap();
    let fields = decode_graph(null.1, null.2, None, LIMITS).unwrap().0;
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), null.2);
    let point = records.iter().rev().find(|record| record.0 == 4).unwrap();
    let fields = decode_graph(point.1, point.2, None, LIMITS).unwrap().0;
    for kind in [1u16, 3, 65535] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.stored_kind",
            &kind.to_le_bytes(),
        );
        let error = decode_graph(point.1, &changed, None, LIMITS).unwrap_err();
        assert_eq!(
            error.class,
            if kind == 1 {
                crate::FailureClass::Invalid
            } else {
                crate::FailureClass::Unsupported
            }
        );
    }
}

#[test]
fn reference_dimension_word_edit_matches_independent_golden_and_keeps_original_owned() {
    let original = records()
        .into_iter()
        .filter(|record| record.0 == 2 && record.1 == 19000)
        .nth(3)
        .unwrap();
    let mut input = original.2.to_vec();
    let fields = decode_graph(original.1, &input, None, LIMITS).unwrap().0;
    input.fill(0xa5);
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.raw_118",
        &65535u16.to_le_bytes(),
    );
    assert_eq!(
        changed,
        include_bytes!("../fixtures/reference-dimensions/edited.bin")
    );
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), original.2);
    decode_graph(original.1, &changed, None, LIMITS).unwrap();
}
