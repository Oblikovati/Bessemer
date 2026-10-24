//! Independent semantic oracles, exact reconstruction, edits, and abbreviation contracts.
#[path = "support/biography.rs"]
mod biography_support;

use bessemer::biography::*;
use biography_support::{CASES, projection};

fn mixed() -> BiographyArchive {
    BiographyArchive::from_bytes(CASES[3].1).unwrap()
}
fn text(value: &str) -> ArchiveString {
    ArchiveString::new(value, TextEncoding::Utf16).unwrap()
}

#[test]
fn every_raw_and_expanded_field_matches_the_independent_reference() {
    for (name, bytes) in CASES {
        let archive = BiographyArchive::from_bytes(bytes).unwrap();
        let expected = include_str!("fixtures/biography/expected.tsv")
            .lines()
            .find(|line| line.split('\t').next() == Some(name))
            .unwrap();
        let expected: Vec<_> = expected.split('\t').collect();
        assert_eq!(projection(archive.records()), expected[1], "raw {name}");
        assert_eq!(
            projection(&archive.expanded_records()),
            expected[2],
            "expanded {name}"
        );
        assert_eq!(archive.to_bytes().unwrap(), *bytes);
        let rebuilt =
            BiographyArchive::new(archive.root_schema(), archive.records().to_vec()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), *bytes, "rebuilt {name}");
    }
}

#[test]
fn native_baseline_and_extreme_values_are_observations_not_constraints() {
    let native = BiographyArchive::from_bytes(CASES[0].1).unwrap();
    assert_eq!(native.records().len(), 2);
    let first = &native.records()[0];
    assert_eq!((first.file_version, first.date_code), (19000, 2026100));
    assert_eq!(
        (first.processor_architecture, first.processor_count),
        (9, 16)
    );
    assert_eq!(
        (
            first.os_platform_id,
            first.os_major,
            first.os_minor,
            first.os_build
        ),
        (2, 10, 0, 26200)
    );
    assert_eq!(native.root_schema(), 1);
    let mixed = mixed();
    assert_eq!(mixed.root_schema(), 17);
    assert_eq!(mixed.records()[0].file_version, u32::MAX);
    assert_eq!(mixed.records()[0].date_code, i32::MIN);
    assert_eq!(
        mixed.records()[0].snapshots[0].paths[0].available_disk_bytes,
        u64::MAX
    );
    assert_eq!(mixed.records()[0].processor_count, 0);
}

#[test]
fn expansion_chains_follow_the_same_phase_and_group_without_inheriting_numbers() {
    let source = mixed();
    let resolved = source.expanded_records();
    assert_eq!(source.records()[1].os_csd_version.text(), "*");
    assert_eq!(resolved[2].os_csd_version.text(), "SPé");
    assert_eq!(
        resolved[2].snapshots[0].paths[0].path.text(),
        "C:\\phase-0\\group-0\\"
    );
    assert_eq!(
        resolved[2].snapshots[1].paths[0].path.text(),
        "C:\\phase-1\\group-0\\"
    );
    assert_eq!(resolved[2].snapshots[1].paths[1].path.text(), "");
    for (raw, expanded) in source.records().iter().zip(&resolved) {
        for (raw, expanded) in raw.snapshots.iter().zip(&expanded.snapshots) {
            assert_eq!(raw.filetime, expanded.filetime);
            assert_eq!(
                raw.available_page_file_low32,
                expanded.available_page_file_low32
            );
            for (raw, expanded) in raw.paths.iter().zip(&expanded.paths) {
                assert_eq!(
                    (raw.drive_type, raw.available_disk_bytes),
                    (expanded.drive_type, expanded.available_disk_bytes)
                );
            }
        }
    }
}

#[test]
fn abbreviation_is_explicit_nonempty_and_semantically_idempotent() {
    let source = mixed();
    let mut records = source.expanded_records();
    records[0].os_csd_version = text("*");
    records[1].os_csd_version = text("*");
    records[2].os_csd_version = text("");
    records[1].snapshots[1].paths[1].path = text("");
    let archive = source.with_records(records).unwrap();
    let before = archive.to_bytes().unwrap();
    let abbreviated = archive
        .with_records(archive.abbreviated_records().unwrap())
        .unwrap();
    assert_eq!(abbreviated.records()[0].os_csd_version.text(), "*");
    assert_eq!(abbreviated.records()[1].os_csd_version.text(), "*");
    assert_eq!(abbreviated.records()[2].os_csd_version.text(), "");
    assert_eq!(
        abbreviated.records()[1].snapshots[1].paths[1].path.text(),
        ""
    );
    assert_eq!(
        projection(&abbreviated.expanded_records()),
        projection(&archive.expanded_records())
    );
    assert_eq!(
        abbreviated.abbreviated_records().unwrap(),
        abbreviated.records()
    );
    assert_eq!(archive.to_bytes().unwrap(), before);
}

