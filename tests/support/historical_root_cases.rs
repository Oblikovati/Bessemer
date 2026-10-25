use super::*;

#[test]
fn historical_scene_wall_pen_and_component_members_obey_exact_version_gates() {
    for case in cases().into_iter().filter(|case| case.scenario == 192) {
        let archive = parsed(case);
        let gates = [
            ("root.scene.pw_scene_data.use_sky_shader", 0x17dc),
            ("root.scene.flatten_floor", 0x1827),
            ("root.scene.environment_size", 0x1827),
            ("root.scene.flattened_floor_height", 0x1841),
            ("root.scene.accurate_environment_lighting", 0x185e),
            ("root.scene.flatten_floor_xform", 0x186c),
            ("root.scene.walls[1].wall.radius", 0x1827),
            ("root.scene.walls[1].wall.unknown_38", 0x1841),
            ("root.scene.walls[2].wall.radius", 0x1827),
            (
                "root.views[0].display_items.items[3].data.unknown_20",
                0x1859,
            ),
        ];
        for (path, threshold) in gates {
            assert_eq!(
                has_field(&archive, path),
                case.version > threshold,
                "{} {:?} {path}",
                case.version,
                case.kind
            );
        }
    }
}

#[test]
fn historical_root_edits_match_independent_bytes_and_own_sources() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(6000, kind, 192);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(6000, kind, 193).bytes);
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
fn historical_layouts_reject_every_truncated_prefix_without_publishing_partial_results() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(6000, kind, 192);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{kind:?} prefix {end}"
            );
        }
    }
}

#[test]
fn historical_component_state_fields_and_parallel_edits_are_context_local() {
    for version in [6000, 0x17f3, 0x17f4, 0x17f5, 0x17f6, 0x183d, 0x183e, 6999] {
        let archive = parsed(selected(version, DisplayDocumentKind::Assembly, 192));
        for (label, threshold) in [
            ("view_comp_type", 0x17f3),
            ("asm_feat_type", 0x17f5),
            ("belt_has_part", 0x183d),
        ] {
            let path = format!("root.tessellation.components[0].viewer_comp_data.{label}");
            assert_eq!(
                has_field(&archive, &path),
                version > threshold,
                "{version} {path}"
            );
        }
    }
    let archive = parsed(selected(6000, DisplayDocumentKind::Assembly, 192));
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(
                worker.join().unwrap(),
                selected(6000, DisplayDocumentKind::Assembly, 193).bytes
            );
        }
    });
}
