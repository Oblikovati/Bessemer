use super::*;

#[test]
fn transient_text_and_geometry_members_obey_their_individual_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario == 224) {
        let archive = parsed(case);
        let root = "root.views[0].display_items.items";
        for (suffix, present) in [
            (
                "[2].data.unknown_v1448",
                (0x1448..0x144a).contains(&case.version),
            ),
            ("[3].data.unknown_60", case.version > 0x13ec),
            ("[4].data.unknown_20", case.version > 0x146e),
            ("[4].data.unknown_28", case.version > 0x146e),
            ("[4].data.unknown_30", case.version >= 0x1473),
        ] {
            assert_eq!(
                has_field(&archive, &format!("{root}{suffix}")),
                present,
                "{} {suffix}",
                case.version
            );
        }
        for (label, threshold) in [
            ("default_width", 0x1415),
            ("default_height", 0x1415),
            ("lxp_filename", 0x1421),
            ("round_sharp_edges", 0x1462),
        ] {
            assert_eq!(
                archive.contents().fields.iter().any(|field| field
                    .path
                    .ends_with(&format!(".material.rv_app_props.{label}"))),
                case.version > threshold,
                "{} {:?} {label}",
                case.version,
                case.kind
            );
        }
        assert_eq!(
            has_field(&archive, "root.scene.concept.flags2"),
            case.version > 0x143a
        );
        assert_eq!(
            has_field(&archive, "root.visual_properties.unknown_78"),
            case.version > 0x144b
        );
    }
}

#[test]
fn earliest_appearance_edits_preserve_owned_sources_and_reopen_independent_bytes() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(5000, kind, 224);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(5000, kind, 225).bytes);
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
        assert_eq!(
            DisplayArchive::from_bytes(&edited.to_bytes().unwrap(), case_context(case), LIMITS)
                .unwrap()
                .contents(),
            edited.contents()
        );
    }
}

#[test]
fn transient_text_and_old_face_layouts_reject_every_truncated_prefix() {
    for version in [5000, 0x1448, 0x1449] {
        let case = selected(version, DisplayDocumentKind::Part, 224);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
    }
}
