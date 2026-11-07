//! Complete native ELF initializer reconstruction and isolated complete-carrier edits.
// The existing cursor is shared with other frozen-reference targets.
#[path = "common/elf_index_failures.rs"]
mod index_failures;
#[path = "common/elf_initial_failures.rs"]
mod initial_failures;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod pair_reference;
#[path = "common/elf_reference.rs"]
mod reference;
use bessemer::simulation::*;
use pair_reference::Cursor;
use std::io::Read;

fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(include_bytes!("fixtures/simulation-elf/owned.bin").as_slice())
        .read_to_end(&mut raw)
        .unwrap();
    raw
}

#[test]
fn complete_owned_initializers_match_all_frozen_constructor_fields_and_zero_bytes() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"ELFO");
    assert_eq!(cursor.word(), 4);
    for _ in 0..4 {
        let bytes = cursor.span();
        let archive = check_initial(bytes, cursor.span());
        check_parameter_edits(&archive);
    }
    assert!(!cursor.0.is_empty()); // The remaining packet holds separate selected-directory/store projections.
}

#[test]
fn selected_owned_directory_mapping_header_and_scalar_projections_match_every_reference_field() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"ELFO");
    for _ in 0..cursor.word() {
        cursor.span();
        cursor.span();
    }
    reference::remaining_owned(&mut cursor);
    assert!(cursor.0.is_empty());
}

fn check_initial(bytes: &[u8], expected: &[u8]) -> SimulationElfInitialArchive {
    let archive = SimulationElfInitialArchive::from_bytes(bytes).unwrap();
    let mut cursor = Cursor(expected);
    for word in archive.header_words() {
        assert_eq!(word, cursor.word());
    }
    let parameters = archive.parameters();
    for word in [
        244,
        61,
        60,
        6,
        5,
        100,
        parameters.parameter_a,
        parameters.parameter_b,
        6,
        100,
        0,
        parameters.parameter_b,
        1220,
        1224,
    ] {
        assert_eq!(word, cursor.word());
    }
    assert_eq!(&archive.to_bytes()[32..], cursor.span());
    assert!(cursor.0.is_empty());
    assert_eq!(archive.to_bytes(), bytes);
    assert_eq!(
        SimulationElfInitialArchive::from_parameters(parameters).to_bytes(),
        bytes
    );
    assert_eq!(archive.coverage().empty_directory_pairs, 100);
    archive
}

fn check_parameter_edits(archive: &SimulationElfInitialArchive) {
    for (parameter, offset) in [
        (SimulationElfParameter::A, 16),
        (SimulationElfParameter::B, 24),
    ] {
        let id = archive.parameter_id(parameter);
        let mut expected = archive.to_bytes();
        expected[offset..offset + 4].copy_from_slice(&0x12345678_u32.to_le_bytes());
        let edit = archive.edit_parameter(&id, 0x12345678).unwrap();
        assert_eq!(edit.archive.to_bytes(), expected);
        assert_eq!(edit.archive.parameter(&edit.result).unwrap(), 0x12345678);
        assert!(edit.archive.parameter(&id).is_err());
        assert_eq!(edit.parent, id);
        assert_eq!(edit.coverage, archive.coverage());
        assert_eq!(edit.parameter_pattern_error_bits, 0);
        assert_eq!(edit.diagnostics, archive.diagnostics());
        let reverted = edit
            .archive
            .edit_parameter(&edit.result, archive.parameter(&id).unwrap())
            .unwrap();
        assert_eq!(reverted.archive.to_bytes(), archive.to_bytes());
        let unchanged = archive
            .edit_parameter(&id, archive.parameter(&id).unwrap())
            .unwrap();
        assert_eq!(unchanged.archive.revision(), archive.revision());
    }
}

const LIMITS: bessemer::compound::CompoundLimits = bessemer::compound::CompoundLimits {
    maximum_file_bytes: 100_000_000,
    maximum_stream_bytes: 100_000_000,
    maximum_total_stream_bytes: 100_000_000,
    maximum_directory_entries: 100_000,
    maximum_storage_depth: 1000,
};

#[test]
#[ignore = "requires separately frozen installed ELF corpus in target/simulation-elf-native.bin"]
fn complete_native_initial_edit_reopens_in_a_carrier_and_preserves_unrelated_streams() {
    let raw = std::fs::read("target/simulation-elf-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"ELFN");
    assert_eq!(cursor.word(), 1);
    let carrier = cursor.span();
    let original = cursor.span();
    check_initial(original, cursor.span());
    let context = SimulationElfIndexContext::from_bytes(original, reference::INDEX_LIMITS).unwrap();
    let selector = SimulationElfSelector::new(1).unwrap();
    reference::check_directory(&context.directory(selector).unwrap(), cursor.span());
    assert_eq!(
        context
            .project_index_update(selector)
            .unwrap()
            .header_body_bytes(),
        cursor.span()
    );
    let golden = cursor.span();
    assert!(cursor.0.is_empty());
    check_carrier_edit(carrier, golden);
}

fn check_carrier_edit(carrier: &[u8], golden: &[u8]) {
    let study = SimulationElfInitialStudy::from_bytes(carrier, LIMITS).unwrap();
    let id = study.initial().parameter_id(SimulationElfParameter::A);
    let edit = study.edit_parameter(&id, 0x12345678).unwrap();
    assert_eq!(edit.study.initial().to_bytes(), golden);
    for entry in study.paired().carrier().entries() {
        if entry.kind() != bessemer::compound::CompoundEntryKind::Stream {
            continue;
        }
        let history = edit
            .carrier_edit
            .history
            .entries
            .iter()
            .find(|history| history.parent == *entry.id())
            .unwrap();
        let actual = edit
            .study
            .paired()
            .carrier()
            .stream(&history.result)
            .unwrap();
        let expected = if entry.name_units() == [69, 76, 70] {
            golden
        } else {
            study.paired().carrier().stream(entry.id()).unwrap()
        };
        assert_eq!(actual, expected);
    }
    let encoded = edit.study.paired().carrier().to_bytes();
    let reopened = SimulationElfInitialStudy::from_bytes(&encoded, LIMITS).unwrap();
    assert_eq!(reopened.initial().to_bytes(), golden);
    assert_eq!(
        edit.study.paired().image().revision(),
        study.paired().image().revision()
    );
    assert_eq!(study.paired().carrier().to_bytes(), carrier);
    std::fs::write("target/simulation-elf-native-initial-edit.cwr", encoded).unwrap();
}
