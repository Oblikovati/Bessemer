//! Populated shared-map bodies and facets, with explicit cross-stream face context.
use bessemer::{
    Document,
    display_lists::*,
    tessellation::{ArchiveVersion, FacetArchive, FacetArchiveMode, TessellationContext},
};
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, check_scalar_table, context, decode_document, set_integer};
const INLINE: &[u8] = include_bytes!("fixtures/display-lists/inline-shaded-part.bin");
const EXTERNAL: &[u8] = include_bytes!("fixtures/display-lists/external-shaded-assembly.bin");

fn external_context() -> DisplayArchiveContext {
    DisplayArchiveContext {
        kind: DisplayDocumentKind::Assembly,
        face_tessellation_components: std::sync::Arc::new(["Assem1".to_owned()].into()),
        ..context()
    }
}

#[test]
fn inline_facets_reuse_the_complete_shared_codec_without_normalizing_bits_or_graph_tags() {
    let archive = DisplayArchive::from_bytes(INLINE, context(), LIMITS).unwrap();
    let facets: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            DisplayFieldValue::Facet(value) => Some(value.as_ref()),
            _ => None,
        })
        .collect();
    let expected = FacetArchive::from_bytes(
        include_bytes!("fixtures/tessellation/facet-4a38-1.bin"),
        TessellationContext {
            archive_version: ArchiveVersion::new(19000),
            mode: FacetArchiveMode::Full,
        },
        bessemer::tessellation::TessellationLimits {
            maximum_components: 0,
            maximum_faces: 1,
            maximum_array_elements: 1000,
        },
    )
    .unwrap();
    assert_eq!(facets, vec![expected.contents(), expected.contents()]);
    assert_eq!(
        DisplayArchive::new(archive.contents().clone(), context(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        INLINE
    );
    let oracle = "root.tessellation.bodies[0].faces[0].unknown_58[1][0]=i:-3\nroot.tessellation.bodies[0].faces[1].face_pers_id=i:1\nroot.tessellation.bodies[0].body_prop_infos[0].value.unknown_8=i:-8\nroot.tessellation.bodies[0].unknown_68=i:-104\n";
    assert_eq!(check_scalar_table(&archive, oracle), 4);
}

#[test]
fn external_facets_keep_component_and_order_and_require_the_declared_context() {
    let archive = DisplayArchive::from_bytes(EXTERNAL, external_context(), LIMITS).unwrap();
    let sources: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            DisplayFieldValue::ExternalFacet {
                component_id,
                index,
            } => Some((component_id.text(), *index)),
            _ => None,
        })
        .collect();
    assert_eq!(sources, [("Assem1", 0), ("Assem1", 1)]);
    assert!(
        !archive
            .contents()
            .fields
            .iter()
            .any(|field| matches!(field.value, DisplayFieldValue::Facet(_)))
    );
    assert_eq!(
        DisplayArchive::new(archive.contents().clone(), external_context(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        EXTERNAL
    );
    let missing = DisplayArchiveContext {
        kind: DisplayDocumentKind::Assembly,
        ..context()
    };
    assert!(DisplayArchive::from_bytes(EXTERNAL, missing, LIMITS).is_err());
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| matches!(field.value, DisplayFieldValue::ExternalFacet { .. }))
        .unwrap();
    let DisplayFieldValue::ExternalFacet { index, .. } = &mut field.value else {
        panic!("external face")
    };
    *index = 42;
    assert!(archive.with_contents(contents, LIMITS).is_err());
}

#[test]
fn all_facet_array_entries_share_the_outer_archive_budget() {
    // Each independently generated facet contains 37 variable entries; the root adds 60.
    for (bytes, context, total) in [(INLINE, context(), 134), (EXTERNAL, external_context(), 65)] {
        let exact = DisplayArchiveLimits {
            maximum_entries: total,
            ..LIMITS
        };
        let short = DisplayArchiveLimits {
            maximum_entries: total - 1,
            ..LIMITS
        };
        let archive = DisplayArchive::from_bytes(bytes, context.clone(), exact).unwrap();
        assert!(DisplayArchive::from_bytes(bytes, context.clone(), short).is_err());
        assert!(DisplayArchive::new(archive.contents().clone(), context, short).is_err());
        assert!(
            archive
                .with_contents(archive.contents().clone(), short)
                .is_err()
        );
    }
}

#[test]
fn facet_edits_are_isolated_reopen_in_a_container_and_revert_all_original_bytes() {
    let original = DisplayArchive::from_bytes(INLINE, context(), LIMITS).unwrap();
    let mut contents = original.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| matches!(field.value, DisplayFieldValue::Facet(_)))
        .unwrap();
    let DisplayFieldValue::Facet(facet) = &mut field.value else {
        panic!("facet")
    };
    facet.container_processed = 17;
    set_integer(
        &mut contents,
        "root.tessellation.bodies[0].unknown_20",
        -123,
    );
    let edited = original.with_contents(contents.clone(), LIMITS).unwrap();
    let mut source = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    source
        .replace_stream("Contents/DisplayLists", INLINE.to_vec())
        .unwrap();
    let source_bytes = source.to_bytes().unwrap();
    let mut document = Document::from_bytes(&source_bytes).unwrap();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let bytes = document.to_bytes().unwrap();
    assert_eq!(
        decode_document(&bytes, DisplayDocumentKind::Part).contents(),
        &contents
    );
    let reopened = Document::from_bytes(&bytes).unwrap();
    for stream in source
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            source.read_stream(stream.name).unwrap()
        );
    }
    assert_eq!(original.to_bytes().unwrap(), INLINE);
    let reverted = edited
        .with_contents(original.contents().clone(), LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap();
    document
        .replace_stream("Contents/DisplayLists", reverted)
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), source_bytes);
}

