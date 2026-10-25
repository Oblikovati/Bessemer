use crate::archive_scalar_trace;
use bessemer::display_lists::{
    DisplayArchive, DisplayField, DisplayFieldValue, DisplayObjectPointer,
};

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let scopes = archive
        .contents()
        .fields
        .iter()
        .filter_map(graphics_scope)
        .collect::<Vec<_>>();
    let mut rows = std::collections::BTreeMap::new();
    for field in &archive.contents().fields {
        if !visible(&field.path, &scopes) {
            continue;
        }
        if let Some(value) = archive_scalar_trace::scalar(&field.value) {
            rows.insert(field.path.to_string(), value);
        }
    }
    render(rows)
}

fn graphics_scope(field: &DisplayField) -> Option<&str> {
    let DisplayFieldValue::ObjectTag(tag) = &field.value else {
        return None;
    };
    if !matches!(tag.target, DisplayObjectPointer::New(_)) {
        return None;
    }
    let class = tag.runtime_class.as_ref()?;
    matches!(
        class.name.as_str(),
        "moGraphicsConnectRefMgrData_c"
            | "moGraphicsMagneticConnectRefData_c"
            | "moGraphicsRefWrapData_c"
    )
    .then_some(field.path.as_ref())
}

fn visible(path: &str, scopes: &[&str]) -> bool {
    let leaf = path.rsplit('.').next().unwrap_or("");
    !matches!(
        leaf,
        "count" | "has_graphics_ground_face_data" | "has_unknown_18"
    ) && scopes.iter().any(|scope| {
        path.strip_prefix(scope)
            .is_some_and(|suffix| suffix.starts_with('.'))
    })
}

fn render(rows: std::collections::BTreeMap<String, String>) -> Vec<u8> {
    rows.into_iter()
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}
