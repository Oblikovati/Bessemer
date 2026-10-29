use super::{ModelDecoder, ModelObjectRole, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the direct derived-feature collections before the stock-specific body.
/// Example: `stock_collections::derived(reader, map)?`.
pub(super) fn derived(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2b68 {
        for label in ["pairs_800", "pairs_880"] {
            super::configuration_io::integer_pairs(reader, label, false)?;
        }
        let count = reader.unsigned_count("words_900.count", 4)?;
        sketch_arrays::integers(reader, "words_900", count)?;
    }
    if reader.version() >= 0x373c {
        holes(reader, map, "records_950")?;
    }
    Ok(())
}

fn holes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 10)? {
        reader.i32(&format!("{label}[{index}].raw_00"))?;
        reader.i32(&format!("{label}[{index}].raw_04"))?;
        reader.object(
            &format!("{label}[{index}].hole_info"),
            map,
            ModelObjectRole::Declared(&["moHoleWizardInfo_c"]),
        )?;
    }
    Ok(())
}

/// Keep signed stock pair counts distinct from the inherited unsigned counts.
/// Example: `stock_collections::pairs(reader, "pairs_cc8")?`.
pub(super) fn pairs(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 8)? {
        reader.i32(&format!("{label}[{index}].first"))?;
        reader.i32(&format!("{label}[{index}].second"))?;
    }
    Ok(())
}

/// Decode the complete stock tail without normalizing duplicates into runtime sets.
/// Example: `stock_collections::late(reader, map)?`.
pub(super) fn late(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u32("raw_ddc")?;
    reader.i32("raw_de0")?;
    reader.scope("array_de8", super::sketch_intersections::dynamic_words)?;
    pairs(reader, "pairs_df8")?;
    edge_helpers(reader, map)?;
    reader.i32("raw_dd8")?;
    edge_sets(reader, map)?;
    reader.i32("raw_eb0")?;
    pairs(reader, "pairs_eb8")?;
    pairs(reader, "pairs_e90")?;
    if 0x3335 < reader.version() && reader.version() < 0x373c {
        holes(reader, map, "legacy_hole_info_950")?;
    }
    Ok(())
}

fn edge_helpers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("pairs_e20.count", 4)? {
        reader.object(
            &format!("pairs_e20[{index}].edge"),
            map,
            ModelObjectRole::Declared(&["moCompEdge_c"]),
        )?;
        reader.object(
            &format!("pairs_e20[{index}].helper"),
            map,
            ModelObjectRole::Declared(&["moMirrorHWHelper_c"]),
        )?;
    }
    Ok(())
}

fn edge_sets(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("sets_e60.count", 6)? {
        reader.object(
            &format!("sets_e60[{index}].edge"),
            map,
            ModelObjectRole::Declared(&["moCompEdge_c"]),
        )?;
        let label = format!("sets_e60[{index}].members");
        for member in 0..reader.signed_count(&format!("{label}.count"), 2)? {
            reader.object(
                &format!("{label}[{member}]"),
                map,
                ModelObjectRole::Declared(&["moCompEdge_c"]),
            )?;
        }
    }
    Ok(())
}
