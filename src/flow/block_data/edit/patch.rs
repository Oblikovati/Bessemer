use super::*;
use fields::ScalarKind;

pub(super) fn selected(
    body: &FlowLegacyBlockData,
    request: FlowBlockFieldEdit,
) -> Result<(usize, Vec<u8>)> {
    match request {
        FlowBlockFieldEdit::GlobalCount { field, count } => global_count(body, field, count),
        FlowBlockFieldEdit::BitmapByte { index, byte } => bitmap_byte(body, index, byte),
        FlowBlockFieldEdit::Block {
            index,
            field,
            value,
        } => block_patch(body, index, field, value),
        FlowBlockFieldEdit::Entry {
            index,
            field,
            value,
        } => entry_patch(body, index, field, value),
    }
}

fn global_count(
    body: &FlowLegacyBlockData,
    field: FlowBlockGlobalCount,
    count: i32,
) -> Result<(usize, Vec<u8>)> {
    let prefix = updated_prefix(body.prefix, field, count)?;
    let offset = match field {
        FlowBlockGlobalCount::Fluid => 4,
        FlowBlockGlobalCount::Solid => 8,
    };
    let size = decode::bitmap_count(prefix);
    if size != body.bitmap_bytes().len() {
        return Err(structural(format!(
            "{field:?} count {count} gives bitmap length {size}; expected unchanged length {}",
            body.bitmap_bytes().len()
        )));
    }
    Ok((offset, count.to_le_bytes().to_vec()))
}

fn bitmap_byte(body: &FlowLegacyBlockData, index: usize, byte: u8) -> Result<(usize, Vec<u8>)> {
    if index < body.bitmap_bytes().len() {
        return Ok((12 + index, vec![byte]));
    }
    Err(invalid_block(format!(
        "bitmap byte {index}; expected index below {}",
        body.bitmap_bytes().len()
    )))
}

fn block_patch(
    body: &FlowLegacyBlockData,
    index: usize,
    field: FlowBlockHeaderField,
    value: FlowBlockFieldValue,
) -> Result<(usize, Vec<u8>)> {
    let block = body.block(index).ok_or_else(|| {
        invalid_block(format!(
            "block index {index}; expected below {}",
            body.blocks.len()
        ))
    })?;
    let selected = selection::block_field(block, field);
    let bytes = encode(selected.kind, value)?;
    if field == FlowBlockHeaderField::Count12
        && let FlowBlockFieldValue::Word16(raw) = value
    {
        let count = (raw as i16).max(0) as usize;
        if count != block.entries().len() {
            return Err(structural(format!(
                "block {index} count word {raw}; expected max(signed count,0) == {} existing entries",
                block.entries().len()
            )));
        }
    }
    Ok((selected.offset, bytes))
}

fn entry_patch(
    body: &FlowLegacyBlockData,
    index: usize,
    field: FlowBlockEntryField,
    value: FlowBlockFieldValue,
) -> Result<(usize, Vec<u8>)> {
    let entry = body.entry(index).ok_or_else(|| {
        invalid_block(format!(
            "entry index {index}; expected below {}",
            body.entries.len()
        ))
    })?;
    let selected = selection::entry_field(entry, field)?;
    let bytes = encode(selected.kind, value)?;
    check_entry_cardinality(entry, field, value)?;
    Ok((selected.offset, bytes))
}

fn check_entry_cardinality(
    entry: FlowLegacyBlockEntry<'_>,
    field: FlowBlockEntryField,
    value: FlowBlockFieldValue,
) -> Result<()> {
    let actual = match (field, value) {
        (FlowBlockEntryField::Descriptor { index: 6 }, FlowBlockFieldValue::Word16(count)) => {
            Some((usize::from(count), entry.first_words().word_count()))
        }
        (FlowBlockEntryField::SecondCount, FlowBlockFieldValue::Signed32(count)) => {
            Some((count.max(0) as usize, entry.second_words().word_count()))
        }
        _ => None,
    };
    if let Some((count, expected)) = actual
        && count != expected
    {
        return Err(structural(format!(
            "entry {} field {field:?} value {value:?} consumes {count} words; expected unchanged {expected} existing words",
            entry.ordinal
        )));
    }
    Ok(())
}

fn encode(kind: ScalarKind, value: FlowBlockFieldValue) -> Result<Vec<u8>> {
    use FlowBlockFieldValue::*;
    let bytes = match (kind, value) {
        (ScalarKind::Byte, Byte(byte)) => vec![byte],
        (ScalarKind::NarrowWord16, Word16(word)) if word <= u8::MAX.into() => vec![word as u8],
        (ScalarKind::Word16, Word16(word)) => word.to_le_bytes().to_vec(),
        (ScalarKind::Signed32, Signed32(word)) => word.to_le_bytes().to_vec(),
        (ScalarKind::Word32, Word32(word)) => word.to_le_bytes().to_vec(),
        (ScalarKind::Signed8, Signed32(word)) if (-128..=127).contains(&word) => {
            vec![word as i8 as u8]
        }
        (ScalarKind::UnsignedCount16, Signed32(word)) if (0..=65535).contains(&word) => {
            (word as u16).to_le_bytes().to_vec()
        }
        _ => {
            return Err(invalid_block(format!(
                "field value {value:?}; expected matching stored kind and representable width {kind:?}"
            )));
        }
    };
    Ok(bytes)
}

fn structural(cause: String) -> BessemerError {
    block_failure(FailureClass::Unsupported, cause)
}

fn updated_prefix(
    mut prefix: FlowBlockCountPrefix,
    field: FlowBlockGlobalCount,
    count: i32,
) -> Result<FlowBlockCountPrefix> {
    if count < 0 {
        return Err(invalid_block(format!(
            "{field:?} count {count}; expected nonnegative signed i32"
        )));
    }
    match field {
        FlowBlockGlobalCount::Fluid => {
            prefix.g_fluid = count;
        }
        FlowBlockGlobalCount::Solid => {
            prefix.g_solid = count;
        }
    };
    prefix.mathematical_total = prefix.g_fluid as u64 + prefix.g_solid as u64;
    prefix.native_total = prefix.mathematical_total as u32 as i32;
    Ok(prefix)
}
