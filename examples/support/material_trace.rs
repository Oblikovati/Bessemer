use crate::archive_scalar_trace;
use bessemer::display_lists::DisplayArchive;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = std::collections::BTreeMap::new();
    for field in &archive.contents().fields {
        if !visible(&field.path) {
            continue;
        }
        if let Some(value) = archive_scalar_trace::scalar(&field.value) {
            rows.insert(field.path.to_string(), value);
        }
    }
    rows.into_iter()
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}

fn visible(path: &str) -> bool {
    let leaf = path.rsplit('.').next().unwrap_or("");
    (path.contains(".material.") || path.contains(".dspbr."))
        && !path.starts_with("root.scene.")
        && !path.contains(".view_bucket.")
        && !matches!(
            leaf,
            "has_texture" | "has_texture2" | "has_rv_app_props" | "count"
        )
}
