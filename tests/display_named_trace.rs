//! Independent field-oracle output must retain decoded names and exact scalar bits.
use bessemer::display_lists::*;
#[path = "../examples/support/named_trace.rs"]
mod named_trace;
#[path = "support/display_named.rs"]
mod support;
use support::*;

#[test]
fn traces_cover_absent_empty_and_populated_arrays_without_losing_stored_nan_bits() {
    for (version, scenario, bytes) in vectors() {
        let archive = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
        let trace = String::from_utf8(named_trace::trace(&archive)).unwrap();
        if version.value() <= 0x1be7 {
            assert_eq!(trace, "root=none\n");
            continue;
        }
        assert!(trace.contains(&format!("root=a:{}\n", [0, 1, 1, 2][scenario as usize])));
        if scenario == 0 {
            continue;
        }
        assert!(trace.contains("root[0].view_name=s:e98bbc00cebb\n"));
        assert!(trace.contains("root[0].model_to_view_xform.translation[0]=f:8000000000000000\n"));
        assert!(trace.contains("root[0].pan_zoom_xform.matrix5=a:5\n"));
        assert!(trace.contains("root[0].pan_zoom_xform.matrix5[0][2]=f:7ff0000000000042\n"));
        if version.value() > 0x2003 && scenario == 3 {
            assert!(trace.contains("root[0].is_part=i:-999\n"));
        }
    }
}

#[test]
fn extended_perspective_traces_preserve_signaling_nan_and_negative_zero_field_identity() {
    let (version, _, bytes) = *vectors()
        .iter()
        .find(|(v, scenario, _)| v.value() == 19000 && *scenario == 1)
        .unwrap();
    let original = NamedViews::from_bytes(bytes, version, LIMITS).unwrap();
    let mut views = original.contents().clone().unwrap();
    let PerspectivePayload::Base(base) = views[0].model_to_view.perspective.clone() else {
        panic!("base");
    };
    views[0].model_to_view.flags = 0x27;
    views[0].model_to_view.perspective = PerspectivePayload::Extended {
        base,
        minimum_far_clip: StoredDouble::from_bits(0x7ff0000000000042),
        camera_fov_plane: StoredDouble::from_bits(0x8000000000000000),
    };
    let edited = NamedViews::new(Some(views), version, LIMITS).unwrap();
    let trace = String::from_utf8(named_trace::trace(&edited)).unwrap();
    assert!(trace.contains("root[0].model_to_view_xform.perspective_info.min_far_clip_plane_distance=f:7ff0000000000042\n"));
    assert!(trace.contains("root[0].model_to_view_xform.perspective_info.camera_fov_plane_distance=f:8000000000000000\n"));
}
