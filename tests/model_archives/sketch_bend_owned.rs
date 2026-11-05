use super::{
    ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, SketchBendOwnedProfile,
    StoredDouble,
};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const EMPTY: &[u8] = include_bytes!("../fixtures/sketch-bend-owned/empty.bin");
const RICH: &[u8] = include_bytes!("../fixtures/sketch-bend-owned/rich.bin");
const ALIASES: &[u8] = include_bytes!("../fixtures/sketch-bend-owned/aliases.bin");
const WORDS: [i32; 6] = [i32::MIN, -1, 0, 1, 65535, i32::MAX];
const BITS: [u64; 16] = [
    0,
    0x8000000000000000,
    1,
    0x8000000000000001,
    0x7fefffffffffffff,
    0xffefffffffffffff,
    0x7ff0000000000000,
    0xfff0000000000000,
    0x7ff8000000001234,
    0xfff8000000004321,
    0x7ff0000000000001,
    0xfff0000000000001,
    0x0010000000000000,
    0x8010000000000000,
    0x3ff0000000000000,
    0xbff0000000000000,
];
const FLOATS: [&str; 8] = [
    "vector_b8[0]",
    "vector_b8[1]",
    "vector_b8[2]",
    "vector_d0[0]",
    "vector_d0[1]",
    "vector_d0[2]",
    "raw_e8",
    "raw_f0",
];

fn owned_reader(
    raw: &[u8],
    version: u32,
    limits: super::ModelFieldLimits,
) -> crate::Result<ModelDecoder<'_>> {
    Ok(ModelDecoder::new_in_context(
        raw,
        ArchiveVersion::new(version),
        limits,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_kernel(Some(super::kernel_frame_tests::context()))
    .with_sketch_bend_owned_profile(Some(SketchBendOwnedProfile::CanonicalRuledGrid)))
}

fn decode_owned(version: u32, raw: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut reader = owned_reader(raw, version, super::annotation_view_tests::LIMITS)?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

fn owned_field<'a>(fields: &'a [ModelField], suffix: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|field| field.path.ends_with(suffix))
        .unwrap()
}

