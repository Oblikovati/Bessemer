use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("word_10")?;
    let state = reader.scope("body", kernel_frames::read_body_state)?;
    let count = reader.signed_count("pairs.count", 8)?;
    pairs(reader, count)?;
    reader.scope("references", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    reader.scope("reference_data", reference_array)?;
    if state == kernel_frames::KernelBodyState::Serialized {
        extra_bodies(reader)?;
    }
    Ok(())
}

pub(super) fn pairs(reader: &mut ModelDecoder<'_>, count: usize) -> Result<()> {
    for index in 0..count {
        reader.u32(&format!("pairs[{index}][0]"))?;
        reader.u32(&format!("pairs[{index}][1]"))?;
    }
    Ok(())
}

fn extra_bodies(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.unsigned_count("extra_bodies.count", 5)? {
        reader.i32(&format!("extra_bodies[{index}][0]"))?;
        reader.scope(
            &format!("extra_bodies[{index}][1]"),
            kernel_frames::read_body,
        )?;
    }
    Ok(())
}

pub(super) fn reference_array(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let size = reader.u32("element_size")?;
    if size == 0 {
        return Ok(());
    }
    if size % 4 != 0 {
        return reader.checked(Err(super::invalid(format!(
            "reference element size {size}; expected zero or a multiple of four"
        ))));
    }
    reader.u32("capacity")?;
    reader.u32("growth")?;
    let count = reader.unsigned_count("count", size as usize)?;
    reference_words(reader, count, (size / 4) as usize)
}

fn reference_words(reader: &mut ModelDecoder<'_>, count: usize, width: usize) -> Result<()> {
    let words = count.checked_mul(width).ok_or_else(|| {
        super::invalid(format!(
            "reference array {count} * {width}; expected representable word count"
        ))
    });
    let words = reader.checked(words)?;
    reader.entries(words, "profile reference words")?;
    for row in 0..count {
        for column in 0..width {
            reader.u32(&format!("words[{row}][{column}]"))?;
        }
    }
    Ok(())
}
