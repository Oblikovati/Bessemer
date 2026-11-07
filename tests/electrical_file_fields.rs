//! Complete native descriptor transports retain stored values without provider conversion.
#[path = "electrical_file_fields/fixture.rs"]
mod fixture;
#[path = "electrical_file_fields/projection.rs"]
mod projection;
#[path = "support/sqlite_wire.rs"]
mod sqlite_wire;
use bessemer::{electrical::*, sqlite_image::SqliteValue};

#[test]
fn all_complete_maps_match_independent_native_transport_and_sqlite_values() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 44);
    let mut native_files = 0;
    for case in cases.iter().filter(|case| case.missing.is_none()) {
        let archive = fixture::archive(&case.raw);
        let view = archive.project_records(fixture::project_limits()).unwrap();
        let maps = fixture::maps(&view).unwrap();
        assert!(projection::render(&maps) == case.expected, "{}", case.name);
        if case.name.ends_with(".tewzip") {
            native_files += maps.len();
        }
        for mapped in maps {
            verify_coverage(&mapped);
        }
        assert_eq!(view.source().to_bytes(), case.raw);
    }
    assert_eq!(native_files, 17);
}

fn verify_coverage(mapped: &ElectricalFileReadMap) {
    assert_eq!(mapped.reads().len(), 38);
    assert_eq!(
        mapped.coverage().mode,
        ElectricalFileReadCheckMode::DeclaredNativeTransport
    );
    assert_eq!(mapped.coverage().mapped_reads, 38);
    assert_eq!(
        mapped.coverage().preserved_columns,
        mapped.record().columns().len()
    );
    assert_eq!(mapped.diagnostics().len(), 6);
    for read in mapped.reads() {
        assert_eq!(Some(&read.value), mapped.record().value(read.column));
    }
}

#[test]
fn every_missing_direct_column_is_a_typed_error_not_a_partial_map() {
    let cases = fixture::cases();
    let mut count = 0;
    for case in cases.iter().filter(|case| case.missing.is_some()) {
        let archive = fixture::archive(&case.raw);
        let view = archive.project_records(fixture::project_limits()).unwrap();
        let error = fixture::maps(&view).unwrap_err();
        assert_eq!(error.operation, "Electrical file read map");
        assert_eq!(error.class, bessemer::FailureClass::Missing);
        assert_eq!(Some(error.entity.as_str()), case.missing.as_deref());
        assert!(error.cause.contains("expected all 38"));
        assert_eq!(view.source().to_bytes(), case.raw);
        count += 1;
    }
    assert_eq!(count, 38);
}

#[test]
fn nulls_and_incompatible_dynamic_types_are_retained_without_coercion() {
    let cases = fixture::cases();
    let mixed = fixture::archive(&cases[0].raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let maps = fixture::maps(&mixed).unwrap();
    let mapped = &maps[0];
    assert!(
        mapped
            .reads()
            .iter()
            .any(|read| matches!(read.value, SqliteValue::Blob(_)))
    );
    assert!(
        mapped
            .reads()
            .iter()
            .any(|read| matches!(read.value, SqliteValue::Text(_)))
    );
    assert_eq!(mapped.record().columns().len(), 41);
    assert_eq!(mapped.unmapped_columns().count(), 3);
}

#[test]
fn present_nulls_are_distinct_from_missing_required_columns() {
    let cases = fixture::cases();
    let nulls = fixture::archive(&cases[1].raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let maps = fixture::maps(&nulls).unwrap();
    assert!(
        maps[0]
            .reads()
            .iter()
            .all(|read| read.value == SqliteValue::Null)
    );
    assert_eq!(
        maps[0].reads()[0].destination,
        ElectricalFileReadDestination::IdentityTemporary
    );
}

#[test]
fn reused_temporaries_and_runtime_offsets_remain_distinct() {
    let case = fixture::cases().remove(0);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let maps = fixture::maps(&view).unwrap();
    let reads = maps[0].reads();
    assert_eq!(
        reads[8].destination,
        ElectricalFileReadDestination::SharedBoolTemporary
    );
    assert_eq!(reads[8].destination, reads[12].destination);
    assert_eq!(reads[9].destination, reads[13].destination);
    assert_eq!(reads[10].destination, reads[17].destination);
}

#[test]
fn direct_object_byte_offsets_and_date_requests_keep_their_observed_values() {
    let case = fixture::cases().remove(0);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let maps = fixture::maps(&view).unwrap();
    let reads = maps[0].reads();
    assert_eq!(
        reads[1].destination,
        ElectricalFileReadDestination::ObjectByteOffset(0x40)
    );
    assert_eq!(reads[22].reader, ElectricalFileReaderKind::Date);
    assert_eq!(
        reads[22].destination,
        ElectricalFileReadDestination::ObjectByteOffset(0xc0)
    );
    assert_eq!(
        reads[37].destination,
        ElectricalFileReadDestination::ObjectByteOffset(0x1c8)
    );
}

#[test]
fn complete_unmapped_native_columns_keep_their_original_values_and_order() {
    let cases = fixture::cases();
    for case in cases.iter().filter(|case| case.name.ends_with(".tewzip")) {
        let view = fixture::archive(&case.raw)
            .project_records(fixture::project_limits())
            .unwrap();
        for mapped in fixture::maps(&view).unwrap() {
            let names: Vec<_> = mapped.unmapped_columns().map(|(name, _)| name).collect();
            assert_eq!(names, ["fil_state", "fil_updatedate"]);
            for (name, value) in mapped.unmapped_columns() {
                assert_eq!(Some(value), mapped.record().value(name));
            }
        }
    }
}

#[test]
fn immutable_owned_maps_and_eight_workers_preserve_input_isolation() {
    verify_send_sync();
    let mut case = fixture::cases().remove(0);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let maps = fixture::maps(&view).unwrap();
    case.raw.fill(0);
    run_field_workers(&maps, &view, &case.expected);
}

fn run_field_workers(
    maps: &[ElectricalFileReadMap],
    view: &ElectricalProjectView,
    expected: &[u8],
) {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let copied = maps.to_vec();
                    assert!(projection::render(&copied) == expected);
                    assert!(projection::render(&fixture::maps(view).unwrap()) == expected);
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

fn verify_send_sync() {
    assert_send_sync::<ElectricalFileReadMap>();
    assert_send_sync::<ElectricalFileRead>();
    assert_send_sync::<ElectricalFileReadDestination>();
    assert_send_sync::<ElectricalFileReaderKind>();
    assert_send_sync::<ElectricalFileReadCoverage>();
    assert_send_sync::<ElectricalFileReadCheckMode>();
    assert_send_sync::<ElectricalFileReadDiagnostic>();
}
fn assert_send_sync<T: Send + Sync>() {}
