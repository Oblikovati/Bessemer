//! Version boundaries, exact wire fidelity and resource contracts for named-view bodies.
use bessemer::display_lists::*;
#[path = "support/display_named.rs"]
mod support;
use support::*;

#[test]
fn exact_version_markers_normalize_once_without_guessing_or_process_defaults() {
    for (name, value) in [
        ("19000", 19000),
        ("019000", 19000),
        ("-1", u32::MAX),
        ("-2147483648", 0x80000000),
        ("4294967295", u32::MAX),
        ("-0", 0),
    ] {
        let marker = format!("_DL_VERSION_{name}/DLUpdateStamp");
        assert_eq!(
            DisplayVersion::from_stream_names([marker.as_str()])
                .unwrap()
                .value(),
            value
        );
    }
    assert_eq!(
        DisplayVersion::from_stream_names([
            "_DL_VERSION_019000/DLUpdateStamp",
            "_DL_VERSION_19000/DLUpdateStamp"
        ])
        .unwrap()
        .value(),
        19000
    );
    for name in [
        "",
        "-",
        "+1",
        "1.0",
        "-2147483649",
        "4294967296",
        " 1",
        "１",
        "--1",
    ] {
        let marker = format!("_DL_VERSION_{name}/DLUpdateStamp");
        assert!(
            DisplayVersion::from_stream_names([marker.as_str()]).is_err(),
            "{name}"
        );
    }
    let missing =
        DisplayVersion::from_stream_names(["_DL_VERSION_19000/Other", "Header2"]).unwrap_err();
    assert_eq!(missing.class, bessemer::FailureClass::Missing);
    let conflict = DisplayVersion::from_stream_names([
        "_DL_VERSION_19000/DLUpdateStamp",
        "_DL_VERSION_18999/DLUpdateStamp",
    ])
    .unwrap_err();
    assert_eq!(conflict.class, bessemer::FailureClass::Invalid);
}

#[test]
fn all_conditional_boundaries_and_noncanonical_cstring_prefixes_reconstruct_exactly() {
    for (version, scenario, bytes) in vectors() {
        let archive = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
        assert_eq!(archive.version(), version);
        assert_eq!(archive.to_bytes().unwrap(), bytes);
        let constructed = NamedViews::new(archive.contents().clone(), version, LIMITS).unwrap();
        assert_eq!(constructed.to_bytes().unwrap(), bytes);
        if version.value() <= 0x1be7 {
            assert!(archive.contents().is_none());
            continue;
        }
        let views = archive.contents().as_ref().unwrap();
        assert_eq!(views.len(), [0, 1, 1, 2][scenario as usize]);
        for view in views {
            check_view(view, version, scenario);
        }
    }
}

fn check_view(view: &NamedView, version: DisplayVersion, scenario: u8) {
    assert_eq!(view.view_name.text(), "鋼\0λ");
    assert_eq!(view.view_name.encoding(), TextEncoding::Utf16);
    assert_eq!(view.unknown_string.text(), "é\0kept");
    assert_eq!(view.model_to_view.has_matrix, 2);
    assert_eq!(view.model_to_view.flags, 0x1f);
    assert_eq!(view.pan_zoom.flags, 0x4a);
    assert_eq!(
        view.unknown_bitset,
        if version.value() > 0x200d {
            Some([i32::MIN, i32::MAX])
        } else {
            None
        }
    );
    assert_eq!(view.section_planes.is_some(), version.value() > 0x1bf4);
    if let Some(planes) = &view.section_planes {
        assert_eq!(planes.len(), if scenario >= 2 { 2 } else { 0 });
        for plane in planes {
            check_plane(plane, version);
        }
    }
    check_selections(view, version, scenario);
}

fn check_plane(plane: &NamedSectionPlane, version: DisplayVersion) {
    assert_eq!(
        plane.root_point.map(|value| value.bits()),
        [0, 0x8000000000000000, 0x7ff0000000000000]
    );
    assert_eq!(
        plane.normal.map(|value| value.bits()),
        [0xfff0000000000000, 0x7ff8000000000123, 0x7ff0000000000042]
    );
    assert_eq!(plane.unknown_double.bits(), 1);
    assert_eq!(plane.unknown_i0, -17);
    assert_eq!(
        plane.unknown_pair,
        if version.value() > 0x2001 {
            Some([-123, 456])
        } else {
            None
        }
    );
    assert_eq!(
        plane.unknown_word,
        if version.value() > 0x2007 {
            Some(0xfedcba98)
        } else {
            None
        }
    );
}

