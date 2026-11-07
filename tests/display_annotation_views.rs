//! Independent annotation-view boundaries, retained raw fields and immutable result contracts.
use bessemer::display_lists::*;
#[path = "support/display_annotation.rs"]
mod support;
use support::*;

#[test]
fn every_annotation_boundary_and_raw_flag_matches_independent_vectors_and_exact_reconstruction() {
    for (version, scenario, bytes) in vectors() {
        let archive = AnnotationViews::from_bytes(bytes, version, LIMITS).unwrap();
        assert_eq!(archive.version(), version);
        assert_eq!(archive.to_bytes().unwrap(), bytes);
        assert_eq!(
            AnnotationViews::new(archive.contents().clone(), version, LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
        if version.value() <= 0x1c39 {
            assert!(archive.contents().is_none());
            continue;
        }
        let contents = archive.contents().as_ref().unwrap();
        assert_eq!(contents.views.len(), scenario as usize);
        assert_eq!(
            contents.unknown_bitset,
            if version.value() > 0x1fa8 {
                Some([i32::MIN, i32::MAX])
            } else {
                None
            }
        );
        for view in &contents.views {
            check_view(view, version, scenario);
        }
    }
}

fn check_view(view: &AnnotationView, version: DisplayVersion, scenario: u8) {
    assert_eq!(view.view_name.text(), "鋼\0λ");
    assert_eq!(view.view_name.encoding(), TextEncoding::Utf16);
    assert_eq!(
        (view.view_id.value(), view.is_shown, view.is_current),
        (-99, 2, -1)
    );
    assert_eq!(
        view.unknown_flag,
        if version.value() > 0x1c40 {
            Some(-17)
        } else {
            None
        }
    );
    assert_eq!(view.unknown_name2.text(), "second\0é");
    assert_eq!(view.unknown_string.text(), "retained é");
    assert_eq!(
        (
            view.model_to_view.has_matrix,
            view.model_to_view.flags,
            view.pan_zoom.flags
        ),
        (2, 0x2f, 0x4a)
    );
    assert_eq!(view.section_planes.len(), if scenario == 2 { 2 } else { 0 });
    for plane in &view.section_planes {
        assert_eq!(
            plane.root_point.map(|value| value.bits()),
            [0, 0x8000000000000000, 0x7ff0000000000000]
        );
        assert_eq!(plane.normal[2].bits(), 0x7ff0000000000042);
        assert_eq!(plane.unknown_double.bits(), 1);
        assert_eq!(plane.unknown_integer, -23);
    }
}

#[test]
fn every_truncation_trailing_byte_and_invalid_count_rejects_before_publication() {
    for (version, _, bytes) in vectors() {
        for length in 0..bytes.len() {
            assert!(
                AnnotationViews::from_bytes(&bytes[..length], version, LIMITS).is_err(),
                "{version:?}: {length}"
            );
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(AnnotationViews::from_bytes(&extra, version, LIMITS).is_err());
    }
    for count in [-1, i32::MAX] {
        assert!(
            AnnotationViews::from_bytes(&count.to_le_bytes(), DisplayVersion::new(19000), LIMITS)
                .is_err()
        );
    }
    let mut invalid = 1i32.to_le_bytes().to_vec();
    invalid.push(0);
    invalid.extend_from_slice(&[0; 16]);
    invalid.push(0);
    invalid.extend_from_slice(&[0; 68]);
    invalid.push(0);
    invalid.extend_from_slice(&(-1i32).to_le_bytes());
    invalid.extend_from_slice(&[0; 8]);
    assert!(AnnotationViews::from_bytes(&invalid, DisplayVersion::new(19000), LIMITS).is_err());
}

#[test]
fn collection_level_bitsets_and_per_view_flags_obey_their_own_version_gates() {
    for (version, scenario, bytes) in vectors() {
        let original = AnnotationViews::from_bytes(bytes, version, LIMITS).unwrap();
        if version.value() <= 0x1c39 {
            assert!(
                AnnotationViews::new(
                    Some(AnnotationViewContents {
                        views: Vec::new(),
                        unknown_bitset: None
                    }),
                    version,
                    LIMITS
                )
                .is_err()
            );
            continue;
        }
        assert!(original.with_contents(None, LIMITS).is_err());
        let mut contents = original.contents().clone().unwrap();
        contents.unknown_bitset = if version.value() > 0x1fa8 {
            None
        } else {
            Some([0; 2])
        };
        assert!(original.with_contents(Some(contents), LIMITS).is_err());
        if scenario != 0 {
            let mut contents = original.contents().clone().unwrap();
            contents.views[0].unknown_flag = if version.value() > 0x1c40 {
                None
            } else {
                Some(0)
            };
            assert!(original.with_contents(Some(contents), LIMITS).is_err());
        }
        assert_eq!(original.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn exact_physical_view_and_cumulative_plane_limits_apply_to_all_operation_paths() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(version, scenario, _)| version.value() == 19000 && *scenario == 2)
        .unwrap();
    let original = AnnotationViews::from_bytes(bytes, version, LIMITS).unwrap();
    let exact = DisplayViewLimits {
        maximum_bytes: bytes.len(),
        maximum_views: 2,
        maximum_nested_entries: 4,
    };
    assert!(AnnotationViews::new(original.contents().clone(), version, exact).is_ok());
    for limits in [
        DisplayViewLimits {
            maximum_bytes: bytes.len() - 1,
            ..exact
        },
        DisplayViewLimits {
            maximum_views: 1,
            ..exact
        },
        DisplayViewLimits {
            maximum_nested_entries: 3,
            ..exact
        },
    ] {
        assert!(AnnotationViews::from_bytes(bytes, version, limits).is_err());
        assert!(AnnotationViews::new(original.contents().clone(), version, limits).is_err());
        assert!(
            original
                .with_contents(original.contents().clone(), limits)
                .is_err()
        );
    }
    let mut canonical = original.contents().clone().unwrap();
    for view in &mut canonical.views {
        view.view_name =
            ArchiveString::new(view.view_name.text(), view.view_name.encoding()).unwrap();
    }
    let shorter = AnnotationViews::new(Some(canonical.clone()), version, LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap()
        .len();
    assert!(shorter < bytes.len());
    assert!(
        original
            .with_contents(
                Some(canonical),
                DisplayViewLimits {
                    maximum_bytes: shorter,
                    ..LIMITS
                }
            )
            .is_err()
    );
}

#[test]
fn failed_edits_clone_mutations_equivalent_edits_and_authentic_reverts_preserve_sources() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(version, scenario, _)| version.value() == 19000 && *scenario == 2)
        .unwrap();
    let original = AnnotationViews::from_bytes(bytes, version, LIMITS).unwrap();
    let mut contents = original.contents().clone().unwrap();
    contents.views[0].view_name = ArchiveString::new("edited", TextEncoding::Latin1).unwrap();
    let edited = original
        .with_contents(Some(contents.clone()), LIMITS)
        .unwrap();
    contents.views[0].view_id = AnnotationViewId::new(9999);
    assert_eq!(
        edited.contents().as_ref().unwrap().views[0].view_id.value(),
        -99
    );
    let reopened =
        AnnotationViews::from_bytes(&edited.to_bytes().unwrap(), version, LIMITS).unwrap();
    assert_eq!(reopened.contents(), edited.contents());
    assert_eq!(
        edited
            .with_contents(original.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    assert_eq!(
        original
            .with_contents(original.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

#[test]
fn public_annotation_types_are_send_sync_and_concurrent_edits_equal_serial_results() {
    fn shared<T: Send + Sync>() {}
    shared::<AnnotationViews>();
    shared::<AnnotationViewContents>();
    shared::<AnnotationView>();
    shared::<AnnotationViewId>();
    shared::<AnnotationSectionPlane>();
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(version, scenario, _)| version.value() == 19000 && *scenario == 2)
        .unwrap();
    let original = AnnotationViews::from_bytes(bytes, version, LIMITS).unwrap();
    let expected: Vec<Vec<u8>> = (0..8).map(|index| edit(&original, index)).collect();
    for _ in 0..8 {
        std::thread::scope(|scope| {
            let tasks: Vec<_> = (0..8)
                .map(|index| {
                    let original = &original;
                    scope.spawn(move || edit(original, index))
                })
                .collect();
            for (task, expected) in tasks.into_iter().zip(&expected) {
                assert_eq!(task.join().unwrap(), *expected);
            }
        });
    }
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

fn edit(original: &AnnotationViews, index: usize) -> Vec<u8> {
    let mut contents = original.contents().clone().unwrap();
    contents.views[0].is_current = index as i32;
    original
        .with_contents(Some(contents), LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap()
}
