//! Complete modern reference-geometry variants, independently constructed inside owned roots.
use bessemer::{Document, display_lists::*};
#[path = "support/display_archive.rs"]
mod archive_support;
#[path = "../examples/support/reference_trace.rs"]
mod reference_trace;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, check_scalar_table, context, decode_document, set_integer};
const GEOMETRY: &[u8] = include_bytes!("fixtures/display-lists/reference-kinds.bin");
const PERMUTED: &[u8] = include_bytes!("fixtures/display-lists/reference-permuted.bin");
const EDITED: &[u8] = include_bytes!("fixtures/display-lists/reference-edited.bin");
const COLLECTOR: &str = "root.tessellation.ref_geom_feature_data_collector";
const COLLECTOR_START: usize = 5493;

fn geometry_path(index: usize, field: &str) -> String {
    format!("{COLLECTOR}.entries[{index}].{field}")
}

fn edit_extra_float(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let target = geometry_path(5, "unknown_c0.f0");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == target)
        .unwrap();
    assert!(matches!(field.value, DisplayFieldValue::F64(_)));
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn every_documented_kind_and_reordered_duplicates_match_independent_fields_and_bytes() {
    let fixtures: [(&[u8], &str, usize, &[i32]); 3] = [
        (
            GEOMETRY,
            include_str!("fixtures/display-lists/reference-kinds.txt"),
            123,
            &[0, 1, 2, 3, 4, 5, 6],
        ),
        (
            PERMUTED,
            include_str!("fixtures/display-lists/reference-permuted.txt"),
            142,
            &[6, 5, 4, 3, 2, 1, 0, 2],
        ),
        (
            EDITED,
            include_str!("fixtures/display-lists/reference-edited.txt"),
            123,
            &[0, 1, 2, 3, 4, 5, 6],
        ),
    ];
    for (bytes, oracle, count, kinds) in fixtures {
        let archive = DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap();
        assert_eq!(check_scalar_table(&archive, oracle), count);
        for (index, kind) in kinds.iter().enumerate() {
            let path = geometry_path(index, "type");
            let field = archive
                .contents()
                .fields
                .iter()
                .find(|field| field.path.as_ref() == path)
                .unwrap();
            assert_eq!(field.value, DisplayFieldValue::I32(*kind));
        }
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), context(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn reference_plane_uses_the_fixed_overload_and_preserves_all_original_bits() {
    let archive = DisplayArchive::from_bytes(GEOMETRY, context(), LIMITS).unwrap();
    let path = geometry_path(3, "unknown_68");
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::FixedTransform(transform) = &field.value else {
        panic!("fixed plane transform")
    };
    assert_eq!(transform.unknown_48, [-1, i32::MAX]);
    assert_eq!(transform.unknown_70, [i32::MIN, 123456789]);
    assert_eq!(transform.matrix[1].bits(), 0x8000_0000_0000_0000);
    assert_eq!(transform.matrix[5].bits(), 0x7ff0_0000_0000_0042);
    assert_eq!(transform.scaling.bits(), 0x7ff8_0000_0000_0123);
    let vectors = include_bytes!("fixtures/display-lists/transforms.bin");
    assert_eq!(
        DisplayTransform::new(TransformContents::Fixed(transform.clone()))
            .unwrap()
            .to_bytes(),
        vectors[vectors.len() - 120..]
    );
}

#[test]
fn invalid_kind_values_are_typed_failures_before_any_variant_body_is_published() {
    let archive = DisplayArchive::from_bytes(GEOMETRY, context(), LIMITS).unwrap();
    for invalid in [-1, 7, i32::MIN, i32::MAX] {
        let mut bytes = GEOMETRY.to_vec();
        bytes[COLLECTOR_START + 8..COLLECTOR_START + 12].copy_from_slice(&invalid.to_le_bytes());
        let error = DisplayArchive::from_bytes(&bytes, context(), LIMITS).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains(&invalid.to_string()));
        assert!(error.cause.contains("expected 0..=6"));
        let mut contents = archive.contents().clone();
        set_integer(&mut contents, &geometry_path(0, "type"), invalid);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    assert_eq!(archive.to_bytes().unwrap(), GEOMETRY);
}

#[test]
fn collector_counts_budgets_and_every_new_body_truncation_are_checked() {
    for (bytes, total) in [(GEOMETRY, 56), (PERMUTED, 57)] {
        let exact = DisplayArchiveLimits {
            maximum_entries: total,
            ..LIMITS
        };
        let short = DisplayArchiveLimits {
            maximum_entries: total - 1,
            ..LIMITS
        };
        let archive = DisplayArchive::from_bytes(bytes, context(), exact).unwrap();
        assert!(DisplayArchive::from_bytes(bytes, context(), short).is_err());
        assert!(DisplayArchive::new(archive.contents().clone(), context(), short).is_err());
        for end in COLLECTOR_START..bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&bytes[..end], context(), LIMITS).is_err(),
                "cut {end}"
            );
        }
        let mut impossible = bytes.to_vec();
        impossible[COLLECTOR_START..COLLECTOR_START + 4].copy_from_slice(&1_u32.to_le_bytes());
        assert!(DisplayArchive::from_bytes(&impossible, context(), LIMITS).is_err());
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, context(), LIMITS).is_err());
    }
}

