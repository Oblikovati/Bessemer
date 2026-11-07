// #34: callers bind the formatter once per crate, including integration-test aliases.
use crate::archive_scalar_trace;
use bessemer::display_lists::{DisplayArchive, DisplayFieldValue, FixedTransform, StoredDouble};
use std::collections::BTreeMap;

pub(crate) fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = BTreeMap::new();
    for field in &archive.contents().fields {
        if reference_path(&field.path) {
            append(&mut rows, &field.path, &field.value);
        }
    }
    rows.into_iter()
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}

fn reference_path(path: &str) -> bool {
    path.contains(".ref_geom_feature_data_collector.") || path.contains(".ref_geom_data_collector.")
}

fn append(rows: &mut BTreeMap<String, String>, path: &str, value: &DisplayFieldValue) {
    if let Some(value) = archive_scalar_trace::scalar(value) {
        rows.insert(path.into(), value);
    } else if let DisplayFieldValue::FixedTransform(value) = value {
        fixed(rows, path, value);
    }
}

fn fixed(rows: &mut BTreeMap<String, String>, path: &str, transform: &FixedTransform) {
    for (label, values) in [
        ("matrix", transform.matrix.as_slice()),
        ("translation", &transform.translation),
    ] {
        double_array(rows, &format!("{path}.{label}"), values);
    }
    rows.insert(
        format!("{path}.scaling"),
        format!("f:{:016x}", transform.scaling.bits()),
    );
    for (label, values) in [
        ("unknown_48", transform.unknown_48),
        ("unknown_70", transform.unknown_70),
    ] {
        for (index, value) in values.iter().enumerate() {
            rows.insert(format!("{path}.{label}[{index}]"), format!("i:{value}"));
        }
    }
}

fn double_array(rows: &mut BTreeMap<String, String>, path: &str, values: &[StoredDouble]) {
    for (index, value) in values.iter().enumerate() {
        rows.insert(
            format!("{path}[{index}]"),
            format!("f:{:016x}", value.bits()),
        );
    }
}
