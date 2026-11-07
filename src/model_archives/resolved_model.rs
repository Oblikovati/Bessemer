use super::{
    ModelDecoder, detail_definitions, environment, layers, model_arrays, post_views,
    resolved_start, views,
};
use crate::Result;
use crate::archive_codec::ObjectReader;

/// Read the common resolved stages before the root-specific reference and final tails.
/// Example: `resolved_model::prefix(reader, map)?` inside a bounded resolved section.
pub(super) fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    resolved_start::read(reader, map)?;
    reader.scope("detail_definitions", |reader| {
        detail_definitions::read(reader, map)
    })?;
    reader.scope("after_details", |reader| environment::read(reader, map))?;
    reader.scope("views", |reader| views::read(reader, map))?;
    reader.scope("postviews", |reader| post_views::read(reader, map))?;
    reader.scope("arrays", |reader| model_arrays::read(reader, map))?;
    reader.scope("layers", |reader| layers::read(reader, map))
}
