//! Complete Material standalone/prefix reads and construction against all actual reference candidates (#140).
use bessemer::simulation::*;

const OWNED: &[u8] = include_bytes!("fixtures/simulation-material/owned.bin");
const LIMITS: SimulationMaterialLimits = SimulationMaterialLimits {
    maximum_bytes: 1000000,
    maximum_fields: 100000,
    maximum_entries: 2000000,
    maximum_text_bytes: 1000000,
};

fn word(cursor: &mut &[u8]) -> usize {
    let (raw, remaining) = cursor.split_at(4);
    *cursor = remaining;
    u32::from_le_bytes(raw.try_into().unwrap()) as usize
}

fn take<'a>(cursor: &mut &'a [u8], size: usize) -> &'a [u8] {
    let (raw, remaining) = cursor.split_at(size);
    *cursor = remaining;
    raw
}

fn assert_source_spans(body: &SimulationStudyMaterial, cursor: &mut &[u8]) {
    let mut fields = body.field_offsets().iter().peekable();
    for _ in 0..word(cursor) {
        let (start, end) = (word(cursor), word(cursor));
        assert_source_span(&mut fields, start, end);
    }
    assert!(fields.next().is_none());
}

fn assert_complete(body: &SimulationStudyMaterial, raw: &[u8]) {
    assert_eq!(body.to_bytes(), raw);
    let constructed =
        SimulationStudyMaterial::from_fields(body.fields().to_vec(), body.version(), LIMITS)
            .unwrap();
    assert_eq!(constructed.to_bytes(), raw);
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_coverage(body, raw);
    assert_ne!(constructed.field_id(0).unwrap(), body.field_id(0).unwrap());
}

fn assert_prefix(body: &SimulationStudyMaterial, raw: &[u8]) {
    let suffix = b"\xa5\x7fnext Property\0\xff";
    let mut combined = raw.to_vec();
    combined.extend_from_slice(suffix);
    let original = combined.clone();
    let (prefix, consumed) =
        SimulationStudyMaterial::from_prefix(&combined, body.version(), LIMITS).unwrap();
    assert_eq!(consumed, raw.len());
    assert_eq!(prefix.to_bytes(), raw);
    assert_prefix_fields(&prefix, body);
    assert!(SimulationStudyMaterial::from_bytes(&combined, body.version(), LIMITS).is_err());
    assert_eq!(combined, original);
    assert_eq!(&combined[consumed..], suffix);
    assert_text_parents(&prefix, raw);
    combined.fill(0);
    assert_eq!(prefix.to_bytes(), raw);
}

fn assert_text_parents(body: &SimulationStudyMaterial, expected: &[u8]) {
    for field in body.fields() {
        if let SimulationMaterialValue::Text(text) = &field.value {
            assert_eq!(text.source_parent(), expected);
            assert!(text.source_offsets().2 <= expected.len());
        }
    }
}

fn replay(mut cursor: &[u8]) -> (usize, usize) {
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(word(&mut cursor) as i32).unwrap();
        let expected = word(&mut cursor) != 0;
        let size = word(&mut cursor);
        let end = word(&mut cursor);
        let raw = take(&mut cursor, size);
        let result = SimulationStudyMaterial::from_bytes(raw, version, LIMITS);
        assert_eq!(
            result.is_ok(),
            expected,
            "version {version:?}, size {size}; expected actual reference outcome"
        );
        consume_case(&mut cursor, raw, result, end);
        accepted += usize::from(expected);
        rejected += usize::from(!expected);
    }
    (accepted, rejected)
}

fn consume_case(
    cursor: &mut &[u8],
    raw: &[u8],
    result: bessemer::Result<SimulationStudyMaterial>,
    end: usize,
) {
    let Ok(body) = result else {
        let count = word(cursor);
        take(cursor, count * 8);
        return;
    };
    assert_eq!(end, raw.len());
    assert_complete(&body, raw);
    assert_prefix(&body, raw);
    assert_source_spans(&body, cursor);
}

#[test]
fn all_owned_historical_presence_and_empty_bodies_match_complete_source_spans() {
    assert_eq!(replay(OWNED), (300, 0));
}

#[test]
#[ignore = "Requires explicitly recovered complete native Material inputs and all candidate outcomes"]
fn all_native_candidates_preserve_every_accepted_and_failed_alternative() {
    let path = std::env::var("BESSEMER_MATERIAL_NATIVE_INPUT")
        .expect("BESSEMER_MATERIAL_NATIVE_INPUT=<unset>; expected explicit complete native candidate packet path");
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native candidate packet")
    });
    assert_eq!(replay(&packet), (1230, 67));
}

#[path = "simulation_material/boundaries.rs"]
mod boundaries;
#[path = "simulation_material/concurrency.rs"]
mod concurrency;
#[path = "simulation_material/edits.rs"]
mod edits;
#[path = "simulation_material/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_material/malformed.rs"]
mod malformed;
#[path = "simulation_material/projection.rs"]
mod projection;

fn assert_source_span(
    fields: &mut std::iter::Peekable<std::slice::Iter<'_, (usize, usize)>>,
    start: usize,
    end: usize,
) {
    let mut consumed = start;
    while fields
        .peek()
        .is_some_and(|span| span.0 == consumed && (consumed < end || span.1 == end))
    {
        let (next, last) = *fields.next().unwrap();
        assert_eq!(next, consumed);
        consumed = last;
        if consumed == end {
            break;
        }
    }
    assert_eq!(
        consumed, end,
        "expected complete actual span {start}..{end}"
    );
}

fn assert_coverage(body: &SimulationStudyMaterial, raw: &[u8]) {
    assert_eq!(
        body.coverage().boundary,
        SimulationMaterialBoundary::Standalone
    );
    assert_eq!(
        body.coverage().check_mode,
        SimulationMaterialCheckMode::ExactStandalone
    );
    assert_eq!(
        body.coverage().completion,
        SimulationMaterialCompletion::CompletedBody
    );
    assert_eq!(body.coverage().checked_bytes, raw.len());
    assert_eq!(body.coverage().checked_fields, body.fields().len());
    assert_eq!(body.coverage().requested_pattern_error_bits, 0);
    assert_eq!(body.coverage().diagnostics.len(), 4);
}

fn assert_prefix_fields(prefix: &SimulationStudyMaterial, body: &SimulationStudyMaterial) {
    assert_eq!(prefix.fields(), body.fields());
    assert_eq!(prefix.field_offsets(), body.field_offsets());
    assert_eq!(
        prefix.coverage().boundary,
        SimulationMaterialBoundary::Prefix
    );
    assert_eq!(
        prefix.coverage().check_mode,
        SimulationMaterialCheckMode::ExactPrefix
    );
}

#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
