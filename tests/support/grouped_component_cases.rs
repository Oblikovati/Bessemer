use super::*;

#[test]
fn grouped_folder_tooltip_appearance_and_annotation_fields_follow_their_stored_bounds() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version <= 10000 && case.scenario == 128)
    {
        let archive = parsed(case);
        for (path, boundary) in [
            ("root.visual_properties.unknown_7c", 0x2419),
            ("root.scene.concept.tool_tip_name", 0x23cd),
            (
                "root.views[0].display_items.items[0].data.always_2d",
                0x238c,
            ),
            (
                "root.views[0].display_items.items[1].data.edw_annot_data[0].unknown_s3",
                0x23fe,
            ),
        ] {
            assert_eq!(
                has_field(&archive, path),
                case.version > boundary,
                "{} {path}",
                case.version
            );
        }
        if case.kind == DisplayDocumentKind::Assembly {
            assert_eq!(
                has_field(
                    &archive,
                    "root.tessellation.components[0].viewer_comp_data.grouped_folder_comp"
                ),
                case.version > 0x23e4
            );
        }
    }
}

#[test]
fn assembly_configurations_omit_the_early_visual_pointer_without_inventing_a_null_tag() {
    for version in [0x2404, 0x240e, 0x240f, 0x2419, 0x241a] {
        let case = selected(version, DisplayDocumentKind::Assembly, 132);
        let archive = parsed(case);
        assert!(has_field(
            &archive,
            "root.configurations.configs[0].shd_data.count_word"
        ));
        assert_eq!(
            has_field(&archive, "root.configurations.configs[0].viz_prop"),
            version > 0x240e
        );
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}
#[test]
fn older_footer_omission_still_rejects_every_prefix_and_preserves_independent_edits() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(9000, kind, 128);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{kind:?} prefix {end}"
            );
        }
        let archive = parsed(case);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(9000, kind, 129).bytes);
        assert_eq!(
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}
