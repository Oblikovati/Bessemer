use crate::stored_fields::StoredField;
use bessemer::flow::{FlowBlockWordArray, FlowTableWideArray};
pub fn check_words(array: FlowBlockWordArray<'_>, field: &StoredField) {
    assert_eq!(array.as_bytes(), field.raw);
    let origin = field.offset as usize;
    assert_eq!(array.payload_interval(), (origin, origin + field.raw.len()));
    assert_eq!(array.word_count(), field.count as usize);
    let (chunks, remainder) = field.raw.as_chunks::<4>();
    assert!(remainder.is_empty());
    for (index, chunk) in chunks.iter().enumerate() {
        assert_eq!(array.raw_word(index), Some(u32::from_le_bytes(*chunk)));
    }
    assert_eq!(array.raw_word(array.word_count()), None);
    assert_eq!(array.raw_word(usize::MAX), None);
}
pub fn check_wides(array: FlowTableWideArray<'_>, field: &StoredField) {
    assert_eq!(array.as_bytes(), field.raw);
    let origin = field.offset as usize;
    assert_eq!(array.payload_interval(), (origin, origin + field.raw.len()));
    assert_eq!(array.word_count(), field.count as usize);
    let (chunks, remainder) = field.raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    for (index, chunk) in chunks.iter().enumerate() {
        assert_eq!(array.raw_bits(index), Some(u64::from_le_bytes(*chunk)));
    }
    assert_eq!(array.raw_bits(array.word_count()), None);
    assert_eq!(array.raw_bits(usize::MAX), None);
}
