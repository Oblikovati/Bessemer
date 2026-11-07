use bessemer::simulation::*;

#[test]
fn primary_source_drop_returned_mutation_and_full_reconstruction_are_isolated() {
    let mut bytes = vec![
        0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 128, 255, 255, 255, 127,
    ];
    let expected = bytes.clone();
    let version = SimulationPersistVersion::new(830032).unwrap();
    let record =
        SimulationPrimaryVersion::from_bytes(&bytes, version, SimulationPersistReadMode::Full, 0)
            .unwrap();
    bytes.fill(0);
    drop(bytes);
    verify_primary_input_isolation(&record, &expected);
}

fn verify_primary_input_isolation(record: &SimulationPrimaryVersion, expected: &[u8]) {
    let rebuilt = SimulationPrimaryVersion::from_parts(
        record.text(),
        record.numeric_version(),
        record.read_mode(),
        record.words(),
        record.remainder(),
        0,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), expected);
    let mut exported = record.to_bytes();
    exported.fill(0x81);
    assert_eq!(record.to_bytes(), expected);
    assert_eq!(record.text().source_parent(), expected);
}

#[test]
fn legacy_source_drop_and_returned_mutation_preserve_all_suffix_bytes() {
    let mut bytes: Vec<_> = "500070\0"
        .encode_utf16()
        .flat_map(|unit| unit.to_le_bytes())
        .collect();
    bytes.extend(0..114);
    let expected = bytes.clone();
    let record = SimulationLegacyVersion::from_bytes(&bytes).unwrap();
    bytes.fill(0);
    drop(bytes);
    let mut exported = record.to_bytes();
    exported.fill(0xff);
    assert_eq!(record.to_bytes(), expected);
    assert_eq!(record.opaque_suffix(), &expected[14..]);
}

#[test]
fn all_study_texts_share_one_owned_parent_and_survive_caller_source_mutation() {
    let mut bytes = vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let expected = bytes.clone();
    let record = SimulationStudyOrder::from_bytes(&bytes, 2, 0).unwrap();
    bytes.fill(0x81);
    drop(bytes);
    for text in std::iter::once(record.leading()).chain(record.items()) {
        assert_eq!(text.source_parent(), expected);
        assert_eq!(
            text.source_parent().as_ptr(),
            record.leading().source_parent().as_ptr()
        );
    }
    assert_eq!(record.to_bytes(), expected);
}

#[test]
fn default_text_limit_boundary_and_explicit_larger_odd_limit_are_distinct() {
    for count in [0x100000usize, 0x100001] {
        let mut bytes = (count as u32).to_le_bytes().to_vec();
        bytes.resize(count + 4, 0);
        bytes[4] = 0x81;
        assert_eq!(
            SimulationPersistText::read_from(&bytes, 0, 0x100000).is_ok(),
            count == 0x100000
        );
        let record = SimulationPersistText::read_from(&bytes, 0, count).unwrap();
        assert_eq!(record.to_bytes(), bytes);
        assert_eq!(record.utf16_units().is_some(), count.is_multiple_of(2));
        assert_eq!(
            SimulationPersistText::from_payload(Some(&bytes[4..]), 0x100000).is_ok(),
            count == 0x100000
        );
    }
}

#[test]
fn configured_study_item_limit_accepts_boundary_and_rejects_next_count() {
    let mut bytes = vec![0, 0, 0, 0];
    bytes.extend_from_slice(&10000u32.to_le_bytes());
    bytes.resize(40008, 0);
    let record = SimulationStudyOrder::from_bytes(&bytes, 10000, 0).unwrap();
    assert_eq!(record.items().len(), 10000);
    assert!(SimulationStudyOrder::from_bytes(&bytes, 9999, 0).is_err());
    bytes[4..8].copy_from_slice(&10001u32.to_le_bytes());
    bytes.extend_from_slice(&[0; 4]);
    assert!(SimulationStudyOrder::from_bytes(&bytes, 10000, 0).is_err());
    assert_eq!(record.items().len(), 10000);
}
