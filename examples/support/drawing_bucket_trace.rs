use bessemer::display_lists::DrawingBucketArchive;

pub(crate) fn trace(archive: &DrawingBucketArchive) -> Vec<u8> {
    let mut rows = std::collections::BTreeMap::new();
    for field in &archive.contents().fields {
        if field.path.contains("view_buckets[")
            || field.path.contains(".tessellation")
            || field.path.contains(".old_lights")
            || field.path.contains(".visual_properties")
        {
            continue;
        }
        if let Some(value) = crate::archive_scalar_trace::scalar(&field.value) {
            rows.insert(field.path.to_string(), value);
        }
    }
    rows.into_iter()
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}
