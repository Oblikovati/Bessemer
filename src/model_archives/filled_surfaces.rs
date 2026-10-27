use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn keeper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_48", ModelObjectRole::Any)
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["10", "30", "50"] {
        model_lists::pointers(
            reader,
            map,
            &format!("array_{offset}"),
            ModelObjectRole::Any,
        )?;
    }
    reader.i32("raw_70")?;
    // #54 / 11.231 / C-273: the owned keeper has no tag or presence word.
    reader.scope("keeper_118", |reader| keeper(reader, map))?;
    reader.i32s(&["raw_78", "raw_74", "raw_108"])?;
    reader.u8("raw_10c")?;
    reader.i32s(&["raw_08", "raw_148", "raw_150", "raw_154", "raw_14c"])
}

pub(super) fn contact(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_50", ModelObjectRole::Any)?;
    reader.f64("raw_90")?;
    reader.i32("raw_98")?;
    declared(reader, map, "angle_a0", &["moDisplayAngularDim_c"])?;
    reader.f64("raw_a8")?;
    reader.i32s(&["raw_b0", "raw_b8", "raw_c8", "raw_cc"])?;
    declared(reader, map, "synch_e0", &["moLoftSynchRef_c"])?;
    reader.i32s(&["raw_40", "raw_44", "raw_48"])?;
    reader.f64("raw_c0")
}

pub(super) fn contact_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(
        reader,
        map,
        "wrapper_18",
        super::model_references::WRAPPER_CLASSES,
    )?;
    reader.object("reference_08", map, ModelObjectRole::Any)?;
    reader.u32("raw_10")?;
    Ok(())
}
