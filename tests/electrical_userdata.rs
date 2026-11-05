//! Independent SQL-affinity, lexical userdata and native signed identity-gate tests.
#[path = "electrical_userdata/fixture.rs"]
mod fixture;
#[path = "support/sqlite_row_wire.rs"]
mod sqlite_row_wire;
#[path = "support/sqlite_wire.rs"]
mod sqlite_wire;
use bessemer::{
    electrical::*,
    sqlite_image::{SqliteRows, SqliteValue},
};

#[test]
fn all_78_owned_and_native_queries_match_complete_independent_rows() {
    let (cases, _) = fixture::cases();
    assert_eq!(cases.len(), 7);
    let mut count = 0;
    for case in cases {
        let archive = fixture::archive(&case.raw);
        for (key, expected) in &case.queries {
            let query = fixture::query(&archive, *key);
            assert_rows(query.rows(), expected, &case.name);
            assert_eq!(query.key(), *key);
            assert_eq!(query.exists(), !query.rows().rows().is_empty());
            assert_eq!(query.source().to_bytes(), case.raw);
            assert_eq!(query.diagnostics().len(), 3);
            assert_eq!(
                query.coverage().wire_bytes,
                archive.read_member(query.member()).unwrap().len()
            );
            count += 1;
        }
    }
    assert_eq!(count, 78);
}

#[test]
fn lexical_object_ids_preserve_all_numbers_unknown_columns_and_source_values() {
    let (cases, _) = fixture::cases();
    for case in cases {
        let view = fixture::archive(&case.raw)
            .project_records(fixture::project_limits())
            .unwrap();
        let database = &view.projects()[0].databases[0];
        for (object_id, expected) in case.lexical {
            let rows = database.userdata_for_object(object_id).unwrap();
            assert_rows(&rows, &expected, &case.name);
            assert!(rows.accounted_bytes() <= database.userdata.accounted_bytes());
        }
        assert_eq!(view.source().to_bytes(), case.raw);
    }
}

#[test]
fn all_81_signed_id_transitions_match_the_original_gate() {
    let (_, cases) = fixture::cases();
    assert_eq!(cases.len(), 81);
    for (current, loaded, expected) in cases {
        assert_eq!(electrical_userdata_id_transition(current, loaded), expected);
    }
}

#[test]
fn text_affinity_and_numeric_lexical_equality_are_separate_predicates() {
    let (cases, _) = fixture::cases();
    let text = fixture::archive(&cases[1].raw);
    let query = fixture::query(&text, key(26, 0));
    assert_eq!(query.rows().rows().len(), 1);
    assert_eq!(
        query.rows().rows()[0][0],
        SqliteValue::Text(b"26".as_slice().into())
    );
    let view = text.project_records(fixture::project_limits()).unwrap();
    assert!(
        view.projects()[0].databases[0]
            .userdata_for_object(26)
            .unwrap()
            .rows()
            .is_empty()
    );
}

#[test]
fn integral_real_matches_are_lexical_but_text_and_blob_keys_are_not() {
    let (cases, _) = fixture::cases();
    let numeric = fixture::archive(&cases[2].raw);
    assert_eq!(fixture::query(&numeric, key(7, 0)).rows().rows().len(), 2);
    let view = numeric.project_records(fixture::project_limits()).unwrap();
    assert_eq!(
        view.projects()[0].databases[0]
            .userdata_for_object(7)
            .unwrap()
            .rows()
            .len(),
        3
    );
}

#[test]
fn row_existence_duplicates_null_blob_and_id_rejection_remain_independent() {
    let (cases, _) = fixture::cases();
    let archive = fixture::archive(&cases[0].raw);
    let query = fixture::query(&archive, key(-1, 0));
    assert!(query.exists());
    assert!(!electrical_userdata_id_transition(7, -1).delegated_success);
    let rows = fixture::query(&archive, key(7, 0));
    assert_eq!(rows.rows().rows().len(), 34);
    assert_eq!(rows.rows().columns().len(), 22);
    assert!(!rows.rows().columns().iter().any(|name| name == "unknown"));
    assert_eq!(rows.rows().rows()[0][3], SqliteValue::Null);
    assert_eq!(rows.rows().rows()[0][4], SqliteValue::Blob([0, 255].into()));
    assert_eq!(
        rows.rows().rows()[0][5],
        SqliteValue::RealBits(1.25_f64.to_bits())
    );
    let empty = fixture::query(&archive, key(7, 999));
    assert!(!empty.exists());
    assert_eq!(empty.rows().columns().len(), 22);
    assert!(empty.rows().rows().is_empty());
}

