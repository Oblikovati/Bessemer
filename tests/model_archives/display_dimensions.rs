use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/display-dimension-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/display-dimension-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/display-dimension-edited.bin");

fn read_mode(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, mode: u8) -> Result<()> {
    match mode {
        0 => reader.object("object", map, ModelObjectRole::Any),
        1 => super::display_placement::parameters(reader),
        2 => super::display_item::read(reader, map),
        value => Err(super::invalid(format!(
            "display fixture mode {value}; expected 0..=2"
        ))),
    }
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| read_mode(reader, map, mode))
}

#[test]
fn complete_display_dimensions_match_all_independent_fields_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (336, 54251));
}

#[test]
fn display_dimension_prefixes_and_trailing_bytes_cannot_publish_partial_objects() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(mode, version, &bytes[..length]).is_err(),
                "mode {mode}, prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(mode, version, &trailing).is_err());
    }
}

#[test]
fn complete_display_edit_matches_independent_bytes_and_preserves_original() {
    let original = decode(0, 19000, ORIGINAL).unwrap();
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.dimension.word_8e0")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn display_dimension_objects_own_all_fields_after_source_overwrite() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn parallel_display_writes_preserve_self_references_and_shared_handles() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), ORIGINAL);
        }
    });
}

fn changed_value(path: &str, value: ModelFieldValue) -> Vec<u8> {
    let mut fields = decode(0, 19000, ORIGINAL).unwrap();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = value;
    encode_fields(&fields, 30000).unwrap()
}

#[test]
fn unsupported_display_payloads_and_perspective_never_publish_prefixes() {
    for path in ["has_link_data", "item.has_object_288", "item.has_watermark"] {
        let bytes = changed_value(&format!("root.object.{path}"), ModelFieldValue::I32(-1));
        let error = decode(0, 19000, &bytes).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("-1"));
    }
    let bytes = changed_value(
        "root.object.dimension.transform_5c0.words[28]",
        ModelFieldValue::U32(0x10),
    );
    assert_eq!(
        decode(0, 19000, &bytes).unwrap_err().class,
        crate::FailureClass::Unsupported
    );
}

#[test]
fn display_count_bounds_and_typed_parameter_widths_reject_invalid_shapes() {
    for (member, count) in [
        ("item.points_4f0.count", 5),
        ("item.vectors_550.count", 3),
        ("suffix.callout_strings.count", 5),
    ] {
        let bytes = changed_value(
            &format!("root.object.{member}"),
            ModelFieldValue::I32(count),
        );
        assert_eq!(
            decode(0, 19000, &bytes).unwrap_err().class,
            crate::FailureClass::Invalid
        );
    }
    for width in [1u32, 4, 16, u32::MAX] {
        let error = decode(1, 19000, &width.to_le_bytes()).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains(&width.to_string()));
    }
    assert!(decode(255, 19000, &[]).unwrap_err().cause.contains("255"));
}

#[test]
fn nonfinite_values_and_serialized_transform_words_are_not_normalized() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    let value = &fields
        .iter()
        .find(|field| field.path.as_ref() == "root.object.dimension.unknown_8a8")
        .unwrap()
        .value;
    assert!(matches!(value, ModelFieldValue::F64(value) if value.bits() == 0x7ff000000000002a));
    let count = fields
        .iter()
        .filter(|field| {
            field
                .path
                .starts_with("root.object.dimension.transform_5c0.words[")
        })
        .count();
    assert_eq!(count, 30);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), ORIGINAL);
}

#[test]
fn display_dimension_handles_reject_null_and_unrelated_new_objects() {
    let fields = decode(0, 19000, ORIGINAL).unwrap();
    let index = fields
        .iter()
        .position(|field| field.path.as_ref() == "root.object.dimension.handles[0]")
        .unwrap();
    for tag in [vec![0, 0], unrelated_tag()] {
        let bytes = [encode_fields(&fields[..index], 30000).unwrap(), tag].concat();
        assert_eq!(
            decode(0, 19000, &bytes).unwrap_err().class,
            crate::FailureClass::Invalid
        );
    }
}

fn unrelated_tag() -> Vec<u8> {
    let name = b"moNodeName_c";
    [
        65535u16.to_le_bytes().as_slice(),
        &1u16.to_le_bytes(),
        &(name.len() as u16).to_le_bytes(),
        name,
    ]
    .concat()
}

#[test]
fn display_kind_boundaries_are_finite_and_result_fields_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<super::display_dimension::DisplayDimensionKind>();
    assert_send_sync::<ModelField>();
    for name in super::display_dimension::CLASSES {
        assert!(super::display_dimension::DisplayDimensionKind::parse(name).is_some());
    }
    for name in [
        "",
        "moDisplayUnknownDim_c",
        "moDisplayDim_c ",
        "modisplaydim_c",
    ] {
        assert!(super::display_dimension::DisplayDimensionKind::parse(name).is_none());
    }
}

#[test]
fn display_dynamic_collection_lengths_reject_negative_and_impossible_counts() {
    const SIGNED_COUNTS: &[&str] = &[
        "item.indices_2b0.count",
        "item.leaders.count",
        "item.arrow_sizes.count",
        "item.spline_leaders.count",
        "item.points_4f0.count",
        "item.vectors_550.count",
        "dimension.handles.count",
        "suffix.callout_strings.count",
    ];
    for member in SIGNED_COUNTS {
        for value in [i32::MIN, i32::MAX] {
            let bytes = changed_value(
                &format!("root.object.{member}"),
                ModelFieldValue::I32(value),
            );
            assert!(decode(0, 19000, &bytes).is_err(), "{member} {value}");
        }
    }
    let bytes = changed_value(
        "root.object.dimension.parameters_5b0.count",
        ModelFieldValue::U32(u32::MAX),
    );
    assert!(decode(0, 19000, &bytes).is_err());
}

#[test]
fn display_parameter_values_have_an_independently_counted_entry_budget() {
    use super::annotation_view_tests::LIMITS;
    use super::direct_collection_tests::decode_limited;
    let bytes: Vec<u8> = [8u32, 0, 0, 3, 0, 0, 1, 0, 2, 0]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    for maximum_entries in [2, 3] {
        let limits = super::ModelFieldLimits {
            maximum_entries,
            ..LIMITS
        };
        let result = decode_limited(19000, &bytes, limits, |reader, _| {
            super::display_placement::parameters(reader)
        });
        assert_eq!(result.is_ok(), maximum_entries == 3);
    }
}
