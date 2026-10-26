use super::annotation_view_tests::LIMITS;
use super::direct_collection_tests::decode_kernel;
use super::kernel_frames::CompressedKernelContext;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::parasolid::{FieldValues, TransmitBody};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/profile-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/profile-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/profile-edited.bin");

fn read_mode(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, mode: u8) -> crate::Result<()> {
    match mode {
        0 => reader.object("object", map, ModelObjectRole::Any),
        1 => super::profile_geometry::geometry(reader, map),
        2 => super::profile_geometry::reference_array(reader),
        value => Err(super::invalid(format!(
            "profile fixture mode {value}; expected 0..=2"
        ))),
    }
}

fn decode(
    mode: u8,
    version: u32,
    bytes: &[u8],
    context: &CompressedKernelContext,
) -> crate::Result<Vec<ModelField>> {
    decode_kernel(
        version,
        bytes,
        LIMITS,
        Some(context.clone()),
        |reader, map| read_mode(reader, map, mode),
    )
}

#[test]
fn complete_profiles_contours_chains_and_word_groups_match_independent_values() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, &context).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (180, 11072));
}

#[test]
fn profile_prefixes_and_trailing_bytes_never_publish_partial_results() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(mode, version, &bytes[..length], &context).is_err(),
                "mode {mode}, prefix {length}"
            );
        }
        assert!(decode(mode, version, &[bytes, &[0]].concat(), &context).is_err());
    }
}

#[test]
fn profile_edit_matches_independent_bytes_and_preserves_original() {
    let context = super::kernel_frame_tests::context();
    let original = decode(0, 19000, ORIGINAL, &context).unwrap();
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.suffix.word_424")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED, &context).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn profile_graphs_and_framed_bodies_own_their_source() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source, &context).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn profile_parallel_writes_preserve_kernel_frames_and_cyclic_references() {
    let fields = decode(0, 19000, ORIGINAL, &super::kernel_frame_tests::context()).unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), ORIGINAL);
        }
    });
}

#[test]
fn primary_and_extra_profile_kernel_bodies_have_independent_typed_values() {
    let fields = decode(0, 19000, ORIGINAL, &super::kernel_frame_tests::context()).unwrap();
    let frames: Vec<_> = fields
        .iter()
        .filter_map(|field| match &field.value {
            ModelFieldValue::Kernel(frame) => Some(frame),
            _ => None,
        })
        .collect();
    assert_eq!(frames.len(), 2);
    for (frame, identifier) in frames.into_iter().zip([-7, -11]) {
        let TransmitBody::Partition(list) = &frame.contents().body else {
            panic!("expected partition");
        };
        assert_eq!(list.records.len(), 1);
        assert_eq!(
            list.records[0].fields,
            vec![FieldValues::Integers(vec![identifier])]
        );
    }
}

#[test]
fn framed_profile_context_is_required_while_suppression_needs_no_context() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut framed, mut suppressed) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, &context).unwrap();
        let result = decode_kernel(version, bytes, LIMITS, None, |reader, map| {
            read_mode(reader, map, mode)
        });
        if fields
            .iter()
            .any(|field| matches!(field.value, ModelFieldValue::Kernel(_)))
        {
            assert_eq!(result.unwrap_err().class, crate::FailureClass::Missing);
            framed += 1;
        } else if fields
            .iter()
            .any(|field| field.path.ends_with("body.suppressed"))
        {
            assert!(result.is_ok());
            suppressed += 1;
        }
    }
    assert_eq!((framed, suppressed), (48, 24));
}

#[test]
fn reference_array_alignment_and_extent_are_checked_before_word_iteration() {
    let context = super::kernel_frame_tests::context();
    for size in [1u32, 2, 3, 5, u32::MAX] {
        let error = decode(2, 19000, &size.to_le_bytes(), &context).unwrap_err();
        assert!(error.cause.contains(&size.to_string()));
    }
    let bytes: Vec<u8> = [8u32, 0, 0, u32::MAX]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert!(decode(2, 19000, &bytes, &context).is_err());
    assert!(
        decode(255, 19000, &[], &context)
            .unwrap_err()
            .cause
            .contains("255")
    );
}

