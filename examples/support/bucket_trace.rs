use crate::{item_trace, view_trace};
use bessemer::display_lists::DisplayArchive;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = view_trace::Trace::new();
    for field in &archive.contents().fields {
        if field.path.contains(".view_bucket.") && visible(&field.path) {
            item_trace::insert(&mut rows, &field.path, &field.value);
        }
    }
    rows.retain(|_, value| !value.starts_with("a:"));
    view_trace::finish(rows)
}

fn visible(path: &str) -> bool {
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
    )
}
