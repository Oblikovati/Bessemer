use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

const NUMBERS: [[u64; 3]; 6] = [
    [
        0x8000_0000_0000_0000,
        0x7ff8_0000_0000_002a,
        0x7ff0_0000_0000_0000,
    ],
    [
        0x7ff8_0000_0000_002b,
        0x7ff0_0000_0000_002c,
        0xfff0_0000_0000_0000,
    ],
    [1, 0x8000_0000_0000_0000, 4],
    [
        0x3ff4_0000_0000_0000,
        0x8000_0000_0000_0000,
        0x7ff8_0000_0000_0043,
    ],
    [
        0xfff0_0000_0000_0000,
        0x7ff8_0000_0000_0099,
        0x4000_0000_0000_0000,
    ],
    [
        0x7ff8_0000_0000_00aa,
        0x7ff0_0000_0000_00bb,
        0x8000_0000_0000_0000,
    ],
];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sew-surfaces/cases.bin"),
        b"SEW1",
        108,
    )
}

#[test]
fn complete_sew_surfaces_and_keepers_match_independent_bytes_slots_and_primitives() {
    assert_complete_records(records());
}

#[test]
fn sew_surfaces_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn sew_payload_minimum_is_58_bytes_and_keeper_adds_no_body_or_phantom_map_slots() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|r| [0, 6].contains(&r.0))
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        if kind == 6 {
            let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
            assert_eq!(bytes.len(), 6 + name_length);
            assert_eq!(slots, 3);
            continue;
        }
        let payload: Vec<_> = fields
            .into_iter()
            .filter(|f| f.path.starts_with("root.object.payload."))
            .collect();
        assert_eq!(super::encode_fields(&payload, 1000).unwrap().len(), 58);
        assert_eq!(slots, 5);
    }
}

fn assert_raw_payload(fields: &[super::ModelField], style: usize) {
    let words = [i32::MIN, -1, 0, 1, 2, i32::MAX];
    for index in 0..6 {
        let suffix = format!(".raw_flag_words[{index}]");
        for field in fields.iter().filter(|f| f.path.ends_with(&suffix)) {
            assert_eq!(
                field.value,
                ModelFieldValue::I32(words[(index + style) % 6])
            );
        }
    }
    for (column, offset) in ["18", "28", "20"].into_iter().enumerate() {
        let suffix = format!(".raw_{offset}");
        for field in fields.iter().filter(|f| f.path.ends_with(&suffix)) {
            assert!(
                matches!(field.value,ModelFieldValue::F64(value) if value.bits()==NUMBERS[style][column])
            );
        }
    }
}

#[test]
fn sew_flag_words_and_nonmonotonic_double_order_preserve_every_raw_bit() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_raw_payload(&fields, usize::from(kind));
        assert!(fields.iter().all(|f| !f.path.contains("array_d0")));
    }
}

fn assert_invalid_replacement(version: u32, bytes: &[u8], path: &str, replacement: &[u8]) {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, replacement);
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected impossible count or unrelated keeper");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn sew_direct_arrays_reject_impossible_packed_counts_before_allocating() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0 && r.1 == 19000) {
        for offset in ["30", "50", "70", "90", "b0"] {
            let path = format!("root.object.payload.array_{offset}.count.count_word");
            assert_invalid_replacement(version, bytes, &path, &[255; 6]);
        }
    }
}

#[test]
fn actual_sew_header_keeper_rejects_null_and_cached_array_targets() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 11) {
        let path = "root.object.items[1].body.id_keeper";
        assert_invalid_replacement(version, bytes, path, &[0, 0]);
        assert_invalid_replacement(version, bytes, path, &2u16.to_le_bytes());
    }
}
