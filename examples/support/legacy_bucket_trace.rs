use crate::view_trace::Trace;
use bessemer::display_lists::{DisplayArchive, DisplayFieldValue};

#[derive(Default)]
struct PolygonFlags {
    flags: u32,
    count: i32,
    explicit_layer_flags: bool,
}

pub(super) fn append(archive: &DisplayArchive, rows: &mut Trace) {
    if archive.context().version.value() > 0x41a0 {
        return;
    }
    let mut polygons = std::collections::BTreeMap::<&str, PolygonFlags>::new();
    for field in &archive.contents().fields {
        append_field(&mut polygons, field);
    }
    for (prefix, mut polygon) in polygons {
        if polygon.count > 0 && !polygon.explicit_layer_flags {
            polygon.flags |= 0x300;
        }
        rows.insert(format!("{prefix}.flags"), format!("i:{}", polygon.flags));
    }
}

fn append_field<'a>(
    polygons: &mut std::collections::BTreeMap<&'a str, PolygonFlags>,
    field: &'a bessemer::display_lists::DisplayField,
) {
    if !field.path.contains(".view_bucket.") {
        return;
    }
    let Some((prefix, leaf)) = field.path.rsplit_once('.') else {
        return;
    };
    let DisplayFieldValue::I32(value) = field.value else {
        return;
    };
    if leaf == "smooth" {
        polygons.insert(prefix, PolygonFlags::default());
    }
    if let Some(polygon) = polygons.get_mut(prefix) {
        update(polygon, leaf, value);
    }
}

fn update(polygon: &mut PolygonFlags, leaf: &str, value: i32) {
    if leaf == "count" {
        polygon.count = value;
        return;
    }
    let Some(bit) = flag_mask(leaf) else {
        return;
    };
    if matches!(leaf, "layer_visible" | "layer_printable") {
        polygon.explicit_layer_flags = true;
    }
    if value != 0 {
        polygon.flags |= bit;
    }
}

fn flag_mask(leaf: &str) -> Option<u32> {
    Some(match leaf {
        "smooth" => 2,
        "new_boundary" => 4,
        "new_region" => 8,
        "bos" => 0x40,
        "section_cut_new" => 0x80,
        "layer_visible" => 0x100,
        "layer_printable" => 0x200,
        "has_arc_data" => 0x20,
        "has_silh_data" => 0x10,
        _ => return None,
    })
}