#[test]
fn nested_variant_edit_matches_independent_wire_reopens_and_reverts_the_full_document() {
    let original = DisplayArchive::from_bytes(GEOMETRY, context(), LIMITS).unwrap();
    let edited = edit_extra_float(&original);
    assert_eq!(edited.to_bytes().unwrap(), EDITED);
    let mut source = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    source
        .replace_stream("Contents/DisplayLists", GEOMETRY.to_vec())
        .unwrap();
    let source_bytes = source.to_bytes().unwrap();
    let mut document = Document::from_bytes(&source_bytes).unwrap();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let written = document.to_bytes().unwrap();
    assert_eq!(
        decode_document(&written, DisplayDocumentKind::Part).contents(),
        edited.contents()
    );
    let reopened = Document::from_bytes(&written).unwrap();
    for stream in source
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            source.read_stream(stream.name).unwrap()
        );
    }
    let reverted = edited
        .with_contents(original.contents().clone(), LIMITS)
        .unwrap();
    document
        .replace_stream("Contents/DisplayLists", reverted.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), source_bytes);
    assert_eq!(original.to_bytes().unwrap(), GEOMETRY);
}

#[test]
fn missing_variant_fields_and_equivalent_nested_text_edits_do_not_bypass_validation() {
    let archive = DisplayArchive::from_bytes(GEOMETRY, context(), LIMITS).unwrap();
    let mut invalid = archive.contents().clone();
    let path = geometry_path(5, "unknown_c0.sb0");
    invalid.fields.retain(|field| field.path.as_ref() != path);
    assert!(archive.with_contents(invalid, LIMITS).is_err());
    let mut equivalent = archive.contents().clone();
    let path = geometry_path(5, "unknown_c0.sa8");
    let field = equivalent
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::Text(text) = &field.value else {
        panic!("nested text")
    };
    field.value =
        DisplayFieldValue::Text(ArchiveString::new(text.text(), text.encoding()).unwrap());
    let short = DisplayArchiveLimits {
        maximum_bytes: GEOMETRY.len() - 1,
        ..LIMITS
    };
    assert!(DisplayArchive::new(equivalent.clone(), context(), short).is_ok());
    assert!(archive.with_contents(equivalent, short).is_err());
}

#[test]
fn source_bytes_and_parallel_reference_edits_remain_isolated() {
    let mut bytes = GEOMETRY.to_vec();
    let original = DisplayArchive::from_bytes(&bytes, context(), LIMITS).unwrap();
    bytes.fill(0);
    let jobs: Vec<_> = (0..8)
        .map(|_| {
            let copy = original.clone();
            std::thread::spawn(move || edit_extra_float(&copy).to_bytes().unwrap())
        })
        .collect();
    for job in jobs {
        assert_eq!(job.join().unwrap(), EDITED);
    }
    assert_eq!(original.to_bytes().unwrap(), GEOMETRY);
}

#[test]
fn complete_geometry_traces_include_every_fixed_and_scalar_field() {
    let fixtures: [(&[u8], &[u8]); 3] = [
        (
            GEOMETRY,
            include_bytes!("fixtures/display-lists/reference-kinds-trace.txt"),
        ),
        (
            PERMUTED,
            include_bytes!("fixtures/display-lists/reference-permuted-trace.txt"),
        ),
        (
            EDITED,
            include_bytes!("fixtures/display-lists/reference-edited-trace.txt"),
        ),
    ];
    for (bytes, expected) in fixtures {
        let archive = DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap();
        assert_eq!(reference_trace::trace(&archive), expected);
    }
}
