use crate::archive_scalar_trace;
use bessemer::display_lists::DisplayArchive;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = std::collections::BTreeMap::new();
    for field in &archive.contents().fields {
        if !field.path.starts_with("root.old_lights.items[") || field.path.ends_with(".count") {
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
