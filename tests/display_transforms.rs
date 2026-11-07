//! Independent binary vectors for the two distinct native transform serializers.
use bessemer::display_lists::*;
const BITS: [u64; 8] = [
    0,
    0x8000000000000000,
    0x7ff0000000000000,
    0xfff0000000000000,
    0x7ff8000000000123,
    0x7ff0000000000042,
    1,
    0x7fefffffffffffff,
];
const VECTORS: &[u8] = include_bytes!("fixtures/display-lists/transforms.bin");

fn vectors() -> Vec<(TransformEncoding, &'static [u8])> {
    let count = u32::from_le_bytes(VECTORS[..4].try_into().unwrap()) as usize;
    let mut offset = 4;
    let mut records = Vec::new();
    for _ in 0..count {
        let kind = match VECTORS[offset] {
            0 => TransformEncoding::Compact,
            1 => TransformEncoding::Fixed,
            other => panic!("kind {other}"),
        };
        let length =
            u32::from_le_bytes(VECTORS[offset + 1..offset + 5].try_into().unwrap()) as usize;
        records.push((kind, &VECTORS[offset + 5..offset + 5 + length]));
        offset += 5 + length;
    }
    assert_eq!(offset, VECTORS.len());
    assert_eq!(records.len(), 1025);
    records
}

fn check_bits(values: &[StoredDouble], start: usize) {
    for (index, value) in values.iter().enumerate() {
        assert_eq!(value.bits(), BITS[(start + index) % 8]);
    }
}

#[test]
fn every_flag_byte_and_presence_spelling_matches_independent_wire_bits_and_reconstructs_exactly() {
    for (kind, bytes) in vectors() {
        let transform = DisplayTransform::from_bytes(bytes, kind).unwrap();
        assert_eq!(transform.to_bytes(), bytes);
        assert_eq!(
            DisplayTransform::new(transform.contents().clone())
                .unwrap()
                .to_bytes(),
            bytes
        );
        match transform.contents() {
            TransformContents::Compact(compact) => check_compact(compact, bytes),
            TransformContents::Fixed(fixed) => check_fixed(fixed),
        }
    }
}

fn check_compact(compact: &CompactTransform, bytes: &[u8]) {
    assert_eq!(compact.has_matrix, bytes[0]);
    assert_eq!(compact.matrix.is_some(), bytes[0] != 0);
    if let Some(matrix) = &compact.matrix {
        for (index, row) in matrix.iter().enumerate() {
            check_bits(row, index * 3);
        }
    }
    check_bits(&compact.translation, 9);
    assert_eq!(compact.scaling.bits(), BITS[4]);
    let position = if compact.has_matrix == 0 { 33 } else { 105 };
    assert_eq!(compact.flags, bytes[position]);
    check_perspective(compact.flags, &compact.perspective);
}

fn check_perspective(flags: u8, perspective: &PerspectivePayload) {
    match perspective {
        PerspectivePayload::Base(base) => {
            assert_eq!(flags & 0xf0, 0x10);
            check_base(base);
        }
        PerspectivePayload::Extended {
            base,
            minimum_far_clip,
            camera_fov_plane,
        } => {
            assert_eq!(flags & 0xf0, 0x20);
            check_base(base);
            check_bits(&[*minimum_far_clip, *camera_fov_plane], 17);
        }
        PerspectivePayload::Matrix5 { discarded, matrix } => {
            assert_eq!(flags & 0xf0, 0x40);
            assert_eq!(*discarded, -1234567);
            check_bits(matrix.as_ref(), 19);
        }
        PerspectivePayload::None => assert!(![0x10, 0x20, 0x40].contains(&(flags & 0xf0))),
    }
}

fn check_base(base: &PerspectiveBase) {
    check_bits(&base.eye_point, 13);
    assert_eq!(base.plane_distance.bits(), BITS[0]);
}

fn check_fixed(fixed: &FixedTransform) {
    check_bits(&fixed.matrix, 0);
    assert_eq!(fixed.unknown_48, [-1, 2147483647]);
    check_bits(&fixed.translation, 9);
    assert_eq!(fixed.scaling.bits(), BITS[4]);
    assert_eq!(fixed.unknown_70, [-2147483648, 123456789]);
}

