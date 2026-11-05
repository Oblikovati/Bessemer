use super::property_domains_3086_tests::{assert_property_splice, assert_signed_property_domain};
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{TrimEdgeRecord, records_with_count};
use super::{ModelField, ModelFieldValue};

fn reference_curve_cases() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/reference-curves/cases.bin"),
        b"RCV1",
        60,
    )
}

#[test]
fn reference_curve_blank_flag_preserves_every_unsigned16_value_and_following_fields() {
    let (_, version, raw, _, _) = reference_curve_cases()
        .into_iter()
        .find(|row| row.0 == 0 && row.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, raw, true).unwrap().0;
    let path = "root.object.payload.raw_858";
    for value in 0..=u16::MAX {
        let expected = ModelFieldValue::U16(value);
        assert_property_splice(version, &fields, path, &value.to_le_bytes(), expected);
    }
    eprintln!("REFCURVE_BLANKFLAG_PATCHES\t65536");
}

fn projection_flag_domain(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    let ModelFieldValue::I32(original) = field.value else {
        panic!("{}: expected signed32 projection flags", field.path);
    };
    let mut patterns = vec![0, u32::MAX, 1, 2, 0x12345678, 0x80000000];
    for bit in 0..32 {
        patterns.push(1u32 << bit);
        patterns.push(!(1u32 << bit));
    }
    for pattern in &patterns {
        let value = ((pattern & !2) | (original as u32 & 2)) as i32;
        let expected = ModelFieldValue::I32(value);
        assert_property_splice(version, fields, &field.path, &value.to_le_bytes(), expected);
    }
    patterns.len()
}

fn curve_property_patches(version: u32, fields: &[ModelField], field: &ModelField) -> usize {
    if field.path.ends_with(".payload.raw_878") {
        return projection_flag_domain(version, fields, field);
    }
    if field.path.ends_with(".payload.raw_930") || field.path.ends_with(".payload.raw_934") {
        assert_signed_property_domain(version, fields, &field.path);
        return 6;
    }
    0
}

#[test]
fn reference_curve_direction_bidir_and_projection_flags_keep_unassigned_bits() {
    let mut patches = 0;
    for (_, version, raw, _, _) in reference_curve_cases() {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in &fields {
            patches += curve_property_patches(version, &fields, field);
        }
    }
    assert!(patches > 0);
    eprintln!("REFCURVE_NAMED_FLAG_PATCHES\t{patches}");
}

fn move_copy_setting(field: &&ModelField) -> bool {
    let path = field.path.as_ref();
    path.ends_with(".raw_4c0") || path.ends_with(".raw_4a4")
}

#[test]
fn move_copy_absolute_transform_and_euler_words_preserve_full_signed_domains() {
    let packet = include_bytes!("../fixtures/move-copy-bodies/cases.bin");
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for (_, version, raw, _, _) in records_with_count(packet, b"MCB1", 162) {
        let fields = decode_graph(version, raw, true).unwrap().0;
        for field in fields.iter().filter(move_copy_setting) {
            assert_signed_property_domain(version, &fields, &field.path);
            *counts
                .entry(field.path.rsplit('.').next().unwrap().to_owned())
                .or_insert(0) += 6;
        }
    }
    assert_eq!(counts.len(), 2);
    eprintln!("MOVE_COPY_NAMED_WORD_PATCHES\t{counts:?}");
}
