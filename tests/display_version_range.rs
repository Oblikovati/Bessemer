//! Complete roots at every material, scene and spatial version boundary from 18000 to 19000.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "../examples/support/framing.rs"]
mod framing;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(
    dead_code,
    reason = "Shared view helpers are exercised by view-specific tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

#[derive(Clone, Copy)]
struct VersionCase {
    version: u32,
    edited: bool,
    bytes: &'static [u8],
    expected: &'static [u8],
}

fn cases() -> Vec<VersionCase> {
    let mut remaining: &'static [u8] = include_bytes!("fixtures/display-lists/version-range.bin");
    let count = framing::take_u32(&mut remaining).unwrap();
    framing::require_count(count, remaining.len(), 13).unwrap();
    let records = (0..count).map(|_| read_case(&mut remaining)).collect();
    framing::require_finished(remaining).unwrap();
    records
}

fn read_case(remaining: &mut &'static [u8]) -> VersionCase {
    let version = framing::take_u32(remaining).unwrap();
    let edited = framing::take(remaining, 1).unwrap()[0] != 0;
    let length = framing::take_u32(remaining).unwrap() as usize;
    let bytes = framing::take(remaining, length).unwrap();
    let length = framing::take_u32(remaining).unwrap() as usize;
    let expected = framing::take(remaining, length).unwrap();
    VersionCase {
        version,
        edited,
        bytes,
        expected,
    }
}

fn selected(version: u32, edited: bool) -> VersionCase {
    cases()
        .into_iter()
        .find(|case| case.version == version && case.edited == edited)
        .unwrap()
}

fn version_context(version: u32) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..context()
    }
}

fn parsed(case: VersionCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, version_context(case.version), LIMITS).unwrap()
}

fn assert_trace(actual: &[u8], expected: &[u8], version: u32) {
    let actual = std::str::from_utf8(actual)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let expected = std::str::from_utf8(expected)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(actual, expected, "version {version}, row {index}");
    }
    assert_eq!(actual.len(), expected.len(), "version {version}");
}

#[test]
fn every_supported_gate_matches_independent_semantic_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 12);
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
        assert_trace(&trace, case.expected, case.version);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), archive.context(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}

#[test]
fn five_version_gates_have_explicit_presence_and_preserve_discarded_symbol_fields() {
    for case in cases() {
        let archive = parsed(case);
        let fields = &archive.contents().fields;
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("unknown_symbol.strings[0]")),
            case.version < 0x46ba
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.visual_properties.dspbr.unknown_0"),
            case.version > 0x46b4
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.visual_properties.dspbr.unknown_158"),
            case.version > 0x4971
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.scene.field_of_view"),
            case.version > 0x46ba
        );
        let field = fields
            .iter()
            .find(|field| field.path.as_ref() == "root.view_3d_transform")
            .unwrap();
        let DisplayFieldValue::SpatialViews(Some(views)) = &field.value else {
            panic!("spatial views")
        };
        assert_eq!(views.len(), 2);
        assert_eq!(
            views[0].unknown_active,
            if case.version > 0x46c9 {
                Some(-17)
            } else {
                None
            }
        );
        assert_eq!(views[0].section_planes.len(), 2);
    }
    let archive = parsed(selected(18000, false));
    let table = "root.visual_properties.material.rv_app_props.unknown_symbol.uints[0]=i:4294967295\nroot.visual_properties.material.rv_app_props.unknown_symbol.uints[1]=i:4275878552\nroot.visual_properties.material.rv_app_props.unknown_symbol.ints[0]=i:-1\n";
    assert_eq!(archive_support::check_scalar_table(&archive, table), 3);
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| {
            field.path.as_ref()
                == "root.visual_properties.material.rv_app_props.unknown_symbol.floats[5]"
        })
        .unwrap();
    let DisplayFieldValue::F32(value) = field.value else {
        panic!("stored float")
    };
    assert_eq!(value.bits(), 0x7f800042);
}

#[test]
fn context_range_limits_and_cross_gate_shape_requirements_are_explicit() {
    let oldest = selected(18000, false);
    for version in [0, 3195, 19001, u32::MAX] {
        let error =
            DisplayArchive::from_bytes(oldest.bytes, version_context(version), LIMITS).unwrap_err();
        assert!(error.to_string().contains("Unsupported"));
    }
    let alternate_file = DisplayArchiveContext {
        file_version: 18000,
        ..version_context(18000)
    };
    assert_eq!(
        DisplayArchive::from_bytes(oldest.bytes, alternate_file, LIMITS)
            .unwrap()
            .contents(),
        parsed(oldest).contents()
    );
    for (before, after) in [
        (18100, 18101),
        (18105, 18106),
        (18106, 18107),
        (18121, 18122),
        (18801, 18802),
    ] {
        let archive = parsed(selected(before, false));
        assert!(
            DisplayArchive::new(archive.contents().clone(), version_context(after), LIMITS)
                .is_err(),
            "{before}->{after}"
        );
    }
    let recent = selected(19000, false);
    assert!(DisplayArchive::from_bytes(recent.bytes, version_context(18999), LIMITS).is_ok());
}

fn edited_exposure(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.hdr_exposure")
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_old_context_edit_reopens_with_explicit_context_and_reverts_exactly() {
    let original = parsed(selected(18000, false));
    let edited = edited_exposure(&original);
    assert_eq!(edited.to_bytes().unwrap(), selected(18000, true).bytes);
    assert_eq!(
        edited
            .with_contents(original.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        selected(18000, false).bytes
    );
    let initial = include_bytes!("fixtures/blank-part.SLDPRT");
    let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
    let mut document = bessemer::Document::from_bytes(initial).unwrap();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let written = document.to_bytes().unwrap();
    let reopened = bessemer::Document::from_bytes(&written).unwrap();
    let raw = reopened.read_stream("Contents/DisplayLists").unwrap();
    assert_eq!(
        DisplayArchive::from_bytes(&raw, original.context(), LIMITS)
            .unwrap()
            .contents(),
        edited.contents()
    );
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
    assert_eq!(
        archive_support::decode_document(&document.to_bytes().unwrap(), DisplayDocumentKind::Part)
            .contents(),
        native.contents()
    );
}

#[test]
fn exact_global_budgets_cover_symbol_fields_and_populated_spatial_views() {
    for version in [18000, 18101, 18106, 18107, 18122, 18802, 19000] {
        let case = selected(version, false);
        let archive = parsed(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: 68,
            maximum_objects: 49,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, archive.context(), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), archive.context(), exact).is_ok());
        for limited in [
            DisplayArchiveLimits {
                maximum_entries: 67,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: 48,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, archive.context(), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn missing_fixed_fields_raw_flags_and_every_old_root_prefix_are_checked() {
    let case = selected(18000, false);
    let archive = parsed(case);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], archive.context(), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut contents = archive.contents().clone();
    contents
        .fields
        .retain(|field| !field.path.ends_with("unknown_symbol.uints[1]"));
    assert!(archive.with_contents(contents, LIMITS).is_err());
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, "root.scene.dspbr_tone_map", i32::MIN);
    let changed = archive.with_contents(contents, LIMITS).unwrap();
    assert_eq!(
        archive_support::check_scalar_table(&changed, "root.scene.dspbr_tone_map=i:-2147483648\n"),
        1
    );
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn old_version_results_own_source_storage_and_parallel_edits_match_the_oracle() {
    let case = selected(18000, false);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, version_context(18000), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_exposure(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(18000, true).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
