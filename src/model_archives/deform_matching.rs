use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, configuration_io};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Remember each parsed subcount for the later native matching pass.
/// Example: `let subcounts = deform_matching::first(reader, map)?;`.
pub(super) fn first(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<Vec<usize>> {
    reader.require_ordinary_part()?;
    let count = configuration_io::positive_count(reader, "declared_match_count", 28)?;
    let mut subcounts = Vec::with_capacity(count);
    for index in 0..count {
        subcounts.push(reader.scope(&format!("matches_8c8[{index}]"), |reader| {
            match_record(reader, map)
        })?);
    }
    Ok(subcounts)
}

fn match_record(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<usize> {
    for side in 0..2 {
        reader.scope(&format!("sides[{side}]"), |reader| {
            reader.scope("array", |reader| body_features::nullable_array(reader, map))?;
            reader.i32s(&["raw_20", "raw_30"])
        })?;
    }
    let count = configuration_io::positive_count(reader, "declared_subcount", 112)?;
    for index in 0..count {
        reader.scope(&format!("subdata[{index}]"), |reader| {
            subrecord(reader, map)
        })?;
    }
    Ok(count)
}

fn subrecord(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for side in 0..2 {
        reader.scope(&format!("sides[{side}]"), |reader| {
            reader.i32("raw_word")?;
            declared(
                reader,
                map,
                "curve_point",
                &["moCurvePointRef_w", "moEndPointRef_w"],
            )?;
            reader.object("reference", map, ModelObjectRole::Any)?;
            double_vector(reader, "point")?;
            double_vector(reader, "vector")
        })?;
    }
    Ok(())
}

/// Consume second-pass records in their original first-pass order.
/// Example: `deform_matching::second(reader, map, version, &subcounts)?;`.
pub(super) fn second(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    version: i32,
    subcounts: &[usize],
) -> Result<()> {
    reader.require_ordinary_part()?;
    if version < 9 {
        return Ok(());
    }
    for (index, count) in subcounts.iter().copied().enumerate() {
        reader.scope(&format!("matches_8c8[{index}]"), |reader| {
            second_record(reader, map, version, count)
        })?;
    }
    Ok(())
}

fn second_record(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    version: i32,
    count: usize,
) -> Result<()> {
    for side in 0..2 {
        reader.scope(&format!("second_sides[{side}]"), |reader| {
            reader.scope("array", |reader| body_features::nullable_array(reader, map))?;
            reader.i32("raw_word").map(|_| ())
        })?;
    }
    reader.i32("raw_48")?;
    if version < 10 {
        return Ok(());
    }
    second_vectors(reader, count)
}

fn second_vectors(reader: &mut ModelDecoder<'_>, count: usize) -> Result<()> {
    for index in 0..count {
        for vector in 0..2 {
            double_vector(reader, &format!("subdata[{index}].vectors_88_a0[{vector}]"))?;
        }
    }
    Ok(())
}
