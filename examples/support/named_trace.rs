use crate::view_trace::{Trace, array, compact, double, finish, integer, integers, point, text};
use bessemer::display_lists::{NamedSectionPlane, NamedView, NamedViews};

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
    finish(rows)
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
