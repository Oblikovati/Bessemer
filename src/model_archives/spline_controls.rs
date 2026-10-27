use super::{ModelDecoder, ModelObjectRole, sketch_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

fn sketch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("sketch_08", map, ModelObjectRole::Declared(&["sgSketch"]))
}

fn derivative(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch(reader, map)?;
    reader.scope("spline_handle_40", sketch_references::handle)?;
    reader.scope("point_handle_18", sketch_references::handle)
}

pub(super) fn polygon(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch(reader, map)?;
    reader.scope("spline_handle_18", sketch_references::handle)
}

pub(super) fn tangency(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    derivative(reader, map)?;
    double_vector(reader, "point_128")?;
    double_vector(reader, "vector_140")
}

pub(super) fn curvature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    derivative(reader, map)?;
    for label in ["point_c0", "vector_d8", "vector_f0"] {
        double_vector(reader, label)?;
    }
    reader.f64("raw_108")?;
    Ok(())
}
