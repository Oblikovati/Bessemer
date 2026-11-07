use super::*;

struct Patch {
    offset: usize,
    before: FlowBlockFieldEdit,
    after: FlowBlockFieldEdit,
    bytes: Vec<u8>,
}

fn patches(case: &fixture::BlockCase) -> Vec<Patch> {
    let mut result = Vec::new();
    prefix_patches(&mut result, case);
    let mut entry_index = 0;
    for block in &case.blocks {
        block_patches(&mut result, block);
        for entry in &block.entries {
            entry_patches(&mut result, case.prefix.version, entry_index, entry);
            entry_index += 1;
        }
    }
    result
}

fn prefix_patches(result: &mut Vec<Patch>, case: &fixture::BlockCase) {
    for (field, count, offset) in [
        (FlowBlockGlobalCount::Fluid, case.prefix.fluid, 4),
        (FlowBlockGlobalCount::Solid, case.prefix.solid, 8),
    ] {
        push_global(result, case, field, count, offset);
    }
    for (index, byte) in case.bitmap.iter().enumerate() {
        push_bitmap(result, index, *byte);
    }
}

fn next_global(case: &fixture::BlockCase, field: FlowBlockGlobalCount, count: i32) -> i32 {
    let other = if field == FlowBlockGlobalCount::Fluid {
        case.prefix.solid
    } else {
        case.prefix.fluid
    };
    for next in [
        count.saturating_add(1),
        count.saturating_sub(1).max(0),
        0,
        1,
        i32::MAX,
    ] {
        if next != count && bitmap_size(case.prefix.version, next, other) == case.bitmap.len() {
            return next;
        }
    }
    count
}

fn block_patches(result: &mut Vec<Patch>, block: &fixture::ExpectedBlock) {
    use FlowBlockFieldValue::*;
    use FlowBlockHeaderField::*;
    push_value(
        result,
        block_request(block, BlockByte, Byte(block.byte)),
        block.start as usize,
        1,
        None,
    );
    push_value(
        result,
        block_request(block, Word10, Word16(block.word_10)),
        block.start as usize + 1,
        2,
        None,
    );
    push_block_count(result, block);
}

fn header_fields(
    entry: &fixture::ExpectedEntry,
    version: i32,
) -> Vec<(FlowBlockEntryField, Option<FlowBlockFieldValue>)> {
    use FlowBlockEntryField::*;
    use FlowBlockFieldValue as V;
    let mut fields = vec![
        (Identifier, Some(V::Signed32(entry.identifier))),
        (LegacyIndex, entry.legacy_index.map(V::Signed32)),
    ];
    if version != 1 {
        fields.push((Member20, entry.member_20.map(V::Signed32)));
    }
    fields.extend([
        (Member28, Some(V::Signed32(entry.member_28))),
        (DiscardedPrefix, entry.discarded_prefix.map(V::Byte)),
        (State, Some(V::Byte(entry.state))),
        (Flag2, entry.flag_2.map(V::Byte)),
        (Flag4, entry.flag_4.map(V::Byte)),
    ]);
    fields
}

fn entry_patches(
    result: &mut Vec<Patch>,
    version: i32,
    index: usize,
    entry: &fixture::ExpectedEntry,
) {
    let mut offset = entry.start as usize;
    for (field, value) in header_fields(entry, version) {
        if let Some(value) = value {
            offset += push_entry_value(result, index, field, value, offset);
        }
    }
    tail_patches(result, version, index, offset, &entry.tail);
}

fn tail_patches(
    result: &mut Vec<Patch>,
    version: i32,
    index: usize,
    offset: usize,
    tail: &fixture::ExpectedTail,
) {
    let offset = descriptor_patches(result, version, index, offset, tail);
    let offset = array_patches(result, index, offset, &tail.first, true);
    let offset = second_count_patch(result, version, index, offset, tail.second_count);
    let offset = array_patches(result, index, offset, &tail.second, false);
    if let Some(word) = tail.discarded_suffix {
        let request = entry_request(
            index,
            FlowBlockEntryField::DiscardedSuffix,
            FlowBlockFieldValue::Word16(word),
        );
        push_value(result, request, offset, 2, None);
    }
}

fn array_patches(
    result: &mut Vec<Patch>,
    entry: usize,
    mut offset: usize,
    words: &[u32],
    first: bool,
) -> usize {
    for (index, word) in words.iter().enumerate() {
        let field = if first {
            FlowBlockEntryField::FirstWord { index }
        } else {
            FlowBlockEntryField::SecondWord { index }
        };
        push_value(
            result,
            FlowBlockFieldEdit::Entry {
                index: entry,
                field,
                value: FlowBlockFieldValue::Word32(*word),
            },
            offset,
            4,
            None,
        );
        offset += 4;
    }
    offset
}

