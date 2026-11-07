use bessemer::simulation::*;

#[test]
fn study_lists_preserve_literal_null_duplicates_and_all_boundaries() {
    let bytes = [
        2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 65, 0, 0, 0, 4, 0, 0, 0, 65, 0, 0, 0,
    ];
    let record = SimulationStudyOrder::from_bytes(&bytes, 3, 4).unwrap();
    assert_eq!(record.count_offset(), 6);
    assert_eq!(record.count_header(), [3, 0, 0, 0]);
    assert_eq!(record.leading().utf16_units(), Some(vec![]));
    assert!(record.items()[0].is_null());
    assert_eq!(record.items()[1].to_bytes(), record.items()[2].to_bytes());
    assert_eq!(record.items()[2].source_offsets(), (22, 26, 30));
    assert_eq!(record.items()[1].source_parent(), bytes);
    assert_eq!(record.to_bytes(), bytes);
    verify_study_construction_and_identities(&record, &bytes);
}

fn verify_study_construction_and_identities(record: &SimulationStudyOrder, bytes: &[u8]) {
    let rebuilt = SimulationStudyOrder::from_parts(record.leading(), record.items(), 3, 4).unwrap();
    assert_eq!(rebuilt.to_bytes(), bytes);
    assert_eq!(record.coverage().consumed_bytes, 30);
    assert_eq!(
        record.coverage().check_mode,
        SimulationPersistCheckMode::ExactStudyOrder
    );
    assert_eq!(
        record.text(&record.leading_id()).unwrap().to_bytes(),
        record.leading().to_bytes()
    );
    assert_eq!(
        record.text(&record.item_id(2).unwrap()).unwrap().to_bytes(),
        record.items()[2].to_bytes()
    );
}

#[test]
fn study_count_bounds_all_prefixes_extra_bytes_and_foreign_ids_fail() {
    let bytes = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
    for end in 0..bytes.len() {
        assert!(SimulationStudyOrder::from_bytes(&bytes[..end], 1, 0).is_err());
    }
    let record = SimulationStudyOrder::from_bytes(&bytes, 1, 0).unwrap();
    assert!(SimulationStudyOrder::from_bytes(&bytes, 0, 0).is_err());
    assert!(
        SimulationStudyOrder::from_bytes(&[0, 0, 0, 0, 255, 255, 255, 255], usize::MAX, 0).is_err()
    );
    let mut extended = bytes.to_vec();
    extended.push(0);
    assert!(SimulationStudyOrder::from_bytes(&extended, 1, 0).is_err());
    assert!(record.item_id(1).is_err());
    assert!(record.item_id(usize::MAX).is_err());
    let foreign = SimulationStudyOrder::from_bytes(&bytes, 1, 0).unwrap();
    assert_ne!(record.revision(), foreign.revision());
    assert!(record.text(&foreign.item_id(0).unwrap()).is_err());
    assert!(SimulationStudyOrder::from_parts(record.leading(), record.items(), 0, 0).is_err());
}

#[test]
fn completed_empty_list_preserves_odd_leading_text_and_limits_construction() {
    let leading = SimulationPersistText::from_payload(Some(&[0x81, 0, 0]), 3).unwrap();
    let record = SimulationStudyOrder::from_parts(&leading, &[], 0, 3).unwrap();
    assert_eq!(record.to_bytes(), [3, 0, 0, 0, 0x81, 0, 0, 0, 0, 0, 0]);
    assert!(record.items().is_empty());
    assert_eq!(
        record.coverage().completion,
        SimulationPersistCompletion::CompletedFraming
    );
    assert!(SimulationStudyOrder::from_parts(&leading, &[], 0, 2).is_err());
}
