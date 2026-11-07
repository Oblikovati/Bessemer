use super::*;
use bessemer::FailureClass;

#[test]
fn all_273_profiles_match_every_frozen_field_raw_word_span_and_signed_branch() {
    for case in fixture::cases() {
        let parsed = parse_case(&case);
        assert_eq!(parsed.decoded_bytes(), case.decoded);
        check_prefix(&parsed, &case);
        assert_eq!(parsed.blocks().len(), case.blocks.len());
        assert!(parsed.block(case.blocks.len()).is_none());
        assert!(parsed.entry(case.totals[0] as usize).is_none());
        for (actual, expected) in parsed.blocks().zip(&case.blocks) {
            check_block(actual, expected, case.prefix.version);
        }
        assert_eq!(parsed.entries().len(), case.totals[0] as usize);
    }
}

fn check_prefix(parsed: &FlowLegacyBlockData, case: &fixture::BlockCase) {
    assert_eq!(
        parsed.prefix(),
        FlowBlockCountPrefix {
            version: case.prefix.version,
            g_fluid: case.prefix.fluid,
            g_solid: case.prefix.solid,
            mathematical_total: case.prefix.mathematical,
            native_total: case.prefix.native
        }
    );
    assert_eq!(parsed.bitmap_bytes(), case.bitmap);
    assert_eq!(parsed.bitmap_interval(), (12, 12 + case.bitmap.len()));
    assert_eq!(
        parsed.entries_equal_global_total(),
        case.entries_equal_total
    );
    assert_eq!(parsed.options().leaf_count.value(), case.leaf_count);
    assert_eq!(parsed.record().ordinal(), 1);
    assert_eq!(parsed.diagnostics().len(), 6);
    check_coverage(parsed.coverage(), case);
}

fn check_coverage(coverage: FlowBlockDataCoverage, case: &fixture::BlockCase) {
    assert_eq!(
        coverage.mode,
        if case.compressed {
            FlowBlockDataCheckMode::ExactZlibLegacyBody
        } else {
            FlowBlockDataCheckMode::ExactRawLegacyBody
        }
    );
    assert_eq!(coverage.stored_bytes, case.stored.len());
    assert_eq!(coverage.decoded_bytes, case.decoded.len());
    assert_eq!(coverage.checked_blocks, case.blocks.len());
    assert_eq!(coverage.checked_entries, case.totals[0] as usize);
    assert_eq!(coverage.first_integer_words, case.totals[1] as usize);
    assert_eq!(coverage.second_integer_words, case.totals[2] as usize);
    assert_eq!(coverage.achieved_error, 0);
}

fn check_block(actual: FlowLegacyBlock<'_>, expected: &fixture::ExpectedBlock, version: i32) {
    check_block_headers(actual, expected);
    assert_eq!(actual.signed_entry_count(), expected.signed_count);
    assert_eq!(expected.native_assign, version >= 13);
    assert_eq!(
        actual.native_byte_operation(),
        if expected.native_assign {
            FlowNativeBlockByteOperation::Assign
        } else {
            FlowNativeBlockByteOperation::AndE7OfRuntimeValue
        }
    );
    assert_eq!(actual.entries().len(), expected.entries.len());
    for (actual, expected) in actual.entries().zip(&expected.entries) {
        check_entry(actual, expected);
    }
}

fn check_entry(actual: FlowLegacyBlockEntry<'_>, expected: &fixture::ExpectedEntry) {
    use FlowBlockEntryField::*;
    use FlowBlockFieldValue as V;
    check_entry_interval(actual, expected);
    for (field, value) in expected_scalars(expected) {
        check_scalar(actual, field, value);
    }
    for (index, word) in expected.tail.descriptors.iter().enumerate() {
        check_scalar(actual, Descriptor { index }, Some(V::Word16(*word)));
    }
    assert_eq!(actual.second_count_raw_small(), expected.tail.raw_second);
    check_words(actual.first_words(), &expected.tail.first);
    check_words(actual.second_words(), &expected.tail.second);
    for (index, word) in expected.tail.first.iter().enumerate() {
        check_scalar(actual, FirstWord { index }, Some(V::Word32(*word)));
    }
    for (index, word) in expected.tail.second.iter().enumerate() {
        check_scalar(actual, SecondWord { index }, Some(V::Word32(*word)));
    }
}

fn check_scalar(
    entry: FlowLegacyBlockEntry<'_>,
    field: FlowBlockEntryField,
    expected: Option<FlowBlockFieldValue>,
) {
    if let Some(value) = expected {
        assert_eq!(entry.field(field).unwrap(), value);
        return;
    }
    assert_eq!(entry.field(field).unwrap_err().class, FailureClass::Missing);
}

fn check_words(actual: FlowBlockWordArray<'_>, expected: &[u32]) {
    assert_eq!(actual.word_count(), expected.len());
    let span = actual.payload_interval();
    assert_eq!(span.1 - span.0, expected.len() * 4);
    assert_eq!(
        actual.as_bytes(),
        expected
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect::<Vec<_>>()
    );
    for (index, word) in expected.iter().enumerate() {
        assert_eq!(actual.raw_word(index), Some(*word));
    }
    assert_eq!(actual.raw_word(expected.len()), None);
    assert_eq!(actual.raw_word(usize::MAX), None);
}

fn check_block_headers(actual: FlowLegacyBlock<'_>, expected: &fixture::ExpectedBlock) {
    use FlowBlockFieldValue::*;
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(
        actual.field(FlowBlockHeaderField::BlockByte).unwrap(),
        Byte(expected.byte)
    );
    assert_eq!(
        actual.field(FlowBlockHeaderField::Word10).unwrap(),
        Word16(expected.word_10)
    );
    assert_eq!(
        actual.field(FlowBlockHeaderField::Count12).unwrap(),
        Word16(expected.raw_count)
    );
}

fn expected_scalars(
    expected: &fixture::ExpectedEntry,
) -> [(FlowBlockEntryField, Option<FlowBlockFieldValue>); 10] {
    use FlowBlockEntryField::*;
    use FlowBlockFieldValue as V;
    [
        (Identifier, Some(V::Signed32(expected.identifier))),
        (LegacyIndex, expected.legacy_index.map(V::Signed32)),
        (Member20, expected.member_20.map(V::Signed32)),
        (Member28, Some(V::Signed32(expected.member_28))),
        (DiscardedPrefix, expected.discarded_prefix.map(V::Byte)),
        (State, Some(V::Byte(expected.state))),
        (Flag2, expected.flag_2.map(V::Byte)),
        (Flag4, expected.flag_4.map(V::Byte)),
        (SecondCount, Some(V::Signed32(expected.tail.second_count))),
        (
            DiscardedSuffix,
            expected.tail.discarded_suffix.map(V::Word16),
        ),
    ]
}

fn check_entry_interval(actual: FlowLegacyBlockEntry<'_>, expected: &fixture::ExpectedEntry) {
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
}
