use super::{
    ModelDecoder, annotation_defaults, center_helpers, detail_helpers, detail_prefix, detail_tail,
};
use super::{dimension_defaults, label_defaults, table_defaults, weld_helpers};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    defaults(reader, map)?;
    helpers(reader, map)
}

fn defaults(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("prefix", detail_prefix::read)?;
    reader.scope("dimensions", |reader| dimension_defaults::read(reader, map))?;
    reader.scope("annotations", annotation_defaults::read)?;
    reader.scope("tables", table_defaults::read)?;
    reader.scope("labels", label_defaults::read)
}

fn helpers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("data_helpers", |reader| detail_helpers::read(reader, map))?;
    reader.scope("weld_gtol", |reader| weld_helpers::read(reader, map))?;
    reader.scope("center_helpers", center_helpers::read)?;
    reader.scope("tail", detail_tail::read)
}
