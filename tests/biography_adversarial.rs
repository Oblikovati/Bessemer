//! Biography framing, bounded lengths, nonminimal strings, and generated corruption.
use bessemer::biography::BiographyArchive;

const MIXED: &[u8] = include_bytes!("fixtures/biography/mixed.bin");

#[test]
fn every_truncation_and_invalid_record_count_fails_without_partial_records() {
    for end in 0..MIXED.len() {
        assert!(
            BiographyArchive::from_bytes(&MIXED[..end]).is_err(),
            "{end}"
        );
    }
    for count in [i32::MIN, -1, i32::MAX] {
        let mut bytes = MIXED.to_vec();
        bytes[19..23].copy_from_slice(&count.to_le_bytes());
        let error = BiographyArchive::from_bytes(&bytes).unwrap_err();
        assert!(error.cause.contains(&count.to_string()));
    }
    let mut trailing = MIXED.to_vec();
    trailing.push(0);
    assert!(BiographyArchive::from_bytes(&trailing).is_err());
    assert!(BiographyArchive::from_bytes(&[0, 0]).is_err());
}

#[test]
fn nonminimal_cstring_widths_survive_equivalent_edits_and_reconstruction() {
    let mut bytes = MIXED.to_vec();
    assert_eq!(&bytes[59..63], &[3, b'S', b'P', 0xe9]);
    bytes.splice(59..60, [255, 255, 255, 3, 0, 0, 0]);
    let source = BiographyArchive::from_bytes(&bytes).unwrap();
    assert_eq!(
        source
            .with_records(source.records().to_vec())
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
    let rebuilt = BiographyArchive::new(source.root_schema(), source.records().to_vec()).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
}

#[test]
fn generated_mutations_are_either_typed_failures_or_stable_complete_archives() {
    for index in 0..MIXED.len() {
        let mut bytes = MIXED.to_vec();
        bytes[index] ^= 0xff;
        let Ok(parsed) = BiographyArchive::from_bytes(&bytes) else {
            continue;
        };
        let rebuilt =
            BiographyArchive::new(parsed.root_schema(), parsed.records().to_vec()).unwrap();
        let reopened = BiographyArchive::from_bytes(&rebuilt.to_bytes().unwrap()).unwrap();
        assert_eq!(reopened.records(), parsed.records(), "mutation {index}");
        assert_eq!(reopened.root_schema(), parsed.root_schema());
    }
}

#[test]
fn wrong_root_class_and_wrong_map_tag_are_rejected() {
    let mut bytes = MIXED.to_vec();
    bytes[6] = b'x';
    assert!(
        BiographyArchive::from_bytes(&bytes)
            .unwrap_err()
            .cause
            .contains("moBiography_c")
    );
    for tag in [1u16, 0x8001, 0x7fff] {
        let mut bytes = MIXED.to_vec();
        bytes[..2].copy_from_slice(&tag.to_le_bytes());
        assert!(BiographyArchive::from_bytes(&bytes).is_err());
    }
}