fn owned_records() -> Vec<&'static [u8]> {
    let mut cursor =
        ArchiveCursor::new(include_bytes!("../fixtures/sketch-bend-owned/domains.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"SK71");
    assert_eq!(cursor.u32().unwrap(), 576);
    let records = (0..576)
        .map(|_| {
            let size = cursor.u32().unwrap() as usize;
            cursor.take(size).unwrap()
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

#[test]
fn canonical_kind_seven_graphs_reconstruct_the_complete_feature_and_required_frame() {
    for (raw, slots) in [(EMPTY, 5), (RICH, 23), (ALIASES, 31)] {
        let (fields, actual) = decode_owned(19000, raw).unwrap();
        assert_eq!(actual, slots);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        assert!(
            fields
                .iter()
                .any(|field| field.path.ends_with("body_bf0.frame"))
        );
    }
    let fields = decode_owned(19000, EMPTY).unwrap().0;
    let owned: Vec<_> = fields
        .iter()
        .filter(|field| field.path.contains(".owned_be8."))
        .cloned()
        .collect();
    assert_eq!(super::encode_fields(&owned, 76).unwrap().len(), 76);
}

#[test]
fn grid_and_face_words_and_all_double_categories_match_independent_literal_records() {
    for (case, raw) in owned_records().into_iter().enumerate() {
        let fields = decode_owned(19000, raw).unwrap().0;
        for (offset, suffix) in FLOATS.iter().enumerate() {
            assert_eq!(
                owned_field(&fields, &format!("owned_be8.grid_58.{suffix}")).value,
                ModelFieldValue::F64(StoredDouble::from_bits(BITS[(case / 36 + offset) % 16]))
            );
        }
        assert_eq!(
            owned_field(&fields, "grid_58.raw_b0").value,
            ModelFieldValue::I32(WORDS[(case / 6) % 6])
        );
        assert_eq!(
            owned_field(&fields, "owned_be8.raw_138").value,
            ModelFieldValue::I32(WORDS[case % 6])
        );
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    }
}

#[test]
fn default_replay_still_rejects_kind_seven_before_owned_data() {
    let error = super::solid_reference_tests::decode_graph(19000, RICH, true).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("profile None"));
    assert!(error.cause.contains("CanonicalRuledGrid"));
}

#[test]
fn canonical_profile_requires_schema_one_and_explicit_kernel_context() {
    for schema in [0u16, 2, 7, u16::MAX] {
        let mut raw = EMPTY.to_vec();
        raw[2..4].copy_from_slice(&schema.to_le_bytes());
        let error = decode_owned(19000, &raw).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains(&schema.to_string()));
    }
    let mut reader = owned_reader(EMPTY, 19000, super::annotation_view_tests::LIMITS)
        .unwrap()
        .with_kernel(None);
    let error = reader
        .object("object", &mut ObjectReader::new(), ModelObjectRole::Any)
        .unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Missing);
}

#[test]
fn modern_profile_boundaries_and_all_proper_feature_prefixes_are_checked() {
    let minimum = include_bytes!("../fixtures/sketch-bend-owned/minimum-rich.bin");
    let fields = decode_owned(0x27cc, minimum).unwrap().0;
    assert_eq!(
        super::encode_fields(&fields, minimum.len()).unwrap(),
        minimum
    );
    assert!(decode_owned(0x27cb, minimum).is_err());
    for raw in [EMPTY, RICH, ALIASES] {
        for end in 0..raw.len() {
            assert!(decode_owned(19000, &raw[..end]).is_err(), "prefix {end}");
        }
        assert!(decode_owned(19000, &[raw, &[0]].concat()).is_err());
    }
}

#[test]
fn required_raw_body_has_no_suppression_byte_and_rejects_other_kernel_types() {
    let fields = decode_owned(19000, EMPTY).unwrap().0;
    let frame = owned_field(&fields, "body_bf0.frame").clone();
    let bytes = super::encode_fields(&[frame], EMPTY.len()).unwrap();
    assert_eq!(&EMPTY[EMPTY.len() - bytes.len()..], bytes);
    let offset = EMPTY.len() - bytes.len();
    let mut unsupported = EMPTY.to_vec();
    unsupported[offset + 4] = 1;
    assert_eq!(
        decode_owned(19000, &unsupported).unwrap_err().class,
        crate::FailureClass::Unsupported
    );
    let mut shifted = EMPTY[..offset].to_vec();
    shifted.push(0);
    shifted.extend_from_slice(&bytes);
    assert!(decode_owned(19000, &shifted).is_err());
}

#[test]
fn generic_grid_arrays_retain_nulls_back_references_and_distinct_face_scalar() {
    let fields = decode_owned(19000, RICH).unwrap().0;
    let owner = owned_field(&fields, "grid_58.array_118[0]").clone();
    assert_eq!(super::encode_fields(&[owner], 2).unwrap(), [2, 0]);
    let null = owned_field(&fields, "grid_58.array_118[2]").clone();
    assert_eq!(super::encode_fields(&[null], 2).unwrap(), [0, 0]);
    assert_eq!(
        owned_field(&fields, "grid_58.array_138.count.count_dword").value,
        ModelFieldValue::U32(2)
    );
    assert_eq!(
        owned_field(&fields, "owned_be8.raw_138").value,
        ModelFieldValue::I32(i32::MAX)
    );
}

#[test]
fn canonical_profile_limits_bound_bytes_fields_entries_depth_and_objects() {
    let limits = super::annotation_view_tests::LIMITS;
    let mut bounded = [limits; 5];
    bounded[0].maximum_bytes = RICH.len() - 1;
    bounded[1].maximum_fields = 3;
    bounded[2].maximum_entries = 0;
    bounded[3].maximum_depth = 0;
    bounded[4].maximum_objects = 0;
    for limits in bounded {
        let result = owned_reader(RICH, 19000, limits).and_then(|mut reader| {
            reader.object("object", &mut ObjectReader::new(), ModelObjectRole::Any)?;
            reader.finish()
        });
        assert!(result.is_err());
    }
}

#[test]
fn strict_frame_expansion_is_charged_across_distinct_kind_seven_features() {
    let fields = decode_owned(19000, EMPTY).unwrap().0;
    let mut kernel = super::kernel_frame_tests::context();
    kernel.maximum_expanded = owned_expanded_bytes(&fields);
    let mut reader = owned_reader(ALIASES, 19000, super::annotation_view_tests::LIMITS)
        .unwrap()
        .with_kernel(Some(kernel));
    assert!(
        reader
            .object("object", &mut ObjectReader::new(), ModelObjectRole::Any)
            .is_err()
    );
    assert_eq!(
        fields
            .iter()
            .filter(|field| matches!(field.value, ModelFieldValue::Kernel(_)))
            .count(),
        1
    );
}

fn owned_expanded_bytes(fields: &[ModelField]) -> usize {
    match &owned_field(fields, "body_bf0.frame").value {
        ModelFieldValue::Kernel(frame) => frame.expanded_bytes(),
        _ => panic!("BF0 field; expected a complete kernel frame"),
    }
}

#[test]
fn direct_grid_array_counts_are_bounded_before_consuming_any_item() {
    let fields = decode_owned(19000, RICH).unwrap().0;
    for offset in ["118", "138"] {
        let path = format!("root.object.owned_be8.grid_58.array_{offset}.count.count_dword");
        let raw =
            super::manager_context_tests::replace_field(&fields, &path, &u32::MAX.to_le_bytes());
        assert_eq!(
            decode_owned(19000, &raw).unwrap_err().class,
            crate::FailureClass::Invalid
        );
    }
}
