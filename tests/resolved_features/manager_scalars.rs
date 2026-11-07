use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/manager-scalars/cases.bin"),
        b"MSC1",
        72,
    )
}

#[test]
fn complete_manager_scalars_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records(records());
}

#[test]
fn manager_scalars_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn scalar_material_bits_and_signed_pair_words_remain_verbatim() {
    let words = [i32::MIN, i32::MAX, -1, 0, 1, -32769];
    let bits = [
        0,
        0x8000000000000000,
        0x7ff0000000000042,
        0x7ff8000000001234,
        0x7ff0000000000000,
        0xfff0000000000000,
    ];
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let style = usize::from(kind % 6);
        let field = fields
            .iter()
            .find(|f| {
                f.path
                    .ends_with(if kind < 6 { ".raw_08" } else { ".raw_20" })
            })
            .unwrap();
        assert_eq!(field.value, super::ModelFieldValue::I32(words[style]));
        if kind < 6 {
            continue;
        }
        let field = fields.iter().find(|f| f.path.ends_with(".raw_10")).unwrap();
        assert_eq!(
            field.value,
            super::ModelFieldValue::F64(crate::archive_codec::StoredDouble::from_bits(bits[style]))
        );
    }
}

#[test]
fn material_string_counts_reject_impossible_extents_before_allocation() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed =
        super::manager_context_tests::replace_field(&fields, "root.object.name_08", &[255; 7]);
    assert!(decode_graph(version, &changed, false).is_err());
}

#[test]
fn complete_owned_manager_maps_match_independent_fields_and_reconstruction() {
    for (bytes, oracle, reads) in OWNED {
        let parsed = super::manager_context_tests::decode(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&parsed.fields, bytes, oracle, *reads);
        assert_eq!(super::encode_fields(&parsed.fields, 30000).unwrap(), *bytes);
    }
}

#[test]
fn owned_scalar_maps_reject_every_truncation_and_appended_bytes() {
    for (bytes, _, _) in OWNED {
        for end in 0..bytes.len() {
            assert!(super::manager_context_tests::decode(&bytes[..end]).is_err());
        }
        let mut appended = bytes.to_vec();
        appended.push(0);
        assert!(super::manager_context_tests::decode(&appended).is_err());
    }
}

#[test]
fn independent_manager_scalar_edits_preserve_other_map_entries_and_inputs() {
    for (original, edited, path) in [
        (2, 12, "map_640[0].pair.raw_08"),
        (8, 13, "map_5f8[0].material.raw_20"),
    ] {
        let parsed = super::manager_context_tests::decode(OWNED[original].0).unwrap();
        let changed = super::manager_context_tests::replace_field(
            &parsed.fields,
            &format!("root.manager.{path}"),
            &12345_i32.to_le_bytes(),
        );
        assert_eq!(changed, OWNED[edited].0);
        assert!(super::manager_context_tests::decode(&changed).is_ok());
        assert_eq!(
            super::encode_fields(&parsed.fields, 30000).unwrap(),
            OWNED[original].0
        );
    }
}

const OWNED: &[(&[u8], &str, usize)] = &[
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-0.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-0.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-1.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-1.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-2.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-2.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-3.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-3.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-4.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-4.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-5.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-5.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-0.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-0.tsv"),
        526,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-1.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-1.tsv"),
        528,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-2.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-2.tsv"),
        532,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-3.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-3.tsv"),
        530,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-4.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-4.tsv"),
        534,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-5.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-5.tsv"),
        532,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-0-2-edited.bin"),
        include_str!("../fixtures/manager-scalars/owned-0-2-edited.tsv"),
        522,
    ),
    (
        include_bytes!("../fixtures/manager-scalars/owned-1-2-edited.bin"),
        include_str!("../fixtures/manager-scalars/owned-1-2-edited.tsv"),
        532,
    ),
];