fn check_selections(view: &NamedView, version: DisplayVersion, scenario: u8) {
    let expected = version.value() > 0x2003 && scenario >= 2;
    assert_eq!(view.selections.is_some(), expected);
    if !expected {
        return;
    }
    let selections = view.selections.as_ref().unwrap();
    assert_eq!(
        selections.has_selections,
        if scenario == 3 { -1 } else { 0 }
    );
    assert_eq!(selections.values.is_some(), scenario == 3);
    if let Some(values) = &selections.values {
        assert_eq!(values.unknown_sel0, -400);
        assert_eq!(values.is_part, -999);
        assert_eq!(
            values
                .names
                .iter()
                .map(|name| name.text())
                .collect::<Vec<_>>(),
            ["near\0Cé", "鋼\0λ"]
        );
    }
}

#[test]
fn every_truncation_trailing_byte_negative_count_and_oversized_count_fails() {
    for (version, _, bytes) in vectors() {
        for length in 0..bytes.len() {
            assert!(
                NamedViews::from_bytes(&bytes[..length], version, LIMITS).is_err(),
                "{version:?}, {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(NamedViews::from_bytes(&trailing, version, LIMITS).is_err());
    }
    for count in [-1, i32::MAX] {
        assert!(
            NamedViews::from_bytes(&count.to_le_bytes(), DisplayVersion::new(19000), LIMITS)
                .is_err()
        );
    }
    assert!(
        NamedViews::from_bytes(&invalid_nested(false), DisplayVersion::new(19000), LIMITS).is_err()
    );
    assert!(
        NamedViews::from_bytes(&invalid_nested(true), DisplayVersion::new(19000), LIMITS).is_err()
    );
}

fn invalid_nested(selection: bool) -> Vec<u8> {
    let mut bytes = 1i32.to_le_bytes().to_vec();
    bytes.extend_from_slice(b"\x01A");
    bytes.extend_from_slice(&[0; 34]);
    bytes.extend_from_slice(&[0; 34]);
    bytes.extend_from_slice(b"\x01B");
    if selection {
        bytes.extend_from_slice(&1i32.to_le_bytes());
        bytes.extend_from_slice(&[0; 72]);
        bytes.extend_from_slice(&2i32.to_le_bytes());
        bytes.extend_from_slice(&[0; 8]);
    }
    bytes.extend_from_slice(&(-1i32).to_le_bytes());
    bytes.extend_from_slice(&[0; 8]);
    bytes
}

#[test]
fn physical_view_and_cumulative_nested_budgets_apply_to_reads_construction_and_equivalent_edits() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, scenario, _)| v.value() == 19000 && *scenario == 3)
        .unwrap();
    let archive = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
    let exact = DisplayViewLimits {
        maximum_bytes: bytes.len(),
        maximum_views: 2,
        maximum_nested_entries: 8,
    };
    assert!(NamedViews::new(archive.contents().clone(), version, exact).is_ok());
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
            maximum_nested_entries: 7,
            ..exact
        },
    ] {
        assert!(NamedViews::from_bytes(bytes, version, limits).is_err());
        assert!(NamedViews::new(archive.contents().clone(), version, limits).is_err());
        assert!(
            archive
                .with_contents(archive.contents().clone(), limits)
                .is_err()
        );
    }
    assert_eq!(archive.to_bytes().unwrap(), bytes);
}

#[test]
fn conditional_shape_mismatches_never_publish_partial_or_reinterpreted_views() {
    for (version, scenario, bytes) in vectors()
        .into_iter()
        .filter(|(_, scenario, _)| *scenario != 0)
    {
        let original = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
        if version.value() <= 0x1be7 {
            assert!(original.with_contents(Some(Vec::new()), LIMITS).is_err());
            continue;
        }
        assert!(original.with_contents(None, LIMITS).is_err());
        let mut views = original.contents().clone().unwrap();
        views[0].unknown_bitset = if version.value() > 0x200d {
            None
        } else {
            Some([0; 2])
        };
        assert!(original.with_contents(Some(views), LIMITS).is_err());
        if version.value() > 0x2003 && scenario >= 2 {
            reject_selection_shape(&original);
        }
        if version.value() > 0x1bf4 && scenario >= 2 {
            reject_plane_shape(&original, version);
        }
        assert_eq!(original.to_bytes().unwrap(), bytes);
    }
}

