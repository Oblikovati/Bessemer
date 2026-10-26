use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) const CLASSES: &[&str] = &[
    "moPointBackedUpData_c",
    "moLineBackedUpData_c",
    "moSegmentBackedUpData_c",
    "moArcBackedUpData_c",
    "moEllipseBackedUpData_c",
    "mo3DPlaneBackedUpData_c",
    "mo3DSphereBackedUpData_c",
];
const POINT: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moPointBackedUpData_c"]);
const LINE: ModelObjectRole<'_> =
    ModelObjectRole::Declared(&["moLineBackedUpData_c", "moSegmentBackedUpData_c"]);

pub(super) fn point(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    double_vector(reader, "point_08")
}

pub(super) fn plane(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["point_08", "vector_20", "vector_38"] {
        double_vector(reader, label)?;
    }
    Ok(())
}

pub(super) fn sphere(reader: &mut ModelDecoder<'_>) -> Result<()> {
    point(reader)?;
    reader.f64("raw_20")
}

pub(super) fn line(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..2 {
        reader.object(&format!("points[{index}]"), map, POINT)?;
    }
    reader.i32s(&["unknown_18", "unknown_1c"])
}

pub(super) fn arc(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("line_base", |reader| line(reader, map))?;
    reader.i32("word_20")?;
    for (name, role) in [
        ("point_28", POINT),
        ("point_30", POINT),
        ("line_38", LINE),
        ("line_40", LINE),
        ("point_48", POINT),
    ] {
        reader.object(name, map, role)?;
    }
    Ok(())
}

pub(super) fn ellipse(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("line_base", |reader| line(reader, map))?;
    reader.i32("word_20")?;
    for offset in ["28", "30", "38", "40", "48"] {
        reader.object(&format!("point_{offset}"), map, POINT)?;
    }
    Ok(())
}
