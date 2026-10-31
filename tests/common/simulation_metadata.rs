use super::packet::Cursor;
use bessemer::simulation::*;
use std::io::Read;

pub const LIMITS: SimulationMetadataLimits = SimulationMetadataLimits {
    carrier: bessemer::compound::CompoundLimits {
        maximum_file_bytes: 100_000_000,
        maximum_stream_bytes: 100_000_000,
        maximum_total_stream_bytes: 100_000_000,
        maximum_directory_entries: 100_000,
        maximum_storage_depth: 1000,
    },
    maximum_guid_stream_bytes: 6_000_000,
};

pub fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/simulation-metadata/owned.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

pub fn kind(tag: u32) -> SimulationScalarKind {
    match tag {
        0 => SimulationScalarKind::TimeStamp,
        1 => SimulationScalarKind::RunState,
        _ => panic!("unknown scalar kind {tag}"),
    }
}

pub fn scalar_oracle(archive: &SimulationScalarArchive, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    assert_eq!(archive.kind().stream_name().as_bytes(), cursor.span());
    assert_eq!(archive.pattern(), cursor.word());
    assert_eq!(archive.signed_value() as u32, cursor.word());
    assert_eq!(
        archive.meaning() == SimulationScalarMeaning::ModelDocumentUpdateStamp,
        cursor.word() != 0
    );
    assert_eq!(
        archive.coverage().semantic_binding_complete,
        cursor.word() != 0
    );
    assert_eq!(archive.coverage().transfer_prefix, cursor.word() != 0);
    assert_eq!(archive.to_bytes(), cursor.span());
    assert!(cursor.0.is_empty());
    assert_eq!(
        SimulationScalarKind::from_stream_name(archive.kind().stream_name()).unwrap(),
        archive.kind()
    );
}

pub fn scalar_cases(cursor: &mut Cursor<'_>) {
    assert_eq!(cursor.word(), 14);
    for _ in 0..14 {
        let kind = kind(cursor.word());
        let pattern = cursor.word();
        let raw = cursor.span();
        let archive = SimulationScalarArchive::from_bytes(kind, raw).unwrap();
        scalar_oracle(&archive, cursor.span());
        assert_eq!(
            archive.to_bytes(),
            SimulationScalarArchive::from_pattern(kind, pattern).to_bytes()
        );
        assert_eq!(
            archive.to_bytes(),
            SimulationScalarArchive::from_signed(kind, pattern as i32).to_bytes()
        );
        check_scalar_edit(&archive);
    }
}

fn check_scalar_edit(archive: &SimulationScalarArchive) {
    let edit = archive.edit_pattern(0x87654321);
    assert_eq!(edit.archive.pattern(), 0x87654321);
    assert_eq!(&edit.parent, archive.id());
    assert_eq!(&edit.result, edit.archive.id());
    assert_ne!(edit.parent, edit.result);
    assert_eq!(edit.coverage, archive.coverage());
    assert_eq!(edit.diagnostics, archive.diagnostics());
    assert_eq!(edit.pattern_error_bits, 0);
    assert_eq!(
        archive.edit_pattern(archive.pattern()).archive.id(),
        archive.id()
    );
    assert_eq!(
        edit.archive
            .edit_pattern(archive.pattern())
            .archive
            .to_bytes(),
        archive.to_bytes()
    );
}

pub fn helper_cases(cursor: &mut Cursor<'_>) {
    assert_eq!(cursor.word(), 7);
    for _ in 0..7 {
        let projection = SimulationScalarHelperProjection::from_update_stamp(cursor.word() as i32);
        assert_eq!(projection.run_state.to_bytes(), cursor.span());
        assert_eq!(projection.time_stamp.to_bytes(), cursor.span());
        assert!(!projection.native_io_executed);
    }
}

pub fn assert_carrier_expected(actual: &bessemer::compound::CompoundFile, expected: &[u8]) {
    let expected = bessemer::compound::CompoundFile::from_bytes(expected, LIMITS.carrier).unwrap();
    for entry in actual.entries() {
        let other = expected
            .entries()
            .iter()
            .find(|other| entry.id().slot() == other.id().slot())
            .unwrap();
        assert_eq!(entry.name_units(), other.name_units());
        if entry.kind() == bessemer::compound::CompoundEntryKind::Stream {
            assert_eq!(
                actual.stream(entry.id()).unwrap(),
                expected.stream(other.id()).unwrap()
            );
        }
    }
}
