use bessemer::display_lists::{
    CompactTransform, NamedSectionPlane, NamedView, NamedViews, PerspectivePayload, StoredDouble,
};
use std::collections::BTreeMap;
type Trace = BTreeMap<String, String>;

pub fn trace(archive: &NamedViews) -> Vec<u8> {
    let mut rows = Trace::new();
    match archive.contents() {
        None => {
            rows.insert("root".into(), "none".into());
        }
        Some(views) => {
            array(&mut rows, "root", views.len());
            for (index, view) in views.iter().enumerate() {
                view_trace(&mut rows, &format!("root[{index}]"), view);
            }
        }
    }
    rows.into_iter()
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}

fn integer(rows: &mut Trace, path: &str, value: i64) {
    rows.insert(path.into(), format!("i:{value}"));
}
fn array(rows: &mut Trace, path: &str, length: usize) {
    rows.insert(path.into(), format!("a:{length}"));
}
fn double(rows: &mut Trace, path: &str, value: StoredDouble) {
    rows.insert(path.into(), format!("f:{:016x}", value.bits()));
}
fn text(rows: &mut Trace, path: &str, value: &str) {
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

fn point(rows: &mut Trace, path: &str, values: &[StoredDouble]) {
    array(rows, path, values.len());
    for (index, value) in values.iter().enumerate() {
        double(rows, &format!("{path}[{index}]"), *value);
    }
}

fn view_trace(rows: &mut Trace, path: &str, view: &NamedView) {
    text(rows, &format!("{path}.view_name"), view.view_name.text());
    compact(
        rows,
        &format!("{path}.model_to_view_xform"),
        &view.model_to_view,
    );
    compact(rows, &format!("{path}.pan_zoom_xform"), &view.pan_zoom);
    text(
        rows,
        &format!("{path}.unknown_string"),
        view.unknown_string.text(),
    );
    planes_trace(rows, path, view);
    selections(rows, path, view);
    if let Some(bitset) = view.unknown_bitset {
        integers(rows, &format!("{path}.unknown_bitset"), &bitset);
    }
}

fn planes_trace(rows: &mut Trace, path: &str, view: &NamedView) {
    let Some(planes) = &view.section_planes else {
        return;
    };
    let prefix = format!("{path}.section_planes");
    array(rows, &prefix, planes.len());
    for (index, plane) in planes.iter().enumerate() {
        plane_trace(rows, &format!("{prefix}[{index}]"), plane);
    }
}

fn plane_trace(rows: &mut Trace, path: &str, plane: &NamedSectionPlane) {
    point(rows, &format!("{path}.root_point"), &plane.root_point);
    point(rows, &format!("{path}.normal"), &plane.normal);
    double(rows, &format!("{path}.unknown_d"), plane.unknown_double);
    integer(
        rows,
        &format!("{path}.unknown_i0"),
        i64::from(plane.unknown_i0),
    );
    if let Some(pair) = plane.unknown_pair {
        integer(rows, &format!("{path}.unknown_i1"), i64::from(pair[0]));
        integer(rows, &format!("{path}.unknown_i2"), i64::from(pair[1]));
    }
    if let Some(word) = plane.unknown_word {
        integer(rows, &format!("{path}.unknown_u"), i64::from(word));
    }
}

fn integers(rows: &mut Trace, path: &str, values: &[i32]) {
    array(rows, path, values.len());
    for (index, value) in values.iter().enumerate() {
        integer(rows, &format!("{path}[{index}]"), i64::from(*value));
    }
}

fn selections(rows: &mut Trace, path: &str, view: &NamedView) {
    let Some(selections) = &view.selections else {
        return;
    };
    integer(
        rows,
        &format!("{path}.has_section_selections"),
        i64::from(selections.has_selections),
    );
    let Some(values) = &selections.values else {
        return;
    };
    integer(
        rows,
        &format!("{path}.unknown_sel0"),
        i64::from(values.unknown_sel0),
    );
    integer(rows, &format!("{path}.is_part"), i64::from(values.is_part));
    selection_names(rows, path, &values.names);
}

fn selection_names(rows: &mut Trace, path: &str, names: &[bessemer::display_lists::ArchiveString]) {
    let prefix = format!("{path}.section_selections");
    array(rows, &prefix, names.len());
    for (index, name) in names.iter().enumerate() {
        text(rows, &format!("{prefix}[{index}]"), name.text());
    }
}

fn compact(rows: &mut Trace, path: &str, transform: &CompactTransform) {
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

fn perspective(rows: &mut Trace, path: &str, payload: &PerspectivePayload) {
    let prefix = format!("{path}.perspective_info");
    match payload {
        PerspectivePayload::None => (),
        PerspectivePayload::Base(base) => base_perspective(rows, &prefix, base),
        PerspectivePayload::Extended { .. } => extended_perspective(rows, &prefix, payload),
        PerspectivePayload::Matrix5 { matrix, .. } => matrix5_trace(rows, path, matrix),
    }
}

fn base_perspective(rows: &mut Trace, path: &str, base: &bessemer::display_lists::PerspectiveBase) {
    point(rows, &format!("{path}.eye_point"), &base.eye_point);
    double(rows, &format!("{path}.plane_distance"), base.plane_distance);
}

fn extended_perspective(rows: &mut Trace, path: &str, payload: &PerspectivePayload) {
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

fn matrix5_trace(rows: &mut Trace, path: &str, matrix: &[StoredDouble; 25]) {
    let prefix = format!("{path}.matrix5");
    array(rows, &prefix, 5);
    for (index, row) in matrix.as_chunks::<5>().0.iter().enumerate() {
        point(rows, &format!("{prefix}[{index}]"), row);
    }
}
