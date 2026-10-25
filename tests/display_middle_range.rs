//! Complete part and assembly roots pin the 14000..18000 material/component/facet gates.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as RangeCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<RangeCase> {
    let mut cases =
        complete_cases::decode(include_bytes!("fixtures/display-lists/middle-range.bin"));
    cases.extend(complete_cases::decode(include_bytes!(
        "fixtures/display-lists/lower-range.bin"
    )));
    cases.extend(complete_cases::decode(include_bytes!(
        "fixtures/display-lists/transition-range-part.bin"
    )));
    cases.extend(complete_cases::decode(include_bytes!(
        "fixtures/display-lists/transition-range-assembly.bin"
    )));
    cases
}

fn selected(version: u32, kind: DisplayDocumentKind, scenario: u8) -> RangeCase {
    cases()
        .into_iter()
        .find(|case| case.version == version && case.kind == kind && case.scenario == scenario)
        .unwrap()
}

fn case_context(case: RangeCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: RangeCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn all_middle_range_roots_match_independent_material_scene_component_fields_and_bytes() {
    assert_eq!(cases().len(), 160);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
        trace.extend(item_trace::trace(&archive));
        assert_eq!(trace, case.expected, "{} {:?}", case.version, case.kind);
        fields += case
            .expected
            .iter()
            .filter(|value| **value == b'\n')
            .count();
        fields += archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 49626);
}

fn has_field(archive: &DisplayArchive, path: &str) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.as_ref() == path)
}

