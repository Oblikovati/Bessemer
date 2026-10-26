use super::{ModelField, ModelFieldValue};
use crate::archive_codec::{CompactTransform, PerspectiveBase, PerspectivePayload, StoredDouble};
use std::collections::BTreeMap;

pub(super) fn assert_fields(fields: &[ModelField], oracle: &str) -> usize {
    let mut rows = BTreeMap::new();
    for field in fields {
        match &field.value {
            ModelFieldValue::Transform(value) => transform(&field.path, value, &mut rows),
            value => {
                if let Some(value) = super::node_units::scalar_value(value) {
                    rows.insert(field.path.to_string(), value);
                }
            }
        }
    }
    for line in oracle.lines() {
        let (path, expected) = line.split_once('=').unwrap();
        assert_eq!(rows.get(path).map(String::as_str), Some(expected), "{path}");
    }
    oracle.lines().count()
}

fn transform(prefix: &str, value: &CompactTransform, rows: &mut BTreeMap<String, String>) {
    if let Some(matrix) = &value.matrix {
        for (index, row) in matrix.iter().enumerate() {
            vector(&format!("{prefix}.matrix[{index}]"), row, rows);
        }
    }
    vector(&format!("{prefix}.translation"), &value.translation, rows);
    double(&format!("{prefix}.scaling"), value.scaling, rows);
    rows.insert(format!("{prefix}.flags"), format!("i:{}", value.flags));
    perspective(prefix, &value.perspective, rows);
}

fn perspective(prefix: &str, value: &PerspectivePayload, rows: &mut BTreeMap<String, String>) {
    match value {
        PerspectivePayload::None => (),
        PerspectivePayload::Base(base) => base_rows(prefix, base, rows),
        PerspectivePayload::Extended {
            base,
            minimum_far_clip,
            camera_fov_plane,
        } => {
            base_rows(prefix, base, rows);
            double(
                &format!("{prefix}.perspective_info.min_far_clip_plane_distance"),
                *minimum_far_clip,
                rows,
            );
            double(
                &format!("{prefix}.perspective_info.camera_fov_plane_distance"),
                *camera_fov_plane,
                rows,
            );
        }
        PerspectivePayload::Matrix5 { matrix, .. } => {
            for index in 0..5 {
                vector(
                    &format!("{prefix}.matrix5[{index}]"),
                    &matrix[index * 5..index * 5 + 5],
                    rows,
                );
            }
        }
    }
}

fn base_rows(prefix: &str, value: &PerspectiveBase, rows: &mut BTreeMap<String, String>) {
    vector(
        &format!("{prefix}.perspective_info.eye_point"),
        &value.eye_point,
        rows,
    );
    double(
        &format!("{prefix}.perspective_info.plane_distance"),
        value.plane_distance,
        rows,
    );
}

fn vector(prefix: &str, values: &[StoredDouble], rows: &mut BTreeMap<String, String>) {
    for (index, value) in values.iter().enumerate() {
        double(&format!("{prefix}[{index}]"), *value, rows);
    }
}

fn double(path: &str, value: StoredDouble, rows: &mut BTreeMap<String, String>) {
    rows.insert(path.into(), format!("f:{:016x}", value.bits()));
}
