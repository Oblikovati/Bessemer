//! First complete native stream checkpoint; unsupported corpus layouts remain explicit.
use bessemer::{Document, display_lists::*};
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, check_scalar_table, context, decode_document, set_integer};
const PART: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
#[test]
fn owned_native_part_decodes_every_byte_and_forces_exact_reconstruction() {
    let document = Document::from_bytes(PART).unwrap();
    let bytes = document.read_stream("Contents/DisplayLists").unwrap();
    let archive = DisplayArchive::from_bytes(&bytes, context(), LIMITS).unwrap();
    assert!(!archive.contents().fields.is_empty());
    assert_eq!(archive.to_bytes().unwrap(), bytes);
    let rebuilt = DisplayArchive::new(archive.contents().clone(), context(), LIMITS).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
}

#[test]
fn shared_font_alias_preserves_the_global_map_and_forces_exact_reconstruction() {
    let bytes = include_bytes!("fixtures/display-lists/shared-font-part.bin");
    let archive = DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap();
    let pointer = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == "root.line_font_mgr.line_fonts[1].value")
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &pointer.value else {
        panic!("tag");
    };
    assert_eq!(
        tag.target,
        DisplayObjectPointer::Reference(DisplayObjectId::new(6).unwrap())
    );
    assert_eq!(tag.runtime_class.as_ref().unwrap().name, "uiLFConfig_c");
    assert_eq!(
        DisplayArchive::new(archive.contents().clone(), context(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
}

#[test]
fn aliased_object_roles_cannot_silently_substitute_an_unrelated_runtime_class() {
    let bytes = include_bytes!("fixtures/display-lists/shared-font-part.bin");
    let archive = DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap();
    let mut contents = archive.contents().clone();
    let env = contents
        .fields
        .iter()
        .find(|field| field.path.as_ref() == "root.user_model_env")
        .unwrap();
    let DisplayFieldValue::ObjectTag(env) = &env.value else {
        panic!("tag");
    };
    let class = env.runtime_class.clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.line_font_mgr.line_fonts[1].value")
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("tag");
    };
    tag.tag = 2;
    tag.target = DisplayObjectPointer::Reference(DisplayObjectId::new(2).unwrap());
    tag.runtime_class = class;
    assert!(archive.with_contents(contents, LIMITS).is_err());
}

fn native_archive() -> DisplayArchive {
    decode_document(PART, DisplayDocumentKind::Part)
}

#[test]
fn absent_ink_rows_do_not_impose_a_physical_stride_on_configuration_dimensions() {
    let original = native_archive();
    let mut contents = original.contents().clone();
    set_integer(
        &mut contents,
        "root.markup_ink_views.configuration_count",
        123,
    );
    let edited = original.with_contents(contents.clone(), LIMITS).unwrap();
    let bytes = edited.to_bytes().unwrap();
    assert_eq!(bytes.len(), original.to_bytes().unwrap().len());
    assert_eq!(
        DisplayArchive::from_bytes(&bytes, context(), LIMITS)
            .unwrap()
            .contents(),
        &contents
    );
    set_integer(
        &mut contents,
        "root.markup_ink_views.configuration_count",
        -1,
    );
    assert!(original.with_contents(contents, LIMITS).is_err());
}

#[test]
fn full_stream_edit_reopens_in_container_and_reverts_without_mutating_source() {
    let original = native_archive();
    let mut contents = original.contents().clone();
    set_integer(
        &mut contents,
        "root.markup_ink_views.configuration_count",
        7,
    );
    let edited = original.with_contents(contents.clone(), LIMITS).unwrap();
    let mut document = Document::from_bytes(PART).unwrap();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let bytes = reopened.read_stream("Contents/DisplayLists").unwrap();
    assert_eq!(
        DisplayArchive::from_bytes(&bytes, context(), LIMITS)
            .unwrap()
            .contents(),
        &contents
    );
    for stream in document
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            document.read_stream(stream.name).unwrap()
        );
    }
    let reverted = edited
        .with_contents(original.contents().clone(), LIMITS)
        .unwrap();
    document
        .replace_stream("Contents/DisplayLists", reverted.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), PART);
    assert_eq!(
        original.to_bytes().unwrap(),
        native_archive().to_bytes().unwrap()
    );
}

#[test]
fn complete_stream_rejects_every_truncation_and_trailing_bytes() {
    let bytes = native_archive().to_bytes().unwrap();
    for end in 0..bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&bytes[..end], context(), LIMITS).is_err(),
            "cut {end}"
        );
    }
    let mut trailing = bytes;
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, context(), LIMITS).is_err());
}

