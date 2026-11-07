use bessemer::composer::arrays::*;
use std::io::{Cursor, Read};

pub struct ValueCase<'a> {
    pub layout: ComposerVectorLayout,
    pub wire: &'a [u8],
    pub expected: &'a [u8],
}

pub fn value_cases(raw: &[u8]) -> Vec<ValueCase<'_>> {
    let mut reader = Cursor::new(raw);
    assert_eq!(span(&mut reader, 4), b"VVAL");
    let cases = (0..word(&mut reader))
        .map(|_| read_value_case(&mut reader))
        .collect();
    assert_eq!(reader.position() as usize, raw.len());
    cases
}

fn read_value_case<'a>(reader: &mut Cursor<&'a [u8]>) -> ValueCase<'a> {
    let dimensions = match span(reader, 1)[0] {
        2 => ComposerVectorDimensions::Two,
        3 => ComposerVectorDimensions::Three,
        value => panic!("dimension {value}; expected 2 or 3"),
    };
    let (count, wire_size, expected_size) = (word(reader), word(reader), word(reader));
    ValueCase {
        layout: ComposerVectorLayout { count, dimensions },
        wire: span(reader, wire_size as usize),
        expected: span(reader, expected_size as usize),
    }
}

fn span<'a>(reader: &mut Cursor<&'a [u8]>, size: usize) -> &'a [u8] {
    let raw = *reader.get_ref();
    let start = reader.position() as usize;
    reader.set_position((start + size) as u64);
    &raw[start..start + size]
}

fn word(reader: &mut Cursor<&[u8]>) -> u32 {
    let mut raw = [0; 4];
    reader.read_exact(&mut raw).unwrap();
    u32::from_le_bytes(raw)
}

pub fn verify_case(case: &ValueCase<'_>) -> Vec<u8> {
    let block = ComposerVectorBlock::from_bytes(case.wire, case.layout, case.wire.len()).unwrap();
    let values = block
        .reconstruct(
            ComposerVectorProfile::NearestEvenGradual79V1,
            case.expected.len() / 4,
        )
        .unwrap();
    let raw = pattern_bytes(&values);
    assert_patterns(&raw, case);
    assert_eq!(block.to_bytes(), case.wire);
    verify_report(&values, case);
    raw
}

fn pattern_bytes(values: &ComposerVectorValues) -> Vec<u8> {
    values
        .patterns()
        .iter()
        .flat_map(|value| value.bits().to_le_bytes())
        .collect()
}

fn assert_patterns(raw: &[u8], case: &ValueCase<'_>) {
    assert_eq!(raw.len(), case.expected.len());
    if let Some(first) = raw.iter().zip(case.expected).position(|(a, b)| a != b) {
        assert_eq!(
            raw[first], case.expected[first],
            "byte {first}, layout {:?}; expected declared-profile bits",
            case.layout
        );
    }
}

fn verify_report(values: &ComposerVectorValues, case: &ValueCase<'_>) {
    assert_eq!(values.layout(), case.layout);
    assert_eq!(values.report().coverage.components, case.expected.len() / 4);
    assert_eq!(
        values.report().coverage.mode,
        ComposerVectorValueCheckMode::ExactDeclaredProfile
    );
}
