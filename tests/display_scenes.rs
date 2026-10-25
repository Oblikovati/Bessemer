//! Complete modern scene layouts, pointer roles, count semantics and independent edits.
use bessemer::{Document, display_lists::*};
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, check_scalar_table, context, decode_document, set_integer};
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(
    dead_code,
    reason = "Shared trace helpers are exercised by view-specific integration tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;
const CUBE: &[u8] = include_bytes!("fixtures/display-lists/scene-cubical.bin");
const SPHERE: &[u8] = include_bytes!("fixtures/display-lists/scene-spherical.bin");
const EMPTY: &[u8] = include_bytes!("fixtures/display-lists/scene-empty.bin");
const OMITTED: &[u8] = include_bytes!("fixtures/display-lists/scene-omitted.bin");
const EDITED: &[u8] = include_bytes!("fixtures/display-lists/scene-edited.bin");

fn parsed(bytes: &[u8]) -> DisplayArchive {
    DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap()
}

fn scene_field<'a>(contents: &'a DisplayArchiveContents, path: &str) -> &'a DisplayFieldValue {
    &contents
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value
}

fn edited_exposure(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.hdr_exposure")
        .unwrap();
    assert!(matches!(field.value, DisplayFieldValue::F64(_)));
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn both_scene_types_all_wall_shapes_and_empty_modes_match_independent_fields_and_bytes() {
    let fixtures: [(&[u8], &[u8]); 5] = [
        (
            CUBE,
            include_bytes!("fixtures/display-lists/scene-cubical-trace.txt"),
        ),
        (
            SPHERE,
            include_bytes!("fixtures/display-lists/scene-spherical-trace.txt"),
        ),
        (
            EMPTY,
            include_bytes!("fixtures/display-lists/scene-empty-trace.txt"),
        ),
        (
            OMITTED,
            include_bytes!("fixtures/display-lists/scene-omitted-trace.txt"),
        ),
        (
            EDITED,
            include_bytes!("fixtures/display-lists/scene-edited-trace.txt"),
        ),
    ];
    for (bytes, expected) in fixtures {
        let archive = parsed(bytes);
        assert_eq!(scene_trace::trace(&archive), expected);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), context(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
    let archive = parsed(CUBE);
    let raw_counts = "root.scene.concept.has_merged_feature_list=i:-17\nroot.scene.walls.count=i:3\nroot.scene.pw_scene_data.wall_info.count=i:3\nroot.scene.concept.merged_feature_list.nodes.count_word=i:65535\nroot.scene.concept.merged_feature_list.nodes.count_dword=i:1\nroot.scene.concept.feature_list.nodes.count_dword=i:2\n";
    assert_eq!(check_scalar_table(&archive, raw_counts), 6);
    let DisplayFieldValue::F32(value) = scene_field(
        archive.contents(),
        "root.scene.walls[0].wall.unknown_20[0][1]",
    ) else {
        panic!("stored float")
    };
    assert_eq!(value.bits(), 0x80000000);
    assert_eq!(
        scene_field(
            archive.contents(),
            "root.scene.concept.merged_feature_list.nodes.count_word"
        ),
        &DisplayFieldValue::U16(u16::MAX)
    );
}

#[test]
fn nonpositive_wall_counts_are_completed_empty_collections_with_their_raw_values_retained() {
    let archive = parsed(EMPTY);
    assert_eq!(
        scene_field(archive.contents(), "root.scene.walls.count"),
        &DisplayFieldValue::I32(-2)
    );
    assert_eq!(
        scene_field(
            archive.contents(),
            "root.scene.pw_scene_data.wall_info.count"
        ),
        &DisplayFieldValue::I32(-3)
    );
    let mut contents = archive.contents().clone();
    for path in [
        "root.scene.walls.count",
        "root.scene.pw_scene_data.wall_info.count",
    ] {
        set_integer(&mut contents, path, i32::MIN);
    }
    let changed = archive.with_contents(contents, LIMITS).unwrap();
    assert_eq!(scene_trace::trace(&changed), scene_trace::trace(&archive));
    assert_ne!(changed.to_bytes().unwrap(), EMPTY);
    assert!(
        !parsed(OMITTED)
            .contents()
            .fields
            .iter()
            .any(|field| field.path.starts_with("root.scene."))
    );
}

#[test]
fn heterogeneous_cobject_slots_accept_aliases_but_declared_material_roles_still_reject_them() {
    let archive = parsed(CUBE);
    let name = scene_field(archive.contents(), "root.scene.floor_selection").clone();
    let DisplayFieldValue::ObjectTag(tag) = &name else {
        panic!("name alias")
    };
    assert_eq!(tag.runtime_class.as_ref().unwrap().name, "moNodeName_c");
    let mut valid = archive.contents().clone();
    valid
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.concept.feature_list.nodes[0]")
        .unwrap()
        .value = name.clone();
    assert!(archive.with_contents(valid, LIMITS).is_ok());
    let mut invalid = archive.contents().clone();
    invalid
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.walls[0].wall.viz_prop")
        .unwrap()
        .value = name;
    let error = archive.with_contents(invalid, LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains("moVisualProperties_c"));
    assert_eq!(archive.to_bytes().unwrap(), CUBE);
}

#[test]
fn generic_cobject_slots_do_not_turn_unknown_class_bodies_into_successful_opaque_records() {
    let archive = parsed(CUBE);
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.concept.feature_list.nodes[1]")
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("new child object")
    };
    let unknown = DisplayClassDescriptor {
        schema: 1,
        name: "unknownSceneNode_c".into(),
    };
    tag.tag = u16::MAX;
    tag.new_class = Some(unknown.clone());
    tag.runtime_class = Some(unknown);
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    assert!(error.cause.contains("unknownSceneNode_c"));
}

#[test]
fn exact_entry_and_object_budgets_cover_node_lists_wall_maps_and_new_cobject_loads() {
    for (bytes, entries, objects) in [(CUBE, 62, 49), (EMPTY, 53, 48), (OMITTED, 53, 46)] {
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            ..LIMITS
        };
        let archive = DisplayArchive::from_bytes(bytes, context(), exact).unwrap();
        for short in [
            DisplayArchiveLimits {
                maximum_entries: entries - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: objects - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(bytes, context(), short).is_err());
            assert!(DisplayArchive::new(archive.contents().clone(), context(), short).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), short)
                    .is_err()
            );
        }
    }
}

