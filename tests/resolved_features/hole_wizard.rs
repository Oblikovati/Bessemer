use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const PACKET: &[u8] = include_bytes!("../fixtures/hole-wizard/cases.bin");
type HoleRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<HoleRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"HWF1");
    assert_eq!(cursor.u32().unwrap(), 74);
    let slots: Vec<_> = (0..74).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

fn decode_graph(version: u32, bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut slots = 0;
    let fields = decode_collection(version, bytes, |reader, map| {
        reader.object("object", map, ModelObjectRole::Any)?;
        slots = std::mem::replace(map, ObjectReader::new())
            .into_snapshot()
            .slot_count();
        Ok(())
    })?;
    Ok((fields, slots))
}

fn current_slot() -> Vec<ModelField> {
    records()
        .into_iter()
        .filter(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .map(|(_, version, bytes, _, _)| decode(version, bytes).unwrap())
        .find(|fields| {
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.object.payload.dimension_a68")
        })
        .unwrap()
}

#[test]
fn complete_holes_and_tagged_axes_match_independent_fields_bytes_and_map_slots() {
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn holes_and_axes_reject_every_truncated_prefix_trailing_byte_and_old_context() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn pair_and_slot_counts_reject_negative_and_unbounded_values() {
    let fields = current_slot();
    for label in ["pairs_958.count", "slot_axes_bd8.count"] {
        for count in [-1i32, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("root.object.payload.{label}"),
                &count.to_le_bytes(),
            );
            let error = decode(19000, &changed).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("count"), "{}", error.cause);
        }
    }
}

#[test]
fn slot_dimensions_reject_the_cached_enclosing_feature_as_an_unrelated_class() {
    let fields = current_slot();
    for label in ["dimension_a68", "dimension_a80"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(19000, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moHoleWzd_c"));
    }
}

#[test]
fn compatibility_word_slot_count_and_relocated_word_follow_independent_gates() {
    for (kind, version, bytes, _, _) in records() {
        if kind != 0 {
            continue;
        }
        let fields = decode(version, bytes).unwrap();
        let has = |suffix: &str| fields.iter().any(|field| field.path.ends_with(suffix));
        let enabled = has("payload.dimension_a68");
        assert_eq!(has("raw_base_compat"), (0x2f46..0x2f68).contains(&version));
        assert_eq!(has("slot_axes_bd8.count"), enabled && version >= 0x2b5e);
        assert_eq!(has("raw_83c"), enabled || version >= 0x42ea);
        assert_eq!(has("raw_bf8"), version > 0x3f02);
    }
}

#[test]
fn unknown_parameter_bits_survive_native_normalization_conditions() {
    let fields = current_slot();
    for (suffix, expected) in [
        ("raw_848", ModelFieldValue::I32(5)),
        ("raw_910", ModelFieldValue::I32(99)),
        ("raw_a40", ModelFieldValue::I32(-3)),
        ("raw_bf8", ModelFieldValue::U8(255)),
    ] {
        assert_eq!(
            fields
                .iter()
                .find(|field| field.path.ends_with(suffix))
                .unwrap()
                .value,
            expected
        );
    }
    let nan = fields
        .iter()
        .find(|field| field.path.ends_with("raw_a48"))
        .unwrap();
    assert!(matches!(nan.value, ModelFieldValue::F64(value) if value.bits() == 0x7ff8000000000042));
}
