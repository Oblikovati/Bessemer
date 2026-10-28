use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, features, kernel_frames, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) const DISPLAY_ITEMS: &[&str] = &[
    "moDisplayItem_c",
    "moDisplayWeld_c",
    "moDisplayDim_c",
    "moDisplayDistanceDim_c",
    "moDisplayRadialDim_c",
    "moDisplayAngularDim_c",
    "moDisplayRevolveDim_c",
];

/// Preserve the complete feature prefix before the tagged property list.
/// Example: `cosmetic_weld_tree::subfolder(reader, map)?` for a subfolder.
pub(super) fn subfolder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("feature", |reader| features::read(reader, map))?;
    declared(reader, map, "properties_350", &["moCustomPropertyList"])?;
    reader.i32("raw_358")?;
    Ok(())
}

/// Retain typed bead payload and display-item identities after the feature.
/// Example: `cosmetic_weld_tree::bead(reader, map)?` for a bead.
pub(super) fn bead(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("feature", |reader| features::read(reader, map))?;
    declared(reader, map, "data_6b8", &["moCosmeticWeldBeadData_c"])?;
    // #54 / C-387 and the display-dimension readers prove these direct item prefixes.
    declared(reader, map, "display_6c0", DISPLAY_ITEMS)?;
    reader.i32s(&["raw_6c8", "raw_6cc"])
}

/// Decode the constructor-owned arrays and endpoints without outer object tags.
/// Example: `cosmetic_weld_tree::bead_data(reader, map)?` for a bead payload.
pub(super) fn bead_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    arrays(reader, map, &["48", "68"])?;
    for offset in ["e8", "110"] {
        reader.scope(&format!("endpoint_{offset}"), endpoint)?;
    }
    reader.i32("raw_1b8")?;
    parameters(reader, map)?;
    reader.f64("raw_1e0")?;
    reader.scope("body_1e8", kernel_frames::read_body)?;
    arrays(reader, map, &["270"])?;
    reader.f64("raw_290")?;
    arrays(reader, map, &["08", "28", "138"])?;
    reader.i32("raw_1bc")?;
    arrays(reader, map, &["198", "158", "178"])
}

fn parameters(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "from_to_1c0", &["moCWeldBeadFromToParams_c"])?;
    declared(
        reader,
        map,
        "intermittent_1c8",
        &["moIntermittentCWeldBeadParams_c"],
    )?;
    declared(reader, map, "symbol_1d0", &["moCosmeticWeldBeadSymbol_c"])
}

fn arrays(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        model_lists::pointers(
            reader,
            map,
            &format!("array_{offset}"),
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}

fn endpoint(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64s(&["raw_08", "raw_10"])?;
    reader.i32s(&["raw_18", "raw_1c"])?;
    reader.f64("raw_20")?;
    Ok(())
}

/// Preserve the fixed twenty-byte from/to parameter record.
/// Example: `cosmetic_weld_tree::from_to(reader)?` for the exact parameter class.
pub(super) fn from_to(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    reader.f64s(&["raw_10", "raw_18"])
}

/// Preserve the fixed twenty-four-byte intermittent parameter record.
/// Example: `cosmetic_weld_tree::intermittent(reader)?` for intermittent parameters.
pub(super) fn intermittent(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c"])?;
    reader.f64s(&["raw_10", "raw_18"])
}

/// Keep signed list counts, including negative empty lists, without padding.
/// Example: `cosmetic_weld_tree::wire_map(reader)?` for an edge-to-face map.
pub(super) fn wire_map(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u32("raw_08")?;
    for offset in ["18", "28"] {
        let label = format!("values_{offset}");
        let count = super::configuration_io::positive_count(reader, &format!("{label}.count"), 4)?;
        sketch_arrays::integers(reader, &label, count)?;
    }
    Ok(())
}
