use super::{Result, SimulationPersistText, invalid, require, text::text_extent};

pub(super) fn append_frame_offsets(
    bytes: &[u8],
    mut cursor: usize,
    count: usize,
    maximum_text: usize,
    offsets: &mut Vec<usize>,
) -> Result<usize> {
    for _ in 0..count {
        offsets.push(cursor);
        cursor = text_extent(bytes, cursor, maximum_text)?.1;
    }
    Ok(cursor)
}

pub(super) fn replace_frame(
    source: &[u8],
    original: &SimulationPersistText,
    replacement: &SimulationPersistText,
    entity: &str,
) -> Result<Vec<u8>> {
    let (start, _, end) = original.source_offsets();
    let size = (source.len() - original.encoded().len()).checked_add(replacement.encoded().len()).ok_or_else(|| invalid(entity, format!("{} original bytes, {} replacement bytes; expected representable complete result", source.len(), replacement.encoded().len())))?;
    let mut bytes = super::reserve_bytes(size)?;
    bytes.extend_from_slice(&source[..start]);
    bytes.extend_from_slice(replacement.encoded());
    bytes.extend_from_slice(&source[end..]);
    Ok(bytes)
}

pub(super) fn verify_frame_preservation(
    source: &[u8],
    result: &[u8],
    original: &SimulationPersistText,
    replacement: &SimulationPersistText,
    entity: &str,
) -> Result<()> {
    let (start, _, end) = original.source_offsets();
    let replacement_end = start + replacement.encoded().len();
    require(
        result[..start] == source[..start]
            && result[start..replacement_end] == *replacement.encoded()
            && result[replacement_end..] == source[end..],
        entity,
        format!(
            "{} result bytes; expected complete original prefix/suffix and exact supplied {}-byte replacement",
            result.len(),
            replacement.encoded().len()
        ),
    )
}
