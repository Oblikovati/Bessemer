use super::fixture;
use bessemer::electrical::ElectricalProjectLimits;

#[test]
fn every_whole_operation_limit_axis_rejects_without_mutating_the_carrier() {
    let case = fixture::cases().remove(1);
    let archive = fixture::archive(&case.raw);
    for axis in 0..5 {
        let mut limits = fixture::project_limits();
        *limit_mut(&mut limits, axis) = 1;
        assert!(archive.project_records(limits).is_err(), "axis {axis}");
        *limit_mut(&mut limits, axis) = 0;
        assert!(archive.project_records(limits).is_err());
        assert_eq!(archive.to_bytes(), case.raw);
    }
}

fn limit_mut(limits: &mut ElectricalProjectLimits, axis: usize) -> &mut usize {
    match axis {
        0 => &mut limits.maximum_projects,
        1 => &mut limits.maximum_database_links,
        2 => &mut limits.maximum_file_links,
        3 => &mut limits.maximum_checked_images,
        4 => &mut limits.maximum_accounted_bytes,
        _ => panic!("unexpected axis {axis}"),
    }
}

#[test]
fn each_relationship_limit_accepts_its_exact_independent_denominator() {
    let case = fixture::cases().remove(1);
    let archive = fixture::archive(&case.raw);
    for (axis, exact) in [12, 10, 50, 5].into_iter().enumerate() {
        let mut limits = fixture::project_limits();
        *limit_mut(&mut limits, axis) = exact;
        assert!(archive.project_records(limits).is_ok());
        *limit_mut(&mut limits, axis) -= 1;
        assert!(archive.project_records(limits).is_err());
    }
    let mut limits = fixture::project_limits();
    limits.maximum_accounted_bytes = usize::MAX;
    assert!(archive.project_records(limits).is_ok());
}

#[test]
fn completed_accounting_bound_is_exact_and_empty_results_still_validate_sqlite_limits() {
    let cases = fixture::cases();
    let archive = fixture::archive(&cases[1].raw);
    let view = archive.project_records(fixture::project_limits()).unwrap();
    let mut limits = fixture::project_limits();
    limits.maximum_accounted_bytes = view.coverage().accounted_bytes;
    assert!(archive.project_records(limits).is_ok());
    limits.maximum_accounted_bytes -= 1;
    assert!(archive.project_records(limits).is_err());
    let empty = fixture::archive(&cases[0].raw);
    limits = fixture::project_limits();
    limits.sqlite.maximum_rows = 0;
    assert!(empty.project_records(limits).is_err());
}

#[test]
fn corrupt_or_missing_required_tables_are_errors_with_member_context_and_input_isolation() {
    let case = fixture::cases().remove(3);
    let archive = fixture::archive(&case.raw);
    let catalog = archive
        .member_named("Database/tew_app_project.sqlite")
        .unwrap();
    let database = archive
        .member_named("Projects/2/Database/tew_project_data.sqlite")
        .unwrap();
    for replacement in [
        b"corrupt".as_slice(),
        archive.read_member(&catalog).unwrap(),
    ] {
        let edit = archive.edit_member(&database, replacement).unwrap();
        require_member_context_error(edit.result());
        assert_eq!(archive.to_bytes(), case.raw);
    }
}

fn require_member_context_error(archive: &bessemer::electrical::ElectricalArchive) {
    let error = archive
        .project_records(fixture::project_limits())
        .unwrap_err();
    assert_eq!(error.operation, "Electrical project records");
    assert!(error.entity.contains("member"));
}
