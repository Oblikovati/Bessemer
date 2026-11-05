//! Independent SQLite schema/row snapshots and operation isolation.
#[path = "sqlite_images/fixture.rs"]
mod fixture;
#[path = "sqlite_images/projection.rs"]
mod projection;
#[path = "sqlite_images/rejections.rs"]
mod rejections;
#[path = "support/sqlite_wire.rs"]
mod sqlite_wire;
use bessemer::sqlite_image::{
    SqliteCheckMode, SqliteCoverage, SqliteDiagnostic, SqliteImage, SqliteLimits, SqliteRows,
    SqliteSchemaObject, SqliteValue,
};

#[test]
fn every_owned_and_native_image_matches_independent_python_schema_and_rows() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 18);
    for case in cases {
        let image = SqliteImage::from_bytes(&case.image, fixture::limits()).unwrap();
        assert_eq!(image.as_bytes(), case.image);
        assert_eq!(image.coverage().mode, SqliteCheckMode::FullIntegrity);
        assert_eq!(
            image.coverage().checked_schema_objects,
            image.objects().len()
        );
        assert_eq!(image.coverage().wire_bytes, case.image.len());
        assert_eq!(projection::render(&image), case.expected, "{}", case.name);
        assert!(
            image
                .diagnostics()
                .contains(&SqliteDiagnostic::NativeApplicationNotChecked)
        );
    }
}

#[test]
fn empty_results_are_complete_and_missing_case_variants_are_errors() {
    let cases = fixture::cases();
    let empty = SqliteImage::from_bytes(&cases[0].image, fixture::limits()).unwrap();
    assert!(empty.objects().is_empty());
    let mixed = SqliteImage::from_bytes(&cases[1].image, fixture::limits()).unwrap();
    let rows = mixed.table_rows("parent").unwrap();
    assert!(rows.rows().is_empty());
    assert_eq!(rows.columns(), ["id"]);
    assert!(rows.accounted_bytes() > 0);
    assert_eq!(
        mixed.table_rows("Parent").unwrap_err().class,
        bessemer::FailureClass::Missing
    );
    assert!(mixed.table_rows("untouched_view").is_err());
}

#[test]
fn exact_types_duplicates_generated_fields_and_quoted_names_survive() {
    let case = fixture::cases().remove(1);
    let image = SqliteImage::from_bytes(&case.image, fixture::limits()).unwrap();
    let rows = image.table_rows("odd\";DROP TABLE nope;--").unwrap();
    assert_eq!(rows.rows().len(), 4);
    assert_eq!(rows.rows()[0], rows.rows()[2]);
    assert_eq!(rows.rows()[0][0], SqliteValue::Integer(i64::MIN));
    assert_eq!(rows.rows()[1][0], SqliteValue::Integer(i64::MAX));
    assert_eq!(rows.rows()[1][2], SqliteValue::Text([0x80, 0xff, 0].into()));
    assert_eq!(rows.rows()[1][3], SqliteValue::Blob([].into()));
    assert_eq!(rows.rows()[0][4], SqliteValue::Null);
    assert_eq!(
        image.table_rows("child").unwrap().rows()[0][2],
        SqliteValue::Integer(6)
    );
}

#[test]
fn real_engine_bits_and_nondefault_page_sizes_are_preserved() {
    let cases = fixture::cases();
    let image = SqliteImage::from_bytes(&cases[1].image, fixture::limits()).unwrap();
    let rows = image.table_rows("odd\";DROP TABLE nope;--").unwrap();
    assert_eq!(
        rows.rows()[0][1],
        SqliteValue::RealBits((-1.25_f64).to_bits())
    );
    assert_eq!(
        rows.rows()[1][1],
        SqliteValue::RealBits(f64::INFINITY.to_bits())
    );
    for case in &cases[2..5] {
        let image = SqliteImage::from_bytes(&case.image, fixture::limits()).unwrap();
        assert_eq!(
            image.table_rows("sized").unwrap().rows()[0],
            [SqliteValue::Integer(17)]
        );
        assert_eq!(image.as_bytes(), case.image);
    }
}

#[test]
fn published_types_are_send_sync_and_source_buffers_are_isolated() {
    assert_send_sync::<SqliteImage>();
    assert_send_sync::<SqliteRows>();
    assert_send_sync::<SqliteSchemaObject>();
    assert_send_sync::<SqliteValue>();
    assert_send_sync::<SqliteLimits>();
    assert_send_sync::<SqliteCoverage>();
    assert_send_sync::<SqliteDiagnostic>();
    assert_send_sync::<SqliteCheckMode>();
    let mut case = fixture::cases().remove(1);
    let image = SqliteImage::from_bytes(&case.image, fixture::limits()).unwrap();
    case.image.fill(0);
    assert_eq!(projection::render(&image), case.expected);
    assert!(image.table_rows("missing").is_err());
    assert_eq!(projection::render(&image.clone()), case.expected);
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn eight_operation_local_connections_match_serial_results() {
    let case = fixture::cases().remove(1);
    let image = SqliteImage::from_bytes(&case.image, fixture::limits()).unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    assert!(image.table_rows("missing").is_err());
                    assert_eq!(projection::render(&image), case.expected);
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(projection::render(&image), case.expected);
}

#[test]
fn electrical_member_adapter_checks_the_same_immutable_image() {
    let raw = include_bytes!("fixtures/electrical-projects/JIS.proj.tewzip");
    let limits = bessemer::electrical::ElectricalLimits {
        maximum_bytes: 1 << 24,
        maximum_members: 100,
        maximum_member_bytes: 1 << 24,
        maximum_expanded_bytes: 1 << 26,
        maximum_ini_records: 1000,
    };
    let archive = bessemer::electrical::ElectricalArchive::from_bytes(raw, limits).unwrap();
    let id = archive
        .member_named("Database/tew_app_project.sqlite")
        .unwrap();
    let image = archive.sqlite_member(&id, fixture::limits()).unwrap();
    assert_eq!(image.as_bytes(), archive.read_member(&id).unwrap());
    assert_eq!(image.table_rows("tew_project").unwrap().rows().len(), 1);
    assert_eq!(archive.to_bytes(), raw);
}
