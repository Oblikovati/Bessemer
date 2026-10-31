use super::*;

fn reject_bad_shape(body: &SimulationResultsProperty, fields: Vec<SimulationResultsField>) {
    assert!(SimulationResultsProperty::from_fields(fields, body.context(), LIMITS).is_err());
}

fn assert_bad_shapes(body: &SimulationResultsProperty) {
    let mut fields = body.fields().to_vec();
    fields[0].path = format!("{}.invalid", fields[0].path).into();
    reject_bad_shape(body, fields);
    let mut fields = body.fields().to_vec();
    fields[0].value = SimulationResultsValue::Unsigned(0);
    reject_bad_shape(body, fields);
    let mut fields = body.fields().to_vec();
    fields.pop();
    reject_bad_shape(body, fields);
    let mut fields = body.fields().to_vec();
    fields.push(fields[0].clone());
    reject_bad_shape(body, fields);
}

#[test]
fn every_context_rejects_wrong_paths_kinds_missing_and_extra_fields() {
    contracts::each_owned(|_, _, body| assert_bad_shapes(&body));
}

fn assert_incomplete_images(body: &SimulationResultsProperty) {
    let raw = body.to_bytes();
    for end in 0..raw.len() {
        assert!(
            SimulationResultsProperty::from_bytes(&raw[..end], body.context(), LIMITS).is_err()
        );
    }
    let mut trailing = raw.clone();
    trailing.push(0);
    assert!(SimulationResultsProperty::from_bytes(&trailing, body.context(), LIMITS).is_err());
    assert_eq!(body.to_bytes(), raw);
}

#[test]
fn every_context_rejects_every_proper_prefix_and_appended_byte() {
    contracts::each_owned(|_, _, body| assert_incomplete_images(&body));
}

fn check_mutation(cursor: &mut &[u8]) -> bool {
    let level = read_word(cursor) as usize;
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let accepted = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let parsed = SimulationResultsProperty::from_bytes(raw, context(level, version), LIMITS);
    assert_eq!(parsed.is_ok(), accepted, "{level}/{version:?}/{raw:?}");
    if let Ok(body) = parsed {
        assert_constructed(&body);
    }
    accepted
}

#[test]
fn all_targeted_count_and_text_mutations_match_actual_complete_reference_acceptance() {
    let mut cursor = &include_bytes!("../fixtures/simulation-results-headers/malformed.bin")[..];
    let mut counts = [0; 2];
    while !cursor.is_empty() {
        counts[usize::from(check_mutation(&mut cursor))] += 1;
    }
    assert_eq!(counts, [3962, 3]);
    println!("complete malformed rejected/accepted counts: {counts:?}");
}