#[test]
fn scene_kind_edits_share_the_documented_body_and_nested_edits_match_independent_expected_wire() {
    let archive = parsed(CUBE);
    let mut contents = archive.contents().clone();
    set_integer(&mut contents, "root.scene_type", 2);
    assert_eq!(
        archive
            .with_contents(contents, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        SPHERE
    );
    let edited = edited_exposure(&archive);
    assert_eq!(edited.to_bytes().unwrap(), EDITED);
    let mut source = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    source
        .replace_stream("Contents/DisplayLists", CUBE.to_vec())
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
            source.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
    document
        .replace_stream(
            "Contents/DisplayLists",
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
        )
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), source_bytes);
}

#[test]
fn every_new_scene_prefix_invalid_wall_kind_and_impossible_count_rejects_before_publication() {
    for bytes in [CUBE, EMPTY] {
        for end in 6288..bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&bytes[..end], context(), LIMITS).is_err(),
                "cut {end}"
            );
        }
    }
    let archive = parsed(CUBE);
    for invalid in [-1, 0, 4, i32::MAX] {
        let mut contents = archive.contents().clone();
        set_integer(&mut contents, "root.scene.walls[0].type", invalid);
        let error = archive.with_contents(contents, LIMITS).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains(&invalid.to_string()));
    }
    for path in [
        "root.scene.walls.count",
        "root.scene.pw_scene_data.wall_info.count",
    ] {
        let mut contents = archive.contents().clone();
        set_integer(&mut contents, path, i32::MAX);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
}

#[test]
fn conditional_shape_and_equivalent_name_edits_cannot_bypass_full_stream_validation() {
    let archive = parsed(CUBE);
    let mut invalid = archive.contents().clone();
    set_integer(
        &mut invalid,
        "root.scene.concept.has_merged_feature_list",
        0,
    );
    assert!(archive.with_contents(invalid, LIMITS).is_err());
    let mut equivalent = archive.contents().clone();
    let field = equivalent
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.scene.concept.name.name")
        .unwrap();
    let DisplayFieldValue::Text(text) = &field.value else {
        panic!("node name")
    };
    field.value =
        DisplayFieldValue::Text(ArchiveString::new(text.text(), text.encoding()).unwrap());
    let short = DisplayArchiveLimits {
        maximum_bytes: CUBE.len() - 1,
        ..LIMITS
    };
    assert!(DisplayArchive::new(equivalent.clone(), context(), short).is_ok());
    assert!(archive.with_contents(equivalent, short).is_err());
}

#[test]
fn caller_mutation_and_concurrent_scene_edits_leave_published_inputs_isolated() {
    let mut bytes = CUBE.to_vec();
    let archive = parsed(&bytes);
    bytes.fill(0);
    let jobs: Vec<_> = (0..8)
        .map(|_| {
            let copy = archive.clone();
            std::thread::spawn(move || edited_exposure(&copy).to_bytes().unwrap())
        })
        .collect();
    for job in jobs {
        assert_eq!(job.join().unwrap(), EDITED);
    }
    assert_eq!(archive.to_bytes().unwrap(), CUBE);
}