fn push_value(
    result: &mut Vec<Patch>,
    before: FlowBlockFieldEdit,
    offset: usize,
    width: usize,
    replacement: Option<FlowBlockFieldValue>,
) {
    let next = replacement.unwrap_or_else(|| mutate(request_value(before), width));
    let after = with_value(before, next);
    result.push(Patch {
        offset,
        before,
        after,
        bytes: encoded(next, width),
    });
}

fn mutate(value: FlowBlockFieldValue, width: usize) -> FlowBlockFieldValue {
    use FlowBlockFieldValue::*;
    match value {
        Byte(byte) => Byte(byte ^ 255),
        Word16(word) => Word16(word ^ if width == 1 { 1 } else { 0x8001 }),
        Signed32(word) => Signed32(word ^ 0x80000001_u32 as i32),
        Word32(word) => Word32(word ^ 0x80000001),
    }
}

fn encoded(value: FlowBlockFieldValue, width: usize) -> Vec<u8> {
    use FlowBlockFieldValue::*;
    let raw = match value {
        Byte(byte) => vec![byte],
        Word16(word) => word.to_le_bytes().to_vec(),
        Signed32(word) => word.to_le_bytes().to_vec(),
        Word32(word) => word.to_le_bytes().to_vec(),
    };
    raw[..width].to_vec()
}

#[test]
fn every_owned_existing_field_has_noop_complete_patch_history_and_fresh_reverse() {
    for case in fixture::cases().into_iter().skip(1) {
        let parsed = parse_case(&case);
        for request in patches(&case) {
            check_patch(&parsed, &case, request);
        }
    }
}

#[test]
#[ignore = "explicit full native record-406 field census"]
fn every_native_stored_field_has_complete_patch_history_and_fresh_reverse() {
    let case = fixture::cases().remove(0);
    let parsed = parse_case(&case);
    let requests = patches(&case);
    println!("native record-406 requests: {}", requests.len());
    for request in requests {
        check_patch(&parsed, &case, request);
    }
}

fn check_patch(parsed: &FlowLegacyBlockData, case: &fixture::BlockCase, request: Patch) {
    check_noop(parsed, request.before);
    let edit = parsed.replace_field(request.after).unwrap();
    let mut expected = case.decoded.clone();
    expected[request.offset..request.offset + request.bytes.len()].copy_from_slice(&request.bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().record(), parsed.record());
    assert_eq!(
        edit.source().source().as_bytes(),
        parsed.source().as_bytes()
    );
    check_history(edit.carrier_edit(), expected != case.decoded);
    check_reverse(parsed, edit.result(), request.before, case);
}

fn check_history(edit: &FlowCarrierEdit, changed: bool) {
    assert_eq!(edit.history().len(), 3);
    for (ordinal, (before, after)) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .enumerate()
    {
        check_relation(&edit.history()[ordinal], ordinal, changed, before, after);
    }
}

fn bitmap_size(version: i32, first: i32, second: i32) -> usize {
    let total = (first as u32).wrapping_add(second as u32) as i32;
    if version >= 8 && total > 0 {
        total as usize / 8 + 1
    } else {
        0
    }
}

fn push_global(
    result: &mut Vec<Patch>,
    case: &fixture::BlockCase,
    field: FlowBlockGlobalCount,
    count: i32,
    offset: usize,
) {
    let next = next_global(case, field, count);
    let before = FlowBlockFieldEdit::GlobalCount { field, count };
    let after = FlowBlockFieldEdit::GlobalCount { field, count: next };
    result.push(Patch {
        offset,
        before,
        after,
        bytes: next.to_le_bytes().to_vec(),
    });
}

fn push_bitmap(result: &mut Vec<Patch>, index: usize, byte: u8) {
    let before = FlowBlockFieldEdit::BitmapByte { index, byte };
    let after = FlowBlockFieldEdit::BitmapByte {
        index,
        byte: byte ^ 255,
    };
    result.push(Patch {
        offset: 12 + index,
        before,
        after,
        bytes: vec![byte ^ 255],
    });
}

fn block_request(
    block: &fixture::ExpectedBlock,
    field: FlowBlockHeaderField,
    value: FlowBlockFieldValue,
) -> FlowBlockFieldEdit {
    FlowBlockFieldEdit::Block {
        index: block.ordinal as usize,
        field,
        value,
    }
}

