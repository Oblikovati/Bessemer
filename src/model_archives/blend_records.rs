use super::display_dimension::CLASSES as DIMENSION_CLASSES;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, invalid, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const DIRECTIONS: &[&str] = &["moDirectionSpec_c"];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let (dimensions, second) = reader.scope("data", |reader| contents(reader, map))?;
    reader.i32("raw_c80")?;
    for label in ["c84", "c88", "c8c", "c90"] {
        reader.u8(&format!("raw_{label}"))?;
    }
    if dimensions != 0 {
        declared(reader, map, "dimension_c70", DIMENSION_CLASSES)?;
    }
    if dimensions != 0 && second == 3 {
        declared(reader, map, "dimension_c78", DIMENSION_CLASSES)?;
    }
    reader.u8("raw_c94")?;
    Ok(())
}

fn contents(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<(u8, i32)> {
    let declared = reader.i32("declared_count_418")?;
    if declared < 0 {
        return reader.checked(Err(invalid(format!(
            "blend declared count {declared}; expected a nonnegative preserved i32",
        ))));
    }
    let count = model_lists::counted_pointers(reader, map, "array_330", ModelObjectRole::Any)?;
    array(reader, map, "310")?;
    array(reader, map, "350")?;
    super::blend_columns::read(reader, count)?;
    directions(reader, map)?;
    model_lists::pointers(reader, map, "list_80", ModelObjectRole::Any)?;
    model_lists::pointers(reader, map, "list_c8", ModelObjectRole::Any)?;
    tail(reader, map)
}

fn directions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["raw_110", "raw_114"])?;
    reader.f64("raw_118")?;
    reader.f64("raw_120")?;
    for label in ["300", "374", "378"] {
        reader.u8(&format!("raw_{label}"))?;
    }
    reader.f64("raw_58")?;
    let first = reader.i32("raw_60")?;
    let second = reader.i32("raw_64")?;
    for (label, value) in [("68", first), ("70", second)] {
        if value & 255 == 2 {
            declared(reader, map, &format!("direction_{label}"), DIRECTIONS)?;
        }
    }
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<(u8, i32)> {
    reader.i32("raw_230")?;
    reader.f64("raw_420")?;
    reader.f64("raw_428")?;
    let dimensions = reader.u8("raw_234")?;
    reader.f64("raw_238")?;
    reader.f64("raw_240")?;
    let second = reader.i32("raw_248")?;
    reader.i32("raw_24c")?;
    for label in ["380", "3a0", "3c0", "3e0"] {
        array(reader, map, label)?;
    }
    reader.i32s(&["raw_78", "raw_7c", "raw_304", "raw_308"])?;
    Ok((dimensions, second))
}

fn array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    model_lists::pointers(reader, map, &format!("array_{label}"), ModelObjectRole::Any)
}
