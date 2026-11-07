use super::*;

type ExpectedField = (usize, FlowGroupValue, FlowGroupValue, Option<usize>);

fn assert_member_values(actual: &FlowGroupView<'_>, expected: &fixture::ExpectedGroup) {
    let words: Vec<_> = expected.members.iter().map(|member| member.word).collect();
    let wide: Vec<_> = expected.members.iter().map(|member| member.wide).collect();
    assert_eq!(actual.words().collect::<Vec<_>>(), words);
    assert_eq!(actual.wide_words().collect::<Vec<_>>(), wide);
}

fn assert_flags(actual: &FlowGroupView<'_>, expected: &fixture::ExpectedGroup) {
    let flags = expected.flags_present.then(|| {
        expected
            .members
            .iter()
            .map(|member| member.flag.unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(actual.flags().map(Iterator::collect::<Vec<_>>), flags);
}

fn assert_payload_coverage(record: &FlowGroupedRecord, case: &fixture::GroupCase) {
    let payload = record.source().records().nth(case.ordinal).unwrap();
    assert_eq!(record.coverage().stored_bytes, payload.payload.len());
    assert_eq!(record.diagnostics().len(), 3);
}

fn expected_auxiliary(group: &fixture::ExpectedGroup, value: i32) -> ExpectedField {
    (
        group.start + 4,
        FlowGroupValue::Auxiliary(group.auxiliary.unwrap()),
        FlowGroupValue::Auxiliary(value),
        None,
    )
}

fn values_start(version: i32, group: &fixture::ExpectedGroup) -> usize {
    group.start + 4 + usize::from(version == 4) * 4
}

fn expected_word(
    version: i32,
    group: &fixture::ExpectedGroup,
    member: usize,
    value: u32,
) -> ExpectedField {
    let offset = values_start(version, group)
        + if version == 6 {
            member * 13
        } else {
            member * 4
        };
    (
        offset,
        FlowGroupValue::Word(group.members[member].word),
        FlowGroupValue::Word(value),
        Some(member),
    )
}

fn expected_wide(
    version: i32,
    group: &fixture::ExpectedGroup,
    member: usize,
    value: u64,
) -> ExpectedField {
    let offset = values_start(version, group)
        + if version == 6 {
            member * 13 + 4
        } else {
            group.members.len() * 4 + member * 8
        };
    (
        offset,
        FlowGroupValue::WideWord(group.members[member].wide),
        FlowGroupValue::WideWord(value),
        Some(member),
    )
}

fn expected_flag(
    version: i32,
    group: &fixture::ExpectedGroup,
    member: usize,
    value: u8,
) -> ExpectedField {
    let offset = values_start(version, group)
        + if version == 6 {
            member * 13 + 12
        } else {
            group.members.len() * 12 + member
        };
    (
        offset,
        FlowGroupValue::Flag(group.members[member].flag.unwrap()),
        FlowGroupValue::Flag(value),
        Some(member),
    )
}

fn assert_source_links(edit: &FlowGroupEdit, source: &FlowGroupedRecord) {
    assert_eq!(
        edit.source().source().as_bytes(),
        source.source().as_bytes()
    );
    assert_eq!(
        edit.carrier_edit().source().as_bytes(),
        source.source().as_bytes()
    );
    assert_eq!(
        edit.carrier_edit().history().len(),
        source.source().records().len()
    );
    assert_eq!(
        edit.diagnostics().len(),
        usize::from(source.compression() == FlowGroupCompression::Zlib)
    );
}

pub(super) fn assert_group(
    actual: &FlowGroupView<'_>,
    expected: &fixture::ExpectedGroup,
    index: usize,
) {
    assert_eq!(actual.index(), index);
    assert_eq!(actual.start(), expected.start);
    assert_eq!(actual.end(), expected.end);
    assert_eq!(actual.len(), expected.members.len());
    assert_eq!(actual.is_empty(), expected.members.is_empty());
    assert_eq!(actual.auxiliary(), expected.auxiliary);
    assert_member_values(actual, expected);
    assert_flags(actual, expected);
}

pub(super) fn assert_coverage(record: &FlowGroupedRecord, case: &fixture::GroupCase) {
    let coverage = record.coverage();
    let mode = if case.compressed {
        FlowGroupCheckMode::ExactZlibGroupedFrames
    } else {
        FlowGroupCheckMode::ExactRawGroupedFrames
    };
    assert_eq!(coverage.mode, mode);
    assert_eq!(record.compression(), options(case.compressed).compression);
    assert_eq!(coverage.decoded_bytes, case.decoded.len());
    assert_eq!(coverage.checked_groups, case.groups.len());
    assert_eq!(
        coverage.checked_members,
        case.groups
            .iter()
            .map(|group| group.members.len())
            .sum::<usize>()
    );
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_payload_coverage(record, case);
}

pub(super) fn requests(group: &fixture::ExpectedGroup) -> Vec<FlowGroupFieldEdit> {
    let mut selected = Vec::new();
    if let Some(value) = group.auxiliary {
        selected.push(FlowGroupFieldEdit::Auxiliary(value));
        selected.push(FlowGroupFieldEdit::Auxiliary(value.wrapping_add(0x1020304)));
    }
    for (member, expected) in group.members.iter().enumerate() {
        selected.extend(member_requests(member, expected));
    }
    selected
}

fn member_requests(member: usize, expected: &fixture::ExpectedMember) -> Vec<FlowGroupFieldEdit> {
    let mut selected = vec![
        FlowGroupFieldEdit::Word {
            member,
            value: expected.word,
        },
        FlowGroupFieldEdit::Word {
            member,
            value: expected.word.wrapping_add(0x1020304),
        },
        FlowGroupFieldEdit::WideWord {
            member,
            value: expected.wide,
        },
        FlowGroupFieldEdit::WideWord {
            member,
            value: expected.wide ^ 0x8fedcba987654321,
        },
    ];
    append_flags(&mut selected, member, expected.flag);
    selected
}

fn append_flags(selected: &mut Vec<FlowGroupFieldEdit>, member: usize, flag: Option<u8>) {
    if let Some(value) = flag {
        selected.push(FlowGroupFieldEdit::Flag { member, value });
        selected.push(FlowGroupFieldEdit::Flag {
            member,
            value: value ^ 0xa5,
        });
    }
}

pub(super) fn assert_edit(
    record: &FlowGroupedRecord,
    case: &fixture::GroupCase,
    group: usize,
    request: FlowGroupFieldEdit,
) {
    let (offset, before, after, member) =
        expected_field(case.version, &case.groups[group], request);
    let edit = record.replace_field(group, request).unwrap();
    let mut expected = case.decoded.clone();
    let bytes = encoded_value(after);
    expected[offset..offset + bytes.len()].copy_from_slice(&bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_history(&edit, record, group, member, before, after);
    assert_eq!(untouched(record), untouched(edit.result()));
    let reversed = edit
        .result()
        .replace_field(group, replacement(before, member))
        .unwrap();
    assert_eq!(reversed.result().decoded_bytes(), case.decoded);
    if !case.compressed || before == after {
        assert_eq!(reversed.result().source().as_bytes(), case.wire);
    }
}

fn expected_field(
    version: i32,
    group: &fixture::ExpectedGroup,
    request: FlowGroupFieldEdit,
) -> ExpectedField {
    match request {
        FlowGroupFieldEdit::Auxiliary(value) => expected_auxiliary(group, value),
        FlowGroupFieldEdit::Word { member, value } => expected_word(version, group, member, value),
        FlowGroupFieldEdit::WideWord { member, value } => {
            expected_wide(version, group, member, value)
        }
        FlowGroupFieldEdit::Flag { member, value } => expected_flag(version, group, member, value),
    }
}

fn encoded_value(value: FlowGroupValue) -> Vec<u8> {
    match value {
        FlowGroupValue::Word(value) => value.to_le_bytes().to_vec(),
        FlowGroupValue::WideWord(value) => value.to_le_bytes().to_vec(),
        FlowGroupValue::Flag(value) => vec![value],
        FlowGroupValue::Auxiliary(value) => value.to_le_bytes().to_vec(),
    }
}

fn replacement(value: FlowGroupValue, member: Option<usize>) -> FlowGroupFieldEdit {
    match value {
        FlowGroupValue::Word(value) => FlowGroupFieldEdit::Word {
            member: member.unwrap(),
            value,
        },
        FlowGroupValue::WideWord(value) => FlowGroupFieldEdit::WideWord {
            member: member.unwrap(),
            value,
        },
        FlowGroupValue::Flag(value) => FlowGroupFieldEdit::Flag {
            member: member.unwrap(),
            value,
        },
        FlowGroupValue::Auxiliary(value) => FlowGroupFieldEdit::Auxiliary(value),
    }
}

fn assert_history(
    edit: &FlowGroupEdit,
    source: &FlowGroupedRecord,
    group: usize,
    member: Option<usize>,
    before: FlowGroupValue,
    after: FlowGroupValue,
) {
    let history = edit.history();
    assert_eq!(&history.source_record, source.record());
    assert_eq!(&history.result_record, edit.result().record());
    assert_eq!(
        (history.group, history.member, history.before, history.after),
        (group, member, before, after)
    );
    let expected = if before == after {
        FlowPayloadRelation::Copied
    } else {
        FlowPayloadRelation::Derived
    };
    assert_eq!(history.relation, expected);
    assert_source_links(edit, source);
}

fn untouched(record: &FlowGroupedRecord) -> (&[u8], &[u8]) {
    let selected = record
        .source()
        .records()
        .nth(record.record().ordinal())
        .unwrap();
    let wire = record.source().as_bytes();
    (
        &wire[..selected.header_offset + 4],
        &wire[selected.header_offset + 12 + selected.payload.len()..],
    )
}
