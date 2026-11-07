use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/fixed-valued-transforms/cases.bin"),
        b"VTF1",
        216,
    )
}

fn root(kind: u8) -> &'static str {
    if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

#[test]
fn native_fixed_valued_transform_regression_accepts_complete_suffix() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 1 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn complete_fixed_valued_graphs_match_every_independent_byte_field_and_slot() {
    assert_complete_records(records());
}

#[test]
fn fixed_valued_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn full_presence_words_select_exactly_thirty_unsigned_words() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let prefix = root(kind);
        let presence = fields
            .iter()
            .find(|f| f.path.as_ref() == format!("{prefix}.has_plane_transform"))
            .unwrap();
        assert_eq!(
            presence.value,
            ModelFieldValue::U16([0, 1, 2, 65535, 32768, 255][usize::from(kind % 6)])
        );
        let count = fields
            .iter()
            .filter(|f| {
                f.path
                    .starts_with(&format!("{prefix}.plane_transform.words["))
            })
            .count();
        assert_eq!(count, if kind % 6 == 0 { 0 } else { 30 });
    }
}

#[test]
fn unsupported_perspective_bits_fail_at_the_fixed_flag_word() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 1) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for bit in [0x10u32, 0x20, 0x40, 0x80] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("{}.plane_transform.words[28]", root(kind)),
                &bit.to_le_bytes(),
            );
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected perspective rejection")
            };
            assert_eq!(error.class, crate::FailureClass::Unsupported);
            assert!(error.cause.contains("0xF0"));
        }
    }
}

#[test]
fn preserved_legacy_compact_vectors_fail_without_a_production_fallback() {
    use crate::archive_codec::ArchiveCursor;
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/fixed-valued-transforms/legacy-compact-cases.bin"
    ));
    let mut rejected = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = super::lightweight_section_tests::next_case(&mut cursor);
        if mode != 0 || !legacy_valued(bytes) {
            continue;
        }
        if decode_graph(version, bytes, false).is_err() {
            rejected += 1;
        }
    }
    assert_eq!(rejected, 104);
}

fn legacy_valued(bytes: &[u8]) -> bool {
    let length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
    let name = std::str::from_utf8(&bytes[6..6 + length]).unwrap();
    super::sketch_dimensions::G1_CLASSES.contains(&name)
        || super::sketch_dimensions::G2_CLASSES.contains(&name)
        || super::sketch_dimensions::OFFSET_CLASSES.contains(&name)
        || name == "sgValG3Dim"
}
