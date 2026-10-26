use super::annotation_view_tests::LIMITS;
use super::direct_collection_tests::decode_kernel;
use super::lightweight_section_tests::next_case;
use super::{
    ModelDecoder, ModelField, ModelFieldValue, encode_fields, sketch_splines, spline_arrays,
};
use crate::archive_codec::ObjectReader;
use crate::{Result, archive_codec::ArchiveCursor};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-spline-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/sketch-spline-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/sketch-spline-edited.bin");
const BAD_ARRAY_COUNTS: &[(&str, ModelFieldValue)] = &[
    ("parameter_count", ModelFieldValue::I32(-1)),
    ("control_point_count", ModelFieldValue::I32(i32::MAX)),
    ("tangent_count", ModelFieldValue::U32(u32::MAX)),
    ("interpolation.count", ModelFieldValue::U32(u32::MAX - 1)),
    ("interpolation.width", ModelFieldValue::I32(-1)),
    (
        "secondary_interpolation.count",
        ModelFieldValue::U32(u32::MAX),
    ),
    ("fixed_length_count", ModelFieldValue::U32(u32::MAX)),
];

fn read_mode(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, mode: u8) -> Result<()> {
    match mode {
        0 => sketch_splines::read(reader, map),
        1 => spline_arrays::interpolation(reader),
        2 => spline_arrays::secondary(reader),
        3 => spline_arrays::curve(reader),
        value => Err(super::invalid(format!(
            "spline fixture mode {value}; expected 0..=3"
        ))),
    }
}

fn decode(mode: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_context(mode, version, bytes, &super::kernel_frame_tests::context())
}

fn decode_context(
    mode: u8,
    version: u32,
    bytes: &[u8],
    context: &super::kernel_frames::CompressedKernelContext,
) -> Result<Vec<ModelField>> {
    decode_kernel(
        version,
        bytes,
        LIMITS,
        Some(context.clone()),
        |reader, map| read_mode(reader, map, mode),
    )
}

#[test]
fn complete_direct_splines_and_arrays_match_independent_values_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(mode, version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (934, 80434));
}

#[test]
fn spline_prefixes_and_trailing_bytes_reject_incomplete_results() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode_context(mode, version, &bytes[..length], &context).is_err(),
                "mode {mode}, prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode_context(mode, version, &trailing, &context).is_err());
    }
}

#[test]
fn spline_edit_matches_independent_wire_and_preserves_original_after_source_mutation() {
    let mut source = ORIGINAL.to_vec();
    let original = decode(0, 19000, &source).unwrap();
    source.fill(0xa5);
    let mut changed = original.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.splines[0].unknown_190")
        .unwrap();
    field.value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn interpolation_shapes_have_independent_extents_and_checked_overflow() {
    assert_eq!(
        spline_arrays::interpolation_shape(2, 2, 60).unwrap(),
        (10, 4)
    );
    assert!(spline_arrays::interpolation_shape(2, 2, 59).is_err());
    assert_eq!(
        spline_arrays::interpolation_shape(0, i32::MAX, 0).unwrap(),
        (0, 0)
    );
    assert!(spline_arrays::interpolation_shape(0, -1, 0).is_err());
    assert!(spline_arrays::interpolation_shape(u32::MAX, i32::MAX, usize::MAX).is_err());
}

#[test]
fn spline_curve_header_values_preserve_periodicity_and_reject_invalid_shapes() {
    assert_eq!(spline_arrays::curve_value_count(0, 0, 0, 0).unwrap(), 0);
    assert_eq!(spline_arrays::curve_value_count(3, 2, 1, 0).unwrap(), 6);
    assert_eq!(
        spline_arrays::curve_value_count(4, 99, 2, 65535).unwrap(),
        11
    );
    assert_eq!(
        spline_arrays::curve_value_count(4, 65535, 65535, 0).unwrap(),
        393210
    );
    for header in [(0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1), (2, 0, 0, 0)] {
        assert!(spline_arrays::curve_value_count(header.0, header.1, header.2, header.3).is_err());
    }
}

#[test]
fn spline_subtype_masks_admit_only_the_documented_bits() {
    let supported = [0, 1, 4, 5, 8, 9, 12, 13, 16, 17, 20, 21, 24, 25, 28, 29];
    for mask in 0..=u16::MAX {
        assert_eq!(
            sketch_splines::require_subtype(mask).is_ok(),
            supported.contains(&mask)
        );
    }
    assert!(decode(255, 19000, &[]).unwrap_err().cause.contains("255"));
}

#[test]
fn spline_aggregate_arrays_have_an_independently_counted_budget() {
    let ctx = Some(super::kernel_frame_tests::context());
    let exact = super::ModelFieldLimits {
        maximum_entries: 29,
        ..LIMITS
    };
    assert!(decode_kernel(19000, ORIGINAL, exact, ctx.clone(), sketch_splines::read).is_ok());
    let below = super::ModelFieldLimits {
        maximum_entries: 28,
        ..LIMITS
    };
    assert!(decode_kernel(19000, ORIGINAL, below, ctx, sketch_splines::read).is_err());
}

#[test]
fn spline_nested_lengths_reject_invalid_values_before_array_iteration() {
    let original = decode(0, 19000, ORIGINAL).unwrap();
    for (member, value) in BAD_ARRAY_COUNTS {
        let mut fields = original.clone();
        let path = format!("root.splines[0].{member}");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = value.clone();
        let bytes = encode_fields(&fields, 30000).unwrap();
        assert!(decode(0, 19000, &bytes).is_err(), "{member}");
    }
}

#[test]
fn spline_null_active_point_lists_and_unrecognized_subtypes_fail() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (_, version, bytes, _) = next_case(&mut cursor);
    let original = decode(0, version, bytes).unwrap();
    for (member, value, class) in [
        ("entity.flags_54", 0xfffe, crate::FailureClass::Invalid),
        ("stored_category_mask", 2, crate::FailureClass::Unsupported),
    ] {
        let mut fields = original.clone();
        let path = format!("root.splines[0].{member}");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = ModelFieldValue::U16(value);
        let error = decode(0, version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
        assert_eq!(error.class, class);
    }
}

#[test]
fn spline_kernel_frames_require_context_but_suppression_does_not() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut framed, mut suppressed) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes).unwrap();
        if has_kernel(&fields) {
            let error =
                decode_kernel(version, bytes, LIMITS, None, sketch_splines::read).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Missing);
            framed += 1;
        }
        if is_suppressed(&fields) {
            assert!(decode_kernel(version, bytes, LIMITS, None, sketch_splines::read).is_ok());
            suppressed += 1;
        }
    }
    assert_eq!((framed, suppressed), (12, 12));
}

#[test]
fn spline_parallel_writes_keep_typed_lists_and_equation_fields() {
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

fn has_kernel(fields: &[ModelField]) -> bool {
    fields
        .iter()
        .any(|field| matches!(field.value, ModelFieldValue::Kernel(_)))
}

fn is_suppressed(fields: &[ModelField]) -> bool {
    fields.iter().any(|field| {
        field.path.ends_with("kernel_body.suppressed") && field.value == ModelFieldValue::U8(195)
    })
}

fn assert_parallel_encoding(fields: &[ModelField], expected: &[u8]) {
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), expected);
        }
    });
}

#[test]
fn every_spline_case_owns_its_source_and_kernel_encoding_is_parallel() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut framed = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        if !has_kernel(&fields) {
            continue;
        }
        assert_parallel_encoding(&fields, bytes);
        framed += 1;
    }
    assert_eq!(framed, 12);
}
