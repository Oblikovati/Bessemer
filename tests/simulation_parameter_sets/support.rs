use bessemer::simulation::*;

pub(super) const EMPTY_MODERN_WORDS: [i32; 11] = [
    0,
    -1,
    i32::MAX,
    0x12345678,
    -0x76543211,
    0x81abcdefu32 as i32,
    0xf0000000u32 as i32,
    0,
    0x89abcdefu32 as i32,
    0x76543210,
    -1,
];

pub(super) fn construct(
    parts: &SimulationParameterParts,
    numeric: i32,
    limits: SimulationParameterLimits,
) -> SimulationParameterSets {
    SimulationParameterSets::from_parts(parts, version(numeric), limits).unwrap()
}

pub(super) fn find_role(
    source: &SimulationParameterSets,
    role: SimulationParameterRole,
) -> &SimulationParameterField {
    source
        .fields()
        .iter()
        .find(|field| field.role == role)
        .unwrap()
}

pub(super) fn expect_invalid_edit(
    source: &SimulationParameterSets,
    id: &SimulationParameterFieldId,
    replacement: SimulationParameterValue,
) {
    let error = source.edit_field(id, replacement).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains("expected"));
}

pub(super) fn version(value: i32) -> SimulationPersistVersion {
    SimulationPersistVersion::new(value).unwrap()
}

pub(super) fn header(value: i32, columns: usize) -> SimulationParameterHeader {
    SimulationParameterHeader {
        words: [
            if value < 810026 {
                i32::MIN
            } else {
                columns.try_into().unwrap()
            },
            -1,
            i32::MAX,
            0x12345678,
            -0x76543211,
        ],
        gate700013: (value > 700013).then_some(0x81abcdefu32 as i32),
        gate700018: (value > 700018).then_some(0xf0000000u32 as i32),
        tail: 0x89abcdefu32 as i32,
        gate820015: (value > 820015).then_some(0x76543210),
        gate830009: (value > 830009).then_some(-1),
    }
}

pub(super) fn numbers(value: i32, columns: usize) -> SimulationParameterRecordNumbers {
    SimulationParameterRecordNumbers {
        signed_word: i32::MIN,
        unsigned_words: [u32::MAX, 0x80000000],
        conditional_word: (value > 700012).then_some(0x01ffffff),
        conditional_double_bits: (value > 700013).then_some(0x7ff0000000000001),
        values_bits: (0..columns)
            .map(|index| 0x8000000000000000 ^ index as u64)
            .collect(),
    }
}

pub(super) fn parts(value: i32, columns: usize, populated: bool) -> SimulationParameterParts {
    let text = SimulationPersistText::from_payload(None, 8).unwrap();
    let records = if populated {
        vec![SimulationParameterRecordParts {
            text,
            numbers: numbers(value, columns),
        }]
    } else {
        vec![]
    };
    SimulationParameterParts {
        header: header(value, columns),
        table: (0..columns).map(|index| index as i32 ^ i32::MIN).collect(),
        records,
    }
}

pub(super) fn modern() -> SimulationParameterSets {
    SimulationParameterSets::from_parts(
        &parts(830010, 2, true),
        version(830010),
        SimulationParameterLimits::default(),
    )
    .unwrap()
}

pub(super) fn verify_complete_field_partition(source: &SimulationParameterSets) {
    let mut offset = 0;
    let mut raw = Vec::new();
    for (ordinal, field) in source.fields().iter().enumerate() {
        assert_eq!(field.id, source.field_id(ordinal).unwrap());
        assert_eq!(field.stream_offset, offset);
        let pattern = source.field(&field.id).unwrap().value.to_bytes();
        offset += pattern.len();
        raw.extend(pattern);
    }
    assert_eq!(raw, source.to_bytes());
    assert_eq!(source.coverage().checked_bytes, offset);
    assert_eq!(source.coverage().requested_pattern_error_bits, 0);
    assert_eq!(source.limits(), source.coverage().limits);
    assert_eq!(source.version(), source.coverage().version);
}

pub(super) fn changed_pattern(value: &SimulationParameterValue) -> SimulationParameterValue {
    match value {
        SimulationParameterValue::Signed(value) => {
            SimulationParameterValue::Signed(value ^ i32::MIN)
        }
        SimulationParameterValue::Unsigned(value) => {
            SimulationParameterValue::Unsigned(value ^ 0x80000000)
        }
        SimulationParameterValue::DoubleBits(value) => {
            SimulationParameterValue::DoubleBits(value ^ 0x8000000000000000)
        }
        SimulationParameterValue::Text(_) => SimulationParameterValue::Text(
            SimulationPersistText::from_payload(Some(&[0x81, 0, 0]), 8).unwrap(),
        ),
    }
}

pub(super) fn verify_edit(edit: &SimulationParameterEdit) {
    let original = edit.source.field(&edit.parent).unwrap();
    let mut expected = edit.source.to_bytes();
    let replacement = edit.replacement.to_bytes();
    expected.splice(
        original.stream_offset..original.stream_offset + original.value.to_bytes().len(),
        replacement.clone(),
    );
    assert_eq!(edit.archive.to_bytes(), expected);
    assert_eq!(
        edit.archive.field(&edit.result).unwrap().value.to_bytes(),
        replacement
    );
    let changed = original.value.to_bytes() != replacement;
    verify_edit_coverage(edit, changed);
    for (ordinal, entry) in edit.history.iter().enumerate() {
        verify_history(edit, ordinal, entry, changed);
    }
}

fn verify_edit_coverage(edit: &SimulationParameterEdit, changed: bool) {
    assert_eq!(changed, edit.source.revision() != edit.archive.revision());
    assert_eq!(
        changed,
        edit.coverage.completion == SimulationParameterCompletion::Changed
    );
    assert_eq!(edit.history.len(), edit.source.fields().len());
    assert_eq!(
        edit.coverage.check_mode,
        SimulationParameterCheckMode::ExactSelectedProfileAndPreservation
    );
}

pub(super) fn verify_history(
    edit: &SimulationParameterEdit,
    ordinal: usize,
    entry: &SimulationParameterHistory,
    changed: bool,
) {
    assert_eq!(entry.parent, edit.source.field_id(ordinal).unwrap());
    assert_eq!(entry.result, edit.archive.field_id(ordinal).unwrap());
    let before = edit.source.field(&entry.parent).unwrap();
    let after = edit.archive.field(&entry.result).unwrap();
    assert_eq!(before.role, after.role);
    let derived = changed && entry.parent == edit.parent;
    assert_eq!(
        entry.relation == SimulationParameterRelation::Derived,
        derived
    );
    let expected = if derived {
        edit.replacement.to_bytes()
    } else {
        before.value.to_bytes()
    };
    assert_eq!(after.value.to_bytes(), expected);
    assert!(changed || entry.parent == entry.result);
}
