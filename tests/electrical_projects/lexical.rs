use super::*;
use std::ops::Range;

const INI_DIAGNOSTICS: [ElectricalIniDiagnostic; 2] = [
    ElectricalIniDiagnostic::StoredValuesNotApplicationValidated,
    ElectricalIniDiagnostic::NativeLoadNotChecked,
];

#[test]
fn every_native_and_owned_ini_view_matches_all_frozen_lexical_records() {
    for case in cases() {
        let archive = parse(&case.raw);
        for (member, expected) in archive.members().zip(&case.members) {
            match &expected.ini {
                Ini::Unselected => assert!(archive.archive_ini(&member.id).is_err()),
                Ini::Opaque => assert!(archive.archive_ini(&member.id).unwrap().is_none()),
                Ini::Records(records) => {
                    let view = archive.archive_ini(&member.id).unwrap().unwrap();
                    assert_records(&view, &expected.payload, records);
                }
            }
        }
    }
}

fn optional_span(bytes: &[u8], range: Option<Range<usize>>) -> Option<&[u8]> {
    range.map(|range| &bytes[range])
}

fn assert_records(view: &ElectricalIniDocument, payload: &[u8], expected: &[IniRecord]) {
    assert_eq!(view.bytes(), payload);
    assert_eq!(view.to_bytes(), payload);
    assert_eq!(view.records().len(), expected.len());
    for (actual, expected) in view.records().iter().zip(expected) {
        assert_record(view, actual, expected);
    }
    assert_lexical_coverage(view, payload.len(), expected);
}

fn assert_record(view: &ElectricalIniDocument, actual: &ElectricalIniRecord, expected: &IniRecord) {
    assert_eq!(actual.kind(), expected.kind);
    assert_eq!(&view.bytes()[actual.raw_range()], expected.raw);
    assert_eq!(
        optional_span(view.bytes(), actual.section_range()),
        expected.section.as_deref()
    );
    assert_eq!(
        optional_span(view.bytes(), actual.key_range()),
        expected.key.as_deref()
    );
    assert_eq!(
        optional_span(view.bytes(), actual.value_range()),
        expected.value.as_deref()
    );
}

fn assert_lexical_coverage(view: &ElectricalIniDocument, length: usize, expected: &[IniRecord]) {
    let coverage = view.coverage();
    assert_eq!(
        coverage.mode,
        ElectricalIniCheckMode::ExactAsciiLexicalRecords
    );
    assert_eq!(coverage.buffer_bytes, length);
    assert_eq!(coverage.records, expected.len());
    assert_eq!(
        coverage.fields,
        expected
            .iter()
            .filter(|record| record.kind == ElectricalIniKind::Field)
            .count()
    );
    assert_eq!(coverage.achieved_byte_error, 0);
}

#[test]
fn all_frozen_field_edits_match_independent_intervals_reopen_and_reverse() {
    let mut checked = 0;
    for case in cases() {
        let archive = parse(&case.raw);
        for (index, member) in case.members.iter().enumerate() {
            if let Ini::Records(records) = &member.ini {
                checked += check_ini_fields(&archive, &case, index, records);
            }
        }
    }
    assert_eq!(checked, 223);
}

fn check_ini_fields(
    archive: &ElectricalArchive,
    case: &Case,
    member: usize,
    records: &[IniRecord],
) -> usize {
    let mut checked = 0;
    for (ordinal, record) in records.iter().enumerate() {
        if let Some(value) = &record.value {
            check_field_edit(archive, case, member, records, ordinal, value);
            checked += 1;
        }
    }
    checked
}

fn check_field_edit(
    source: &ElectricalArchive,
    case: &Case,
    member: usize,
    records: &[IniRecord],
    ordinal: usize,
    value: &[u8],
) {
    let id = source.members().nth(member).unwrap().id;
    check_ini_carrier_noop(source, &id, ordinal, value, &case.raw);
    let range = independent_value_range(records, ordinal);
    let mut replacement = vec![b'@'];
    replacement.extend_from_slice(value);
    let expected = replaced_value(&case.members[member].payload, range.clone(), &replacement);
    let edit = source.edit_ini_value(&id, ordinal, &replacement).unwrap();
    assert_eq!(edit.completion(), ElectricalEditCompletion::Changed);
    carrier::check_member_history(&edit, member);
    let lexical = edit.ini_edit().unwrap();
    assert_eq!(lexical.result().to_bytes(), expected);
    check_byte_history(lexical, &range, replacement.len());
    carrier::check_encoded_records(
        &case.raw,
        &edit.result().to_bytes(),
        case.comment.len(),
        member,
    );
    check_fresh_reversal(&edit, member, ordinal, value, &case.members[member].payload);
    assert_eq!(source.to_bytes(), case.raw);
}

fn check_ini_carrier_noop(
    source: &ElectricalArchive,
    id: &ElectricalMemberId,
    ordinal: usize,
    value: &[u8],
    raw: &[u8],
) {
    let edit = source.edit_ini_value(id, ordinal, value).unwrap();
    assert_eq!(edit.completion(), ElectricalEditCompletion::Unchanged);
    assert_eq!(edit.result().to_bytes(), raw);
    assert!(
        edit.history()
            .iter()
            .all(|row| row.relation == ElectricalMemberRelation::Copied)
    );
    assert!(
        edit.ini_edit()
            .unwrap()
            .history()
            .iter()
            .all(|row| row.relation == ElectricalByteRelation::Copied)
    );
}

