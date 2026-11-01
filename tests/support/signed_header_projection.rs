use super::*;

pub(super) fn read_original(cursor: &mut &[u8]) -> HeaderBody {
    let profile = PROFILES[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    HeaderBody::from_bytes(take_projection(cursor, size), version, profile, LIMITS).unwrap()
}

fn assert_constructed(header: &HeaderBody) {
    let rebuilt = HeaderBody::from_fields(
        header.fields().to_vec(),
        header.version(),
        header.wire_profile(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), header.to_bytes());
    assert_eq!(rebuilt.fields(), header.fields());
    assert_eq!(rebuilt.field_offsets(), header.field_offsets());
    assert_eq!(rebuilt.version(), header.version());
    assert_eq!(rebuilt.wire_profile(), header.wire_profile());
    assert_ne!(rebuilt.field_id(0).unwrap(), header.field_id(0).unwrap());
}

fn assert_coverage(header: &HeaderBody) {
    let coverage = header.coverage();
    assert_eq!(coverage.completion, HeaderCompletion::CompletedBody);
    assert_eq!(coverage.check_mode, HeaderCheckMode::ExactStandalone);
    assert_eq!(coverage.checked_bytes, header.to_bytes().len());
    assert_eq!(coverage.checked_fields, header.fields().len());
    assert_eq!(coverage.version, header.version());
    assert_eq!(coverage.wire_profile, header.wire_profile());
    assert_eq!(coverage.limits, LIMITS);
    assert_eq!(
        (coverage.body_count, coverage.requested_pattern_error_bits),
        (1, 0)
    );
    assert_eq!(coverage.diagnostics.len(), 4);
}

pub(super) fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let header = read_original(cursor);
    assert_constructed(&header);
    assert_coverage(&header);
    project_word(
        output,
        usize::from(header.wire_profile() == HeaderProfile::Writer),
    );
    project_word(output, header.version().value() as usize);
    let raw = header.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, header.fields().len());
    for (field, span) in header.fields().iter().zip(header.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}
