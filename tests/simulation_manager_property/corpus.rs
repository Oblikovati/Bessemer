use bessemer::simulation::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "../common/simulation_images.rs"]
mod packet;
use packet::Cursor;

struct ManagerCase<'a> {
    profile: SimulationManagerPropertyProfile,
    accepted: bool,
    ordinal: i32,
    replacement: i32,
    raw: &'a [u8],
}

fn words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

fn span(output: &mut Vec<u8>, raw: &[u8]) {
    words(output, &[raw.len().try_into().unwrap()]);
    output.extend_from_slice(raw);
}

fn context(output: &mut Vec<u8>, profile: SimulationManagerPropertyProfile) {
    let (kind, version) = match profile {
        SimulationManagerPropertyProfile::Beam(version) => (0, Some(version)),
        SimulationManagerPropertyProfile::Mass(version) => (1, version),
    };
    words(
        output,
        &[
            kind,
            u32::from(version.is_some()),
            version.map_or(0, |value| value.value() as u32),
        ],
    );
}

fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/simulation-manager-property/owned.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

#[test]
fn complete_owned_manager_profiles_and_all_word_edits_match_independent_oracles() {
    let expected = owned();
    assert_eq!(project_packet(&expected), expected);
}

#[test]
#[ignore = "requires injected native manager input and complete projection output paths"]
fn export_complete_native_manager_headers_candidates_and_word_edits() {
    let input = std::env::var_os("BESSEMER_MANAGER_PROPERTY_NATIVE_INPUT")
        .expect("expected injected native input path");
    let output = std::env::var_os("BESSEMER_MANAGER_PROPERTY_PROJECTION_OUTPUT")
        .expect("expected injected projection output path");
    let native = std::fs::read(input).unwrap();
    let mut result = Vec::new();
    span(&mut result, &project_packet(&owned()));
    span(&mut result, &project_packet(&native));
    std::fs::write(output, result).unwrap();
}

fn project_packet(expected: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.take(4), b"MGRP");
    let count = cursor.word();
    let mut output = b"MGRP".to_vec();
    words(&mut output, &[count]);
    for _ in 0..count {
        let case = read_case(&mut cursor);
        let expected = cursor.span();
        let actual = project(&case);
        assert_eq!(
            actual, expected,
            "{:?}, field {}, accepted {}",
            case.profile, case.ordinal, case.accepted
        );
        append_case(&mut output, &case, &actual);
    }
    assert!(cursor.0.is_empty());
    output
}

fn read_case<'a>(cursor: &mut Cursor<'a>) -> ManagerCase<'a> {
    ManagerCase {
        profile: read_profile(cursor),
        accepted: cursor.word() == 1,
        ordinal: cursor.word() as i32,
        replacement: cursor.word() as i32,
        raw: cursor.span(),
    }
}

fn read_profile(cursor: &mut Cursor<'_>) -> SimulationManagerPropertyProfile {
    let kind = cursor.word();
    let present = cursor.word();
    let numeric = cursor.word();
    let version = match present {
        0 => {
            assert_eq!(numeric, 0);
            None
        }
        1 => Some(SimulationPersistVersion::new(numeric.try_into().unwrap()).unwrap()),
        other => panic!("version present {other}; expected 0 or 1"),
    };
    match kind {
        0 => {
            SimulationManagerPropertyProfile::Beam(version.expect("Beam requires explicit version"))
        }
        1 => SimulationManagerPropertyProfile::Mass(version),
        other => panic!("kind {other}; expected 0 or 1"),
    }
}

fn append_case(output: &mut Vec<u8>, case: &ManagerCase<'_>, actual: &[u8]) {
    context(output, case.profile);
    words(
        output,
        &[
            u32::from(case.accepted),
            case.ordinal as u32,
            case.replacement as u32,
        ],
    );
    span(output, case.raw);
    span(output, actual);
}

fn project(case: &ManagerCase<'_>) -> Vec<u8> {
    let parsed = SimulationManagerProperty::from_bytes(case.raw, case.profile);
    if !case.accepted {
        return rejected(parsed.unwrap_err());
    }
    project_accepted(&parsed.unwrap(), case)
}