fn push_block_count(result: &mut Vec<Patch>, block: &fixture::ExpectedBlock) {
    let next = if block.signed_count <= 0 {
        block.raw_count ^ 1 | 0x8000
    } else {
        block.raw_count
    };
    let request = block_request(
        block,
        FlowBlockHeaderField::Count12,
        FlowBlockFieldValue::Word16(block.raw_count),
    );
    push_value(
        result,
        request,
        block.start as usize + 3,
        2,
        Some(FlowBlockFieldValue::Word16(next)),
    );
}

fn entry_request(
    index: usize,
    field: FlowBlockEntryField,
    value: FlowBlockFieldValue,
) -> FlowBlockFieldEdit {
    FlowBlockFieldEdit::Entry {
        index,
        field,
        value,
    }
}

fn push_entry_value(
    result: &mut Vec<Patch>,
    index: usize,
    field: FlowBlockEntryField,
    value: FlowBlockFieldValue,
    offset: usize,
) -> usize {
    let width = if matches!(value, FlowBlockFieldValue::Byte(_)) {
        1
    } else {
        4
    };
    push_value(
        result,
        entry_request(index, field, value),
        offset,
        width,
        None,
    );
    width
}

fn descriptor_patches(
    result: &mut Vec<Patch>,
    version: i32,
    index: usize,
    mut offset: usize,
    tail: &fixture::ExpectedTail,
) -> usize {
    for (slot, word) in tail.descriptors.iter().enumerate() {
        let width = if version == 1 { 1 } else { 2 };
        let value = FlowBlockFieldValue::Word16(*word);
        let request = entry_request(
            index,
            FlowBlockEntryField::Descriptor { index: slot },
            value,
        );
        push_value(result, request, offset, width, (slot == 6).then_some(value));
        offset += width;
    }
    offset
}

fn second_count_patch(
    result: &mut Vec<Patch>,
    version: i32,
    index: usize,
    offset: usize,
    count: i32,
) -> usize {
    let width = second_count_width(version);
    let next = next_second_count(version, count);
    let request = entry_request(
        index,
        FlowBlockEntryField::SecondCount,
        FlowBlockFieldValue::Signed32(count),
    );
    push_value(
        result,
        request,
        offset,
        width,
        Some(FlowBlockFieldValue::Signed32(next)),
    );
    offset + width
}

fn request_value(request: FlowBlockFieldEdit) -> FlowBlockFieldValue {
    match request {
        FlowBlockFieldEdit::Block { value, .. } | FlowBlockFieldEdit::Entry { value, .. } => value,
        _ => panic!("expected block/entry request"),
    }
}

fn with_value(request: FlowBlockFieldEdit, value: FlowBlockFieldValue) -> FlowBlockFieldEdit {
    match request {
        FlowBlockFieldEdit::Block { index, field, .. } => FlowBlockFieldEdit::Block {
            index,
            field,
            value,
        },
        FlowBlockFieldEdit::Entry { index, field, .. } => FlowBlockFieldEdit::Entry {
            index,
            field,
            value,
        },
        _ => panic!("expected block/entry request"),
    }
}

fn check_noop(parsed: &FlowLegacyBlockData, request: FlowBlockFieldEdit) {
    let noop = parsed.replace_field(request).unwrap();
    assert_eq!(noop.result().record(), parsed.record());
    assert_eq!(
        noop.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
    check_history(noop.carrier_edit(), false);
}

fn check_reverse(
    source: &FlowLegacyBlockData,
    changed: &FlowLegacyBlockData,
    request: FlowBlockFieldEdit,
    case: &fixture::BlockCase,
) {
    let reverse = changed.replace_field(request).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.decoded);
    if !case.compressed {
        assert_eq!(
            reverse.result().source().as_bytes(),
            source.source().as_bytes()
        );
    }
}

fn check_relation(
    history: &FlowRecordHistory,
    ordinal: usize,
    changed: bool,
    before: FlowRecord<'_>,
    after: FlowRecord<'_>,
) {
    assert_eq!(history.source, before.id);
    assert_eq!(history.result, after.id);
    let expected = if ordinal == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    };
    assert_eq!(history.relation, expected);
    if ordinal != 1 {
        assert_eq!(before.payload, after.payload);
        assert_eq!(before.identifier, after.identifier);
    }
}

fn next_second_count(version: i32, count: i32) -> i32 {
    if count > 0 || (2..=9).contains(&version) {
        return count;
    }
    if count == -1 { -2 } else { -1 }
}

fn second_count_width(version: i32) -> usize {
    if version == 1 {
        return 1;
    }
    if version < 10 { 2 } else { 4 }
}
