//! Complete versioned 3D-view bodies and raw BITMAP size contracts.
use bessemer::display_lists::*;
#[path = "support/display_spatial.rs"]
mod support;
use support::*;

#[test]
fn every_spatial_boundary_rebuilds_exactly_and_matches_independent_fields() {
    for (version, scenario, bytes) in vectors() {
        let archive = SpatialViews::from_bytes(bytes, version, LIMITS).unwrap();
        assert_eq!(archive.version(), version);
        assert_eq!(archive.to_bytes().unwrap(), bytes);
        assert_eq!(
            SpatialViews::new(archive.contents().clone(), version, LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
        if version.value() <= 0x1c18 {
            assert!(archive.contents().is_none());
            continue;
        }
        let views = archive.contents().as_ref().unwrap();
        assert_eq!(views.len(), scenario as usize);
        for view in views {
            check_view(view, version, scenario);
        }
    }
}

fn check_view(view: &SpatialView, version: DisplayVersion, scenario: u8) {
    assert_eq!(view.view_name.text(), "鋼\0λ");
    assert_eq!(
        view.unknown_active,
        if version.value() > 0x46c9 {
            Some(-17)
        } else {
            None
        }
    );
    assert_eq!(
        (
            view.model_to_view.has_matrix,
            view.model_to_view.flags,
            view.pan_zoom.flags
        ),
        (2, 0x2f, 0x4a)
    );
    assert_eq!(view.unknown_string.text(), "é\0kept");
    assert_eq!(view.preview_bitmap.is_some(), version.value() > 0x1c1e);
    if let Some(bitmap) = &view.preview_bitmap {
        check_bitmap(bitmap, scenario);
    }
    assert_eq!(
        view.unknown_ids_string.as_ref().map(|text| text.text()),
        if version.value() > 0x1c42 {
            Some("raw ids -1,  02")
        } else {
            None
        }
    );
    assert_eq!(view.section_planes.len(), if scenario == 2 { 2 } else { 0 });
    for plane in &view.section_planes {
        check_plane(plane, version);
    }
    assert_eq!(
        view.unknown_after_planes,
        if scenario == 2 && version.value() > 0x2012 {
            Some(-27)
        } else {
            None
        }
    );
    assert_eq!(
        view.unknown_468,
        if version.value() > 0x1fb5 {
            Some(-468)
        } else {
            None
        }
    );
    assert_eq!(
        view.unknown_bitset,
        if version.value() > 0x1fb5 {
            Some([i32::MIN, i32::MAX])
        } else {
            None
        }
    );
    assert_eq!(
        view.unknown_text_string.as_ref().map(|text| text.text()),
        if version.value() > 0x3331 {
            Some("unknown text 鋼")
        } else {
            None
        }
    );
}

fn check_bitmap(bitmap: &ViewBitmap, scenario: u8) {
    assert_eq!((bitmap.planes, bitmap.bits_per_pixel), (0, 64));
    if scenario == 1 {
        assert_eq!((bitmap.width, bitmap.height, bitmap.row_bytes), (42, 0, 17));
        assert!(bitmap.bits.is_empty());
        return;
    }
    assert_eq!(
        (bitmap.width, bitmap.height, bitmap.row_bytes),
        (-99, -2, -3)
    );
    assert_eq!(bitmap.bits, [0, 255, 13, 10, 128, 17]);
}

fn check_plane(plane: &ViewSectionPlane, version: DisplayVersion) {
    assert_eq!(
        plane.root_point.map(|v| v.bits()),
        [0, 0x8000000000000000, 0x7ff0000000000000]
    );
    assert_eq!(plane.normal[2].bits(), 0x7ff0000000000042);
    assert_eq!(plane.unknown_double.bits(), 1);
    assert_eq!(plane.unknown_i0, -11);
    assert_eq!(
        plane.unknown_pair,
        if version.value() > 0x2012 {
            Some([-123, 456])
        } else {
            None
        }
    );
    assert_eq!(
        plane.unknown_word,
        if version.value() > 0x2012 {
            Some(0xfedcba98)
        } else {
            None
        }
    );
}

#[test]
fn every_truncation_trailing_byte_and_negative_or_oversized_collection_count_fails() {
    for (version, _, bytes) in vectors() {
        for length in 0..bytes.len() {
            assert!(
                SpatialViews::from_bytes(&bytes[..length], version, LIMITS).is_err(),
                "{version:?}, {length}"
            );
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(SpatialViews::from_bytes(&extra, version, LIMITS).is_err());
    }
    for count in [-1, i32::MAX] {
        assert!(
            SpatialViews::from_bytes(&count.to_le_bytes(), DisplayVersion::new(19000), LIMITS)
                .is_err()
        );
    }
}

#[test]
fn all_bitmap_products_and_payload_lengths_are_checked_without_inferred_header_constraints() {
    let original = modern();
    for (height, row_bytes) in [
        (-1, 1),
        (1, -1),
        (i32::MIN, i32::MIN),
        (i32::MAX, i32::MAX),
        (1, 67108865),
    ] {
        let mut views = original.contents().clone().unwrap();
        let bitmap = views[0].preview_bitmap.as_mut().unwrap();
        bitmap.height = height;
        bitmap.row_bytes = row_bytes;
        assert!(
            original.with_contents(Some(views), LIMITS).is_err(),
            "{height}, {row_bytes}"
        );
    }
    let mut views = original.contents().clone().unwrap();
    views[0].preview_bitmap.as_mut().unwrap().bits.pop();
    assert!(original.with_contents(Some(views), LIMITS).is_err());
    let mut views = original.contents().clone().unwrap();
    let bitmap = views[0].preview_bitmap.as_mut().unwrap();
    bitmap.height = 2;
    bitmap.row_bytes = 3;
    assert!(original.with_contents(Some(views), LIMITS).is_ok());
    let mut views = original.contents().clone().unwrap();
    let bitmap = views[0].preview_bitmap.as_mut().unwrap();
    bitmap.height = i32::MIN;
    bitmap.row_bytes = 0;
    bitmap.bits.clear();
    assert!(original.with_contents(Some(views), LIMITS).is_ok());
}

fn modern() -> SpatialViews {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, s, _)| v.value() == 19000 && *s == 2)
        .unwrap();
    SpatialViews::from_bytes(bytes, version, LIMITS).unwrap()
}

#[test]
fn versioned_shapes_and_data_dependent_plane_tails_are_validated_on_construction() {
    for (version, scenario, bytes) in vectors().into_iter().filter(|(_, s, _)| *s != 0) {
        let original = SpatialViews::from_bytes(bytes, version, LIMITS).unwrap();
        if version.value() <= 0x1c18 {
            assert!(original.with_contents(Some(Vec::new()), LIMITS).is_err());
            continue;
        }
        assert!(original.with_contents(None, LIMITS).is_err());
        let mut views = original.contents().clone().unwrap();
        views[0].unknown_active = if version.value() > 0x46c9 {
            None
        } else {
            Some(0)
        };
        assert!(original.with_contents(Some(views), LIMITS).is_err());
        if scenario == 2 && version.value() > 0x2012 {
            let mut views = original.contents().clone().unwrap();
            views[0].unknown_after_planes = None;
            assert!(original.with_contents(Some(views), LIMITS).is_err());
            let mut views = original.contents().clone().unwrap();
            views[0].section_planes[0].unknown_word = None;
            assert!(original.with_contents(Some(views), LIMITS).is_err());
        }
    }
}

#[test]
fn physical_view_and_cumulative_plane_limits_cover_reads_builds_and_equivalent_edits() {
    let original = modern();
    let bytes = original.to_bytes().unwrap();
    let version = original.version();
    let exact = DisplayViewLimits {
        maximum_bytes: bytes.len(),
        maximum_views: 2,
        maximum_nested_entries: 4,
    };
    assert!(SpatialViews::new(original.contents().clone(), version, exact).is_ok());
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
        assert!(SpatialViews::from_bytes(&bytes, version, limits).is_err());
        assert!(SpatialViews::new(original.contents().clone(), version, limits).is_err());
        assert!(
            original
                .with_contents(original.contents().clone(), limits)
                .is_err()
        );
    }
    let mut canonical = original.contents().clone().unwrap();
    for view in &mut canonical {
        view.view_name =
            ArchiveString::new(view.view_name.text(), view.view_name.encoding()).unwrap();
    }
    let shorter = SpatialViews::new(Some(canonical.clone()), version, LIMITS)
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
fn clones_failures_owned_edits_and_authentic_reverts_keep_bitmap_storage_isolated() {
    let original = modern();
    let bytes = original.to_bytes().unwrap();
    let mut input = bytes.clone();
    let isolated = SpatialViews::from_bytes(&input, original.version(), LIMITS).unwrap();
    input.fill(0);
    assert_eq!(isolated.to_bytes().unwrap(), bytes);
    let mut views = original.contents().clone().unwrap();
    views[0].preview_bitmap.as_mut().unwrap().bits[0] = 99;
    let edited = original.with_contents(Some(views.clone()), LIMITS).unwrap();
    views[0].preview_bitmap.as_mut().unwrap().bits[0] = 100;
    assert_eq!(
        edited.contents().as_ref().unwrap()[0]
            .preview_bitmap
            .as_ref()
            .unwrap()
            .bits[0],
        99
    );
    let reopened =
        SpatialViews::from_bytes(&edited.to_bytes().unwrap(), original.version(), LIMITS).unwrap();
    assert_eq!(reopened.contents(), edited.contents());
    assert_eq!(
        edited
            .with_contents(original.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

#[test]
fn spatial_types_are_send_sync_and_parallel_bitmap_edits_equal_serial_bytes() {
    fn shared<T: Send + Sync>() {}
    shared::<SpatialViews>();
    shared::<SpatialView>();
    shared::<ViewBitmap>();
    shared::<ViewSectionPlane>();
    let original = modern();
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
}

fn edit(original: &SpatialViews, index: usize) -> Vec<u8> {
    let mut views = original.contents().clone().unwrap();
    views[0].preview_bitmap.as_mut().unwrap().bits[0] = index as u8;
    original
        .with_contents(Some(views), LIMITS)
        .unwrap()
        .to_bytes()
        .unwrap()
}

#[test]
fn malformed_bitmap_headers_fail_during_reading_before_copying_payloads() {
    let version = DisplayVersion::new(0x1c1f);
    for (height, row_bytes) in [
        (-1, 1),
        (1, -1),
        (i32::MIN, i32::MIN),
        (i32::MAX, i32::MAX),
        (1, 67108865),
    ] {
        assert!(
            SpatialViews::from_bytes(&bitmap_body(height, row_bytes), version, LIMITS).is_err()
        );
    }
    assert!(SpatialViews::from_bytes(&bitmap_body(i32::MIN, 0), version, LIMITS).is_ok());
    let original = modern();
    let mut views = original.contents().clone().unwrap();
    let bitmap = views[0].preview_bitmap.as_mut().unwrap();
    bitmap.height = 8192;
    bitmap.row_bytes = 8192;
    bitmap.bits.clear();
    let error = original.with_contents(Some(views), LIMITS).unwrap_err();
    assert!(error.cause.contains("bitmap length 0; expected 67108864"));
}

fn bitmap_body(height: i32, row_bytes: i32) -> Vec<u8> {
    let mut bytes = 1i32.to_le_bytes().to_vec();
    bytes.push(0);
    bytes.extend_from_slice(&[0; 68]);
    bytes.push(0);
    for value in [-99, height, row_bytes] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&64u16.to_le_bytes());
    bytes.extend_from_slice(&0i32.to_le_bytes());
    bytes
}

#[test]
fn each_optional_bitmap_string_and_bitset_slot_rejects_a_missing_required_value() {
    let original = modern();
    for slot in 0..5 {
        let mut views = original.contents().clone().unwrap();
        match slot {
            0 => views[0].preview_bitmap = None,
            1 => views[0].unknown_ids_string = None,
            2 => views[0].unknown_468 = None,
            3 => views[0].unknown_bitset = None,
            4 => views[0].unknown_text_string = None,
            _ => panic!("slot"),
        }
        assert!(
            original.with_contents(Some(views), LIMITS).is_err(),
            "slot {slot}"
        );
    }
}