#[test]
fn sql_failures_are_errors_instead_of_completed_absence() {
    let (cases, _) = fixture::cases();
    let archive = fixture::archive(&cases[0].raw);
    let catalog = archive
        .member_named("Database/tew_app_project.sqlite")
        .unwrap();
    assert!(
        archive
            .userdata_by_key(&catalog, key(7, 0), fixture::sqlite_limits())
            .is_err()
    );
    assert_eq!(archive.to_bytes(), cases[0].raw);
}

#[test]
fn missing_lexical_id_columns_complete_empty_without_rewriting_source_rows() {
    let (cases, _) = fixture::cases();
    let view = fixture::archive(&cases[0].raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let original = &view.projects()[0].databases[0];
    let mut alternate = original.clone();
    alternate.userdata = alternate.locations.clone();
    let selected = alternate.userdata_for_object(7).unwrap();
    assert!(selected.rows().is_empty());
    assert_eq!(selected.columns(), alternate.locations.columns());
    assert_eq!(original.userdata.rows().len(), 40);
}

#[test]
fn foreign_member_identities_are_rejected_and_sources_stay_isolated() {
    let (cases, _) = fixture::cases();
    let archive = fixture::archive(&cases[0].raw);
    let other = fixture::archive(&cases[1].raw);
    let foreign = other
        .member_named("Projects/2/Database/tew_project_data.sqlite")
        .unwrap();
    assert!(
        archive
            .userdata_by_key(&foreign, key(7, 0), fixture::sqlite_limits())
            .is_err()
    );
    assert_eq!(archive.to_bytes(), cases[0].raw);
}

#[test]
fn required_query_budgets_reject_partial_rows_and_interrupted_checks() {
    let (cases, _) = fixture::cases();
    let archive = fixture::archive(&cases[0].raw);
    let member = archive
        .member_named("Projects/2/Database/tew_project_data.sqlite")
        .unwrap();
    let mut limits = fixture::sqlite_limits();
    limits.maximum_rows = 24;
    assert!(archive.userdata_by_key(&member, key(7, 0), limits).is_err());
    assert!(
        archive
            .userdata_by_key(&member, key(0, 0), limits)
            .unwrap()
            .exists()
    );
    limits.maximum_vm_steps = 1;
    assert!(archive.userdata_by_key(&member, key(0, 0), limits).is_err());
    assert_eq!(archive.to_bytes(), cases[0].raw);
}

#[test]
fn send_sync_results_survive_input_mutation_and_eight_mixed_workers() {
    assert_send_sync::<ElectricalUserdataQuery>();
    assert_send_sync::<ElectricalUserdataKey>();
    assert_send_sync::<ElectricalUserdataTransition>();
    let (mut cases, _) = fixture::cases();
    let case = cases.remove(0);
    let archive = fixture::archive(&case.raw);
    let query = fixture::query(&archive, key(7, 0));
    let expected = query.rows().clone();
    let mut raw = case.raw;
    raw.fill(0);
    run_userdata_workers(&archive, &query, &expected);
}

fn run_userdata_workers(
    archive: &ElectricalArchive,
    query: &ElectricalUserdataQuery,
    expected: &SqliteRows,
) {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(fixture::query(archive, key(7, 0)).rows(), expected);
                    assert!(!fixture::query(archive, key(7, 999)).exists());
                    assert_eq!(query.clone().rows(), expected);
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

fn assert_rows(rows: &SqliteRows, expected: &[u8], name: &str) {
    let mut encoded = Vec::new();
    sqlite_row_wire::write_sqlite_rows(&mut encoded, rows);
    assert!(
        encoded == expected,
        "{name}: {} versus {} bytes",
        encoded.len(),
        expected.len()
    );
}
fn key(object_id: i32, object_no: i32) -> ElectricalUserdataKey {
    ElectricalUserdataKey {
        object_id,
        object_no,
    }
}
fn assert_send_sync<T: Send + Sync>() {}
