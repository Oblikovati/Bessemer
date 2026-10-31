use super::*;

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-mesh-control/records-budgets.bin");

fn limit_signature(limits: SimulationMeshControlLimits) -> [usize; 9] {
    [
        limits.maximum_bytes,
        limits.maximum_fields,
        limits.maximum_entries,
        limits.maximum_text_bytes,
        limits.embedded.maximum_bytes,
        limits.embedded.maximum_fields,
        limits.embedded.maximum_entries,
        limits.embedded.maximum_depth,
        limits.embedded.maximum_objects,
    ]
}

fn assert_statistics(body: &SimulationMeshControlRecords, cursor: &mut &[u8]) {
    let coverage = body.coverage();
    assert_eq!(coverage.record_count, read_word(cursor) as usize);
    assert_eq!(coverage.embedded_archives, read_word(cursor) as usize);
    assert_eq!(coverage.embedded_fields, read_word(cursor) as usize);
    assert_eq!(body.record_spans().len(), coverage.record_count);
    for record in body.record_spans() {
        let expected = (
            read_word(cursor) as usize,
            read_word(cursor) as usize,
            read_word(cursor) as usize,
            read_word(cursor) as usize,
        );
        assert_eq!(
            (record.index, record.start, record.base_end, record.end),
            expected
        );
    }
}

fn assert_coverage(body: &SimulationMeshControlRecords, exact: SimulationMeshControlLimits) {
    let coverage = body.coverage();
    let expected = if body.record_spans().is_empty() {
        SimulationMeshControlCompletion::CompletedEmptyRecords
    } else {
        SimulationMeshControlCompletion::CompletedRecords
    };
    assert_eq!(coverage.completion, expected);
    assert_eq!(
        coverage.check_mode,
        SimulationMeshControlCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(coverage.declared_entries, exact.maximum_entries);
    assert_eq!(coverage.version, body.version());
    assert_eq!(limit_signature(coverage.limits), limit_signature(LIMITS));
    assert_eq!(coverage.diagnostics.len(), 4);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
}

fn check_boundary(
    body: &SimulationMeshControlRecords,
    limits: SimulationMeshControlLimits,
    valid: bool,
) {
    let parsed = SimulationMeshControlRecords::from_bytes(&body.to_bytes(), body.version(), limits);
    let constructed =
        SimulationMeshControlRecords::from_fields(body.fields().to_vec(), body.version(), limits);
    assert_eq!(parsed.is_ok(), valid, "{limits:?}");
    assert_eq!(constructed.is_ok(), valid, "{limits:?}");
    if let Ok(parsed) = parsed {
        assert_eq!(parsed.to_bytes(), body.to_bytes());
    }
    if let Ok(constructed) = constructed {
        assert_eq!(constructed.to_bytes(), body.to_bytes());
    }
}

fn check_packet(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        let body = read_collection(&mut cursor);
        let mut exact = load_limits::read_reference_limits(&mut cursor, LIMITS);
        exact.maximum_bytes = body.to_bytes().len();
        assert_coverage(&body, exact);
        assert_statistics(&body, &mut cursor);
        check_boundary(&body, exact, true);
        for reduced in load_limits::reduced_limits(exact) {
            check_boundary(&body, reduced, false);
        }
        count += 1;
    }
    count
}

#[test]
fn all_owned_records_match_independent_statistics_extents_and_seven_resource_boundaries() {
    assert_eq!(check_packet(EXPECTED), 322);
}

#[test]
#[ignore = "requires complete independent native MeshControl record budget and coverage expectations"]
fn all_native_records_match_complete_statistics_and_exact_resource_boundaries() {
    let path = std::env::var_os("BESSEMER_MESH_RECORDS_NATIVE_BUDGET_INPUT")
        .expect("set native record budgets");
    assert_eq!(check_packet(&std::fs::read(path).unwrap()), 12);
}
