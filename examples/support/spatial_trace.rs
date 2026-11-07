use crate::view_trace::{
    Trace, array, compact, double, finish, integer, integers, point, raw_hex, text,
};
use bessemer::display_lists::{SpatialView, SpatialViews, ViewBitmap, ViewSectionPlane};

pub fn trace(archive: &SpatialViews) -> Vec<u8> {
    let mut rows = Trace::new();
    let Some(views) = archive.contents() else {
        rows.insert("root".into(), "none".into());
        return finish(rows);
    };
    array(&mut rows, "root", views.len());
    for (index, view) in views.iter().enumerate() {
        view_trace(&mut rows, &format!("root[{index}]"), view);
    }
    finish(rows)
}

fn view_trace(rows: &mut Trace, path: &str, view: &SpatialView) {
    header(rows, path, view);
    if let Some(bitmap) = &view.preview_bitmap {
        bitmap_trace(rows, &format!("{path}.preview_bitmap"), bitmap);
    }
    if let Some(value) = &view.unknown_ids_string {
        text(rows, &format!("{path}.unknown_ids_string"), value.text());
    }
    let prefix = format!("{path}.section_planes");
    array(rows, &prefix, view.section_planes.len());
    for (index, plane) in view.section_planes.iter().enumerate() {
        plane_trace(rows, &format!("{prefix}[{index}]"), plane);
    }
    tail(rows, path, view);
}

fn header(rows: &mut Trace, path: &str, view: &SpatialView) {
    text(rows, &format!("{path}.view_name"), view.view_name.text());
    if let Some(value) = view.unknown_active {
        integer(rows, &format!("{path}.unknown_active"), i64::from(value));
    }
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
}

fn bitmap_trace(rows: &mut Trace, path: &str, bitmap: &ViewBitmap) {
    integer(rows, &format!("{path}.width"), i64::from(bitmap.width));
    integer(rows, &format!("{path}.height"), i64::from(bitmap.height));
    integer(
        rows,
        &format!("{path}.width_bytes"),
        i64::from(bitmap.row_bytes),
    );
    integer(rows, &format!("{path}.planes"), i64::from(bitmap.planes));
    integer(
        rows,
        &format!("{path}.bits_pixel"),
        i64::from(bitmap.bits_per_pixel),
    );
    raw_hex(rows, &format!("{path}.bits"), &bitmap.bits, "b");
}

fn plane_trace(rows: &mut Trace, path: &str, plane: &ViewSectionPlane) {
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
    if let Some(value) = plane.unknown_word {
        integer(rows, &format!("{path}.unknown_u"), i64::from(value));
    }
}

fn tail(rows: &mut Trace, path: &str, view: &SpatialView) {
    if let Some(value) = view.unknown_after_planes {
        integer(
            rows,
            &format!("{path}.unknown_after_planes"),
            i64::from(value),
        );
    }
    if let Some(value) = view.unknown_468 {
        integer(rows, &format!("{path}.unknown_468"), i64::from(value));
    }
    if let Some(value) = view.unknown_bitset {
        integers(rows, &format!("{path}.unknown_bitset"), &value);
    }
    if let Some(value) = &view.unknown_text_string {
        text(rows, &format!("{path}.unknown_text_string"), value.text());
    }
}
