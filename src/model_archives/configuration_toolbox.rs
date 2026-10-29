use super::raw_fields::RawScalarKind::{Double, Signed, Text};
use super::{ModelDecoder, ModelObjectRole, model_lists, raw_fields};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)?;
    raw_fields::groups(
        reader,
        &[
            (Text, "20 30 38 40 48 50 58 60"),
            (Signed, "7c"),
            (Text, "70"),
            (Signed, "80"),
        ],
    )
}

pub(super) fn item(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    raw_fields::groups(
        reader,
        &[
            (Signed, "08"),
            (Text, "10 18"),
            (Double, "20"),
            (Signed, "28 2c"),
        ],
    )
}
