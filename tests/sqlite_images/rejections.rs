use super::fixture;
use bessemer::{
    FailureClass,
    sqlite_image::{SqliteImage, SqliteLimits},
};

#[test]
fn rejects_wrong_headers_wal_dependencies_partial_and_surplus_pages() {
    let case = fixture::cases().remove(1);
    let mut bad_header = case.image.clone();
    bad_header[0] = 0;
    let mut wal = case.image.clone();
    wal[18] = 2;
    wal[19] = 2;
    let mut surplus = case.image.clone();
    surplus.extend([0; 512]);
    let mut corrupt = case.image.clone();
    corrupt[100] = 0xff;
    let partial = case.image[..case.image.len() - 1].to_vec();
    for bytes in [vec![], bad_header, wal, surplus, corrupt, partial] {
        require_rejected(&bytes);
    }
}

fn require_rejected(bytes: &[u8]) {
    let error = SqliteImage::from_bytes(bytes, fixture::limits()).unwrap_err();
    assert_eq!(error.operation, "SQLite image");
    assert!(error.cause.contains("expected"));
}

#[test]
fn table_row_and_output_budget_exhaustion_keeps_the_checked_source() {
    let case = fixture::cases().remove(1);
    let mut limits = fixture::limits();
    limits.maximum_rows = 8;
    let image = SqliteImage::from_bytes(&case.image, limits).unwrap();
    assert!(image.table_rows("many").is_err());
    assert!(image.table_rows("parent").unwrap().rows().is_empty());
    assert_eq!(image.as_bytes(), case.image);
    limits = fixture::limits();
    limits.maximum_output_bytes = 32 << 10;
    let image = SqliteImage::from_bytes(&case.image, limits).unwrap();
    assert!(image.table_rows("many").is_err());
    assert_eq!(image.table_rows("child").unwrap().rows().len(), 2);
    assert_eq!(image.as_bytes(), case.image);
}

#[test]
fn every_zero_or_unrepresentable_limit_is_rejected_before_allocation() {
    let case = fixture::cases().remove(1);
    for field in 0..7 {
        for invalid in [0, usize::MAX] {
            let mut limits = fixture::limits();
            *limit_mut(&mut limits, field) = invalid;
            let error = SqliteImage::from_bytes(&case.image, limits).unwrap_err();
            assert_eq!(error.entity, "limits");
            assert_eq!(error.class, FailureClass::Invalid);
        }
    }
}

fn limit_mut(limits: &mut SqliteLimits, index: usize) -> &mut usize {
    match index {
        0 => &mut limits.maximum_image_bytes,
        1 => &mut limits.maximum_schema_objects,
        2 => &mut limits.maximum_rows,
        3 => &mut limits.maximum_columns,
        4 => &mut limits.maximum_value_bytes,
        5 => &mut limits.maximum_output_bytes,
        6 => &mut limits.maximum_vm_steps,
        _ => panic!("unexpected limit {index}"),
    }
}

#[test]
fn schema_value_output_column_image_and_vm_budgets_fail_without_partial_publication() {
    let case = fixture::cases().remove(1);
    for (field, bound) in [
        (0, case.image.len() - 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 20),
        (5, 20),
        (6, 1),
    ] {
        let mut limits = fixture::limits();
        *limit_mut(&mut limits, field) = bound;
        assert!(
            SqliteImage::from_bytes(&case.image, limits).is_err(),
            "field {field}"
        );
    }
    let mut exact = fixture::limits();
    exact.maximum_image_bytes = case.image.len();
    assert!(SqliteImage::from_bytes(&case.image, exact).is_ok());
}
