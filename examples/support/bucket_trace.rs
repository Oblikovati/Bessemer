use crate::{item_trace, view_trace};
use bessemer::display_lists::DisplayArchive;
#[path = "legacy_bucket_trace.rs"]
mod legacy_flags;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = view_trace::Trace::new();
    for field in &archive.contents().fields {
        if field.path.contains(".view_bucket.") && visible(&field.path) {
            item_trace::insert(&mut rows, &field.path, &field.value);
        }
    }
    legacy_flags::append(archive, &mut rows);
    append_old_activity(archive, &mut rows);
    rows.retain(|_, value| !value.starts_with("a:"));
    view_trace::finish(rows)
}

fn append_old_activity(archive: &DisplayArchive, rows: &mut view_trace::Trace) {
    if archive.context().version.value() >= 0x394 {
        return;
    }
    for field in &archive.contents().fields {
        if field.path.contains(".view_bucket.")
            && !field.path.contains(".bucket.")
            && field.path.ends_with(".flags")
            && let bessemer::display_lists::DisplayFieldValue::I32(word) = field.value
        {
            rows.insert(
                field.path.to_string(),
                format!("i:{}", i32::from(word != 0)),
            );
        }
    }
}

fn visible(path: &str) -> bool {
    if path.contains("unknown_legacy[")
        || path.contains(".discarded_edge_data")
        || path.ends_with(".silh_id_high")
        || path.ends_with(".silh_id_low")
    {
        return false;
    }
    let leaf = path.rsplit('.').next().unwrap_or("");
    !matches!(
        leaf,
        "count"
            | "count_word"
            | "count_dword"
            | "has_xform"
            | "has_ghost_dl_xform"
            | "has_original_view_bucket_in_detailing_mode"
            | "has_user_line_property"
            | "has_widths"
            | "width_count"
            | "ole_list_absent"
            | "has_texture"
            | "has_texture2"
            | "has_rv_app_props"
            | "has_dspbr"
            | "smooth"
            | "new_boundary"
            | "new_region"
            | "bos"
            | "section_cut_new"
            | "layer_visible"
            | "layer_printable"
            | "has_arc_data"
            | "has_silh_data"
    )
}
