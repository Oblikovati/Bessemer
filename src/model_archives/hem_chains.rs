use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const EDGES: &[&str] = &["moCompEdge_c", "moCompSilhEdge_c"];

/// Retain the chain, profile and edge pointers in their native order.
/// Example: `hem_chains::chain_data(reader, map)?` for one chain-data record.
pub(super) fn chain_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "chain_08", &["moHemEdgeChain_c"])?;
    declared(reader, map, "profile_10", super::profile_features::CLASSES)?;
    reader.i32("raw_28")?;
    declared(reader, map, "edge_60", EDGES)
}

/// Preserve a chain's direct array without serializing runtime-only members.
/// Example: `hem_chains::chain(reader, map)?` for a native chain object.
pub(super) fn chain(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_08", ModelObjectRole::Any)
}

/// Decode an element's sole tagged edge reference.
/// Example: `hem_chains::element(reader, map)?` for a chain element.
pub(super) fn element(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "edge_08", EDGES)
}

/// Retain all relief option words and exact floating-point payload bits.
/// Example: `hem_chains::relief(reader)?` for a complete relief body.
pub(super) fn relief(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c"])?;
    reader.f64s(&["raw_10", "raw_18", "raw_20"])?;
    reader.i32("raw_28")?;
    Ok(())
}
