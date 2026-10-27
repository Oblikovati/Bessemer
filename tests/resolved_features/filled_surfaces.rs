use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

const FLAGS: [u8; 6] = [0, 1, 127, 128, 254, 255];
const SIGNED: [i8; 6] = [0, 1, 127, -128, -2, -1];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/filled-surfaces/cases.bin"),
        b"FLS1",
        264,
    )
}

#[test]
fn filled_surfaces_contacts_and_keepers_match_complete_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn filled_surface_family_rejects_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn filled_surface_minima_keep_direct_keeper_byte_widths_and_real_map_slots() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        if kind == 0 {
            let payload: Vec<_> = fields
                .into_iter()
                .filter(|f| f.path.starts_with("root.object.payload."))
                .collect();
            assert_eq!(super::encode_fields(&payload, 1000).unwrap().len(), 45);
            assert_eq!(slots, 5);
            continue;
        }
        let length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(
            bytes.len() - 6 - length,
            [2, 62, 8][usize::from(kind / 6 - 1)]
        );
        assert_eq!(slots, 3);
    }
}

#[test]
fn filled_byte_sign_interpretation_and_writer_mutated_word_keep_original_bits() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for field in fields.iter().filter(|f| f.path.ends_with(".raw_10c")) {
            let ModelFieldValue::U8(byte) = field.value else {
                panic!("raw_10c: expected one stored byte");
            };
            assert_eq!(byte, FLAGS[usize::from(kind)]);
            assert_eq!(i8::from_ne_bytes([byte]), SIGNED[usize::from(kind)]);
        }
        for field in fields.iter().filter(|f| f.path.ends_with(".raw_74")) {
            assert_eq!(field.value, ModelFieldValue::I32(2));
        }
    }
}

fn assert_invalid_replacement(version: u32, bytes: &[u8], path: &str, replacement: &[u8]) {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    assert!(
        fields.iter().any(|f| f.path.as_ref() == path),
        "missing test field {path}"
    );
    let changed = super::manager_context_tests::replace_field(&fields, path, replacement);
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected count or typed pointer");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn filled_arrays_and_direct_keeper_reject_impossible_counts() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0 && r.1 == 19000) {
        for label in ["array_10", "array_30", "array_50", "keeper_118.array_48"] {
            let path = format!("root.object.payload.{label}.count.count_word");
            assert_invalid_replacement(version, bytes, &path, &[255; 6]);
        }
    }
}

#[test]
fn curve_contact_typed_pointers_and_data_wrapper_reject_cached_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| [14, 20].contains(&r.0)) {
        let labels: &[&str] = if kind == 14 {
            &["angle_a0", "synch_e0"]
        } else {
            &["wrapper_18"]
        };
        for label in labels {
            let path = format!("root.object.items[0].{label}");
            assert_invalid_replacement(version, bytes, &path, &2u16.to_le_bytes());
        }
    }
}

#[test]
fn curve_contact_data_generic_reference_retains_containing_array_alias() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 23) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let field = fields
            .iter()
            .find(|f| f.path.as_ref() == "root.object.items[0].reference_08")
            .unwrap();
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            panic!("reference_08: expected typed CObject tag");
        };
        assert_eq!(tag.framing.tag, 2);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}
