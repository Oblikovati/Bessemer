use super::projections::{self, context, span, words};
use bessemer::simulation::*;
use std::io::Write;
#[allow(dead_code)]
#[path = "../common/simulation_images.rs"]
pub(super) mod packet;
use packet::Cursor;

struct ParameterCase<'a> {
    version: SimulationPersistVersion,
    limits: SimulationParameterLimits,
    ordinal: i32,
    replacement_kind: i32,
    accepted: bool,
    raw: &'a [u8],
    replacement: &'a [u8],
}

#[test]
fn complete_owned_parameters_fields_edits_history_and_rejections_match_independent_oracles() {
    project_owned(&mut std::io::sink());
}

#[test]
#[ignore = "requires injected native ParameterSets input and complete projection output paths"]
fn export_complete_native_parameter_candidates_fields_edits_and_history() {
    let input = std::env::var_os("BESSEMER_PARAMETER_SETS_NATIVE_INPUT")
        .expect("expected injected native input path");
    let output = std::env::var_os("BESSEMER_PARAMETER_SETS_PROJECTION_OUTPUT")
        .expect("expected injected projection output path");
    let native = std::fs::read(input).unwrap();
    let mut result = std::io::BufWriter::new(std::fs::File::create(output).unwrap());
    project_owned(&mut result);
    let projected = project_packet(&native);
    result
        .write_all(&(projected.len() as u32).to_le_bytes())
        .unwrap();
    result.write_all(&projected).unwrap();
    result.flush().unwrap();
}

fn project_owned(output: &mut impl Write) {
    let mut fixture = super::fixture_storage::OwnedParameterFixture::open();
    output
        .write_all(&fixture.expanded_size.to_le_bytes())
        .unwrap();
    output.write_all(b"PARM").unwrap();
    output.write_all(&fixture.count.to_le_bytes()).unwrap();
    let mut expanded = 8usize;
    for _ in 0..fixture.count {
        let row = project_owned_case(fixture.next_case());
        expanded += row.len();
        output.write_all(&row).unwrap();
    }
    assert_eq!(expanded, fixture.expanded_size as usize);
    fixture.finish();
}

fn project_owned_case(stored: super::fixture_storage::StoredParameterCase) -> Vec<u8> {
    let mut cursor = Cursor(&stored.input);
    let case = read_case(&mut cursor);
    assert!(cursor.0.is_empty());
    let actual = checked_projection(&case, &stored.expected);
    let mut output = Vec::new();
    append_case(&mut output, &case, &actual);
    assert_eq!(&output[..stored.input.len()], stored.input);
    output
}

fn project_packet(expected: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.take(4), b"PARM");
    let count = cursor.word();
    let mut output = b"PARM".to_vec();
    words(&mut output, &[count]);
    for _ in 0..count {
        let case = read_case(&mut cursor);
        let expected = cursor.span();
        let actual = checked_projection(&case, expected);
        append_case(&mut output, &case, &actual);
    }
    assert!(cursor.0.is_empty());
    output
}

fn checked_projection(case: &ParameterCase<'_>, expected: &[u8]) -> Vec<u8> {
    let actual = project(case);
    assert_eq!(
        actual,
        expected,
        "version {}, field {}, accepted {}",
        case.version.value(),
        case.ordinal,
        case.accepted
    );
    actual
}

fn read_case<'a>(cursor: &mut Cursor<'a>) -> ParameterCase<'a> {
    ParameterCase {
        version: SimulationPersistVersion::new(cursor.word().try_into().unwrap()).unwrap(),
        limits: SimulationParameterLimits {
            maximum_records: cursor.word() as usize,
            maximum_table_items: cursor.word() as usize,
            maximum_text_bytes: cursor.word() as usize,
        },
        ordinal: cursor.word() as i32,
        replacement_kind: cursor.word() as i32,
        accepted: cursor.word() == 1,
        raw: cursor.span(),
        replacement: cursor.span(),
    }
}

fn append_case(output: &mut Vec<u8>, case: &ParameterCase<'_>, actual: &[u8]) {
    context(output, case.version, case.limits);
    words(
        output,
        &[
            case.ordinal as u32,
            case.replacement_kind as u32,
            u32::from(case.accepted),
        ],
    );
    span(output, case.raw);
    span(output, case.replacement);
    span(output, actual);
}

fn value(case: &ParameterCase<'_>) -> SimulationParameterValue {
    match case.replacement_kind {
        0 => SimulationParameterValue::Signed(i32::from_le_bytes(
            case.replacement.try_into().unwrap(),
        )),
        1 => SimulationParameterValue::Unsigned(u32::from_le_bytes(
            case.replacement.try_into().unwrap(),
        )),
        2 => SimulationParameterValue::DoubleBits(u64::from_le_bytes(
            case.replacement.try_into().unwrap(),
        )),
        3 => SimulationParameterValue::Text(
            SimulationPersistText::read_from(case.replacement, 0, case.limits.maximum_text_bytes)
                .unwrap(),
        ),
        other => panic!("replacement kind {other}; expected 0..4"),
    }
}

fn project(case: &ParameterCase<'_>) -> Vec<u8> {
    let parsed = SimulationParameterSets::from_bytes(case.raw, case.version, case.limits);
    if !case.accepted {
        return rejected(case, parsed);
    }
    accepted_projection(&parsed.unwrap(), case)
}

fn accepted_projection(source: &SimulationParameterSets, case: &ParameterCase<'_>) -> Vec<u8> {
    verify_construction(source);
    let mut output = Vec::new();
    words(&mut output, &[1]);
    if case.ordinal < 0 {
        assert_eq!(case.ordinal, -1);
        output.extend(projections::image(source));
        return output;
    }
    let edit = source
        .edit_field(
            &source.field_id(case.ordinal as usize).unwrap(),
            value(case),
        )
        .unwrap();
    super::support::verify_edit(&edit);
    output.extend(projections::edit(&edit, case.ordinal as usize));
    assert_eq!(source.to_bytes(), case.raw);
    output
}

fn verify_construction(source: &SimulationParameterSets) {
    let rebuilt =
        SimulationParameterSets::from_parts(source.parts(), source.version(), source.limits())
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), source.to_bytes());
}

fn rejected(
    case: &ParameterCase<'_>,
    parsed: bessemer::Result<SimulationParameterSets>,
) -> Vec<u8> {
    let error = if case.ordinal < 0 {
        parsed.unwrap_err()
    } else {
        let source = parsed.unwrap();
        let original = source.to_bytes();
        let error = source
            .edit_field(
                &source.field_id(case.ordinal as usize).unwrap(),
                value(case),
            )
            .unwrap_err();
        assert_eq!(source.to_bytes(), original);
        error
    };
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(!error.operation.is_empty() && !error.entity.is_empty());
    assert!(error.cause.contains("expected"));
    vec![0; 4]
}
