use crate::{archive_scalar_trace, view_trace};
use bessemer::display_lists::{DisplayArchive, DisplayFieldValue};

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = view_trace::Trace::new();
    for field in &archive.contents().fields {
        if !item_path(&field.path) || is_control(&field.path) {
            continue;
        }
        insert(&mut rows, &field.path, &field.value);
    }
    rows.into_iter()
        .filter(|(_, value)| !value.starts_with("a:"))
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}

fn item_path(path: &str) -> bool {
    path.contains(".display_items.items[") || path.starts_with("root.old_display_items.items[")
}

pub(crate) fn insert(rows: &mut view_trace::Trace, path: &str, value: &DisplayFieldValue) {
    if let Some(value) = archive_scalar_trace::scalar(value) {
        rows.insert(path.into(), value);
    }
    if let DisplayFieldValue::Transform(value) = value {
        view_trace::compact(rows, path, value);
    }
}

fn is_control(path: &str) -> bool {
    path.ends_with(".count")
        || path.ends_with(".has_widths")
        || path.ends_with(".width_count")
        || path.ends_with(".ole_list_absent")
}
