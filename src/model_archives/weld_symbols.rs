use super::{
    ModelDecoder, ModelObjectRole, component_detail_references, component_references, dimensions,
    sketch_text,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord, vectors::double_vector};

const DETAILS: &[&str] = &[
    // #54 / C-208: note data inherits this exact detail-item prefix.
    "moNoteData_c",
    "moDetailItem_c",
    "moWeldSymbol_c",
    "moCosmeticWeldBeadSymbol_c",
    "moSFSymbol_c",
];
// #54 / 11.125 proves the complete ordinary parametric attachment subtype.
const ATTACHMENTS: &[&str] = &[
    // #54 / C-209: the absolute-point dispatch has the complete attachment body and no suffix.
    "moAbsolutePoint_c",
    "moAttachPoint_c",
    "moCosmeticWeldBeadAttachPoint_c",
    "moParametricPoint_c",
];

// #54 / C-387 and C-388 prove these exact native ancestry bindings and complete readers.
pub(super) fn assignable(record: &ObjectRecord, role: ModelObjectRole<'_>) -> bool {
    let ModelObjectRole::Declared(expected) = role else {
        return false;
    };
    let class = match record {
        ObjectRecord::Null => return false,
        ObjectRecord::New { class, .. } | ObjectRecord::Reference { class, .. } => {
            class.name.as_str()
        }
    };
    (expected.contains(&"moDetailItem_c") && DETAILS.contains(&class))
        || (expected.contains(&"moAttachPoint_c") && ATTACHMENTS.contains(&class))
}

/// Retain the native weld body and all three directly owned text instances.
/// Example: `weld_symbols::read(reader, map)?` at the exact symbol dispatch.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("detail", |reader| {
        component_detail_references::detail_item(reader, map)
    })?;
    weld_fields(reader)?;
    weld_references(reader, map)
}

fn weld_fields(reader: &mut ModelDecoder<'_>) -> Result<()> {
    strings(reader, 0x88, 18)?;
    reader.f64("raw_130")?;
    double_vector(reader, "vector_138")?;
    reader.u16("raw_128")?;
    reader.i32s(&["raw_118", "raw_11c", "raw_120", "raw_124"])?;
    strings(reader, 0x148, 5)?;
    reader.u16("raw_140")?;
    reader.u16("raw_170")?;
    Ok(())
}

fn weld_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["178", "180"] {
        reader.object(
            &format!("surface_finish_{offset}"),
            map,
            ModelObjectRole::Declared(&["moSFSymbol_c"]),
        )?;
    }
    for offset in ["188", "2e0", "438"] {
        reader.scope(&format!("text_{offset}"), |reader| {
            sketch_text::read(reader, map)
        })?;
    }
    Ok(())
}

fn strings(reader: &mut ModelDecoder<'_>, base: usize, count: usize) -> Result<()> {
    for index in 0..count {
        reader.text(&format!("string_{:x}", base + index * 8))?;
    }
    Ok(())
}

/// Preserve the exact forty-byte cosmetic suffix after the common weld.
/// Example: `weld_symbols::cosmetic(reader, map)?` for the concrete cosmetic class.
pub(super) fn cosmetic(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("weld", |reader| read(reader, map))?;
    reader.i32s(&["raw_590", "raw_594", "raw_598", "raw_59c"])?;
    reader.f64s(&["raw_5a0", "raw_5a8"])?;
    reader.i32s(&["raw_5b0", "raw_5b4"])
}

/// Reuse the complete attachment base before its feature reference and raw double.
/// Example: `weld_symbols::attachment(reader, map)?` for a cosmetic attachment.
pub(super) fn attachment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("attachment", |reader| dimensions::attachment(reader, map))?;
    reader.object(
        "feature_108",
        map,
        ModelObjectRole::Declared(component_references::FEATURE_CLASSES),
    )?;
    reader.f64("raw_110")
}

/// Preserve the complete surface-finish base, direct text and strict late-version gates.
/// Example: `weld_symbols::surface_finish(reader, map)?` at its exact class dispatch.
pub(super) fn surface_finish(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("detail", |reader| {
        component_detail_references::detail_item(reader, map)
    })?;
    reader.i32s(&["raw_8c", "raw_90"])?;
    strings(reader, 0x98, 10)?;
    reader.i32("raw_104")?;
    reader.scope("text_108", |reader| sketch_text::read(reader, map))?;
    reader.i32("raw_100")?;
    finish_suffix(reader)
}

fn finish_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x42de {
        reader.i32("raw_f0")?;
        reader.f64("raw_f8")?;
    }
    if reader.version() > 0x46bf {
        strings(reader, 0x260, 2)?;
        reader.i32s(&["raw_270", "raw_274"])?;
    }
    Ok(())
}