fn replaced_value(raw: &[u8], range: Range<usize>, replacement: &[u8]) -> Vec<u8> {
    let mut output = raw[..range.start].to_vec();
    output.extend_from_slice(replacement);
    output.extend_from_slice(&raw[range.end..]);
    output
}

fn check_fresh_reversal(
    edit: &ElectricalArchiveEdit,
    member: usize,
    ordinal: usize,
    original: &[u8],
    raw: &[u8],
) {
    let fresh = parse(&edit.result().to_bytes());
    let id = fresh.members().nth(member).unwrap().id;
    let reverse = fresh.edit_ini_value(&id, ordinal, original).unwrap();
    let new_id = reverse.result().members().nth(member).unwrap().id;
    assert_eq!(reverse.result().read_member(&new_id).unwrap(), raw);
    assert_eq!(reverse.ini_edit().unwrap().result().to_bytes(), raw);
}

fn check_byte_history(edit: &ElectricalIniEdit, old: &Range<usize>, length: usize) {
    let before = edit.source().bytes();
    let after = edit.result().bytes();
    assert_ini_edit_coverage(edit, after.len());
    let parents = [0..old.start, old.clone(), old.end..before.len()];
    let results = [
        0..old.start,
        old.start..old.start + length,
        old.start + length..after.len(),
    ];
    for (index, span) in edit.history().iter().enumerate() {
        assert_eq!(span.parent, parents[index]);
        assert_eq!(span.result, results[index]);
        if index != 1 {
            assert_eq!(span.relation, ElectricalByteRelation::Copied);
            assert_eq!(before[span.parent.clone()], after[span.result.clone()]);
        } else {
            assert_eq!(span.relation, ElectricalByteRelation::Derived);
        }
    }
    assert_eq!(edit.diagnostics(), &INI_DIAGNOSTICS);
}

fn assert_ini_edit_coverage(edit: &ElectricalIniEdit, length: usize) {
    assert_eq!(edit.coverage().buffer_bytes, length);
    assert_eq!(edit.coverage().records, edit.source().coverage().records);
    assert_eq!(edit.coverage().fields, edit.source().coverage().fields);
    assert_eq!(
        edit.coverage().mode,
        ElectricalIniCheckMode::ExactAsciiLexicalRecords
    );
    assert_eq!(edit.coverage().achieved_byte_error, 0);
}

#[test]
fn lexical_deletion_noops_empty_fields_and_every_ascii_delimiter_are_retained() {
    let raw = b"[S]\r\r\nK=first\nK=\rK=A\x0bK=B\x0cK=C\x1cK=D\x1dK=E\x1e";
    let source = ElectricalIniDocument::from_bytes(raw, ini_limits())
        .unwrap()
        .unwrap();
    let ordinal = source
        .records()
        .iter()
        .position(|record| record.kind() == ElectricalIniKind::Field)
        .unwrap();
    let deleted = source.edit_value(ordinal, &[]).unwrap();
    assert_eq!(
        deleted.result().bytes(),
        b"[S]\r\r\nK=\nK=\rK=A\x0bK=B\x0cK=C\x1cK=D\x1dK=E\x1e"
    );
    assert_lexical_noop(&source, ordinal);
    assert_eq!(source.bytes(), raw);
    assert_empty_and_opaque_lexical();
}

fn assert_lexical_noop(source: &ElectricalIniDocument, ordinal: usize) {
    let noop = source.edit_value(ordinal, b"first").unwrap();
    assert_eq!(noop.completion(), ElectricalEditCompletion::Unchanged);
    assert_eq!(noop.result(), source);
    assert!(
        noop.history()
            .iter()
            .all(|span| span.relation == ElectricalByteRelation::Copied)
    );
}

fn assert_empty_and_opaque_lexical() {
    assert_eq!(
        ElectricalIniDocument::from_bytes(b"", ini_limits())
            .unwrap()
            .unwrap()
            .records()
            .len(),
        0
    );
    assert!(
        ElectricalIniDocument::from_bytes(b"\xff", ini_limits())
            .unwrap()
            .is_none()
    );
}

#[test]
fn invalid_value_shapes_missing_fields_and_non_ascii_fail_without_source_mutation() {
    let raw = b"[S]\nK=source\n#comment\n";
    let source = ElectricalIniDocument::from_bytes(raw, ini_limits())
        .unwrap()
        .unwrap();
    for replacement in [b"\xff".as_slice(), b"value\nNew=injected", b"value\r\n"] {
        assert!(source.edit_value(1, replacement).is_err());
    }
    assert!(source.edit_value(0, b"section").is_err());
    assert!(source.edit_value(2, b"comment").is_err());
    assert_eq!(
        source.edit_value(usize::MAX, &[]).unwrap_err().class,
        FailureClass::Missing
    );
    assert_eq!(source.bytes(), raw);
}

#[test]
fn lexical_snapshots_isolate_inputs_and_eight_concurrent_field_requests() {
    let mut raw = b"[S]\r\nK=value\r\n".to_vec();
    let source = ElectricalIniDocument::from_bytes(&raw, ini_limits())
        .unwrap()
        .unwrap();
    let expected = concurrent_lexical_bytes(&source);
    raw.fill(0);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| concurrent_lexical_bytes(&source)))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
    assert_eq!(source.bytes(), b"[S]\r\nK=value\r\n");
}

fn concurrent_lexical_bytes(source: &ElectricalIniDocument) -> Vec<u8> {
    source
        .edit_value(1, b"longer=value")
        .unwrap()
        .result()
        .to_bytes()
}