#[test]
fn version_gates_omit_fields_instead_of_synthesizing_defaults() {
    for case in cases() {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, "root.visual_properties.has_dspbr"),
            case.version > 0x3f08
        );
        assert_eq!(
            has_field(&archive, "root.scene.hdr_exposure"),
            case.version > 0x42ec
        );
        assert!(!has_field(&archive, "root.scene.field_of_view"));
        let assembly = case.kind == DisplayDocumentKind::Assembly;
        assert_eq!(
            has_field(&archive, "root.envelope_system_options.unknown_color"),
            assembly && case.version > 0x42da
        );
        let prefix = "root.tessellation.components[0].viewer_comp_data";
        assert_eq!(
            has_field(&archive, &format!("{prefix}.routing_assembly")),
            assembly && case.version > 0x42e7
        );
        assert_eq!(
            has_field(&archive, &format!("{prefix}.top_level_speed_pak")),
            assembly && case.version >= 0x42f3
        );
        assert_eq!(
            archive
                .contents()
                .fields
                .iter()
                .filter(|field| matches!(field.value, DisplayFieldValue::Facet(_)))
                .count(),
            2
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
    assert!(matches!(field.value, DisplayFieldValue::F64(_)));
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_ambient_edit_reopens_in_each_document_kind_and_reverts() {
    for (version, scenario, kind) in [
        (14000, 32, DisplayDocumentKind::Part),
        (14000, 32, DisplayDocumentKind::Assembly),
        (16000, 0, DisplayDocumentKind::Part),
        (16000, 0, DisplayDocumentKind::Assembly),
    ] {
        let case = selected(version, kind, scenario);
        let archive = parsed(case);
        let edited = edited_ambient(&archive);
        assert_eq!(
            edited.to_bytes().unwrap(),
            selected(version, kind, scenario + 1).bytes
        );
        assert_eq!(
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        let initial: &[u8] = if kind == DisplayDocumentKind::Part {
            include_bytes!("fixtures/blank-part.SLDPRT")
        } else {
            include_bytes!("fixtures/blank-assembly.SLDASM")
        };
        let native = archive_support::decode_document(initial, kind);
        let original = bessemer::Document::from_bytes(initial).unwrap();
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
}

#[test]
fn full_root_budgets_include_both_inline_facets_at_the_vertex_layout_boundary() {
    for case in cases() {
        let archive = parsed(case);
        let mut entries = if case.kind == DisplayDocumentKind::Part {
            149
        } else {
            154
        } - if case.version <= 0x3ee7 { 8 } else { 0 };
        if case.scenario >= 32 {
            entries -= if case.version <= 0x3751 { 2 } else { 0 };
            entries -= if case.version <= 0x3b28 { 16 } else { 0 };
            if case.kind == DisplayDocumentKind::Assembly {
                entries += if case.version <= 0x3b1b {
                    0
                } else if case.version < 0x3b23 {
                    1
                } else {
                    2
                };
            }
        }
        if case.scenario >= 64 {
            entries += 12
                + match case.scenario % 64 {
                    2 => 99,
                    3 => 7,
                    _ => 0,
                };
        }
        let objects = if case.kind == DisplayDocumentKind::Part {
            52
        } else {
            54
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
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(
                DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err(),
                "{} {:?} limits {limits:?}",
                case.version,
                case.kind
            );
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn truncated_roots_extra_bytes_and_unproven_contexts_do_not_publish_partial_results() {
    for (version, scenario) in [(13000, 64), (14000, 32), (16000, 0)] {
        let case = selected(version, DisplayDocumentKind::Part, scenario);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "prefix {end}"
            );
        }
        let archive = parsed(case);
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, "root.view_count", i32::MAX);
        assert!(archive.with_contents(contents, LIMITS).is_err());
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
    }
    let case = selected(16000, DisplayDocumentKind::Part, 0);
    for version in [0, 2010] {
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        let error = DisplayArchive::from_bytes(case.bytes, context, LIMITS).unwrap_err();
        assert!(error.to_string().contains("Unsupported"));
    }
}

#[test]
fn published_middle_range_roots_own_sources_and_parallel_edits_match_independent_bytes() {
    for (version, scenario) in [(13000, 64), (14000, 32), (16000, 0)] {
        let case = selected(version, DisplayDocumentKind::Assembly, scenario);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(
                    worker.join().unwrap(),
                    selected(version, DisplayDocumentKind::Assembly, scenario + 1).bytes
                );
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn lower_range_texture_thread_face_and_facet_gates_omit_unstored_fields() {
    for case in cases().into_iter().filter(|case| case.scenario >= 32) {
        let archive = parsed(case);
        assert_eq!(
            has_field(
                &archive,
                "root.visual_properties.material.texture.for_c_thread"
            ),
            case.version > 0x372a
        );
        assert_eq!(
            has_field(&archive, "root.visual_properties.material.has_texture2"),
            case.version > 0x372a
        );
        let part = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation"
        } else {
            "root.tessellation.components[0].comp_part_tess_data"
        };
        assert_eq!(
            has_field(&archive, &format!("{part}.show_c_thread")),
            case.version > 0x3751
        );
        assert_eq!(
            has_field(&archive, &format!("{part}.bodies[0].faces[0].unknown_78")),
            case.version >= 0x3743
        );
        for field in &archive.contents().fields {
            if let DisplayFieldValue::Facet(facet) = &field.value {
                assert_eq!(
                    facet.auxiliary_texture_coords.is_some(),
                    case.version > 0x3751
                );
                assert_eq!(
                    facet.save_thread_texture_coords.is_some(),
                    case.version > 0x3757
                );
                assert_eq!(facet.late_stamps.is_some(), case.version > 0x3b0d);
                assert_eq!(facet.edges.is_some(), case.version > 0x3b28);
            }
        }
    }
}

#[test]
fn fixed_component_tessellation_pairs_do_not_serialize_the_implicit_identifier() {
    let prefix = "root.tessellation.components[0].viewer_comp_data";
    for version in [14000, 0x3b1b, 0x3b1c, 0x3b1d, 0x3b22, 0x3b23, 15999, 16000] {
        let archive = parsed(selected(version, DisplayDocumentKind::Assembly, 32));
        assert_eq!(
            has_field(&archive, &format!("{prefix}.display_state_name")),
            version > 0x3b1c
        );
        assert_eq!(
            has_field(
                &archive,
                &format!("{prefix}.viewer_comp_tess_numbers.count")
            ),
            version >= 0x3b23
        );
        assert_eq!(
            has_field(
                &archive,
                &format!("{prefix}.viewer_comp_tess_numbers[0][0]")
            ),
            version >= 0x3b23
        );
        if 0x3b1b < version && version < 0x3b23 {
            let oracle = format!(
                "{prefix}.viewer_comp_tess_numbers[0][1]=i:-91\n{prefix}.viewer_comp_tess_numbers[0][2]=i:92\n"
            );
            assert_eq!(archive_support::check_scalar_table(&archive, &oracle), 2);
        }
        if version >= 0x3b23 {
            let oracle = format!(
                "{prefix}.viewer_comp_tess_numbers.count=i:2\n{prefix}.viewer_comp_tess_numbers[0][0]=i:-1\n{prefix}.viewer_comp_tess_numbers[1][0]=i:-4\n"
            );
            assert_eq!(archive_support::check_scalar_table(&archive, &oracle), 3);
        }
    }
}

#[test]
fn transition_appearance_and_annotation_fields_are_omitted_at_the_documented_bounds() {
    for case in cases().into_iter().filter(|case| case.scenario >= 64) {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, "root.orig_font_scale"),
            case.version > 0x3331
        );
        assert_eq!(
            has_field(&archive, "root.visual_properties.material.plm_id"),
            case.version > 0x3368
        );
        assert_eq!(
            has_field(
                &archive,
                "root.visual_properties.material.rv_app_props.unknown_symbol.strings[0]"
            ),
            case.kind == DisplayDocumentKind::Part && case.version > 0x3347
        );
        let part = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation"
        } else {
            "root.tessellation.components[0].comp_part_tess_data"
        };
        assert_eq!(
            has_field(&archive, &format!("{part}.bodies[0].faces[0].face_pers_id")),
            case.version > 0x336b
        );
        let items = "root.views[0].display_items.items";
        assert_eq!(
            has_field(
                &archive,
                &format!("{items}[0].data.view_names_3d_with_scale")
            ),
            case.version > 0x3332
        );
        assert_eq!(
            has_field(
                &archive,
                &format!("{items}[1].data.edw_annot_data[0].unknown_s4")
            ),
            case.version > 0x332b
        );
        assert_eq!(
            has_field(&archive, &format!("{items}[1].data.id_string")),
            case.version >= 0x35e9
        );
    }
}

#[test]
fn a_hundred_minimal_ink_rows_fit_the_older_five_byte_layout() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let archive = parsed(selected(13000, kind, 66));
        let expected = "root.markup_ink_views.view_count=i:100\nroot.markup_ink_views.configuration_count=i:0\nroot.markup_ink_views.views[99].unknown_id=i:0\n";
        assert_eq!(archive_support::check_scalar_table(&archive, expected), 3);
        assert!(!has_field(
            &archive,
            "root.markup_ink_views.views[99].unknown_string"
        ));
        let before = parsed(selected(0x332b, kind, 67));
        let after = parsed(selected(0x332c, kind, 67));
        assert!(!has_field(
            &before,
            "root.markup_ink_views.views[1].unknown_string"
        ));
        assert!(has_field(
            &after,
            "root.markup_ink_views.views[1].unknown_string"
        ));
    }
}
