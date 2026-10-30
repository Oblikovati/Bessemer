//! Exact native last-dot code-unit projection, without filesystem/COM acceptance claims.
use bessemer::simulation::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;

fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("fixtures/simulation-storage-locator/locators.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

fn read_units(cursor: &mut Cursor<'_>) -> Vec<u16> {
    let bytes = cursor.span();
    assert_eq!(bytes.len() % 2, 0);
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect()
}

#[test]
fn every_literal_parent_and_exact_component_matches_actual_unchanged_reader() {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SLOC");
    assert_eq!(cursor.word(), 256);
    for _ in 0..256 {
        assert_locator(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn assert_locator(cursor: &mut Cursor<'_>) {
    let original = read_units(cursor);
    let expected_storage = read_units(cursor);
    let expected_stream = read_units(cursor);
    let name = SimulationCwrLogicalName::from_units(&original).unwrap();
    let result = name.project_locator().unwrap();
    assert_eq!(result.logical_name.code_units(), original);
    assert_eq!(result.storage_name.as_ref(), expected_storage);
    assert_eq!(result.stream_name.as_ref(), expected_stream);
    assert_eq!(result.storage_mode, cursor.word());
    assert_eq!(result.storage_format, cursor.word());
    assert_eq!(result.stream_mode, cursor.word());
    assert_eq!(u32::from(result.payload_decoded), cursor.word());
    assert_contract(&result);
    assert_eq!(name.code_units(), original);
}

fn assert_contract(result: &SimulationStorageLocator) {
    assert_eq!(
        result.completion,
        SimulationStorageLookupCompletion::CompletedLexicalProjection
    );
    assert_eq!(
        result.check_mode,
        SimulationStorageLookupCheckMode::ExactLastDotUtf16LexicalProjection
    );
    assert_eq!(
        result.checked_input_units,
        result.logical_name.code_units().len()
    );
    assert_eq!(result.requested_pattern_error_bits, 0);
    assert!(!result.native_io_executed);
    assert!(!result.native_name_acceptance_established);
    assert_parent_ranges(result);
}

#[test]
fn absent_ascii_delimiters_and_embedded_nuls_remain_outside_the_native_projection_profile() {
    for input in EXCLUDED_NAMES {
        let units: Vec<_> = input.encode_utf16().collect();
        let error = SimulationCwrLogicalName::from_units(&units).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains("expected"));
    }
    assert!(SimulationCwrLogicalName::from_units(&[0xd800]).is_err());
}

#[test]
fn empty_suffix_directory_dots_and_unpaired_units_are_literal_without_path_cleanup() {
    for (input, storage, stream) in LITERAL_CASES {
        let result = SimulationCwrLogicalName::from_units(input)
            .unwrap()
            .project_locator()
            .unwrap();
        assert_eq!(result.storage_name.as_ref(), storage);
        assert_eq!(result.stream_name.as_ref(), stream);
        assert_contract(&result);
    }
}

#[test]
fn owned_locator_parents_survive_input_drop_caller_mutation_and_returned_arc_mutation() {
    let result = retained_locator();
    let original = result.logical_name.code_units().to_vec();
    let storage = result.storage_name.to_vec();
    let mut returned = result.storage_name.clone();
    std::sync::Arc::make_mut(&mut returned).fill(0);
    assert_eq!(result.storage_name.as_ref(), storage);
    assert_eq!(result.logical_name.code_units(), original);
    assert_contract(&result);
}

fn retained_locator() -> SimulationStorageLocator {
    let mut input: Vec<_> = "C:\\dir.with.dot\\部品😀.gen ".encode_utf16().collect();
    let name = SimulationCwrLogicalName::from_units(&input).unwrap();
    let result = name.project_locator().unwrap();
    input.fill(0);
    assert_ne!(result.logical_name.code_units(), input);
    result
}

#[test]
fn logical_lookup_types_are_send_sync_and_concurrent_complete_components_equal_serial_units() {
    fn check<T: Send + Sync>() {}
    check::<SimulationCwrLogicalName>();
    check::<SimulationStorageLocator>();
    check::<SimulationStorageLookupCompletion>();
    check::<SimulationStorageLookupCheckMode>();
    let name = retained_locator().logical_name;
    let serial = name.project_locator().unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (name, serial) = (&name, &serial);
            scope.spawn(move || repeat_lookup(name, serial));
        }
    });
}

