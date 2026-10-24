//! Annotation semantic traces retain collection-level tails, raw identities and exact scalar bits.
use bessemer::display_lists::*;
#[path = "../examples/support/annotation_trace.rs"]
mod annotation_trace;
#[path = "support/display_annotation.rs"]
mod support;
#[path = "../examples/support/view_trace.rs"]
mod view_trace;
use support::*;

#[test]
fn annotation_oracle_traces_keep_empty_collection_tails_raw_flags_and_all_plane_values() {
    for (version, scenario, bytes) in vectors() {
        let archive = AnnotationViews::from_bytes(bytes, version, LIMITS).unwrap();
        let trace = String::from_utf8(annotation_trace::trace(&archive)).unwrap();
        if version.value() <= 0x1c39 {
            assert_eq!(trace, "root=none\n");
            continue;
        }
        assert!(trace.contains(&format!("root.views=a:{scenario}\n")));
        assert_eq!(
            trace.contains("root.unknown_bitset=a:2\n"),
            version.value() > 0x1fa8
        );
        if scenario == 0 {
            continue;
        }
        assert!(trace.contains("root.views[0].view_id=i:-99\n"));
        assert!(trace.contains("root.views[0].is_shown=i:2\n"));
        assert!(trace.contains("root.views[0].is_current=i:-1\n"));
        assert!(trace.contains("root.views[0].view_name=s:e98bbc00cebb\n"));
        assert!(trace.contains("root.views[0].model_to_view_xform.perspective_info.min_far_clip_plane_distance=f:8000000000000000\n"));
        assert!(trace.contains("root.views[0].pan_zoom_xform.matrix5[0][2]=f:7ff0000000000042\n"));
        if scenario == 2 {
            assert!(trace.contains("root.views[0].section_planes[0].unknown_i=i:-23\n"));
        }
    }
}
