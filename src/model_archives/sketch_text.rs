use super::sketch_arrays::{integers, vectors};
use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("text")?;
    double_vector(reader, "point_38")?;
    reader.i32("unknown_10")?;
    reader.object(
        "character_format",
        map,
        ModelObjectRole::Declared(&["utCharFormat_c"]),
    )?;
    reader.i32("unknown_20")?;
    double_vector(reader, "point_50")?;
    reader.f64("unknown_68")?;
    nested_points(reader)?;
    reader.i32("unknown_c4")?;
    if reader.i32("has_curve_data")? != 0 {
        curve_arrays(reader)?;
    }
    suffix(reader)
}

fn nested_points(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let groups = reader.signed_count("group_count", 4)?;
    let values = lengths(reader, "group_sizes", groups, 8)?;
    reader.entries(values * 2, "dimension-text values and point counts")?;
    integers(reader, "values_88", values)?;
    let points = lengths(reader, "point_counts_80", values, 24)?;
    reader.entries(points, "dimension-text points")?;
    vectors(reader, "points_78", points)
}

fn lengths(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    count: usize,
    width: usize,
) -> Result<usize> {
    let mut total = 0;
    for index in 0..count {
        let value = reader.i32(&format!("{label}[{index}]"))?;
        total = reader.checked(add_length(total, value, label))?;
    }
    let maximum = reader.remaining() / width;
    if total > maximum {
        return reader.checked(Err(super::invalid(format!(
            "{label} total {total}; expected <= {maximum} complete {width}-byte entries"
        ))));
    }
    Ok(total)
}

pub(super) fn add_length(total: usize, value: i32, label: &str) -> Result<usize> {
    usize::try_from(value)
        .ok()
        .and_then(|size| total.checked_add(size))
        .ok_or_else(|| {
            super::invalid(format!(
                "{label} count {value}, accumulated {total}; expected nonnegative sum within usize"
            ))
        })
}

fn curve_arrays(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.signed_count("curve_count", 52)?;
    reader.entries(count * 2, "dimension-text curve values and vectors")?;
    vectors(reader, "points_a8", count)?;
    integers(reader, "values_b0", count)?;
    vectors(reader, "vectors_b8", count)?;
    reader.i32("unknown_c0")?;
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("unknown_d0")?;
    reader.f64s(&["unknown_d8", "unknown_e0"])?;
    reader.i32("unknown_24")?;
    reader.scope("strings_f0", |reader| {
        for index in 0..reader.count_with_minimum("count", 1)? {
            reader.text(&format!("items[{index}]"))?;
        }
        Ok(())
    })?;
    reader.i32s(&["unknown_118", "unknown_120", "unknown_124"])?;
    reader.f64("unknown_128")?;
    reader.i32("unknown_130")?;
    Ok(())
}
