//! Old face pairs, scene flags, property fonts and polyline words retain exact version shapes.
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared document helpers have dedicated whole-file tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(
    dead_code,
    reason = "Shared view formatting also serves view-specific tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    [
        include_bytes!("fixtures/display-lists/earliest-face-part.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-face-assembly.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-face-properties.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-face-holders.bin").as_slice(),
    ]
    .into_iter()
    .flat_map(complete_cases::decode)
    .collect()
}

fn case_context(case: OldCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: OldCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

fn selected(version: u32, kind: DisplayDocumentKind, scenario: u8) -> OldCase {
    cases()
        .into_iter()
        .find(|case| (case.version, case.kind, case.scenario) == (version, kind, scenario))
        .unwrap()
}

fn has_field(archive: &DisplayArchive, suffix: &str) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.ends_with(suffix))
}

#[test]
fn all_complete_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 54);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut actual = material_trace::trace(&archive);
        actual.extend(scene_trace::trace(&archive));
        actual.extend(item_trace::trace(&archive));
        assert_eq!(
            std::str::from_utf8(&actual).unwrap(),
            std::str::from_utf8(case.expected).unwrap(),
            "{} {:?} {}",
            case.version,
            case.kind,
            case.scenario
        );
        fields += actual.iter().filter(|byte| **byte == b'\n').count()
            + archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 20574);
}

#[test]
fn populated_face_pairs_scene_flags_and_polyline_words_follow_distinct_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario == 192) {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, ".unknown_58.count"),
            case.version >= 0x1203
        );
        assert_eq!(
            has_field(&archive, ".unknown_58[1][1]"),
            case.version >= 0x1203
        );
        assert_eq!(
            has_field(&archive, "root.scene.flags"),
            case.version > 0x1211
        );
        assert_eq!(
            has_field(&archive, ".items[5].data.unknown_10"),
            case.version > 0x12b0
        );
        assert!(has_field(&archive, ".items[5].data.points[1][2]"));
    }
}

#[test]
fn property_graphs_keep_twenty_four_or_twenty_five_complete_font_slots() {
    for case in cases().into_iter().filter(|case| case.scenario < 32) {
        let archive = parsed(case);
        let count = archive
            .contents()
            .fields
            .iter()
            .filter(|field| {
                field
                    .path
                    .starts_with("root.scene.concept.feature_list.nodes[1].")
                    && field.path.contains(".line_fonts[")
                    && field.path.ends_with(".standard_width")
            })
            .count();
        assert_eq!(count, if case.version < 0x12a3 { 24 } else { 25 });
    }
}

fn budgets() -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_finished, take_u32};
    let mut bytes = include_bytes!("fixtures/display-lists/earliest-face-budgets.bin").as_slice();
    let count = take_u32(&mut bytes).unwrap();
    let values = (0..count)
        .map(|_| {
            (
                take_u32(&mut bytes).unwrap() as usize,
                take_u32(&mut bytes).unwrap() as usize,
            )
        })
        .collect();
    require_finished(bytes).unwrap();
    values
}

#[test]
fn exact_budgets_charge_polyline_points_face_pairs_and_each_embedded_font() {
    assert_eq!(budgets().len(), cases().len());
    for (case, (entries, objects)) in cases().into_iter().zip(budgets()) {
        let archive = parsed(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok(),
            "{} {:?} {} entries {entries}",
            case.version,
            case.kind,
            case.scenario
        );
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limits in [
            DisplayArchiveLimits {
                maximum_entries: entries - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: objects - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_bytes: case.bytes.len() - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn cross_version_construction_rejects_absent_and_extra_members() {
    for (before, after, scenario) in [
        (0x1202, 0x1203, 192),
        (0x1211, 0x1212, 192),
        (0x12a2, 0x12a3, 0),
        (0x12b0, 0x12b1, 192),
    ] {
        let old = selected(before, DisplayDocumentKind::Part, scenario);
        let new = selected(after, DisplayDocumentKind::Part, scenario);
        assert!(
            DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
        );
        assert!(
            DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
        );
    }
}

fn edited_ambient(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn concurrent_edits_match_independent_bytes_reopen_and_leave_sources_isolated() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(4600, kind, 192);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        let expected = selected(4600, kind, 193).bytes;
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), expected);
            }
        });
        assert_eq!(
            DisplayArchive::from_bytes(expected, case_context(case), LIMITS)
                .unwrap()
                .contents(),
            edited_ambient(&archive).contents()
        );
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn complete_early_roots_reject_every_prefix_and_unbounded_pair_counts() {
    for version in [4600, 0x1202, 0x1203, 0x1211, 0x1212, 0x12b0, 0x12b1] {
        let case = selected(version, DisplayDocumentKind::Part, 192);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
    }
    let case = selected(0x1203, DisplayDocumentKind::Part, 192);
    let archive = parsed(case);
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        "root.tessellation.bodies[0].faces[0].unknown_58.count",
        i32::MAX,
    );
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[path = "support/tessellation.rs"]
mod tessellation_support;

fn document_archive(document: &bessemer::Document, kind: DisplayDocumentKind) -> DisplayArchive {
    let context = DisplayArchiveContext::from_document(
        document,
        kind,
        19000,
        tessellation_support::modern_context(),
        tessellation_support::LIMITS,
        100000,
    )
    .unwrap();
    assert_eq!(context.version.value(), 4600);
    DisplayArchive::from_bytes(
        &document.read_stream("Contents/DisplayLists").unwrap(),
        context,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_documents_resolve_old_contexts_reopen_independent_edits_and_restore_originals() {
    for (kind, source) in [
        (
            DisplayDocumentKind::Part,
            include_bytes!("fixtures/display-lists/earliest-face-document-part.bin").as_slice(),
        ),
        (
            DisplayDocumentKind::Assembly,
            include_bytes!("fixtures/display-lists/earliest-face-document-assembly.bin").as_slice(),
        ),
    ] {
        let mut document = bessemer::Document::from_bytes(source).unwrap();
        let archive = document_archive(&document, kind);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(4600, kind, 193).bytes);
        document
            .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(
            document_archive(&reopened, kind).contents(),
            edited.contents()
        );
        assert_eq!(
            reopened.read_stream("Unrelated/Payload").unwrap(),
            b"retained\0payload"
        );
        document
            .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), source);
    }
}
