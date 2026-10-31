use bessemer::simulation::*;

fn numeric(value: i32) -> SimulationPersistVersion {
    SimulationPersistVersion::new(value).unwrap()
}

fn primary_bytes(words: &[i32]) -> Vec<u8> {
    let mut bytes = vec![2, 0, 0, 0, 0, 0];
    bytes.extend(words.iter().flat_map(|word| word.to_le_bytes()));
    bytes
}

#[test]
fn primary_boundaries_match_literal_word_counts_and_owner_destinations() {
    for (version, count, assignments) in PRIMARY_CASES {
        assert_primary_boundary(version, count, assignments);
    }
}

fn assert_primary_boundary(version: i32, count: usize, assignments: usize) {
    let words = &[-1, i32::MIN, i32::MAX][..count];
    let bytes = primary_bytes(words);
    let record = primary_fixture(&bytes, version, SimulationPersistReadMode::Full).unwrap();
    assert_eq!(record.words(), words);
    assert_eq!(record.word_payload(), &bytes[6..]);
    assert_eq!(record.owner_updates().len(), assignments);
    verify_owner_updates(&record);
    assert_eq!(record.to_bytes(), bytes);
    assert_eq!(record.consumed_end(), bytes.len());
    assert!(record.remainder().is_empty());
    assert_eq!(
        record.coverage().completion,
        SimulationPersistCompletion::CompletedFraming
    );
    assert_eq!(record.read_mode(), SimulationPersistReadMode::Full);
    assert_eq!(record.numeric_version().value(), version);
}

