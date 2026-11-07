use super::*;

#[test]
fn exploded_view_and_background_fit_fields_follow_exact_early_gates() {
    for case in cases().into_iter().filter(|case| case.scenario == 240) {
        let archive = parsed(case);
        let part = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation"
        } else {
            "root.tessellation.components[0].comp_part_tess_data"
        };
        assert_eq!(
            has_field(&archive, &format!("{part}.is_tessed_for_exploded_view")),
            case.version > 0x12fc
        );
        assert_eq!(
            has_field(
                &archive,
                "root.scene.pw_scene_data.bg_fit_to_sw_graphics_area"
            ),
            case.version > 0x1310
        );
    }
}

#[test]
fn early_scene_edits_reopen_match_independent_bytes_and_isolate_concurrent_sources() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(4850, kind, 240);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        let expected = selected(4850, kind, 241).bytes;
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), expected);
            }
        });
        let reopened = DisplayArchive::from_bytes(expected, case_context(case), LIMITS).unwrap();
        assert_eq!(reopened.contents(), edited_ambient(&archive).contents());
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn each_new_gate_rejects_every_truncation_and_cross_version_field_shapes() {
    for version in [4850, 0x12fc, 0x12fd, 0x1310, 0x1311] {
        let case = selected(version, DisplayDocumentKind::Part, 240);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
    }
    for (before, after) in [(0x12fc, 0x12fd), (0x1310, 0x1311)] {
        let old = selected(before, DisplayDocumentKind::Part, 240);
        let new = selected(after, DisplayDocumentKind::Part, 240);
        assert!(
            DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
        );
        assert!(
            DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
        );
    }
}
