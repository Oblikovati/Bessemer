use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("surface_id_keeper", |reader| {
        super::fillets::keeper(reader, map)
    })?;
    initial_pointers(reader, map)?;
    for label in ["raw_858", "raw_898", "raw_870"] {
        reader.u8(label)?;
    }
    for (offset, class) in [
        ("8a0", "moCompEdge_c"),
        ("8a8", "moCompFace_c"),
        ("8b0", "moCompFace_c"),
    ] {
        declared(reader, map, &format!("pointer_{offset}"), &[class])?;
    }
    reader.i32("raw_8b8")?;
    model_lists::unsigned_words(reader, "array_8c0")?;
    late_pointers(reader, map)?;
    reader.u8("raw_874")?;
    if reader.i32("reference_presence_800")? != 0 {
        declared(
            reader,
            map,
            "reference_800",
            super::model_references::WRAPPER_CLASSES,
        )?;
    }
    reader.u8("raw_8e8")?;
    Ok(())
}

fn initial_pointers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["880", "888", "890"] {
        declared(reader, map, &format!("pointer_{offset}"), &["suObList"])?;
    }
    declared(reader, map, "pointer_808", DIMENSIONS)?;
    declared(reader, map, "pointer_810", &["moDisplayAngularDim_c"])?;
    declared(reader, map, "pointer_818", &["moDisplayDistanceDim_c"])
}

fn late_pointers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["820", "828", "830"] {
        declared(
            reader,
            map,
            &format!("pointer_{offset}"),
            &["moDisplayDistanceDim_c"],
        )?;
    }
    for offset in ["838", "840", "848"] {
        declared(reader, map, &format!("pointer_{offset}"), &["moCompEdge_c"])?;
    }
    declared(reader, map, "pointer_850", &["moCompVertex_c"])
}
