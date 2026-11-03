use super::*;

const PHYSICAL_WORD_INDEX: [usize; 17] = [0, 0, 1, 1, 2, 3, 3, 3, 3, 4, 4, 5, 5, 6, 6, 6, 6];

fn component_integer_words(raw: &[u8]) -> [DisplayFieldValue; 6] {
    [
        DisplayFieldValue::U16(u16::from_le_bytes(raw[..2].try_into().unwrap())),
        DisplayFieldValue::U16(u16::from_le_bytes(raw[2..4].try_into().unwrap())),
        DisplayFieldValue::U8(raw[4]),
        DisplayFieldValue::U32(u32::from_le_bytes(raw[5..9].try_into().unwrap())),
        DisplayFieldValue::U16(u16::from_le_bytes(raw[9..11].try_into().unwrap())),
        DisplayFieldValue::I16(i16::from_le_bytes(raw[11..13].try_into().unwrap())),
    ]
}

pub(super) fn component_literal_words(raw: &[u8]) -> Vec<DisplayFieldValue> {
    assert!(matches!(raw.len(), 13 | 17));
    let mut words = component_integer_words(raw).to_vec();
    if raw.len() == 17 {
        words.push(DisplayFieldValue::F32(StoredFloat::from_bits(
            u32::from_le_bytes(raw[13..17].try_into().unwrap()),
        )));
    }
    words
}

pub(super) fn verify_component_bit_writes(source: &DisplayArchive, raw: &[u8], index: usize) {
    let start = component_visual_offset(raw);
    let (_, size) = component_visual_shape(source);
    for bit in 0..size * 8 {
        let mut expected = raw.to_vec();
        expected[start + bit / 8] ^= 1 << (bit % 8);
        let selected = PHYSICAL_WORD_INDEX[bit / 8];
        let replacement = component_literal_words(&expected[start..start + size])[selected].clone();
        let mut contents = source.contents().clone();
        contents.fields[index + selected].value = replacement;
        let result = source
            .edit_with_history(contents, archive_support::LIMITS)
            .unwrap();
        assert_eq!(result.archive().to_bytes().unwrap(), expected);
        assert_eq!(result.completion(), DisplayEditCompletion::Changed);
        assert_eq!(result.changes().len(), 1);
        verify_component_field_history(source, &result, index + selected);
        assert_eq!(source.to_bytes().unwrap(), raw);
    }
}
