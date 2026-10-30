use super::projections::{self, span, word};
use bessemer::simulation::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "../common/simulation_images.rs"]
mod packet;
use packet::Cursor;

struct PersistCase<'a> {
    kind: u32,
    numeric: u32,
    full: u32,
    offset: u32,
    raw: &'a [u8],
    extra: &'a [u8],
}

fn owned() -> Vec<u8> {
    let mut bytes = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/cosworks-persistence/owned.bin").as_slice(),
    )
    .read_to_end(&mut bytes)
    .unwrap();
    bytes
}

#[test]
fn complete_owned_projections_match_actual_frozen_reference_and_literal_goldens() {
    let expected = owned();
    assert_eq!(project_packet(&expected), expected);
}

#[test]
#[ignore = "requires injected hash-bound native input and complete projection output paths"]
fn export_all_owned_and_native_persistence_projections() {
    let input = std::env::var_os("BESSEMER_PERSISTENCE_NATIVE_INPUT")
        .expect("expected explicit native input path");
    let output = std::env::var_os("BESSEMER_PERSISTENCE_PROJECTION_OUTPUT")
        .expect("expected explicit projection output path");
    let native = std::fs::read(input).unwrap();
    let mut bytes = Vec::new();
    span(&mut bytes, &project_packet(&owned()));
    span(&mut bytes, &project_packet(&native));
    std::fs::write(output, bytes).unwrap();
}

fn project_packet(expected: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.take(4), b"CPST");
    let count = cursor.word();
    let mut output = b"CPST".to_vec();
    word(&mut output, count);
    for _ in 0..count {
        let case = read_case(&mut cursor);
        let expected_result = cursor.span();
        let actual = project_case(&case);
        assert_eq!(
            actual, expected_result,
            "kind {} numeric {} mode {} ordinal {}",
            case.kind, case.numeric, case.full, case.offset
        );
        append_case(&mut output, &case, &actual);
    }
    assert!(cursor.0.is_empty());
    output
}

fn read_case<'a>(cursor: &mut Cursor<'a>) -> PersistCase<'a> {
    PersistCase {
        kind: cursor.word(),
        numeric: cursor.word(),
        full: cursor.word(),
        offset: cursor.word(),
        raw: cursor.span(),
        extra: cursor.span(),
    }
}

fn append_case(output: &mut Vec<u8>, case: &PersistCase<'_>, result: &[u8]) {
    for value in [case.kind, case.numeric, case.full, case.offset] {
        word(output, value);
    }
    span(output, case.raw);
    span(output, case.extra);
    span(output, result);
}

fn project_case(case: &PersistCase<'_>) -> Vec<u8> {
    match case.kind {
        0 => projections::text(
            &SimulationPersistText::read_from(case.raw, case.offset as usize, 0x100000).unwrap(),
        ),
        1 => project_study(case.raw),
        2 => project_primary(case),
        3 => project_legacy(case.raw),
        4 => project_selection(case),
        5 => project_edit(case),
        other => panic!("kind {other}; expected 0..=5"),
    }
}

fn context(case: &PersistCase<'_>) -> (SimulationPersistVersion, SimulationPersistReadMode) {
    let numeric = SimulationPersistVersion::new(case.numeric.try_into().unwrap()).unwrap();
    let mode = match case.full {
        0 => SimulationPersistReadMode::Partial,
        1 => SimulationPersistReadMode::Full,
        other => panic!("mode {other}; expected 0 or 1"),
    };
    (numeric, mode)
}

fn project_primary(case: &PersistCase<'_>) -> Vec<u8> {
    let (numeric, mode) = context(case);
    let record = SimulationPrimaryVersion::from_bytes(case.raw, numeric, mode, 0x100000).unwrap();
    let rebuilt = SimulationPrimaryVersion::from_parts(
        record.text(),
        numeric,
        mode,
        record.words(),
        record.remainder(),
        0x100000,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), case.raw);
    projections::primary(&record)
}

fn project_study(raw: &[u8]) -> Vec<u8> {
    let record = SimulationStudyOrder::from_bytes(raw, 10000, 0x100000).unwrap();
    let rebuilt =
        SimulationStudyOrder::from_parts(record.leading(), record.items(), 10000, 0x100000)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), raw);
    projections::study(&record)
}

fn project_legacy(raw: &[u8]) -> Vec<u8> {
    let record = SimulationLegacyVersion::from_bytes(raw).unwrap();
    let rebuilt =
        SimulationLegacyVersion::from_parts(record.numeric_version(), record.opaque_suffix())
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), raw);
    projections::legacy(&record)
}

fn project_selection(case: &PersistCase<'_>) -> Vec<u8> {
    let (numeric, mode) = context(case);
    let selected = SimulationSelectedVersion::select(
        Some(case.raw),
        Some(case.extra),
        Some(numeric),
        mode,
        0x100000,
    )
    .unwrap();
    let SimulationSelectedVersion::Primary(record) = selected else {
        panic!("expected primary precedence");
    };
    let mut output = 1u32.to_le_bytes().to_vec();
    output.extend_from_slice(&projections::primary(&record));
    output
}

fn project_edit(case: &PersistCase<'_>) -> Vec<u8> {
    let source = SimulationStudyOrder::from_bytes(case.raw, 10000, 0x100000).unwrap();
    let replacement = SimulationPersistText::read_from(case.extra, 0, 0x100000).unwrap();
    assert_eq!(replacement.source_offsets().2, case.extra.len());
    let id = if case.offset == 0 {
        source.leading_id()
    } else {
        source.item_id(case.offset as usize - 1).unwrap()
    };
    let edit = source.edit_text(&id, &replacement).unwrap();
    assert_eq!(edit.source.to_bytes(), case.raw);
    assert_eq!(edit.replacement.to_bytes(), case.extra);
    assert_eq!(source.to_bytes(), case.raw);
    project_edit_result(&edit)
}

fn project_edit_result(edit: &SimulationStudyTextEdit) -> Vec<u8> {
    let mut output = Vec::new();
    span(&mut output, &edit.archive.to_bytes());
    span(&mut output, &projections::study(&edit.archive));
    let changed = edit.completion == SimulationStudyEditCompletion::Changed;
    assert_eq!(changed, edit.source.revision() != edit.archive.revision());
    word(&mut output, u32::from(changed));
    word(&mut output, edit.history.len() as u32);
    for (index, entry) in edit.history.iter().enumerate() {
        append_history(&mut output, edit, index, entry);
    }
    assert_eq!(
        edit.coverage.check_mode,
        SimulationPersistCheckMode::ExactStudyOrderAndPreservation
    );
    projections::coverage(&mut output, edit.coverage);
    output
}

fn append_history(
    output: &mut Vec<u8>,
    edit: &SimulationStudyTextEdit,
    index: usize,
    entry: &SimulationStudyTextHistory,
) {
    word(output, index as u32);
    word(
        output,
        u32::from(entry.relation == SimulationStudyTextRelation::Derived),
    );
    span(output, &edit.source.text(&entry.parent).unwrap().to_bytes());
    span(
        output,
        &edit.archive.text(&entry.result).unwrap().to_bytes(),
    );
}
