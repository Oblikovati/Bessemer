use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    reader.scope("surface", |reader| surface(reader, map))?;
    reader.scope("planar", |reader| {
        model_lists::pointers(reader, map, "array_7b0", ModelObjectRole::Any)?;
        reader.i32("raw_7b8")?;
        Ok(())
    })
}

pub(super) fn surface(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("archive_backend_marker")?;
    // The planar class's +AB0 returns zero: a nonzero marker never externalizes these bodies.
    sheets(reader, map)?;
    suffix(reader, map)
}

fn sheets(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("sheet_count", 3)? {
        reader.scope(&format!("sheets[{index}]"), |reader| {
            reader.scope("body", kernel_frames::read_body)?;
            model_lists::pointers(reader, map, "attributes", ModelObjectRole::Any)
        })?;
    }
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("extra_body_presence")? != 0 {
        let state = reader.scope("extra_body", kernel_frames::read_body_state)?;
        if state == kernel_frames::KernelBodyState::Serialized {
            model_lists::pointers(reader, map, "extra_body_attributes", ModelObjectRole::Any)?;
        }
    }
    if reader.archive_version().value() >= 0x2bcd {
        let count = reader.signed_count("raw_words_748.count", 4)?;
        super::sketch_arrays::integers(reader, "raw_words_748", count)?;
    }
    Ok(())
}

pub(super) fn keeper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["array_08", "array_48"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    reader.u32("raw_90")?;
    for index in 0..reader.count_with_minimum("entries_88.count", 4)? {
        let label = format!("entries_88[{index}]");
        model_lists::pointers(
            reader,
            map,
            &format!("{label}.objects"),
            ModelObjectRole::Any,
        )?;
        let raw = reader.u16(&format!("{label}.raw_words.count"))?;
        for word in 0..reader.counted_entries(&label, i64::from(raw), 4)? {
            reader.u32(&format!("{label}.raw_words[{word}]"))?;
        }
    }
    Ok(())
}
