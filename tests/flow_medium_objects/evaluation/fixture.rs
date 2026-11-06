use super::*;
use std::io::Read;
pub fn cases() -> Vec<EvaluationCase> {
    let bytes = include_bytes!("../../fixtures/flow-medium-evaluation/curves.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(bytes.as_slice())
        .take(64_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert!(raw.len() <= 64_000_000);
    let mut cursor = std::io::Cursor::new(raw);
    assert_eq!(number::<4>(&mut cursor), *b"FEV1");
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn case(cursor: &mut PacketCursor) -> EvaluationCase {
    EvaluationCase {
        body: fixture::body(cursor),
        ordinal: signed(cursor),
        query: fixture::wide(cursor),
        expected: optional(cursor, expected),
    }
}
fn expected(cursor: &mut PacketCursor) -> ExpectedEvaluation {
    ExpectedEvaluation {
        bits: fixture::wide(cursor),
        classification: byte(cursor),
        branch: byte(cursor),
        bracket: optional(cursor, bracket),
        reference: fixture::wide(cursor),
        measured_error_bound: optional(cursor, fixture::wide),
    }
}
fn bracket(cursor: &mut PacketCursor) -> (u32, u32) {
    (word(cursor), word(cursor))
}