#[test]
fn reused_profiles_require_a_nonnull_profile_target() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, &context).unwrap();
        let Some(index) = fields
            .iter()
            .position(|field| field.path.ends_with("source_profile"))
        else {
            continue;
        };
        let changed = [encode_fields(&fields[..index], 30000).unwrap(), vec![0, 0]].concat();
        assert_eq!(
            decode(mode, version, &changed, &context).unwrap_err().class,
            crate::FailureClass::Invalid
        );
        checked += 1;
    }
    assert_eq!(checked, 16);
}

#[test]
fn reference_rows_and_words_have_an_independently_counted_budget() {
    let bytes: Vec<u8> = [8u32, 0, 0, 2, 0, 1, 2, 3]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    for maximum_entries in [5, 6] {
        let limits = super::ModelFieldLimits {
            maximum_entries,
            ..LIMITS
        };
        let result = decode_kernel(19000, &bytes, limits, None, |reader, _| {
            super::profile_geometry::reference_array(reader)
        });
        assert_eq!(result.is_ok(), maximum_entries == 6);
    }
}

#[test]
fn profile_dynamic_counts_reject_negative_and_impossible_extents() {
    let context = super::kernel_frame_tests::context();
    let original = decode(0, 19000, ORIGINAL, &context).unwrap();
    for (member, value) in [
        ("chains.items.count", ModelFieldValue::I32(-1)),
        ("geometry.pairs.count", ModelFieldValue::I32(i32::MAX)),
        (
            "geometry.reference_data.count",
            ModelFieldValue::U32(u32::MAX),
        ),
        (
            "geometry.extra_bodies.count",
            ModelFieldValue::U32(u32::MAX),
        ),
    ] {
        let mut fields = original.clone();
        let path = format!("root.object.{member}");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = value;
        assert!(
            decode(0, 19000, &encode_fields(&fields, 30000).unwrap(), &context).is_err(),
            "{member}"
        );
    }
}

#[test]
fn chain_and_chooser_masks_reject_undocumented_bits() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let original = decode(mode, version, bytes, &context).unwrap();
        for (index, field) in original
            .iter()
            .enumerate()
            .filter(|(_, field)| is_profile_mask(field, &original))
        {
            let mut fields = original.clone();
            fields[index].value = ModelFieldValue::U16(4);
            assert_eq!(
                decode(
                    mode,
                    version,
                    &encode_fields(&fields, 30000).unwrap(),
                    &context
                )
                .unwrap_err()
                .class,
                crate::FailureClass::Invalid,
                "{}",
                field.path
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 112);
}

fn is_profile_mask(field: &ModelField, fields: &[ModelField]) -> bool {
    let Some(parent) = field
        .path
        .strip_suffix(".mask")
        .or_else(|| field.path.strip_suffix(".array_mask"))
    else {
        return false;
    };
    let Some(tag) = fields.iter().find(|field| field.path.as_ref() == parent) else {
        return false;
    };
    matches!(&tag.value,ModelFieldValue::ObjectTag(tag) if matches!(&tag.record,
        crate::archive_codec::ObjectRecord::New{class,..} if matches!(class.name.as_str(),"moSketchChain_c"|"moContourChooser_c")))
}

#[test]
fn reused_profile_and_contour_roles_still_reject_unrelated_classes() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, &context).unwrap();
        let Some(index) = fields.iter().position(|field| {
            field.path.ends_with("source_profile") || field.path.ends_with("contour_chooser")
        }) else {
            continue;
        };
        let bytes = [
            encode_fields(&fields[..index], 30000).unwrap(),
            b"\xff\xff\x01\x00\x0c\x00moNodeName_c".to_vec(),
        ]
        .concat();
        let error = decode(mode, version, &bytes, &context).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moNodeName_c"));
        checked += 1;
    }
    assert_eq!(checked, 64);
}
