use crate::{
    archive_scalar_trace,
    view_trace::{self, Trace},
};
use bessemer::display_lists::{DisplayArchive, DisplayFieldValue};
use bessemer::header::HeaderBox;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = Trace::new();
    for field in &archive.contents().fields {
        if visible(&field.path) {
            append(&mut rows, &field.path, &field.value);
        }
    }
    rows.retain(|_, value| !value.starts_with("a:"));
    view_trace::finish(rows)
}

fn visible(path: &str) -> bool {
    let leaf = path.rsplit('.').next().unwrap_or("");
    path.starts_with("root.scene.")
        && !matches!(
            leaf,
            "count"
                | "count_word"
                | "count_dword"
                | "count_low"
                | "count_high"
                | "has_merged_feature_list"
                | "has_texture"
                | "has_texture2"
                | "has_rv_app_props"
                | "has_dspbr"
        )
}

fn append(rows: &mut Trace, path: &str, value: &DisplayFieldValue) {
    match value {
        DisplayFieldValue::Transform(value) => view_trace::compact(rows, path, value),
        DisplayFieldValue::Box(value) => bounds(rows, path, value),
        DisplayFieldValue::Text(value) => view_trace::text(rows, path, value.text()),
        _ => {
            if let Some(encoded) = archive_scalar_trace::scalar(value) {
                rows.insert(path.into(), encoded);
            }
        }
    }
}

fn bounds(rows: &mut Trace, path: &str, value: &HeaderBox) {
    let values = value
        .centre
        .iter()
        .chain(&value.high)
        .chain(&value.low)
        .chain(std::iter::once(&value.radius));
    for (index, value) in values.enumerate() {
        view_trace::double(rows, &format!("{path}[{index}]"), *value);
    }
}
