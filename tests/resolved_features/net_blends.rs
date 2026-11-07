use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

const CONTACTS: [usize; 6] = [0, 0, 2, 1, 3, 2];
const SECOND: [i32; 6] = [0, 2, i32::MIN, 3, -1, 1];
const THIN: [u8; 6] = [0, 0, 1, 2, 128, 255];
const MODES: [i32; 6] = [3, 3, 2, 3, 3, i32::MIN];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/net-blends/cases.bin"),
        b"NBL1",
        570,
    )
}

#[test]
fn net_blends_and_complete_dependencies_match_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn net_blends_reject_all_truncated_prefixes_extra_bytes_and_old_contexts() {
    assert_rejected_boundaries(records());
}

#[test]
fn empty_direct_net_blend_data_has_native_degree_gate_and_no_tagged_slots() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 < 12 && r.0 % 6 == 0)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let direct: Vec<_> = fields
            .into_iter()
            .filter(|f| f.path.starts_with("root.object.data."))
            .collect();
        let expected = if version > 0x2f4b { 206 } else { 194 };
        assert_eq!(super::encode_fields(&direct, 1000).unwrap().len(), expected);
        assert_eq!(slots, 5);
    }
}

#[test]
fn net_component_minima_keep_full_u32_counts_and_direct_owned_implementation() {
    for (kind, version, bytes, _, slots) in
        records().into_iter().filter(|r| r.0 >= 12 && r.0 % 6 == 0)
    {
        let component = 66
            + 4 * usize::from(version > 0x2b60)
            + 4 * usize::from(version > 0x2bb8)
            + 4 * usize::from(version > 0x2f55)
            + 4 * usize::from(version >= 0x3ef6);
        let expected = match kind / 6 {
            2 => 6,
            3 => component + 8,
            _ => component + 84,
        };
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(bytes.len() - 6 - name_length, expected);
        assert_eq!(slots, 3);
    }
}

fn point_field_count(fields: &[super::ModelField], fragment: &str) -> usize {
    fields
        .iter()
        .filter(|field| field.path.contains(fragment))
        .count()
}

#[test]
fn independent_and_negative_second_point_counts_remain_preserved() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 12 && r.1 == 19000) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let style = usize::from(kind % 6);
        let copies = if [2, 4, 5].contains(&style) { 2 } else { 1 };
        assert_eq!(
            point_field_count(&fields, ".points_48["),
            CONTACTS[style] * 9 * copies
        );
        assert_eq!(
            point_field_count(&fields, ".points_50["),
            SECOND[style].max(0) as usize * 9 * copies
        );
        for field in fields
            .iter()
            .filter(|f| f.path.ends_with(".points_50_count"))
        {
            assert_eq!(field.value, ModelFieldValue::I32(SECOND[style]));
        }
    }
}

#[test]
fn net_blend_thin_byte_and_mode_select_only_native_dimension_pointers() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let style = usize::from(kind % 6);
        let present = |suffix: &str| fields.iter().any(|f| f.path.ends_with(suffix));
        assert_eq!(present(".dimension_10f8"), THIN[style] != 0);
        assert_eq!(
            present(".dimension_1100"),
            THIN[style] != 0 && MODES[style] == 3
        );
        for field in fields
            .iter()
            .filter(|f| f.path.ends_with(".want_thin_solid_18"))
        {
            assert_eq!(field.value, ModelFieldValue::U8(THIN[style]));
        }
    }
}

#[test]
fn surface_degree_and_migrated_curve_options_keep_full_stored_values() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let flag = fields
            .iter()
            .find(|f| f.path.ends_with(".surface_degree_8d8.raw_flag_10"));
        assert_eq!(flag.is_some(), version > 0x2f4b);
        if let Some(flag) = flag {
            assert_eq!(flag.value, ModelFieldValue::I32(0x1234_5678));
        }
        if kind % 6 == 2 {
            let option = fields
                .iter()
                .find(|f| f.path.ends_with(".curves_50.data_238.raw_0c"))
                .unwrap();
            assert_eq!(option.value, ModelFieldValue::U8(128));
        }
    }
}

fn assert_invalid_replacement(version: u32, bytes: &[u8], path: &str, replacement: &[u8]) {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    assert!(
        fields.iter().any(|field| field.path.as_ref() == path),
        "missing test field {path}"
    );
    let changed = super::manager_context_tests::replace_field(&fields, path, replacement);
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected malformed count or cached array role");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn net_blend_counts_reject_impossible_packed_signed_and_full_u32_extents() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.1 == 19000 && [0, 24].contains(&r.0))
    {
        if kind == 24 {
            assert_invalid_replacement(
                version,
                bytes,
                "root.object.edges_08.count",
                &u32::MAX.to_le_bytes(),
            );
            continue;
        }
        assert_invalid_replacement(
            version,
            bytes,
            "root.object.data.curves_50.contacts_08.count.count_word",
            &[255; 6],
        );
        assert_invalid_replacement(
            version,
            bytes,
            "root.object.data.curves_50.points_50_count",
            &i32::MAX.to_le_bytes(),
        );
    }
}

#[test]
fn net_blend_dimensions_edges_and_wrapper_components_reject_cached_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| [2, 22, 26].contains(&r.0))
    {
        let path = match kind {
            2 => "root.object.items[0].dimension_10f8",
            22 => "root.object.items[0].component_reference",
            _ => "root.object.items[0].edges_08[0]",
        };
        assert_invalid_replacement(version, bytes, path, &2u16.to_le_bytes());
    }
}
