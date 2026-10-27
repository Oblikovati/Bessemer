use super::display_records::declared;
use super::geometric_references::LINE_REFERENCE_CLASSES;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

// #46 / C-188 and C-232: end-spec point roles retain the verified point-base ancestry.
const END_POINT_ROLE: ModelObjectRole<'_> =
    ModelObjectRole::Declared(super::geometric_references::POINT_REFERENCE_CLASSES);

fn dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    declared(reader, map, label, super::display_dimension::CLASSES)
}

fn dimension_group(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offsets: &[&str],
) -> Result<()> {
    for offset in offsets {
        dimension(reader, map, &format!("dimension_{offset}"))?;
    }
    Ok(())
}

pub(super) fn extrusion(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "end_spec", &["moEndSpec_c"])?;
    reader.f64("value_830")?;
    reader.i32("word_808")?;
    Ok(())
}

pub(super) fn from(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    let kind = reader.i32("type")?;
    if (1..=2).contains(&kind) {
        return declared(reader, map, "reference", &["moRefWrapper_c"]);
    }
    if !(3..=5).contains(&kind) {
        return Ok(());
    }
    if kind == 4 {
        reader.f64("value_18")?;
    } else {
        dimension(reader, map, "dimension")?;
    }
    reader.i32("word_20")?;
    Ok(())
}

pub(super) fn end(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "direction", &["moDirectionSpec_c"])?;
    reader.i32s(&["word_08", "word_88", "word_8c"])?;
    declared(reader, map, "face", &["moFaceRef_c"])?;
    let first = reader.i32("type_0c")?;
    let second = reader.i32("type_10")?;
    dimension_group(reader, map, &["18", "20"])?;
    end_arrays(reader, map, first, second)?;
    reader.f64("value_a0")?;
    reader.i32s(&[
        "word_b8", "word_bc", "word_c0", "word_c4", "word_b0", "word_b4",
    ])?;
    dimension_group(reader, map, &["c8", "d0"])?;
    end_tail(reader, map, first, second)
}

fn end_arrays(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    first: i32,
    second: i32,
) -> Result<()> {
    for (kind, point, offset) in [(first, "68", "28"), (second, "70", "48")] {
        if kind == 3 {
            reader.object(&format!("point_{point}"), map, END_POINT_ROLE)?;
        }
        if reader.u8(&format!("has_array_{offset}"))? != 0 {
            reader.scope(&format!("array_{offset}"), |reader| {
                model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
            })?;
        }
    }
    Ok(())
}

fn end_tail(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    first: i32,
    second: i32,
) -> Result<()> {
    reader.i32s(&["word_a8", "word_ac", "word_d8", "word_dc"])?;
    for label in ["array_e0", "array_100"] {
        reader.scope(label, |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    reader.i32s(&["word_128", "word_12c", "word_130"])?;
    end_references(reader, map, first, second)?;
    declared(reader, map, "from", &["moFromEndSpec_c"])?;
    reader.i32s(&["word_148", "word_14c", "word_150", "word_154"])?;
    if reader.version() >= 0x3b01 {
        reader.i32("word_158")?;
    }
    Ok(())
}

fn end_references(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    first: i32,
    second: i32,
) -> Result<()> {
    for (kind, offset) in [(first, "78"), (second, "80")] {
        if kind == 7 {
            declared(
                reader,
                map,
                &format!("reference_{offset}"),
                &["moRefWrapper_c"],
            )?;
        }
    }
    Ok(())
}

pub(super) fn revolution_end(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["word_08", "word_138", "word_13c", "word_0c", "word_10"])?;
    for offset in ["118", "120"] {
        declared(
            reader,
            map,
            &format!("reference_{offset}"),
            &["moRefWrapper_c"],
        )?;
    }
    for offset in ["128", "130"] {
        reader.object(&format!("point_{offset}"), map, END_POINT_ROLE)?;
    }
    reader.f64s(&["value_38", "value_40"])?;
    reader.i32s(&["word_140", "word_144"])?;
    dimension_group(reader, map, &["18", "20", "28", "30"])
}

pub(super) fn revolution(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    dimension(reader, map, "dimension_800")?;
    reader.u8("byte_81c")?;
    dimension(reader, map, "dimension_808")?;
    reader.i32("word_818")?;
    declared(reader, map, "list_810", &["suObList"])?;
    reader.i32s(&["word_820", "word_868"])?;
    declared(reader, map, "axis", LINE_REFERENCE_CLASSES)?;
    declared(reader, map, "end_spec", &["moRevEndSpec_c"])?;
    if reader.version() >= 0x3f05 {
        reader.u8("byte_8e0")?;
    }
    Ok(())
}

pub(super) fn rib(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("word_834")?;
    reader.u32("word_828")?;
    for (offset, class) in [
        ("838", "moDisplayDistanceDim_c"),
        ("848", "moDisplayAngularDim_c"),
        ("850", "moDisplayRadialDim_c"),
        ("858", "moDisplayDistanceDim_c"),
    ] {
        declared(reader, map, &format!("dimension_{offset}"), &[class])?;
    }
    declared(reader, map, "solid_body", &["moCompSolidBody_c"])
}
