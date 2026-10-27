use super::display_records::declared;
use super::{ModelDecoder, sheet_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    let first = sheet_arrays::count(reader, "array_50")?;
    let third = sheet_arrays::count(reader, "array_60")?;
    let second = sheet_arrays::count(reader, "array_70")?;
    initial(reader, map)?;
    table(reader, [first, second, third])?;
    strings(reader, map)
}

fn initial(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("raw_38")?;
    reader.i32("raw_40")?;
    reader.u8("raw_00")?;
    declared(
        reader,
        map,
        "dimension_08",
        super::display_dimension::CLASSES,
    )?;
    declared(reader, map, "dimension_10", &["moDisplayDistanceDim_c"])?;
    reader.i32s(&["raw_30", "raw_34"])
}

fn table(reader: &mut ModelDecoder<'_>, shape: [usize; 3]) -> Result<()> {
    let count = shape.into_iter().try_fold(1usize, usize::checked_mul);
    let Some(count) = count.filter(|count| *count <= reader.remaining() / 8) else {
        return reader.checked(Err(super::invalid(format!(
            "sheet-metal table shape {shape:?}; expected product <= {} remaining binary64 entries",
            reader.remaining() / 8,
        ))));
    };
    reader.entries(count, "sheet-metal allowance table")?;
    for index in 0..count {
        reader.f64(&format!("raw_table[{index}]"))?;
    }
    Ok(())
}

fn strings(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["raw_110", "raw_100", "raw_f8", "raw_108"] {
        reader.text(label)?;
    }
    if reader.version() > 0x2bae {
        if reader.version() < 0x2bba {
            reader.i32("raw_material_compat")?;
        }
        declared(reader, map, "material_128", &["moSMParamsFromMat_c"])?;
    }
    for label in ["strings_d0", "strings_a8", "strings_80"] {
        super::configuration_io::strings(reader, label)?;
    }
    Ok(())
}