#[test]
fn facet_shape_and_preserved_string_byte_limits_are_revalidated_before_publication() {
    let archive = DisplayArchive::from_bytes(INLINE, context(), LIMITS).unwrap();
    let mut invalid = archive.contents().clone();
    let field = invalid
        .fields
        .iter_mut()
        .find(|field| matches!(field.value, DisplayFieldValue::Facet(_)))
        .unwrap();
    let DisplayFieldValue::Facet(facet) = &mut field.value else {
        panic!("facet")
    };
    facet.face_string = None;
    assert!(archive.with_contents(invalid, LIMITS).is_err());
    let mut equivalent = archive.contents().clone();
    let field = equivalent
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.tessellation.config_name")
        .unwrap();
    let DisplayFieldValue::Text(text) = &field.value else {
        panic!("configuration name")
    };
    field.value =
        DisplayFieldValue::Text(ArchiveString::new(text.text(), text.encoding()).unwrap());
    let short = DisplayArchiveLimits {
        maximum_bytes: INLINE.len() - 1,
        ..LIMITS
    };
    assert!(DisplayArchive::new(equivalent.clone(), context(), short).is_ok());
    assert!(archive.with_contents(equivalent, short).is_err());
}

#[test]
fn external_context_and_embedded_facets_remain_owned_send_sync_and_deterministic() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DisplayArchiveContext>();
    assert_send_sync::<DisplayArchive>();
    assert_send_sync::<DisplayArchiveContents>();
    assert_send_sync::<DisplayField>();
    assert_send_sync::<DisplayFieldValue>();
    assert_send_sync::<DisplayArchiveLimits>();
    assert_send_sync::<DisplayDocumentKind>();
    assert_send_sync::<DisplayClassDescriptor>();
    assert_send_sync::<DisplayObjectId>();
    assert_send_sync::<DisplayObjectPointer>();
    assert_send_sync::<DisplayObjectTag>();
    let mut supplied = external_context();
    let archive = DisplayArchive::from_bytes(EXTERNAL, supplied.clone(), LIMITS).unwrap();
    std::sync::Arc::make_mut(&mut supplied.face_tessellation_components).clear();
    assert!(
        archive
            .context()
            .face_tessellation_components
            .contains("Assem1")
    );
    rebuild_concurrently(&archive, EXTERNAL);
    let inline = DisplayArchive::from_bytes(INLINE, context(), LIMITS).unwrap();
    rebuild_concurrently(&inline, INLINE);
}

#[test]
fn facet_preflight_checks_the_remaining_outer_byte_budget_before_encoding_a_payload() {
    let archive = DisplayArchive::from_bytes(INLINE, context(), LIMITS).unwrap();
    let facet = include_bytes!("fixtures/tessellation/facet-4a38-1.bin");
    let start = INLINE
        .windows(facet.len())
        .position(|span| span == facet)
        .unwrap();
    let short = DisplayArchiveLimits {
        maximum_bytes: start + facet.len() - 1,
        ..LIMITS
    };
    let error = DisplayArchive::new(archive.contents().clone(), context(), short).unwrap_err();
    assert_eq!(
        error.cause,
        format!(
            "encoded facet bytes {}; expected at most {}",
            facet.len(),
            facet.len() - 1
        )
    );
    assert!(
        archive
            .with_contents(archive.contents().clone(), short)
            .is_err()
    );
}

#[test]
fn every_shaded_stream_truncation_and_trailing_byte_fails_without_partial_publication() {
    for (bytes, context) in [(INLINE, context()), (EXTERNAL, external_context())] {
        for end in 0..bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&bytes[..end], context.clone(), LIMITS).is_err(),
                "cut {end}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, context, LIMITS).is_err());
    }
}

#[test]
fn external_faces_share_one_owned_component_id_allocation() {
    let archive = DisplayArchive::from_bytes(EXTERNAL, external_context(), LIMITS).unwrap();
    let ids: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            DisplayFieldValue::ExternalFacet { component_id, .. } => Some(component_id.text()),
            _ => None,
        })
        .collect();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0].len(), 6);
    assert_eq!(
        ids[0].as_ptr(),
        ids[1].as_ptr(),
        "component text must not be copied per omitted facet"
    );
    let mut cloned = archive.contents().clone();
    let field = cloned
        .fields
        .iter_mut()
        .find(|field| matches!(field.value, DisplayFieldValue::ExternalFacet { .. }))
        .unwrap();
    let DisplayFieldValue::ExternalFacet { component_id, .. } = &mut field.value else {
        panic!("external face")
    };
    *std::sync::Arc::make_mut(component_id) =
        ArchiveString::new("changed", TextEncoding::Utf16).unwrap();
    assert!(archive.with_contents(cloned, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), EXTERNAL);
    assert_eq!(ids, ["Assem1", "Assem1"]);
}

fn rebuild_concurrently(archive: &DisplayArchive, expected: &[u8]) {
    let jobs: Vec<_> = (0..8)
        .map(|_| {
            let copy = archive.clone();
            std::thread::spawn(move || {
                DisplayArchive::new(copy.contents().clone(), copy.context(), LIMITS)
                    .unwrap()
                    .to_bytes()
                    .unwrap()
            })
        })
        .collect();
    for job in jobs {
        assert_eq!(job.join().unwrap(), expected);
    }
    assert_eq!(archive.to_bytes().unwrap(), expected);
}