#[test]
fn every_truncation_and_trailing_byte_is_rejected_without_overload_guessing() {
    for (kind, bytes) in vectors() {
        for length in 0..bytes.len() {
            assert!(
                DisplayTransform::from_bytes(&bytes[..length], kind).is_err(),
                "{kind:?} length {length}"
            );
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(DisplayTransform::from_bytes(&extra, kind).is_err());
        let other = if kind == TransformEncoding::Compact {
            TransformEncoding::Fixed
        } else {
            TransformEncoding::Compact
        };
        assert!(DisplayTransform::from_bytes(bytes, other).is_err());
    }
}

#[test]
fn constructed_flag_matrix_and_perspective_mismatches_are_typed_and_leave_sources_isolated() {
    for (kind, bytes) in vectors()
        .into_iter()
        .filter(|(kind, _)| *kind == TransformEncoding::Compact)
        .step_by(16)
    {
        let original = DisplayTransform::from_bytes(bytes, kind).unwrap();
        let TransformContents::Compact(mut edited) = original.contents().clone() else {
            panic!("compact");
        };
        edited.has_matrix = if edited.has_matrix == 0 { 2 } else { 0 };
        assert!(
            original
                .with_contents(TransformContents::Compact(edited))
                .is_err()
        );
        let TransformContents::Compact(mut edited) = original.contents().clone() else {
            panic!("compact");
        };
        edited.perspective = PerspectivePayload::None;
        edited.flags = 0x10;
        let error = original
            .with_contents(TransformContents::Compact(edited))
            .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert_eq!(original.to_bytes(), bytes);
    }
}

#[test]
fn wrong_nonempty_perspective_variant_is_rejected_and_valid_edits_revert_exactly() {
    let (_, bytes) = vectors()[16];
    let original = DisplayTransform::from_bytes(bytes, TransformEncoding::Compact).unwrap();
    let TransformContents::Compact(mut contents) = original.contents().clone() else {
        panic!("compact");
    };
    contents.flags = 0x20;
    assert!(
        original
            .with_contents(TransformContents::Compact(contents.clone()))
            .is_err()
    );
    contents.flags = 0x1f;
    contents.scaling = StoredDouble::from_bits(1.0f64.to_bits());
    let edited = original
        .with_contents(TransformContents::Compact(contents.clone()))
        .unwrap();
    contents.translation[0] = StoredDouble::from_bits(123);
    assert_ne!(edited.contents(), &TransformContents::Compact(contents));
    assert_eq!(
        edited
            .with_contents(original.contents().clone())
            .unwrap()
            .to_bytes(),
        bytes
    );
    assert_eq!(original.to_bytes(), bytes);
}

#[test]
fn transform_public_types_are_send_sync_and_concurrent_encoding_is_deterministic() {
    fn shared<T: Send + Sync>() {}
    shared::<DisplayTransform>();
    shared::<TransformContents>();
    shared::<TransformEncoding>();
    shared::<CompactTransform>();
    shared::<FixedTransform>();
    shared::<PerspectivePayload>();
    shared::<PerspectiveBase>();
    let (_, bytes) = vectors()[64];
    let original = DisplayTransform::from_bytes(bytes, TransformEncoding::Compact).unwrap();
    std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8)
            .map(|_| {
                let original = &original;
                scope.spawn(move || {
                    original
                        .with_contents(original.contents().clone())
                        .unwrap()
                        .to_bytes()
                })
            })
            .collect();
        for task in tasks {
            assert_eq!(task.join().unwrap(), bytes);
        }
    });
}

#[test]
fn cloned_matrix5_payloads_own_their_storage_after_source_and_published_value_edits() {
    let (_, bytes) = vectors()[64];
    let original = DisplayTransform::from_bytes(bytes, TransformEncoding::Compact).unwrap();
    let TransformContents::Compact(mut owned) = original.contents().clone() else {
        panic!("compact");
    };
    let PerspectivePayload::Matrix5 { matrix, .. } = &mut owned.perspective else {
        panic!("matrix5");
    };
    matrix[0] = StoredDouble::from_bits(123);
    let published = original
        .with_contents(TransformContents::Compact(owned.clone()))
        .unwrap();
    let PerspectivePayload::Matrix5 { matrix, .. } = &mut owned.perspective else {
        panic!("matrix5");
    };
    matrix[0] = StoredDouble::from_bits(456);
    assert_ne!(published.contents(), &TransformContents::Compact(owned));
    assert_eq!(original.to_bytes(), bytes);
}
