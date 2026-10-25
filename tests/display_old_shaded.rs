//! Independent older shaded part/assembly streams, inline facets and version-specific tails.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod old_cases;
use old_cases::OldCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/light_trace.rs"]
mod light_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    old_cases::decode(include_bytes!("fixtures/display-lists/old-shaded.bin"))
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

fn selected(scenario: u8) -> OldCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == 10000
                && case.kind == DisplayDocumentKind::Part
                && case.scenario == scenario
        })
        .unwrap()
}

fn assert_trace(actual: &[u8], expected: &[u8]) {
    let actual = std::str::from_utf8(actual)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let expected = std::str::from_utf8(expected)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(actual, expected, "trace row {index}");
    }
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn complete_part_assembly_and_unshaded_roots_match_independent_fields_and_bytes() {
    assert_eq!(cases().len(), 6);
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
        trace.extend(light_trace::trace(&archive));
        trace.extend(item_trace::trace(&archive));
        assert_trace(&trace, case.expected);
        let count = archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            count,
            if case.kind == DisplayDocumentKind::Part {
                0
            } else if case.version == 10000 {
                34
            } else {
                39
            }
        );
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}

#[test]
fn old_tail_presence_and_minimal_annotation_records_follow_the_declared_version() {
    for case in cases() {
        let archive = parsed(case);
        let fields = &archive.contents().fields;
        for absent in [
            "orig_font_scale",
            "hdr_exposure",
            "field_of_view",
            "for_c_thread",
            "has_texture2",
            "plm_id",
            "face_string_empty",
            "show_c_thread",
        ] {
            assert!(
                !fields
                    .iter()
                    .any(|field| field.path.rsplit('.').next() == Some(absent)),
                "{absent}"
            );
        }
        let assembly = case.kind == DisplayDocumentKind::Assembly;
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.configurations.config_count"),
            assembly
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("viewer_comp_data.fixed")),
            assembly && case.version == 11000
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("body_prop_infos.count")),
            case.scenario != 2
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("graphic_mesh_color_property")),
            case.scenario != 2 && case.version == 11000
        );
    }
    let archive = parsed(selected(2));
    let table = "root.views[0].display_items.items[2].data.edw_annot_data.count=i:3\nroot.views[0].display_items.items[2].data.edw_annot_data[2].unknown_i=i:-8\n";
    assert_eq!(archive_support::check_scalar_table(&archive, table), 2);
    assert!(
        !archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.ends_with(".id_string")
                || field.path.ends_with(".view_names_3d_with_scale"))
    );
}

fn edited_material(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_material_edit_reopens_with_explicit_context_preserves_streams_and_reverts() {
    let case = selected(0);
    let archive = parsed(case);
    let edited = edited_material(&archive);
    assert_eq!(edited.to_bytes().unwrap(), selected(1).bytes);
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
    let initial = include_bytes!("fixtures/blank-part.SLDPRT");
    let original = bessemer::Document::from_bytes(initial).unwrap();
    let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
    let mut document = original.clone();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        DisplayArchive::from_bytes(
            &reopened.read_stream("Contents/DisplayLists").unwrap(),
            case_context(case),
            LIMITS
        )
        .unwrap()
        .contents(),
        edited.contents()
    );
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
}

#[test]
fn global_budgets_cover_old_facets_collections_and_conditional_fields() {
    for case in cases() {
        let archive = parsed(case);
        let assembly = case.kind == DisplayDocumentKind::Assembly;
        let entries = if case.scenario == 2 {
            51
        } else {
            87 + usize::from(case.version == 11000) * 4 + usize::from(assembly) * 8
        };
        let objects = if case.scenario == 2 {
            42
        } else if assembly {
            54
        } else {
            52
        };
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limited in [
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
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn unsupported_old_bucket_bodies_never_fall_through_to_the_modern_layout() {
    let case = selected(2);
    let archive = parsed(case);
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.views[0].view_bucket")
        .unwrap();
    let class = DisplayClassDescriptor {
        schema: 1,
        name: "uiViewBucket_c".into(),
    };
    field.value = DisplayFieldValue::ObjectTag(DisplayObjectTag {
        target: DisplayObjectPointer::New(DisplayObjectId::new(47).unwrap()),
        runtime_class: Some(class.clone()),
        tag: u16::MAX,
        extended: None,
        new_class: Some(class),
    });
    let error = archive
        .with_contents(contents, LIMITS)
        .unwrap_err()
        .to_string();
    assert!(error.contains("Unsupported"));
    assert!(error.contains("view bucket version 10000"));
}

#[test]
fn old_component_owner_cycles_aliases_and_native_input_isolation_are_retained() {
    let case = cases()
        .into_iter()
        .find(|case| case.kind == DisplayDocumentKind::Assembly && case.version == 10000)
        .unwrap();
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    let fields = &archive.contents().fields;
    let component = &fields
        .iter()
        .find(|field| field.path.as_ref() == "root.tessellation.components[0]")
        .unwrap()
        .value;
    let owner = &fields
        .iter()
        .find(|field| field.path.as_ref() == "root.tessellation.components[0].owner")
        .unwrap()
        .value;
    let (DisplayFieldValue::ObjectTag(component), DisplayFieldValue::ObjectTag(owner)) =
        (component, owner)
    else {
        panic!("object tags")
    };
    let DisplayObjectPointer::New(identity) = component.target else {
        panic!("new component")
    };
    assert_eq!(owner.target, DisplayObjectPointer::Reference(identity));
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn old_root_truncations_malformed_counts_and_parallel_edits_are_checked() {
    let case = selected(0);
    let archive = parsed(case);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        "root.tessellation.model_viz_props.count",
        i32::MAX,
    );
    assert!(archive.with_contents(contents, LIMITS).is_err());
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_material(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(1).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
