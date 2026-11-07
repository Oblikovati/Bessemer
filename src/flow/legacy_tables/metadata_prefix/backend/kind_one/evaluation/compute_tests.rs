use super::*;
fn wide(bytes: &[u8]) -> FlowTableWideArray<'_> {
    FlowTableWideArray { bytes, start: 0 }
}
#[test]
fn evaluation_table_shape_rejects_empty_and_mismatched_declared_arrays() {
    let first = [0; 16];
    let second = [0; 8];
    assert!(ValidatedTable::new(wide(&[]), wide(&[]), 0).is_err());
    assert!(ValidatedTable::new(wide(&first), wide(&second), 2).is_err());
    assert!(ValidatedTable::new(wide(&second), wide(&second), 2).is_err());
}
#[test]
fn evaluation_word_access_reports_a_typed_bounds_error_without_panicking() {
    let raw = [0; 8];
    let error = raw_number(wide(&raw), usize::MAX).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains(&usize::MAX.to_string()));
    assert!(finite_at(wide(&raw), 1, "output").is_err());
}
#[test]
fn evaluation_nonfinite_output_is_reported_before_deferred_coordinate_order_failure() {
    let coordinates = [1f64.to_le_bytes(), 0f64.to_le_bytes()].concat();
    let outputs = [0f64.to_le_bytes(), f64::NAN.to_le_bytes()].concat();
    let error = ValidatedTable::new(wide(&coordinates), wide(&outputs), 2)
        .err()
        .unwrap();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("output[1]"));
}