#[test]
fn constructor_rejects_path_type_order_and_derived_pointer_metadata_changes() {
    let original = native_archive();
    let mut variants = vec![original.contents().clone(); 4];
    variants[0].fields[0].path = "root.invented".into();
    variants[1].fields[0].value = DisplayFieldValue::U32(1);
    variants[2].fields.swap(0, 1);
    let pointer = variants[3]
        .fields
        .iter_mut()
        .find(|field| matches!(field.value, DisplayFieldValue::ObjectTag(_)))
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut pointer.value else {
        panic!("object tag")
    };
    tag.target = DisplayObjectPointer::Null;
    for contents in variants {
        assert!(DisplayArchive::new(contents, context(), LIMITS).is_err());
    }
    assert_eq!(original.contents(), native_archive().contents());
}

#[test]
fn every_archive_budget_applies_to_read_construct_and_equivalent_edit() {
    let original = native_archive();
    let bytes = original.to_bytes().unwrap();
    let variants = [
        DisplayArchiveLimits {
            maximum_bytes: bytes.len() - 1,
            ..LIMITS
        },
        DisplayArchiveLimits {
            maximum_fields: original.contents().fields.len() - 1,
            ..LIMITS
        },
        DisplayArchiveLimits {
            maximum_objects: 0,
            ..LIMITS
        },
        DisplayArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
        DisplayArchiveLimits {
            maximum_entries: 0,
            ..LIMITS
        },
    ];
    for limits in variants {
        assert!(DisplayArchive::from_bytes(&bytes, context(), limits).is_err());
        assert!(DisplayArchive::new(original.contents().clone(), context(), limits).is_err());
        assert!(
            original
                .with_contents(original.contents().clone(), limits)
                .is_err()
        );
    }
}

#[test]
fn explicit_context_and_object_identity_boundaries_are_validated() {
    let bytes = native_archive().to_bytes().unwrap();
    let variants = [
        DisplayArchiveContext {
            version: DisplayVersion::new(0x1fae),
            ..context()
        },
        DisplayArchiveContext {
            kind: DisplayDocumentKind::Assembly,
            ..context()
        },
    ];
    for context in variants {
        assert!(DisplayArchive::from_bytes(&bytes, context, LIMITS).is_err());
    }
    for invalid in [0, 0x8000_0000, u32::MAX] {
        assert!(DisplayObjectId::new(invalid).is_err());
    }
    assert_eq!(
        DisplayObjectId::new(0x7fff_ffff).unwrap().value(),
        0x7fff_ffff
    );
}

#[test]
fn published_archives_are_owned_send_sync_and_concurrently_deterministic() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DisplayArchive>();
    assert_send_sync::<DisplayArchiveContents>();
    let original = native_archive();
    let mut input = original.to_bytes().unwrap();
    let decoded = DisplayArchive::from_bytes(&input, context(), LIMITS).unwrap();
    input.fill(0);
    let jobs: Vec<_> = (0..8)
        .map(|_| {
            let archive = decoded.clone();
            std::thread::spawn(move || {
                DisplayArchive::new(archive.contents().clone(), archive.context(), LIMITS)
                    .unwrap()
                    .to_bytes()
                    .unwrap()
            })
        })
        .collect();
    for job in jobs {
        assert_eq!(job.join().unwrap(), original.to_bytes().unwrap());
    }
}

#[test]
fn nested_view_packets_share_the_archive_collection_budget() {
    let original = native_archive();
    // The independent blank-part profile has 53 total entries, including its annotation view.
    let exact = DisplayArchiveLimits {
        maximum_entries: 53,
        ..LIMITS
    };
    let short = DisplayArchiveLimits {
        maximum_entries: 52,
        ..LIMITS
    };
    let bytes = original.to_bytes().unwrap();
    assert!(DisplayArchive::from_bytes(&bytes, context(), exact).is_ok());
    assert!(DisplayArchive::from_bytes(&bytes, context(), short).is_err());
    assert!(DisplayArchive::new(original.contents().clone(), context(), short).is_err());
    assert!(
        original
            .with_contents(original.contents().clone(), short)
            .is_err()
    );
}

#[test]
fn typed_fonts_lights_and_materials_match_independent_reference_semantics() {
    let archive = native_archive();
    let tables = [
        include_str!("fixtures/display-lists/blank-part-fonts.txt"),
        include_str!("fixtures/display-lists/blank-part-lights.txt"),
        include_str!("fixtures/display-lists/blank-part-materials.txt"),
    ];
    let checked: usize = tables
        .into_iter()
        .map(|table| check_scalar_table(&archive, table))
        .sum();
    assert_eq!(checked, 535);
}
