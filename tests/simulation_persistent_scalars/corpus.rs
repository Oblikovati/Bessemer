use super::projections::{self, profile_tag, span, words};
use bessemer::simulation::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "../common/simulation_images.rs"]
mod packet;
use packet::Cursor;

struct ScalarCase<'a> {
    profile: SimulationPersistScalarProfile,
    maximum: usize,
    ordinal: i32,
    accepted: bool,
    raw: &'a [u8],
    replacement: &'a [u8],
}

fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/simulation-persistent-scalars/owned.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

#[test]
fn complete_owned_scalar_profiles_edits_and_negative_cases_match_independent_oracles() {
    let expected = owned();
    assert_eq!(project_packet(&expected), expected);
}

#[test]
#[ignore = "requires injected native input and complete scalar projection output paths"]
fn export_complete_native_scalar_frames_edits_and_rejections() {
    let input = std::env::var_os("BESSEMER_PERSISTENT_SCALARS_NATIVE_INPUT")
        .expect("expected injected native input path");
    let output = std::env::var_os("BESSEMER_PERSISTENT_SCALARS_PROJECTION_OUTPUT")
        .expect("expected injected projection output path");
    let native = std::fs::read(input).unwrap();
    let mut result = Vec::new();
    span(&mut result, &project_packet(&owned()));
    span(&mut result, &project_packet(&native));
    std::fs::write(output, result).unwrap();
}

fn project_packet(expected: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.take(4), b"SCAL");
    let count = cursor.word();
    let mut output = b"SCAL".to_vec();
    words(&mut output, &[count]);
    for _ in 0..count {
        let case = read_case(&mut cursor);
        let expected = cursor.span();
        let actual = project(&case);
        assert_eq!(
            actual, expected,
            "profile {:?}, max {}, field {}, accepted {}",
            case.profile, case.maximum, case.ordinal, case.accepted
        );
        append_case(&mut output, &case, &actual);
    }
    assert!(cursor.0.is_empty());
    output
}

fn read_case<'a>(cursor: &mut Cursor<'a>) -> ScalarCase<'a> {
    let profile = match cursor.word() {
        0 => SimulationPersistScalarProfile::CountedI32,
        1 => SimulationPersistScalarProfile::UniqueIdsI64,
        other => panic!("profile {other}; expected 0 or 1"),
    };
    let maximum = cursor.word() as usize;
    let ordinal = cursor.word() as i32;
    let accepted = match cursor.word() {
        0 => false,
        1 => true,
        other => panic!("accepted {other}; expected 0 or 1"),
    };
    ScalarCase {
        profile,
        maximum,
        ordinal,
        accepted,
        raw: cursor.span(),
        replacement: cursor.span(),
    }
}

fn append_case(output: &mut Vec<u8>, case: &ScalarCase<'_>, actual: &[u8]) {
    words(
        output,
        &[
            profile_tag(case.profile),
            case.maximum as u32,
            case.ordinal as u32,
            u32::from(case.accepted),
        ],
    );
    span(output, case.raw);
    span(output, case.replacement);
    span(output, actual);
}

fn value(bytes: &[u8]) -> SimulationPersistScalarValue {
    match bytes.len() {
        4 => SimulationPersistScalarValue::I32(i32::from_le_bytes(bytes.try_into().unwrap())),
        8 => SimulationPersistScalarValue::I64(i64::from_le_bytes(bytes.try_into().unwrap())),
        other => panic!("pattern {other} bytes; expected width 4 or 8"),
    }
}

fn project(case: &ScalarCase<'_>) -> Vec<u8> {
    if !case.accepted {
        return rejected(case);
    }
    let source =
        SimulationPersistScalarArchive::from_bytes(case.raw, case.profile, case.maximum).unwrap();
    verify_construction(&source);
    if case.ordinal < 0 {
        assert_eq!(case.ordinal, -1);
        return projections::image(&source);
    }
    let id = &source.fields()[case.ordinal as usize].id;
    let edit = source.edit_field(id, value(case.replacement)).unwrap();
    assert_eq!(source.to_bytes(), case.raw);
    projections::edit(&edit)
}

fn verify_construction(source: &SimulationPersistScalarArchive) {
    let rebuilt = match source.profile() {
        SimulationPersistScalarProfile::CountedI32 => construct_integer(source),
        SimulationPersistScalarProfile::UniqueIdsI64 => construct_unique(source),
    };
    assert_eq!(rebuilt.to_bytes(), source.to_bytes());
}

fn construct_integer(source: &SimulationPersistScalarArchive) -> SimulationPersistScalarArchive {
    let values: Vec<_> = (0..source.used_count())
        .map(|index| {
            match source
                .field(&source.entry_id(index).unwrap())
                .unwrap()
                .value
            {
                SimulationPersistScalarValue::I32(value) => value,
                other => panic!("{other:?}; expected I32"),
            }
        })
        .collect();
    SimulationPersistScalarArchive::from_integer_values(&values, source.coverage().maximum_entries)
        .unwrap()
}

fn construct_unique(source: &SimulationPersistScalarArchive) -> SimulationPersistScalarArchive {
    let keys: Vec<_> = (0..source.used_count())
        .map(|index| {
            match source
                .field(&source.entry_id(index).unwrap())
                .unwrap()
                .value
            {
                SimulationPersistScalarValue::I64(value) => value,
                other => panic!("{other:?}; expected I64"),
            }
        })
        .collect();
    SimulationPersistScalarArchive::from_unique_keys(
        &keys,
        source.allocation_count().unwrap(),
        source.coverage().maximum_entries,
    )
    .unwrap()
}

fn rejected(case: &ScalarCase<'_>) -> Vec<u8> {
    let parsed = SimulationPersistScalarArchive::from_bytes(case.raw, case.profile, case.maximum);
    let error = if case.ordinal < 0 {
        parsed.unwrap_err()
    } else {
        let source = parsed.unwrap();
        source
            .edit_field(
                &source.fields()[case.ordinal as usize].id,
                value(case.replacement),
            )
            .unwrap_err()
    };
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(!error.operation.is_empty() && !error.entity.is_empty());
    assert!(error.cause.contains("expected"));
    let mut output = Vec::new();
    words(&mut output, &[0, 1]);
    output
}
