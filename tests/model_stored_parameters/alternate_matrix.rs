use super::*;

struct AlternateCase<'a> {
    kind: usize,
    style: usize,
    operation: u32,
    original: &'a [u8],
    expected: &'a [u8],
    request: &'a [u8],
}

fn alternate_blob<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let count = alternate_word(cursor) as usize;
    assert!(count <= LIMITS.maximum_bytes);
    let (value, tail) = cursor.split_at(count);
    *cursor = tail;
    value
}

fn alternate_word(cursor: &mut &[u8]) -> u32 {
    let value = u32::from_le_bytes(cursor[..4].try_into().unwrap());
    *cursor = &cursor[4..];
    value
}

fn alternate_case<'a>(cursor: &mut &'a [u8]) -> AlternateCase<'a> {
    AlternateCase {
        kind: alternate_word(cursor) as usize,
        style: alternate_word(cursor) as usize,
        operation: alternate_word(cursor),
        original: alternate_blob(cursor),
        expected: alternate_blob(cursor),
        request: alternate_blob(cursor),
    }
}

fn alternate_request(case: &AlternateCase<'_>) -> ModelParameterStoredEdit {
    match case.operation {
        0 if case.kind == 3 => ModelParameterStoredEdit::Alternate(ModelParameterValue::Integer(
            i32::from_le_bytes(case.request.try_into().unwrap()),
        )),
        0 => ModelParameterStoredEdit::Alternate(ModelParameterValue::Double(
            StoredDouble::from_bits(u64::from_le_bytes(case.request.try_into().unwrap())),
        )),
        1 => {
            let [byte] = case.request else {
                panic!("expected one presence byte");
            };
            ModelParameterStoredEdit::AlternatePresence(*byte)
        }
        other => panic!("operation {other}; expected alternate0/presence1"),
    }
}

fn alternate_reverse(
    case: &AlternateCase<'_>,
    parameter: &ModelParameterStoredState,
) -> ModelParameterStoredEdit {
    match case.operation {
        0 => ModelParameterStoredEdit::Alternate(parameter.alternate().unwrap().clone()),
        1 => ModelParameterStoredEdit::AlternatePresence(parameter.alternate_presence().unwrap()),
        other => panic!("operation {other}; expected alternate0/presence1"),
    }
}

fn check_alternate_case(case: &AlternateCase<'_>) {
    assert!(case.kind < 5 && (1..4).contains(&case.style));
    let (document, source) = fixture(case.original);
    let parameter = state(&source);
    assert_eq!(parameter.kind(), KINDS[case.kind]);
    let edit = apply_parameter(&source, &parameter, alternate_request(case));
    assert_eq!(edit.archive().to_bytes().unwrap(), case.expected);
    let changed = case.expected != case.original;
    assert_eq!(edit.changes().len(), usize::from(changed));
    assert_alternate_completion(&edit, changed);
    super::edits::assert_document_reversal(
        &document,
        &source,
        &edit,
        alternate_reverse(case, &parameter),
    );
    assert_eq!(source.to_bytes().unwrap(), case.original);
}

fn assert_alternate_completion(edit: &PartResolvedFeaturesEdit, changed: bool) {
    assert_eq!(
        edit.completion(),
        if changed {
            ModelEditCompletion::Changed
        } else {
            ModelEditCompletion::Unchanged
        }
    );
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(state(edit.archive()).achieved_stored_value_error(), 0);
}

#[test]
fn complete_alternate_and_every_positive_presence_matrix_matches_source_and_reverses_documents() {
    let raw = std::fs::read("tests/fixtures/stored-parameters/alternate-matrix/cases.bin").unwrap();
    let mut cursor = raw.as_slice();
    assert_eq!(&cursor[..4], b"BPA1");
    cursor = &cursor[4..];
    assert_eq!(alternate_word(&mut cursor), 3840);
    let mut counts = [0; 2];
    for _ in 0..3840 {
        let case = alternate_case(&mut cursor);
        check_alternate_case(&case);
        counts[case.operation as usize] += 1;
    }
    assert!(cursor.is_empty());
    assert_eq!(counts, [15, 3825]);
}
