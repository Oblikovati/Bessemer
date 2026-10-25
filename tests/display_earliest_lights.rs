//! Old light bodies, appearance words and assembly lists keep their individual archive gates.
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared document helpers also serve other integration crates."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase;
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
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
        include_bytes!("fixtures/display-lists/earliest-light-roots.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-light-kinds.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-light-buckets-3d.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-light-buckets-2d.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-light-decals.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-light-properties.bin").as_slice(),
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

fn trace(archive: &DisplayArchive, case: OldCase) -> Vec<u8> {
    if case.expected.is_empty() {
        return Vec::new();
    }
    if (160..192).contains(&case.scenario) {
        return bucket_trace::trace(archive);
    }
    let mut rows = material_trace::trace(archive);
    rows.extend(scene_trace::trace(archive));
    rows.extend(item_trace::trace(archive));
    rows
}

#[test]
fn complete_roots_match_every_independent_field_and_forced_byte() {
    assert_eq!(cases().len(), 102);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let actual = trace(&archive, case);
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
    assert_eq!(fields, 37709);
}

#[test]
fn all_five_light_kinds_keep_their_versioned_base_and_derived_fields() {
    for case in cases().into_iter().filter(|case| {
        case.kind == DisplayDocumentKind::Part && (128..138).contains(&case.scenario)
    }) {
        let archive = parsed(case);
        let kind = (case.scenario - 128) / 2;
        let prefix = "root.old_lights.items[1]";
        for (leaf, present) in [
            ("enabled_in_pv_initial", case.version > 0x11a3),
            ("pv_brightness", case.version > 0x11a3),
            ("id", case.version > 0x11f5),
            ("pv_shadow_softness", kind == 2 && case.version >= 0x11a4),
            ("pv_soft_edge", kind == 3 && case.version >= 0x11a4),
            ("pv_point_light_radius", kind == 4 && case.version > 0x11a3),
        ] {
            assert_eq!(
                has_field(&archive, &format!("{prefix}.{leaf}")),
                present,
                "{} {kind} {leaf}",
                case.version
            );
        }
        assert!(has_field(
            &archive,
            &format!("{prefix}.user_property.entries[3].type")
        ));
    }
}

#[test]
fn appearance_brightness_hidden_atoms_and_detail_mask_obey_distinct_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario == 128) {
        let archive = parsed(case);
        for (leaf, threshold) in [
            (".rv_app_props.knurl_angle", 0x119e),
            (".rv_app_props.dimple_size", 0x11ac),
            ("root.scene.pw_scene_data.pv_rendering_brightness", 0x1194),
            ("root.detail_mask_12", 0x11ed),
        ] {
            assert_eq!(
                has_field(&archive, leaf),
                case.version > threshold,
                "{} {leaf}",
                case.version
            );
        }
        if case.kind == DisplayDocumentKind::Assembly {
            assert_eq!(
                has_field(&archive, ".hidden_body_atoms.count"),
                case.version > 0x11af
            );
            assert_eq!(
                has_field(&archive, ".hidden_body_atoms[2]"),
                case.version > 0x11af
            );
        }
    }
}

#[test]
fn populated_weld_lists_and_decal_options_obey_their_own_conditions() {
    for case in cases() {
        let archive = parsed(case);
        if matches!(case.scenario, 160 | 176) {
            assert_eq!(
                has_field(&archive, ".weldbead_v.items[0].count"),
                case.version > 0x11a5
            );
            assert_eq!(
                has_field(&archive, ".weldbead_h.items[0].count"),
                case.version > 0x11a5
            );
        }
        if case.expected.is_empty() {
            assert_eq!(
                has_field(&archive, ".use_underlying_appearance"),
                case.version > 0x1197
            );
            assert_eq!(has_field(&archive, ".orig_decal_id"), case.version > 0x11bb);
        }
    }
}

fn budgets() -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_finished, take_u32};
    let mut bytes = include_bytes!("fixtures/display-lists/earliest-light-budgets.bin").as_slice();
    let count = take_u32(&mut bytes).unwrap();
    let rows = (0..count)
        .map(|_| {
            (
                take_u32(&mut bytes).unwrap() as usize,
                take_u32(&mut bytes).unwrap() as usize,
            )
        })
        .collect();
    require_finished(bytes).unwrap();
    rows
}

#[test]
fn exact_budgets_include_light_properties_hidden_atoms_welds_and_decals() {
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
            "{} {:?} {} {entries}",
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
fn cross_version_construction_rejects_new_and_missing_members() {
    for (before, after, scenario, kind) in [
        (4500, 4501, 128, DisplayDocumentKind::Part),
        (0x119e, 0x119f, 128, DisplayDocumentKind::Part),
        (0x11a3, 0x11a4, 128, DisplayDocumentKind::Part),
        (0x11ac, 0x11ad, 128, DisplayDocumentKind::Part),
        (0x11af, 0x11b0, 128, DisplayDocumentKind::Assembly),
        (0x11ed, 0x11ee, 128, DisplayDocumentKind::Part),
        (0x11f5, 0x11f6, 128, DisplayDocumentKind::Part),
        (0x11a5, 0x11a6, 160, DisplayDocumentKind::Part),
    ] {
        let old = selected(before, kind, scenario);
        let new = selected(after, kind, scenario);
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
fn concurrent_old_layout_edits_match_independent_bytes_and_own_sources() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(4500, kind, 128);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(4500, kind, 129).bytes);
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn every_old_light_prefix_and_impossible_hidden_atom_count_is_rejected() {
    for scenario in [128, 130, 132, 134, 136] {
        let case = selected(4500, DisplayDocumentKind::Part, scenario);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{scenario} prefix {end}"
            );
        }
    }
    let case = selected(0x11b0, DisplayDocumentKind::Assembly, 128);
    let archive = parsed(case);
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        "root.tessellation.components[0].hidden_body_atoms.count",
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
    assert_eq!(context.version.value(), 4500);
    DisplayArchive::from_bytes(
        &document.read_stream("Contents/DisplayLists").unwrap(),
        context,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_documents_resolve_4500_contexts_reopen_edits_and_authentically_revert() {
    for (kind, source) in [
        (
            DisplayDocumentKind::Part,
            include_bytes!("fixtures/display-lists/earliest-light-document-part.bin").as_slice(),
        ),
        (
            DisplayDocumentKind::Assembly,
            include_bytes!("fixtures/display-lists/earliest-light-document-assembly.bin")
                .as_slice(),
        ),
    ] {
        let mut document = bessemer::Document::from_bytes(source).unwrap();
        let archive = document_archive(&document, kind);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(4500, kind, 129).bytes);
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
