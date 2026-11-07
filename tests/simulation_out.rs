//! Complete byte-preserving OUT report syntax and ordered numeric occurrences.
// Reuse the existing bounded packet cursor; paired-image oracle functions are exercised in their own target.
#[path = "common/out_failures.rs"]
mod failures;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod pair_reference;
#[path = "common/out_reference.rs"]
mod reference;
#[path = "common/out_totals.rs"]
mod totals;
use bessemer::simulation::*;
use pair_reference::Cursor;
use std::io::Read;

#[test]
fn all_owned_report_lines_fields_legends_and_exact_decimal_patterns_match_reference() {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(include_bytes!("fixtures/simulation-out/owned.bin").as_slice())
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"OUTO");
    assert_eq!(cursor.word(), 10);
    for _ in 0..10 {
        let archive = reference::check(cursor.span(), cursor.span());
        check_golden(&archive, cursor.span());
        check_all_goldens(&archive, &mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn check_golden(archive: &bessemer::simulation::SimulationOutArchive, golden: &[u8]) {
    let Some(field) = archive.fields().first() else {
        assert_eq!(archive.to_bytes(), golden);
        return;
    };
    let before = archive.to_bytes();
    let edit = archive.edit_numeric_lexeme(&field.id, "-2.500D+1").unwrap();
    assert_eq!(edit.archive.to_bytes(), golden);
    assert_eq!(edit.history.len(), archive.fields().len());
    assert_eq!(edit.parent_revision, *archive.revision());
    assert_eq!(edit.coverage, edit.archive.coverage());
    assert_eq!(edit.requested_token_error_bytes, 0);
    assert_eq!(edit.diagnostics, archive.diagnostics());
    assert!(edit.archive.field(&field.id).is_err());
    let derived = edit.archive.field(&edit.history[0].result).unwrap();
    let reverted = edit
        .archive
        .edit_numeric_lexeme(&derived.id, &field.numeric_lexeme)
        .unwrap();
    assert_eq!(reverted.archive.to_bytes(), before);
    assert_eq!(archive.to_bytes(), before);
    let unchanged = archive
        .edit_numeric_lexeme(&field.id, &field.numeric_lexeme)
        .unwrap();
    assert_eq!(unchanged.archive.revision(), archive.revision());
}

#[test]
#[ignore = "requires separately frozen installed OUT report in target/simulation-out-native.bin"]
fn installed_out_report_matches_all_lines_fields_and_legends() {
    let raw = std::fs::read("target/simulation-out-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"OUTN");
    assert_eq!(cursor.word(), 1);
    let carrier = cursor.span();
    let stream = cursor.span();
    let mas = cursor.span();
    let gen_bytes = cursor.span();
    let archive = reference::check(stream, cursor.span());
    assert_eq!(archive.lines().len(), 91);
    assert_eq!(archive.fields().len(), 18);
    assert_eq!(
        archive
            .fields()
            .iter()
            .map(|field| field.legends.len())
            .sum::<usize>(),
        31
    );
    let golden = cursor.span();
    check_golden(&archive, golden);
    let master = SimulationMasterArchive::from_bytes(mas).unwrap();
    let image = SimulationGenArchive::from_bytes(gen_bytes, &master).unwrap();
    assert_eq!(
        archive.paired_totals(&image).nodes.status,
        SimulationOutTotalStatus::Matches
    );
    assert_eq!(
        archive.paired_totals(&image).elements.status,
        SimulationOutTotalStatus::Matches
    );
    check_study(carrier, golden);
    check_all_goldens(&archive, &mut cursor);
    assert!(cursor.0.is_empty());
}

fn check_all_goldens(archive: &SimulationOutArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(archive.fields().len(), cursor.word() as usize);
    for (index, field) in archive.fields().iter().enumerate() {
        let edit = archive.edit_numeric_lexeme(&field.id, "-2.500D+1").unwrap();
        assert_eq!(edit.archive.to_bytes(), cursor.span());
        for (derived, history) in edit.history.iter().enumerate() {
            assert_eq!(history.replaced_token, index == derived);
        }
        let target = &edit.archive.field(&edit.history[index].result).unwrap().id;
        let reverted = edit
            .archive
            .edit_numeric_lexeme(target, &field.numeric_lexeme)
            .unwrap();
        assert_eq!(reverted.archive.to_bytes(), archive.to_bytes());
    }
}

const COMPOUND_LIMITS: bessemer::compound::CompoundLimits = bessemer::compound::CompoundLimits {
    maximum_file_bytes: 100_000_000,
    maximum_stream_bytes: 100_000_000,
    maximum_total_stream_bytes: 100_000_000,
    maximum_directory_entries: 100_000,
    maximum_storage_depth: 1000,
};

fn check_study(carrier: &[u8], golden: &[u8]) {
    let study =
        SimulationOutStudy::from_bytes(carrier, COMPOUND_LIMITS, reference::LIMITS).unwrap();
    assert_eq!(study.paired().carrier().to_bytes(), carrier);
    let edit = study
        .edit_numeric_lexeme(&study.report().fields()[0].id, "-2.500D+1")
        .unwrap();
    assert_eq!(edit.study.report().to_bytes(), golden);
    assert_eq!(
        edit.study.paired().image().revision(),
        study.paired().image().revision()
    );
    assert_eq!(
        edit.study.paired().master().revision(),
        study.paired().master().revision()
    );
    assert_eq!(edit.study.paired_totals(), edit.totals);
    for entry in study.paired().carrier().entries() {
        if entry.kind() != bessemer::compound::CompoundEntryKind::Stream {
            continue;
        }
        let derived = edit
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
            .stream(&derived.result)
            .unwrap();
        let expected = if entry.name_units() == [79, 85, 84] {
            golden
        } else {
            study.paired().carrier().stream(entry.id()).unwrap()
        };
        assert_eq!(actual, expected);
    }
    let encoded = edit.study.paired().carrier().to_bytes();
    let reopened =
        SimulationOutStudy::from_bytes(&encoded, COMPOUND_LIMITS, reference::LIMITS).unwrap();
    assert_eq!(reopened.report().to_bytes(), golden);
    assert_eq!(study.paired().carrier().to_bytes(), carrier);
    let first = &study.report().fields()[0];
    let unchanged = study
        .edit_numeric_lexeme(&first.id, &first.numeric_lexeme)
        .unwrap();
    assert_eq!(unchanged.study.paired().carrier().to_bytes(), carrier);
    assert_eq!(
        unchanged.study.report().revision(),
        study.report().revision()
    );
    assert_eq!(
        unchanged.study.paired().image().revision(),
        study.paired().image().revision()
    );
    std::fs::write("target/simulation-out-native-edit.cwr", encoded).unwrap();
}
