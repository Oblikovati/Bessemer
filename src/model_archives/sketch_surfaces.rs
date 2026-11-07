use super::{ModelDecoder, sketch_entities, sketch_geometry};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let layouts: &[(&str, &str, sketch_geometry::DirectReader)] = &[
        ("planes_count", "planes", plane),
        ("cylinders_count", "cylinders", cylinder),
        ("spheres_count", "spheres", sphere),
        ("cones_count", "cones", cone),
        (
            "parametric_surfaces_count",
            "parametric_surfaces",
            parametric,
        ),
    ];
    for (count, label, body) in layouts {
        sketch_geometry::collection(reader, map, (count, label), None, *body)?;
    }
    Ok(())
}

fn plane(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    parametric(reader, map)?;
    for label in ["origin", "x_axis", "y_axis"] {
        double_vector(reader, label)?;
    }
    reader.f64s(&["half_width", "half_height", "center_x", "center_y"])?;
    for index in 0..4 {
        reader.f64(&format!("border[{index}]"))?;
    }
    reader.u8("auto_sizeable")?;
    plane_suffix(reader)
}

fn plane_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.u8("has_string_188")? != 0 {
        reader.text("string_188")?;
    }
    reader.f64("unknown_190")?;
    if reader.u8("has_point_158")? != 0 {
        double_vector(reader, "point_158")?;
    }
    Ok(())
}

fn cylinder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    parametric(reader, map)?;
    double_vector(reader, "axis")?;
    double_vector(reader, "origin")?;
    reader.f64("radius")
}

fn sphere(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    parametric(reader, map)?;
    double_vector(reader, "center")?;
    reader.f64("radius")
}

fn cone(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    cylinder(reader, map)?;
    reader.f64s(&["sine", "cosine"])
}

fn parametric(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("entity", |reader| sketch_entities::entity(reader, map))?;
    Ok(())
}
