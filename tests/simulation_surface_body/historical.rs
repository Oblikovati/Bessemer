use super::*;

#[test]
fn complete_synthetic_860006_body_keeps_both_declared_words_and_is_not_blacklisted() {
    let version = SimulationPersistVersion::new(860006).unwrap();
    let raw = [0; 92];
    let body = SimulationSurfaceBody::from_bytes(&raw, version, LIMITS).unwrap();
    assert_eq!(body.to_bytes(), raw);
    assert_eq!(body.fields().len(), 18);
    assert_eq!(&body.fields()[16].path[..], "words100_94[0]");
    assert_eq!(&body.fields()[17].path[..], "words100_94[1]");
    assert_eq!(
        body.coverage().completion,
        SimulationSurfaceBodyCompletion::CompletedBody
    );
}

fn check_historical_case(cursor: &mut &[u8]) {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    assert_eq!(version.value(), 860006);
    let size = read_word(cursor) as usize;
    let complete_prefix = read_word(cursor) as usize;
    let required_next = read_word(cursor) as usize;
    let raw = take_projection(cursor, size).to_vec();
    let original = raw.clone();
    assert_eq!(required_next, 8);
    assert_eq!(raw.len() - complete_prefix, 4);
    let error = SimulationSurfaceBody::from_bytes(&raw, version, LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(!error.operation.is_empty() && !error.entity.is_empty());
    assert!(error.cause.contains("expected"));
    assert_eq!(raw, original);
}

#[test]
#[ignore = "requires freshly verified exact C-740 historical failure inputs"]
fn all_six_historical_partial_word_pairs_remain_typed_failures_without_repair() {
    let path = std::env::var_os("BESSEMER_SURFACE_BODY_HISTORICAL_INPUT")
        .expect("set complete historical failure packet");
    let packet = std::fs::read(path).unwrap();
    let mut cursor = packet.as_slice();
    let mut count = 0;
    while !cursor.is_empty() {
        check_historical_case(&mut cursor);
        count += 1;
    }
    assert_eq!(count, 6);
}
