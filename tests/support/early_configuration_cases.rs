use super::*;

#[test]
fn early_configuration_boundaries_omit_unstored_records_and_visibility_prefixes_are_retained() {
    for case in cases() {
        let archive = parsed(case);
        let part = case.kind == DisplayDocumentKind::Part;
        assert_eq!(
            has_field(&archive, "root.configurations.config_count"),
            !part || case.version > 0x2b5e
        );
        if case.scenario % 128 != 4 {
            continue;
        }
        let inline = part && case.version < 0x2b67;
        assert_eq!(
            has_field(
                &archive,
                "root.configurations.configs[0].tessellation.bodies.count_word"
            ),
            inline
        );
        assert_eq!(
            has_field(&archive, "root.configurations.configs[0].tessellation"),
            part && !inline
        );
        assert_eq!(
            has_field(
                &archive,
                "root.configurations.view_objects.entries[0].config_id"
            ),
            case.version > if part { 0x2b61 } else { 0x2b6f }
        );
        assert_eq!(
            has_field(
                &archive,
                "root.configurations.annotation_view_visibility.count"
            ),
            case.version > if part { 0x2b7f } else { 0x2b77 }
        );
        assert_eq!(
            has_field(
                &archive,
                "root.configurations.annotation_view_visibility.unknown_legacy"
            ),
            !part && (0x2b78..0x2b7a).contains(&case.version)
        );
    }
}

fn edited_inline_configuration(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        "root.configurations.configs[0].tessellation.is_tessed_for_exploded_view",
        23,
    );
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn inline_configuration_edit_matches_independent_bytes_and_rejects_every_truncated_root() {
    let case = selected(0x2b5f, DisplayDocumentKind::Part, 132);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    let expected = selected(0x2b5f, DisplayDocumentKind::Part, 133);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_inline_configuration(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected.bytes);
        }
    });
    let edited = edited_inline_configuration(&archive);
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        "root.configurations.configs[0].tessellation.model_viz_props.count",
        i32::MAX,
    );
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
#[test]
fn precollector_part_component_and_facet_fields_obey_each_stored_boundary() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version <= 12000 && case.scenario == 128)
    {
        let archive = parsed(case);
        let part = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation"
        } else {
            "root.tessellation.components[0].comp_part_tess_data"
        };
        assert_eq!(
            has_field(&archive, &format!("{part}.config_id")),
            case.version > 0x2b96
        );
        assert_eq!(
            has_field(&archive, &format!("{part}.has_graphics_connect_ref_mgr")),
            case.version > 0x2b9d
        );
        assert_eq!(
            has_field(&archive, &format!("{part}.bbox")),
            case.version > 0x2ba3
        );
        assert_eq!(
            has_field(
                &archive,
                &format!("{part}.bodies[0].faces[0].face_string_empty")
            ),
            case.version > 0x2bcf
        );
        if case.kind == DisplayDocumentKind::Assembly {
            let viewer = "root.tessellation.components[0].viewer_comp_data";
            assert_eq!(
                has_field(&archive, &format!("{viewer}.seed_reference_id")),
                case.version > 0x2b9b
            );
        }
        for field in &archive.contents().fields {
            if let DisplayFieldValue::Facet(facet) = &field.value {
                assert_eq!(facet.face_string.is_some(), case.version > 0x2bcf);
                assert_eq!(facet.edge_points.is_some(), case.version > 0x2bd0);
            }
        }
    }
}

#[test]
fn inline_configuration_document_edit_reopens_and_authentically_reverts() {
    let case = selected(0x2b5f, DisplayDocumentKind::Part, 132);
    let initial = include_bytes!("../fixtures/blank-part.SLDPRT");
    let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
    let edited = edited_inline_configuration(&parsed(case));
    let mut document = bessemer::Document::from_bytes(initial).unwrap();
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
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
}
