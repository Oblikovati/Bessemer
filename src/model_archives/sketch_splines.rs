use super::{
    ModelDecoder, ModelObjectRole, kernel_frames, sketch_arrays, sketch_geometry, spline_arrays,
};
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord, vectors::double_vector};
use crate::{BessemerError, FailureClass, Result};

const LISTS: &[&str] = &["sgSplineList_c", "modifSplineList_c", "cptsSplineList_c"];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("spline_count", 2)? {
        reader.scope(&format!("splines[{index}]"), |reader| {
            let mask = reader.u16("stored_category_mask")?;
            reader.checked(require_subtype(mask))?;
            spline(reader, map)
        })?;
    }
    Ok(())
}

pub(super) fn require_subtype(mask: u16) -> Result<()> {
    if mask & !0x1d == 0 {
        return Ok(());
    }
    Err(BessemerError::new(
        "model archive",
        "spline subtype",
        FailureClass::Unsupported,
        format!("spline subtype mask {mask:#06x}; expected only bits 0x001d"),
    ))
}

fn spline(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let entity = sketch_geometry::segment(reader, map)?;
    let flags = reader.u32("flags_32c")?;
    reader.u16("unknown_2fc")?;
    let control_list = point_list(reader, map, entity.flags_54 & 1 != 0)?;
    prefix_fields(reader)?;
    let controls = parameter_arrays(reader, flags)?;
    tangents(reader)?;
    reader.scope("interpolation", spline_arrays::interpolation)?;
    interpolation_suffix(reader, controls)?;
    equation_fields(reader, flags)?;
    reader.i32("reserved_word")?;
    control_suffix(reader, flags, control_list)?;
    final_fields(reader, map)
}

fn point_list(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    inactive: bool,
) -> Result<bool> {
    let record = reader.object_record("point_list", map, ModelObjectRole::Declared(LISTS))?;
    let class = match &record {
        ObjectRecord::Null if inactive => return Ok(false),
        ObjectRecord::Null => {
            return reader.checked(Err(super::invalid(
                "point list Null; expected nonnull active spline point list",
            )));
        }
        ObjectRecord::New { class, .. } | ObjectRecord::Reference { class, .. } => class,
    };
    reader.referenced_count(&record, "items.count")?;
    Ok(class.name == "cptsSplineList_c")
}

fn prefix_fields(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.u16("unknown_170")?;
    reader.u16("unknown_f0")?;
    reader.i32("unknown_f4")?;
    reader.f64s(&["unknown_f8", "unknown_100", "unknown_180"])?;
    reader.i32("unknown_154")?;
    double_vector(reader, "vector_118")?;
    double_vector(reader, "vector_130")?;
    reader.u16("unknown_2f8")?;
    Ok(())
}

fn parameter_arrays(reader: &mut ModelDecoder<'_>, flags: u32) -> Result<usize> {
    let count = reader.signed_count("parameter_count", 8)?;
    sketch_arrays::doubles(reader, "parameters_148", count)?;
    if flags & 1 != 0 {
        return Ok(0);
    }
    let count = reader.signed_count("control_point_count", 24)?;
    sketch_arrays::vectors(reader, "control_points_300", count)?;
    Ok(count)
}

fn tangents(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.unsigned_count("tangent_count", 30)? {
        reader.scope(&format!("tangents[{index}]"), |reader| {
            reader.i32("index")?;
            let mode = reader.u16("mode")?;
            double_vector(reader, "vector")?;
            if mode == 2 {
                reader.f64("value")?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

fn interpolation_suffix(reader: &mut ModelDecoder<'_>, controls: usize) -> Result<()> {
    let secondary = reader.i32("unknown_314")?;
    reader.u16("unknown_318")?;
    reader.u16("unknown_31c")?;
    if secondary != 0 && controls != 0 {
        reader.scope("secondary_interpolation", spline_arrays::secondary)?;
    }
    reader.i32("unknown_174")?;
    Ok(())
}

fn equation_fields(reader: &mut ModelDecoder<'_>, flags: u32) -> Result<()> {
    if flags & 4 == 0 {
        return Ok(());
    }
    reader.entries(6, "spline equation strings")?;
    for index in 0..6 {
        reader.text(&format!("equation_strings[{index}]"))?;
    }
    reader.i32s(&["equation_word", "unknown_1e0"])?;
    reader.transform("transform_1e8")
}

fn control_suffix(reader: &mut ModelDecoder<'_>, flags: u32, control_list: bool) -> Result<()> {
    if !control_list {
        return Ok(());
    }
    if flags & 1 == 0 {
        return spline_arrays::tolerance(reader);
    }
    reader.scope("curve", spline_arrays::curve)
}

fn final_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.f64("unknown_188")?;
    for index in 0..reader.unsigned_count("fixed_length_count", 6)? {
        reader.scope(&format!("fixed_length_data[{index}]"), |reader| {
            reader.i32("index")?;
            reader.object(
                "data",
                map,
                ModelObjectRole::Declared(&["sgFixedLengthSplineControlPointData"]),
            )
        })?;
    }
    reader.u16("unknown_340")?;
    if reader.u16("has_kernel_body")? != 0 {
        reader.scope("kernel_body", kernel_frames::read_body)?;
    }
    reader.i32("unknown_190")?;
    Ok(())
}
