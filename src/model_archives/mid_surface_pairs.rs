use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::ObjectReader;

const COMPONENT_EDGES: &[&str] = &["moCompEdge_c", "moCompSilhEdge_c"];

/// Preserve shared faces and a distance dimension; e.g. `faces(reader, map)?`.
pub(super) fn faces(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "face_08", &["moSingleFaceRef_w"])?;
    declared(reader, map, "face_10", &["moSingleFaceRef_w"])?;
    declared(reader, map, "dimension_18", &["moDisplayDistanceDim_c"])
}

/// Retain a direct edge array before its face pointer; e.g. `face_edges(reader, map)?`.
pub(super) fn face_edges(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_10", ModelObjectRole::Any)?;
    declared(reader, map, "face_08", &["moCompFace_c"])
}

/// Preserve nullable component-edge identities; e.g. `edges(reader, map)?`.
pub(super) fn edges(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "edge_08", COMPONENT_EDGES)?;
    declared(reader, map, "edge_10", COMPONENT_EDGES)
}
