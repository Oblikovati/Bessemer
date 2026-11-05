use super::read_word;

#[test]
fn word_reads_reject_missing_and_overflowing_ranges_without_panicking() {
    for offset in [0, 1, usize::MAX] {
        let error = read_word(&[1, 2, 3], offset).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains(&offset.to_string()));
    }
    assert_eq!(read_word(&[1, 2, 3, 4], 0).unwrap(), 0x01020304);
}
