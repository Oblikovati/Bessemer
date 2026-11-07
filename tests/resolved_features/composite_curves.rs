use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelField, ModelFieldValue};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/composite-curves/cases.bin"),
        b"CCV1",
        132,
    )
}

fn prefix(kind: u8) -> &'static str {
    if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

#[test]
fn composite_graphs_match_all_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn composite_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn independent_dword_counts_and_unsigned_high_bits_survive() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for column in 0..4 {
            assert_dword_column(&fields, kind, column);
        }
    }
}

const COUNTS: [[usize; 4]; 6] = [
    [0, 0, 0, 0],
    [3, 0, 1, 2],
    [1, 4, 0, 3],
    [4, 2, 3, 0],
    [0, 1, 4, 2],
    [3, 0, 2, 4],
];

fn assert_dword_column(fields: &[ModelField], kind: u8, column: usize) {
    let offset = ["7a8", "7c8", "7e8", "808"][column];
    let path = format!("{}.payload.dwords_{offset}[", prefix(kind));
    let values: Vec<_> = fields
        .iter()
        .filter(|f| f.path.starts_with(&path))
        .collect();
    assert_eq!(values.len(), COUNTS[usize::from(kind % 6)][column]);
    for (index, field) in values.into_iter().enumerate() {
        let expected = [u32::MAX, 0x80000000, 0, 0x89abcdef][(column + index) % 4];
        assert_eq!(field.value, ModelFieldValue::U32(expected));
    }
}

fn owner_index(fields: &[ModelField], path: &str) -> u16 {
    let owner = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(record) = &owner.value else {
        panic!("expected complete curve tag")
    };
    let crate::archive_codec::ObjectRecord::New { index, .. } = &record.record else {
        panic!("expected new curve")
    };
    u16::try_from(index.value()).unwrap()
}

#[test]
fn only_tagged_array_accepts_array_class_and_cached_identity() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 2) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("{}.payload.tagged_array_778", prefix(kind)),
            &owner_index(&fields, prefix(kind)).to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, true) else {
            panic!("expected rejected curve in array role")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

#[test]
fn escaped_direct_counts_are_bounded_before_reading_entries() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 5) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for offset in ["7a8", "7c8", "7e8", "808"] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("{}.payload.dwords_{offset}.count.count_dword", prefix(kind)),
                &u32::MAX.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, true).is_err());
        }
    }
}

#[test]
fn present_kernel_body_requires_explicit_schema_context() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        assert!(decode_graph(version, bytes, false).is_err());
    }
}
