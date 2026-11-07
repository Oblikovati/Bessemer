use super::checked_count;

#[test]
fn native_count_boundary_is_checked_without_allocating_giant_values() {
    assert_eq!(checked_count(0).unwrap(), 0);
    assert_eq!(checked_count(u32::MAX as usize).unwrap(), u32::MAX);
    let Some(above) = (u32::MAX as usize).checked_add(1) else {
        return;
    };
    let error = checked_count(above).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains(&above.to_string()));
}