fn verify_owner_updates(record: &SimulationPrimaryVersion) {
    use SimulationPersistOwnerSlot::{Owner1A8, Owner1C8};
    let version = record.numeric_version().value();
    let expected = match version {
        ..=800018 => vec![],
        800019..=800076 => vec![(Owner1C8, -1)],
        800077..=830031 => vec![(Owner1C8, -1), (Owner1A8, i32::MIN)],
        _ => vec![(Owner1A8, i32::MIN), (Owner1C8, i32::MAX)],
    };
    assert_eq!(
        record
            .owner_updates()
            .iter()
            .map(|update| (update.destination, update.value))
            .collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn partial_mode_preserves_every_unconsumed_word_and_arbitrary_suffix() {
    let mut bytes = primary_bytes(&[-1, i32::MIN, i32::MAX]);
    bytes.extend_from_slice(&[0xaa, 0xbb, 0xcc]);
    let record = primary_fixture(&bytes, 830032, SimulationPersistReadMode::Partial).unwrap();
    assert_eq!(record.words(), [-1]);
    assert_eq!(record.consumed_end(), 10);
    assert_eq!(record.remainder(), &bytes[10..]);
    assert!(record.owner_updates().is_empty());
    assert_eq!(
        record.coverage().completion,
        SimulationPersistCompletion::IncompleteUnconsumedTail
    );
    assert_eq!(record.coverage().opaque_bytes, 11);
    assert_primary_reconstruction(&record, &bytes);
    let old = primary_fixture(&bytes, 800018, SimulationPersistReadMode::Partial).unwrap();
    assert!(old.words().is_empty());
    assert_eq!(old.remainder(), &bytes[6..]);
}

fn assert_primary_reconstruction(record: &SimulationPrimaryVersion, bytes: &[u8]) {
    let rebuilt = SimulationPrimaryVersion::from_parts(
        record.text(),
        record.numeric_version(),
        record.read_mode(),
        record.words(),
        record.remainder(),
        2,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), bytes);
}

#[test]
fn primary_errors_reject_every_truncation_bad_context_and_wrong_word_shape() {
    let bytes = primary_bytes(&[-1, i32::MIN, i32::MAX]);
    for end in 0..bytes.len() {
        assert!(primary_fixture(&bytes[..end], 830032, SimulationPersistReadMode::Full).is_err());
    }
    let mut extended = bytes.clone();
    extended.push(0);
    assert!(primary_fixture(&extended, 830032, SimulationPersistReadMode::Full).is_err());
    assert!(SimulationPersistVersion::new(-1).is_err());
}

#[test]
fn primary_construction_rejects_wrong_word_shape_and_full_suffix() {
    let text = SimulationPersistText::from_payload(None, 0).unwrap();
    assert!(
        SimulationPrimaryVersion::from_parts(
            &text,
            numeric(830032),
            SimulationPersistReadMode::Full,
            &[],
            &[],
            0
        )
        .is_err()
    );
}

#[test]
fn primary_full_construction_rejects_a_supplied_suffix() {
    let text = SimulationPersistText::from_payload(None, 0).unwrap();
    assert!(
        SimulationPrimaryVersion::from_parts(
            &text,
            numeric(0),
            SimulationPersistReadMode::Full,
            &[],
            &[1],
            0
        )
        .is_err()
    );
}

#[test]
fn primary_text_limit_applies_before_owning_complete_input() {
    let bytes = primary_bytes(&[-1, i32::MIN, i32::MAX]);
    assert!(
        SimulationPrimaryVersion::from_bytes(
            &bytes,
            numeric(830032),
            SimulationPersistReadMode::Full,
            1
        )
        .is_err()
    );
}

#[test]
fn legacy_digits_and_all_opaque_suffix_bits_survive_reconstruction() {
    let tail: Vec<u8> = (0..114).map(|index| (index * 37) as u8).collect();
    for version in [0, 1, 500070, 999999] {
        assert_legacy_roundtrip(version, &tail);
    }
    assert!(SimulationLegacyVersion::from_parts(numeric(1000000), &tail).is_err());
    assert!(SimulationLegacyVersion::from_parts(numeric(1), &tail[..113]).is_err());
}

fn assert_legacy_roundtrip(version: i32, tail: &[u8]) {
    let record = SimulationLegacyVersion::from_parts(numeric(version), tail).unwrap();
    assert_eq!(record.numeric_version().value(), version);
    assert_eq!(
        record.text_units().to_vec(),
        format!("{version:06}").encode_utf16().collect::<Vec<_>>()
    );
    assert_eq!(record.prefix().len(), 14);
    assert_eq!(record.opaque_suffix(), tail);
    assert_eq!(
        [record.prefix(), record.opaque_suffix()].concat(),
        record.to_bytes()
    );
    assert_eq!(record.coverage().opaque_bytes, 114);
    assert_eq!(
        record.coverage().completion,
        SimulationPersistCompletion::CompletedFraming
    );
}

#[test]
fn legacy_unsupported_profiles_never_normalize_bad_digits_or_discard_bytes() {
    let record = SimulationLegacyVersion::from_parts(numeric(500070), &[0xff; 114]).unwrap();
    let bytes = record.to_bytes();
    for end in 0..128 {
        assert!(SimulationLegacyVersion::from_bytes(&bytes[..end]).is_err());
    }
    for index in 0..14 {
        let mut malformed = bytes.clone();
        malformed[index] = 0x80;
        assert!(SimulationLegacyVersion::from_bytes(&malformed).is_err());
    }
    let mut extra = bytes;
    extra.push(0);
    assert!(SimulationLegacyVersion::from_bytes(&extra).is_err());
}

#[test]
fn available_primary_wins_and_empty_or_malformed_never_becomes_unavailable() {
    let primary = primary_bytes(&[]);
    let legacy = SimulationLegacyVersion::from_parts(numeric(999999), &[0xff; 114])
        .unwrap()
        .to_bytes();
    let selected = selection_fixture(Some(&primary), Some(&legacy), Some(1)).unwrap();
    assert!(matches!(selected, SimulationSelectedVersion::Primary(_)));
    assert_eq!(selected.numeric_version().value(), 1);
    assert!(selection_fixture(Some(&[]), Some(&legacy), Some(1)).is_err());
    assert!(selection_fixture(Some(&primary), Some(&legacy), None).is_err());
    assert!(selection_fixture(None, Some(&[]), None).is_err());
    let legacy_selected = selection_fixture(None, Some(&legacy), None).unwrap();
    assert!(matches!(
        legacy_selected,
        SimulationSelectedVersion::Legacy(_)
    ));
    assert_eq!(legacy_selected.numeric_version().value(), 999999);
}

#[test]
fn unavailable_carriers_return_compatibility_constant_without_a_record() {
    let absent =
        SimulationSelectedVersion::select(None, None, None, SimulationPersistReadMode::Full, 0)
            .unwrap();
    assert!(matches!(
        absent,
        SimulationSelectedVersion::CompatibilityDefault
    ));
    assert_eq!(absent.numeric_version().value(), 500070);
}

const PRIMARY_CASES: [(i32, usize, usize); 8] = [
    (0, 0, 0),
    (800018, 0, 0),
    (800019, 1, 1),
    (800076, 1, 1),
    (800077, 2, 2),
    (830031, 2, 2),
    (830032, 3, 2),
    (i32::MAX, 3, 2),
];

fn primary_fixture(
    bytes: &[u8],
    version: i32,
    mode: SimulationPersistReadMode,
) -> Result<SimulationPrimaryVersion, bessemer::BessemerError> {
    SimulationPrimaryVersion::from_bytes(bytes, numeric(version), mode, 2)
}

fn selection_fixture(
    primary: Option<&[u8]>,
    legacy: Option<&[u8]>,
    version: Option<i32>,
) -> Result<SimulationSelectedVersion, bessemer::BessemerError> {
    SimulationSelectedVersion::select(
        primary,
        legacy,
        version.map(numeric),
        SimulationPersistReadMode::Full,
        2,
    )
}
