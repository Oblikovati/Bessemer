//! Spatial traces retain Win32 bitmap bytes separately from text and decoded scalar fields.
use bessemer::display_lists::*;
#[path = "../examples/support/spatial_trace.rs"]
mod spatial_trace;
#[path = "support/display_spatial.rs"]
mod support;
#[path = "../examples/support/view_trace.rs"]
mod view_trace;
use support::*;

#[test]
fn all_spatial_traces_preserve_bitmap_bytes_special_numbers_and_conditional_tails() {
    for (version, scenario, bytes) in vectors() {
        let archive = SpatialViews::from_bytes(bytes, version, LIMITS).unwrap();
        let trace = String::from_utf8(spatial_trace::trace(&archive)).unwrap();
        if version.value() <= 0x1c18 {
            assert_eq!(trace, "root=none\n");
            continue;
        }
        assert!(trace.contains(&format!("root=a:{scenario}\n")));
        if scenario == 0 {
            continue;
        }
        assert!(trace.contains("root[0].view_name=s:e98bbc00cebb\n"));
        assert!(trace.contains("root[0].pan_zoom_xform.matrix5[0][2]=f:7ff0000000000042\n"));
        assert_eq!(
            trace.contains("root[0].unknown_active=i:-17\n"),
            version.value() > 0x46c9
        );
        if version.value() > 0x1c1e {
            let bitmap = if scenario == 1 {
                "root[0].preview_bitmap.bits=b:\n"
            } else {
                "root[0].preview_bitmap.bits=b:00ff0d0a8011\n"
            };
            assert!(trace.contains(bitmap));
            assert!(trace.contains("root[0].preview_bitmap.bits_pixel=i:64\n"));
            if scenario == 2 {
                assert!(trace.contains("root[0].preview_bitmap.width_bytes=i:-3\n"));
            }
        }
        assert_eq!(
            trace.contains("root[0].unknown_after_planes=i:-27\n"),
            scenario == 2 && version.value() > 0x2012
        );
    }
}
