use super::{ModelDecoder, sketch_arrays};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn interpolation(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.u32("count")?;
    if count == u32::MAX {
        return Ok(());
    }
    let width = reader.i32("width")?;
    reader.f64s(&["value_80", "value_88"])?;
    let shape = interpolation_shape(count, width, reader.remaining());
    let (entries, values) = reader.checked(shape)?;
    reader.entries(entries, "spline interpolation arrays")?;
    sketch_arrays::doubles(reader, "parameters", count as usize)?;
    sketch_arrays::words(reader, "words", count as usize)?;
    sketch_arrays::integers(reader, "indices", count as usize)?;
    sketch_arrays::doubles(reader, "values", values)
}

pub(super) fn interpolation_shape(
    count: u32,
    width: i32,
    remaining: usize,
) -> Result<(usize, usize)> {
    let count = count as usize;
    let values = usize::try_from(width)
        .ok()
        .and_then(|width| count.checked_mul(width));
    let shape = values.and_then(|values| {
        let bytes = count.checked_mul(14)?.checked_add(values.checked_mul(8)?)?;
        let entries = count.checked_mul(3)?.checked_add(values)?;
        (bytes <= remaining).then_some((entries, values))
    });
    shape.ok_or_else(|| super::invalid(format!("interpolation count {count}, width {width}, remaining {remaining}; expected nonnegative width and complete arrays within usize")))
}

pub(super) fn secondary(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.u32("count")?;
    reader.i32("width")?;
    reader.f64s(&["value_80", "value_88"])?;
    let count = reader.counted_entries("secondary spline parameters", i64::from(count), 8)?;
    sketch_arrays::doubles(reader, "parameters", count)
}

pub(super) fn curve(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let dimension = reader.u16("header_0")?;
    let knots = reader.u16("header_2")?;
    let points = reader.u16("header_4")?;
    let periodic = reader.u16("header_6")?;
    let count = reader.checked(curve_value_count(dimension, knots, points, periodic))?;
    let count = reader.counted_entries("spline curve values", count as i64, 8)?;
    sketch_arrays::doubles(reader, "values", count)?;
    tolerance(reader)
}

pub(super) fn curve_value_count(
    dimension: u16,
    knots: u16,
    points: u16,
    periodic: u16,
) -> Result<usize> {
    if ![0, 3, 4].contains(&dimension)
        || dimension == 0 && (knots != 0 || points != 0 || periodic != 0)
    {
        return Err(super::invalid(format!(
            "curve header ({dimension}, {knots}, {points}, {periodic}); expected dimension 3/4, or all-zero header"
        )));
    }
    if dimension == 0 {
        return Ok(0);
    }
    Ok((usize::from(dimension) + 1) * usize::from(points)
        + if periodic != 0 { 1 } else { usize::from(knots) })
}

pub(super) fn tolerance(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.u16("has_tolerance")? != 0 {
        reader.f64("tolerance")?;
    }
    Ok(())
}
