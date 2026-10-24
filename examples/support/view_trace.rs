use bessemer::display_lists::{CompactTransform, PerspectivePayload, StoredDouble};
use std::collections::BTreeMap;
pub(crate) type Trace = BTreeMap<String, String>;

pub(crate) fn integer(rows: &mut Trace, path: &str, value: i64) {
    rows.insert(path.into(), format!("i:{value}"));
}

pub(crate) fn array(rows: &mut Trace, path: &str, length: usize) {
    rows.insert(path.into(), format!("a:{length}"));
}

pub(crate) fn double(rows: &mut Trace, path: &str, value: StoredDouble) {
    rows.insert(path.into(), format!("f:{:016x}", value.bits()));
}

pub(crate) fn text(rows: &mut Trace, path: &str, value: &str) {
    rows.insert(
        path.into(),
        format!(
            "s:{}",
            value
                .bytes()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        ),
    );
}

pub(crate) fn point(rows: &mut Trace, path: &str, values: &[StoredDouble]) {
    array(rows, path, values.len());
    for (index, value) in values.iter().enumerate() {
        double(rows, &format!("{path}[{index}]"), *value);
    }
}

pub(crate) fn integers(rows: &mut Trace, path: &str, values: &[i32]) {
    array(rows, path, values.len());
    for (index, value) in values.iter().enumerate() {
        integer(rows, &format!("{path}[{index}]"), i64::from(*value));
    }
}

pub(crate) fn compact(rows: &mut Trace, path: &str, transform: &CompactTransform) {
    if let Some(matrix) = transform.matrix {
        let prefix = format!("{path}.matrix");
        array(rows, &prefix, 3);
        for (index, row) in matrix.iter().enumerate() {
            point(rows, &format!("{prefix}[{index}]"), row);
        }
    }
    point(rows, &format!("{path}.translation"), &transform.translation);
    double(rows, &format!("{path}.scaling"), transform.scaling);
    integer(rows, &format!("{path}.flags"), i64::from(transform.flags));
    perspective(rows, path, &transform.perspective);
}

pub(crate) fn perspective(rows: &mut Trace, path: &str, payload: &PerspectivePayload) {
    let prefix = format!("{path}.perspective_info");
    match payload {
        PerspectivePayload::None => (),
        PerspectivePayload::Base(base) => base_perspective(rows, &prefix, base),
        PerspectivePayload::Extended { .. } => extended_perspective(rows, &prefix, payload),
        PerspectivePayload::Matrix5 { matrix, .. } => matrix5_trace(rows, path, matrix),
    }
}

pub(crate) fn base_perspective(
    rows: &mut Trace,
    path: &str,
    base: &bessemer::display_lists::PerspectiveBase,
) {
    point(rows, &format!("{path}.eye_point"), &base.eye_point);
    double(rows, &format!("{path}.plane_distance"), base.plane_distance);
}

pub(crate) fn extended_perspective(rows: &mut Trace, path: &str, payload: &PerspectivePayload) {
    if let PerspectivePayload::Extended {
        base,
        minimum_far_clip,
        camera_fov_plane,
    } = payload
    {
        base_perspective(rows, path, base);
        double(
            rows,
            &format!("{path}.min_far_clip_plane_distance"),
            *minimum_far_clip,
        );
        double(
            rows,
            &format!("{path}.camera_fov_plane_distance"),
            *camera_fov_plane,
        );
    }
}

pub(crate) fn matrix5_trace(rows: &mut Trace, path: &str, matrix: &[StoredDouble; 25]) {
    let prefix = format!("{path}.matrix5");
    array(rows, &prefix, 5);
    for (index, row) in matrix.as_chunks::<5>().0.iter().enumerate() {
        point(rows, &format!("{prefix}[{index}]"), row);
    }
}

pub(crate) fn finish(rows: Trace) -> Vec<u8> {
    rows.into_iter()
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}
