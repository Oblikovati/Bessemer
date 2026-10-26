use super::annotation_view_tests::{LIMITS, decode};
use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/extruded-surfaces/cases.bin");
type SurfaceRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<SurfaceRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"ESF1");
    assert_eq!(cursor.u32().unwrap(), 62);
    let slots: Vec<_> = (0..62).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn populated_helper() -> Vec<ModelField> {
    records()
        .into_iter()
        .filter(|(kind, version, _, _, _)| *kind == 1 && *version == 19000)
        .map(|(_, version, bytes, _, _)| decode(version, bytes).unwrap())
        .find(|fields| {
            fields
                .iter()
                .any(|field| field.path.ends_with("raw_words_48[1]"))
        })
        .unwrap()
}

#[test]
fn complete_surfaces_helpers_and_keepers_match_independent_fields_bytes_and_map_slots() {
    let mut primitives = 0;
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        let count = oracle.lines().count();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, count);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        primitives += count;
    }
    assert_eq!(primitives, 13948);
}

#[test]
fn surfaces_helpers_and_keepers_reject_all_truncated_prefixes_trailing_bytes_and_old_contexts() {
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
fn required_end_and_optional_ordered_pointers_reject_null_or_unrelated_objects() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|(kind, _, _, _, _)| *kind == 0)
        .unwrap();
    let fields = decode(version, bytes).unwrap();
    let null = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.end_spec_800",
        &[0, 0],
    );
    let error = decode(version, &null).unwrap_err();
    assert!(error.cause.contains("nonnull"));
    for label in ["end_spec_800", "ordered_faces_808"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(version, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moExtruRefSurface_c"));
    }
}

#[test]
fn ordered_group_counts_reject_negative_and_unbounded_values() {
    let fields = populated_helper();
    for label in ["raw_words_48.count", "raw_words_a8.count"] {
        for count in [-1i32, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("root.object.{label}"),
                &count.to_le_bytes(),
            );
            let error = decode(19000, &changed).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("count"), "{}", error.cause);
        }
    }
}

#[test]
fn ordered_words_preserve_unsigned_bits_and_precede_their_direct_arrays() {
    let fields = populated_helper();
    let index = |suffix: &str| {
        fields
            .iter()
            .position(|field| field.path.ends_with(suffix))
            .unwrap()
    };
    assert_eq!(
        fields[index("raw_words_48[0]")].value,
        ModelFieldValue::U32(u32::MAX)
    );
    assert_eq!(
        fields[index("raw_words_48[1]")].value,
        ModelFieldValue::U32(0x80000000)
    );
    assert_eq!(
        fields[index("raw_words_a8[0]")].value,
        ModelFieldValue::U32(0xffffff85)
    );
    assert!(index("raw_words_48[1]") < index("arrays_28[0].items.count.count_word"));
    assert!(index("raw_words_a8[0]") < index("arrays_88[0].items.count.count_word"));
}

#[test]
fn binary_empty_keeper_has_only_tag_slots_and_requires_an_explicit_ordinary_context() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|(kind, _, _, _, _)| *kind == 2)
    {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!((fields.len(), slots), (1, 3));
        let mut reader = ModelDecoder::new(bytes, ArchiveVersion::new(version), LIMITS).unwrap();
        let error = reader
            .object("keeper", &mut ObjectReader::new(), ModelObjectRole::Any)
            .unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn observed_inherited_keeper_role_accepts_only_the_complete_declared_family() {
    let fields = records()
        .into_iter()
        .filter(|(kind, _, _, _, _)| *kind == 0)
        .map(|(_, version, bytes, _, _)| (version, decode(version, bytes).unwrap()))
        .find(|(_, fields)| {
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.object.body.id_keeper")
        })
        .unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields.1,
        "root.object.body.id_keeper",
        &2u16.to_le_bytes(),
    );
    let error = decode(fields.0, &changed).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moExtruRefSurface_c"));
    assert!(error.cause.contains("moExtruRefSurfIdKeeper_c"));
}
