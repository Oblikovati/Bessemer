use super::*;
use FlowBlockFieldValue as Value;
use FlowReferenceEntryField as Field;

pub(super) fn check_entry(actual: FlowReferenceEntry<'_>, expected: &ReferenceEntry, version: i32) {
    let mut offset = expected.start as usize + 4;
    let member = expected.member20.map(Value::Signed32);
    scalar(actual, Field::Member20, member, &mut offset, 4);
    check_references(actual, expected, &mut offset);
    check_selectors(actual, expected, &mut offset);
    check_descriptors(actual, expected, version, &mut offset);
    check_words(
        actual,
        actual.first_words(),
        &expected.first,
        Field::FirstWord,
        &mut offset,
    );
    check_secondary(actual, expected, version, &mut offset);
    let suffix = expected.suffix.map(Value::Word16);
    scalar(actual, Field::DiscardedSuffix, suffix, &mut offset, 2);
    assert_eq!(offset, expected.end as usize);
}

fn check_descriptors(
    actual: FlowReferenceEntry<'_>,
    expected: &ReferenceEntry,
    version: i32,
    offset: &mut usize,
) {
    let width = if version == 1 { 1 } else { 2 };
    for (index, value) in expected.descriptors.iter().enumerate() {
        scalar(
            actual,
            Field::Descriptor(index),
            Some(Value::Word16(*value)),
            offset,
            width,
        );
    }
}

fn check_secondary(
    actual: FlowReferenceEntry<'_>,
    expected: &ReferenceEntry,
    version: i32,
    offset: &mut usize,
) {
    let width = if version == 1 {
        1
    } else if version < 10 {
        2
    } else {
        4
    };
    let value = Some(Value::Signed32(expected.second_count));
    scalar(actual, Field::SecondCount, value, offset, width);
    assert_eq!(actual.raw_small_second_count(), expected.raw_small_count);
    check_words(
        actual,
        actual.second_words(),
        &expected.second,
        Field::SecondWord,
        offset,
    );
}

fn scalar(
    actual: FlowReferenceEntry<'_>,
    field: Field,
    expected: Option<Value>,
    offset: &mut usize,
    width: usize,
) {
    if let Some(value) = expected {
        assert_eq!(actual.field(field).unwrap(), value);
        assert_eq!(
            actual.field_interval(field).unwrap(),
            (*offset, *offset + width)
        );
        *offset += width;
        return;
    }
    assert_eq!(
        actual.field(field).unwrap_err().class,
        bessemer::FailureClass::Missing
    );
    assert!(actual.field_interval(field).is_err());
}

fn check_references(actual: FlowReferenceEntry<'_>, expected: &ReferenceEntry, offset: &mut usize) {
    let start = *offset;
    assert_eq!(actual.reference_count(), expected.references.len());
    assert_eq!(
        actual.reference_words().word_count(),
        expected.references.len()
    );
    for (index, value) in expected.references.iter().enumerate() {
        assert_eq!(actual.reference_word(index), Some(*value));
        scalar(
            actual,
            Field::Reference(index),
            Some(Value::Signed32(*value)),
            offset,
            4,
        );
    }
    check_span(actual.reference_words(), start, *offset);
    assert_eq!(actual.reference_word(expected.references.len()), None);
}

fn check_selectors(actual: FlowReferenceEntry<'_>, expected: &ReferenceEntry, offset: &mut usize) {
    for (field, value) in [
        (Field::DiscardedPrefix, expected.prefix),
        (Field::State, Some(expected.state)),
        (Field::Flag2, expected.flag2),
        (Field::Flag4, expected.flag4),
    ] {
        scalar(actual, field, value.map(Value::Byte), offset, 1);
    }
}

fn check_words(
    actual: FlowReferenceEntry<'_>,
    array: FlowBlockWordArray<'_>,
    expected: &[u32],
    field: fn(usize) -> Field,
    offset: &mut usize,
) {
    let start = *offset;
    assert_eq!(array.word_count(), expected.len());
    assert_eq!(array.as_bytes().len(), expected.len() * 4);
    for (index, bits) in expected.iter().enumerate() {
        assert_eq!(array.raw_word(index), Some(*bits));
        scalar(actual, field(index), Some(Value::Word32(*bits)), offset, 4);
    }
    assert_eq!(array.payload_interval(), (start, *offset));
    assert_eq!(array.raw_word(expected.len()), None);
}

fn check_span(array: FlowBlockWordArray<'_>, start: usize, end: usize) {
    assert_eq!(array.payload_interval(), (start, end));
}