fn reject_selection_shape(original: &NamedViews) {
    let mut views = original.contents().clone().unwrap();
    let selections = views[0].selections.as_mut().unwrap();
    selections.has_selections = if selections.has_selections == 0 { 2 } else { 0 };
    assert!(original.with_contents(Some(views), LIMITS).is_err());
}

fn reject_plane_shape(original: &NamedViews, version: DisplayVersion) {
    let mut views = original.contents().clone().unwrap();
    let plane = &mut views[0].section_planes.as_mut().unwrap()[0];
    plane.unknown_pair = if version.value() > 0x2001 {
        None
    } else {
        Some([0; 2])
    };
    assert!(original.with_contents(Some(views), LIMITS).is_err());
}

#[test]
fn edits_failures_equivalence_and_authentic_reverts_keep_published_collections_isolated() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, scenario, _)| v.value() == 19000 && *scenario == 3)
        .unwrap();
    let original = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
    let mut owned = original.contents().clone().unwrap();
    owned[0].view_name = ArchiveString::new("edited", TextEncoding::Latin1).unwrap();
    let edited = original.with_contents(Some(owned.clone()), LIMITS).unwrap();
    owned[0].view_name = ArchiveString::new("later mutation", TextEncoding::Utf16).unwrap();
    assert_eq!(
        edited.contents().as_ref().unwrap()[0].view_name.text(),
        "edited"
    );
    let reopened = NamedViews::from_bytes(&edited.to_bytes().unwrap(), version, LIMITS).unwrap();
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
fn named_view_types_are_send_sync_and_concurrent_edits_equal_serial_output() {
    fn shared<T: Send + Sync>() {}
    shared::<NamedViews>();
    shared::<NamedView>();
    shared::<NamedSectionPlane>();
    shared::<NamedSectionSelections>();
    shared::<NamedSelectionValues>();
    shared::<DisplayVersion>();
    shared::<DisplayViewLimits>();
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, scenario, _)| v.value() == 19000 && *scenario == 3)
        .unwrap();
    let original = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
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

fn edit(original: &NamedViews, index: usize) -> Vec<u8> {
    let mut views = original.contents().clone().unwrap();
    views[0].view_name = ArchiveString::new(&format!("view-{index}"), TextEncoding::Utf16).unwrap();
    original
        .with_contents(Some(views), LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap()
}

#[test]
fn equivalent_cstring_edits_cannot_return_preserved_bytes_larger_than_the_requested_budget() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, scenario, _)| v.value() == 19000 && *scenario == 3)
        .unwrap();
    let original = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
    let mut canonical = original.contents().clone().unwrap();
    for view in &mut canonical {
        view.view_name =
            ArchiveString::new(view.view_name.text(), view.view_name.encoding()).unwrap();
    }
    let rebuilt = NamedViews::new(Some(canonical.clone()), version, LIMITS).unwrap();
    assert_eq!(rebuilt.contents(), original.contents());
    let canonical_length = rebuilt.to_bytes().unwrap().len();
    assert!(canonical_length < bytes.len());
    let limits = DisplayViewLimits {
        maximum_bytes: canonical_length,
        ..LIMITS
    };
    assert!(original.with_contents(Some(canonical), limits).is_err());
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

#[test]
fn cstring_encoded_size_budgets_cover_every_prefix_width_and_both_character_encodings() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, scenario, _)| v.value() == 19000 && *scenario == 1)
        .unwrap();
    let original = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
    for encoding in [TextEncoding::Latin1, TextEncoding::Utf16] {
        for length in [0, 254, 255, 65533, 65534, 65535] {
            let text = "é".repeat(length);
            let mut views = original.contents().clone().unwrap();
            views[0].view_name = ArchiveString::new(&text, encoding).unwrap();
            let archive = NamedViews::new(Some(views.clone()), version, LIMITS).unwrap();
            let exact = archive.to_bytes().unwrap().len();
            assert!(
                NamedViews::new(
                    Some(views.clone()),
                    version,
                    DisplayViewLimits {
                        maximum_bytes: exact,
                        ..LIMITS
                    }
                )
                .is_ok()
            );
            assert!(
                NamedViews::new(
                    Some(views),
                    version,
                    DisplayViewLimits {
                        maximum_bytes: exact - 1,
                        ..LIMITS
                    }
                )
                .is_err()
            );
            assert_eq!(
                NamedViews::from_bytes(&archive.to_bytes().unwrap(), version, LIMITS)
                    .unwrap()
                    .contents(),
                archive.contents()
            );
        }
    }
}
