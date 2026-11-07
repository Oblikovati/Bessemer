use crate::view_trace::{Trace, array, compact, double, finish, integer, integers, point, text};
use bessemer::display_lists::{AnnotationSectionPlane, AnnotationView, AnnotationViews};

pub fn trace(archive: &AnnotationViews) -> Vec<u8> {
    let mut rows = Trace::new();
    let Some(contents) = archive.contents() else {
        rows.insert("root".into(), "none".into());
        return finish(rows);
    };
    array(&mut rows, "root.views", contents.views.len());
    for (index, view) in contents.views.iter().enumerate() {
        view_trace(&mut rows, &format!("root.views[{index}]"), view);
    }
    if let Some(bitset) = contents.unknown_bitset {
        integers(&mut rows, "root.unknown_bitset", &bitset);
    }
    finish(rows)
}

fn identity(rows: &mut Trace, path: &str, view: &AnnotationView) {
    text(rows, &format!("{path}.view_name"), view.view_name.text());
    integer(
        rows,
        &format!("{path}.view_id"),
        i64::from(view.view_id.value()),
    );
    integer(rows, &format!("{path}.is_shown"), i64::from(view.is_shown));
    integer(
        rows,
        &format!("{path}.is_current"),
        i64::from(view.is_current),
    );
    if let Some(flag) = view.unknown_flag {
        integer(rows, &format!("{path}.unknown_flag"), i64::from(flag));
    }
}

fn view_trace(rows: &mut Trace, path: &str, view: &AnnotationView) {
    identity(rows, path, view);
    text(
        rows,
        &format!("{path}.unknown_name2"),
        view.unknown_name2.text(),
    );
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
    planes_trace(rows, path, &view.section_planes);
}

fn planes_trace(rows: &mut Trace, path: &str, planes: &[AnnotationSectionPlane]) {
    let prefix = format!("{path}.section_planes");
    array(rows, &prefix, planes.len());
    for (index, plane) in planes.iter().enumerate() {
        plane_trace(rows, &format!("{prefix}[{index}]"), plane);
    }
}

fn plane_trace(rows: &mut Trace, path: &str, plane: &AnnotationSectionPlane) {
    point(rows, &format!("{path}.root_point"), &plane.root_point);
    point(rows, &format!("{path}.normal"), &plane.normal);
    double(rows, &format!("{path}.unknown_d"), plane.unknown_double);
    integer(
        rows,
        &format!("{path}.unknown_i"),
        i64::from(plane.unknown_integer),
    );
}