fn repeat_lookup(name: &SimulationCwrLogicalName, serial: &SimulationStorageLocator) {
    for _ in 0..32 {
        let result = name.project_locator().unwrap();
        assert_eq!(result.storage_name, serial.storage_name);
        assert_eq!(result.stream_name, serial.stream_name);
        assert_eq!(
            result.logical_name.code_units(),
            serial.logical_name.code_units()
        );
        assert_contract(&result);
    }
}

#[test]
#[ignore = "requires explicitly injected BESSEMER_STORAGE_LOCATOR_PROJECTION_OUTPUT path"]
fn complete_owned_locator_export_matches_independent_reference_packets() {
    let path = std::env::var_os("BESSEMER_STORAGE_LOCATOR_PROJECTION_OUTPUT").map(std::path::PathBuf::from)
        .expect("BESSEMER_STORAGE_LOCATOR_PROJECTION_OUTPUT must name the explicitly injected export path");
    let raw = owned();
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    let count = cursor.word();
    let mut output = b"SLOV".to_vec();
    output.extend_from_slice(&count.to_le_bytes());
    for _ in 0..count {
        export_locator(&mut cursor, &mut output);
    }
    assert!(cursor.0.is_empty());
    std::fs::write(&path, &output).unwrap();
    assert_eq!(std::fs::read(path).unwrap(), output);
}

fn export_locator(cursor: &mut Cursor<'_>, output: &mut Vec<u8>) {
    let input = read_units(cursor);
    cursor.span();
    cursor.span();
    cursor.take(16);
    let result = SimulationCwrLogicalName::from_units(&input)
        .unwrap()
        .project_locator()
        .unwrap();
    append_locator_bytes(&result, output);
}

fn assert_parent_ranges(result: &SimulationStorageLocator) {
    let original = result.logical_name.code_units();
    let prefix = result.storage_prefix_units;
    assert_eq!(original[prefix], 0x2e);
    assert_eq!(result.stream_parent_offset, prefix + 1);
    assert_eq!(&result.storage_name[..prefix], &original[..prefix]);
    assert_eq!(&result.storage_name[prefix..], &[46, 67, 87, 82]);
    assert_eq!(
        result.stream_name.as_ref(),
        &original[result.stream_parent_offset..]
    );
}

const EXCLUDED_NAMES: &[&str] = &[
    "",
    "study",
    "a\\file",
    "a/file",
    "😀",
    "fullwidth．dot",
    "\0.GEN",
    "study\0.GEN",
    "study.GE\0N",
    "study.GEN\0",
    "\0",
    ".\0",
];

type LocatorUnitsCase = (&'static [u16], &'static [u16], &'static [u16]);
const LITERAL_CASES: [LocatorUnitsCase; 3] = [
    (&[46], &[46, 67, 87, 82], &[]),
    (
        &[97, 46, 98, 92, 102],
        &[97, 46, 67, 87, 82],
        &[98, 92, 102],
    ),
    (&[0xd800, 46, 0xdc00], &[0xd800, 46, 67, 87, 82], &[0xdc00]),
];

fn append_locator_bytes(result: &SimulationStorageLocator, output: &mut Vec<u8>) {
    for units in [
        result.logical_name.code_units(),
        &result.storage_name,
        &result.stream_name,
    ] {
        let bytes: Vec<_> = units.iter().flat_map(|unit| unit.to_le_bytes()).collect();
        output.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        output.extend(bytes);
    }
    for word in [
        result.storage_mode,
        result.storage_format,
        result.stream_mode,
        u32::from(result.payload_decoded),
    ] {
        output.extend_from_slice(&word.to_le_bytes());
    }
}