fn project_accepted(source: &SimulationManagerProperty, case: &ManagerCase<'_>) -> Vec<u8> {
    verify_construction(source);
    let mut output = Vec::new();
    words(&mut output, &[1]);
    if case.ordinal < 0 {
        assert_eq!(case.ordinal, -1);
        image(&mut output, source);
        return output;
    }
    let edit = source
        .edit_field(
            &source.field_id(case.ordinal as usize).unwrap(),
            case.replacement,
        )
        .unwrap();
    super::verify_edit(&edit);
    append_edit(&mut output, &edit, case.ordinal as usize);
    assert_eq!(source.to_bytes(), case.raw);
    output
}

fn verify_construction(source: &SimulationManagerProperty) {
    let values: Vec<i32> = source.fields().iter().map(|field| field.value).collect();
    let rebuilt = SimulationManagerProperty::from_values(&values, source.profile()).unwrap();
    assert_eq!(rebuilt.to_bytes(), source.to_bytes());
}

fn rejected(error: bessemer::BessemerError) -> Vec<u8> {
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(!error.operation.is_empty() && !error.entity.is_empty());
    assert!(error.cause.contains("expected"));
    vec![0; 4]
}

fn image(output: &mut Vec<u8>, source: &SimulationManagerProperty) {
    context(output, source.profile());
    span(output, &source.to_bytes());
    words(output, &[source.fields().len() as u32]);
    for field in source.fields() {
        words(
            output,
            &[field.stream_offset as u32, field.member.offset() as u32],
        );
        span(output, &field.value.to_le_bytes());
        assert_eq!(source.field(&field.id).unwrap().value, field.value);
    }
    coverage(output, source.coverage());
}

fn coverage(output: &mut Vec<u8>, scope: SimulationManagerPropertyCoverage) {
    context(output, scope.profile);
    let completion = match scope.completion {
        SimulationManagerPropertyCompletion::CompletedFraming => 0,
        SimulationManagerPropertyCompletion::Unchanged => 1,
        SimulationManagerPropertyCompletion::Changed => 2,
    };
    let edited =
        scope.check_mode == SimulationManagerPropertyCheckMode::ExactProfileAndPreservation;
    words(
        output,
        &[
            completion,
            u32::from(edited),
            scope.checked_bytes as u32,
            scope.checked_fields as u32,
            diagnostics(&scope.diagnostics),
            scope.requested_pattern_error_bits,
        ],
    );
}

fn diagnostics(entries: &[SimulationManagerPropertyDiagnostic]) -> u32 {
    entries.iter().fold(0, |bits, entry| {
        bits | match entry {
            SimulationManagerPropertyDiagnostic::NativeApplicationNotExecuted => 1,
            SimulationManagerPropertyDiagnostic::RuntimeDispatchNotInferred => 2,
            SimulationManagerPropertyDiagnostic::FieldMeaningsUnassigned => 4,
            SimulationManagerPropertyDiagnostic::NestedObjectsNotDecoded => 8,
        }
    })
}

fn append_edit(output: &mut Vec<u8>, edit: &SimulationManagerPropertyEdit, ordinal: usize) {
    span(output, &edit.source.to_bytes());
    let mut result = Vec::new();
    image(&mut result, &edit.archive);
    span(output, &result);
    words(output, &[ordinal as u32]);
    span(output, &edit.replacement.to_le_bytes());
    words(
        output,
        &[
            u32::from(edit.coverage.completion == SimulationManagerPropertyCompletion::Changed),
            edit.history.len() as u32,
        ],
    );
    for (index, entry) in edit.history.iter().enumerate() {
        append_history(output, edit, index, entry);
    }
    coverage(output, edit.coverage);
}

fn append_history(
    output: &mut Vec<u8>,
    edit: &SimulationManagerPropertyEdit,
    ordinal: usize,
    entry: &SimulationManagerPropertyHistory,
) {
    let parent = edit.source.field(&entry.parent).unwrap();
    let result = edit.archive.field(&entry.result).unwrap();
    words(
        output,
        &[
            ordinal as u32,
            parent.stream_offset as u32,
            result.stream_offset as u32,
            parent.member.offset() as u32,
        ],
    );
    span(output, &parent.value.to_le_bytes());
    span(output, &result.value.to_le_bytes());
    words(
        output,
        &[u32::from(
            entry.relation == SimulationManagerPropertyRelation::Derived,
        )],
    );
}