#[test]
fn edits_reverts_and_source_mutation_keep_published_archives_isolated() {
    let mut bytes = CASES[3].1.to_vec();
    let source = BiographyArchive::from_bytes(&bytes).unwrap();
    bytes.fill(0);
    let mut records = source.records().to_vec();
    records[0].date_code = -7;
    records[0].snapshots[1].paths[5].path = text("Unicode λ😀\0");
    let edited = source.with_records(records.clone()).unwrap();
    records[0].date_code = 0;
    let reopened = BiographyArchive::from_bytes(&edited.to_bytes().unwrap()).unwrap();
    assert_eq!(reopened.records(), edited.records());
    assert_eq!(edited.records()[0].date_code, -7);
    assert_eq!(source.to_bytes().unwrap(), CASES[3].1);
    assert_eq!(
        edited
            .with_records(source.records().to_vec())
            .unwrap()
            .to_bytes()
            .unwrap(),
        CASES[3].1
    );
    assert_eq!(
        source
            .with_records(source.records().to_vec())
            .unwrap()
            .to_bytes()
            .unwrap(),
        CASES[3].1
    );
}

#[test]
fn construction_reordering_deletion_and_empty_results_roundtrip() {
    let source = mixed();
    let mut records = source.expanded_records();
    records.reverse();
    records.remove(1);
    records.push(records[0].clone());
    let archive = BiographyArchive::new(65535, records.clone()).unwrap();
    let reopened = BiographyArchive::from_bytes(&archive.to_bytes().unwrap()).unwrap();
    assert_eq!(reopened.records(), records);
    assert_eq!(reopened.root_schema(), 65535);
    let empty = archive.with_records(Vec::new()).unwrap();
    assert!(
        BiographyArchive::from_bytes(&empty.to_bytes().unwrap())
            .unwrap()
            .records()
            .is_empty()
    );
    assert!(empty.expanded_records().is_empty());
    assert!(empty.abbreviated_records().unwrap().is_empty());
}

#[test]
fn public_types_are_send_sync_and_concurrent_edits_match_serial_output() {
    fn assert_thread_safe<T: Send + Sync>() {}
    assert_thread_safe::<BiographyArchive>();
    assert_thread_safe::<BiographyRecord>();
    assert_thread_safe::<EnvironmentSnapshot>();
    assert_thread_safe::<EnvironmentPath>();
    let source = mixed();
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let source = source.clone();
            std::thread::spawn(move || {
                for _ in 0..30 {
                    let records = source.abbreviated_records().unwrap();
                    let edited = source.with_records(records.clone()).unwrap();
                    let rebuilt = BiographyArchive::new(source.root_schema(), records).unwrap();
                    assert_eq!(edited.to_bytes().unwrap(), rebuilt.to_bytes().unwrap());
                    assert_eq!(source.to_bytes().unwrap(), CASES[3].1);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
fn whole_document_edits_preserve_every_other_record_field_and_stream() {
    for bytes in [
        include_bytes!("fixtures/blank-part.SLDPRT").as_slice(),
        include_bytes!("fixtures/blank-assembly.SLDASM"),
        include_bytes!("fixtures/blank-drawing.SLDDRW"),
    ] {
        let original = bessemer::Document::from_bytes(bytes).unwrap();
        let mut document = original.clone();
        let stream = "_MO_VERSION_19000/Biography";
        let archive = BiographyArchive::from_bytes(&document.read_stream(stream).unwrap()).unwrap();
        let mut records = archive.records().to_vec();
        records[0].date_code = -123;
        let edited = archive.with_records(records).unwrap();
        document
            .replace_stream(stream, edited.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(
            BiographyArchive::from_bytes(&reopened.read_stream(stream).unwrap())
                .unwrap()
                .records(),
            edited.records()
        );
        for other in original.streams().filter(|entry| entry.name != stream) {
            assert_eq!(
                original.read_stream(other.name).unwrap(),
                reopened.read_stream(other.name).unwrap()
            );
        }
        assert_eq!(original.to_bytes().unwrap(), bytes);
    }
}
