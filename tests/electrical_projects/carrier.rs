use super::*;

#[test]
fn every_complete_native_and_owned_carrier_reconstructs_and_matches_all_frozen_members() {
    let all = cases();
    assert_eq!(all.len(), 8);
    for case in all {
        let archive = parse(&case.raw);
        assert_eq!(archive.to_bytes(), case.raw, "{}", case.name);
        assert_eq!(archive.comment(), case.comment);
        assert_members(&archive, &case);
        assert_coverage(&archive, &case);
    }
}

fn assert_coverage(archive: &ElectricalArchive, case: &Case) {
    let coverage = archive.coverage();
    assert_eq!(coverage.mode, ElectricalCheckMode::ExactZipCarrier);
    assert_eq!(coverage.wire_bytes, case.raw.len());
    assert_eq!(coverage.checked_members, case.members.len());
    assert_eq!(
        coverage.expanded_bytes,
        case.members.iter().map(|member| member.payload.len()).sum()
    );
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(
        archive.diagnostics(),
        &[
            ElectricalDiagnostic::MemberSchemasOutsideScope,
            ElectricalDiagnostic::NativeApplicationNotChecked
        ]
    );
}

fn assert_members(archive: &ElectricalArchive, case: &Case) {
    let offsets = directory_offsets(&case.raw, case.comment.len(), case.members.len());
    let members: Vec<_> = archive.members().collect();
    assert_eq!(members.len(), case.members.len());
    for (index, (actual, expected)) in members.iter().zip(&case.members).enumerate() {
        assert_eq!(actual.name.as_ref(), expected.name);
        assert_eq!(archive.read_member(&actual.id).unwrap(), expected.payload);
        assert_eq!(actual.expanded_bytes as usize, expected.payload.len());
        assert_eq!(actual.stored_bytes, u32_at(&case.raw, offsets[index] + 18));
        assert_eq!(actual.crc32, expected.crc);
        assert_compression(actual.compression, expected.method);
        let metadata = archive.member_metadata(&actual.id).unwrap();
        assert_eq!(metadata.flags, expected.flags);
        assert_eq!(metadata.header_offset, offsets[index]);
        assert_eq!(metadata.extra, expected.extra);
        assert_eq!(metadata.comment, expected.comment);
    }
}

fn assert_compression(actual: ElectricalCompression, expected: u16) {
    let value = match actual {
        ElectricalCompression::Stored => 0,
        ElectricalCompression::Deflated => 8,
    };
    assert_eq!(value, expected);
}

#[test]
fn all_fifty_six_member_edits_reopen_with_exact_contents_and_untouched_encoded_records() {
    let mut edits = 0;
    let mut preserved = 0;
    for case in cases() {
        let source = parse(&case.raw);
        for ordinal in 0..case.members.len() {
            check_member_edit(&source, &case, ordinal);
            edits += 1;
            preserved += case.members.len() - 1;
        }
    }
    assert_eq!((edits, preserved), (56, 566));
}

fn check_member_edit(source: &ElectricalArchive, case: &Case, ordinal: usize) {
    let member = source.members().nth(ordinal).unwrap();
    let mut replacement = case.members[ordinal].payload.clone();
    replacement.push(0xa5);
    let edit = source.edit_member(&member.id, &replacement).unwrap();
    assert_eq!(edit.completion(), ElectricalEditCompletion::Changed);
    assert!(edit.ini_edit().is_none());
    assert_eq!(edit.source().to_bytes(), case.raw);
    assert_eq!(
        edit.coverage().expanded_bytes,
        source.coverage().expanded_bytes + 1
    );
    check_member_history(&edit, ordinal);
    check_encoded_records(
        &case.raw,
        &edit.result().to_bytes(),
        case.comment.len(),
        ordinal,
    );
    check_decoded_reversal(&edit, case, ordinal, &replacement);
    assert_eq!(source.to_bytes(), case.raw);
}

fn check_decoded_reversal(
    edit: &ElectricalArchiveEdit,
    case: &Case,
    ordinal: usize,
    replacement: &[u8],
) {
    let fresh = parse(&edit.result().to_bytes());
    let id = fresh.members().nth(ordinal).unwrap().id;
    assert_eq!(fresh.read_member(&id).unwrap(), replacement);
    let restored = fresh
        .edit_member(&id, &case.members[ordinal].payload)
        .unwrap();
    assert_decoded_members(restored.result(), case);
}

fn assert_decoded_members(archive: &ElectricalArchive, case: &Case) {
    assert_eq!(archive.members().len(), case.members.len());
    for (actual, expected) in archive.members().zip(&case.members) {
        assert_eq!(actual.name.as_ref(), expected.name);
        assert_eq!(archive.read_member(&actual.id).unwrap(), expected.payload);
    }
}

pub(super) fn check_member_history(edit: &ElectricalArchiveEdit, ordinal: usize) {
    assert_eq!(edit.history().len(), edit.source().members().len());
    for (index, history) in edit.history().iter().enumerate() {
        let before = edit.source().members().nth(index).unwrap();
        let after = edit.result().members().nth(index).unwrap();
        assert_eq!((&history.parent, &history.result), (&before.id, &after.id));
        let relation = if index == ordinal {
            ElectricalMemberRelation::Derived
        } else {
            ElectricalMemberRelation::Copied
        };
        assert_eq!(history.relation, relation);
    }
    assert_eq!(edit.diagnostics(), edit.source().diagnostics());
}

pub(super) fn check_encoded_records(before: &[u8], after: &[u8], comment: usize, ordinal: usize) {
    let end = before.len() - 22 - comment;
    let count = usize::from(u16_at(before, end + 10));
    let old_offsets = directory_offsets(before, comment, count);
    let new_offsets = directory_offsets(after, comment, count);
    for index in 0..count {
        if index != ordinal {
            assert_eq!(
                encoded_member(before, old_offsets[index]),
                encoded_member(after, new_offsets[index])
            );
        }
    }
    let old = directory_records(before, comment, count);
    let new = directory_records(after, comment, count);
    for index in 0..count {
        check_central_header(old[index], new[index], index == ordinal);
    }
}

fn check_central_header(before: &[u8], after: &[u8], changed: bool) {
    let mut expected = before.to_vec();
    expected[42..46].copy_from_slice(&after[42..46]);
    if changed {
        expected[16..28].copy_from_slice(&after[16..28]);
    }
    assert_eq!(expected, after);
}

#[test]
fn noops_duplicate_occurrences_and_revision_failures_are_explicit() {
    let case = cases().remove(5);
    let source = parse(&case.raw);
    assert_name_resolution_errors(&source);
    let id = source.members().next().unwrap().id;
    let noop = source.edit_member(&id, &case.members[0].payload).unwrap();
    assert_eq!(noop.completion(), ElectricalEditCompletion::Unchanged);
    assert_eq!(noop.result().to_bytes(), case.raw);
    assert!(
        noop.history()
            .iter()
            .all(|row| row.relation == ElectricalMemberRelation::Copied)
    );
    assert_identity_rejections(&case.raw, &id);
    assert_eq!(source.to_bytes(), case.raw);
}

fn assert_name_resolution_errors(source: &ElectricalArchive) {
    assert_eq!(
        source.member_named("Archive.ini").unwrap_err().class,
        FailureClass::Invalid
    );
    assert_eq!(
        source.member_named("absent").unwrap_err().class,
        FailureClass::Missing
    );
}

fn assert_identity_rejections(raw: &[u8], id: &ElectricalMemberId) {
    let other = parse(raw);
    assert!(other.read_member(id).is_err());
    assert!(other.member_metadata(id).is_err());
    assert!(other.edit_member(id, &[]).is_err());
}

#[test]
fn end_record_like_comment_and_literal_path_names_are_preserved_without_extraction() {
    let all = cases();
    let archive = parse(&all[6].raw);
    assert_eq!(archive.members().len(), 1);
    assert_eq!(archive.comment(), all[6].comment);
    assert!(
        archive
            .comment()
            .windows(4)
            .any(|word| word == b"PK\x05\x06")
    );
    let paths = parse(&all[5].raw);
    assert!(paths.member_named("../outside.bin").is_ok());
    assert!(paths.member_named("C:\\folder\\opaque.bin").is_ok());
    assert_eq!(paths.to_bytes(), all[5].raw);
}

#[test]
fn archive_snapshots_isolate_input_mutation_and_eight_concurrent_member_edits() {
    let mut case = cases().remove(0);
    let source = parse(&case.raw);
    let original = source.to_bytes();
    let id = source.members().next().unwrap().id;
    let expected = independent_member_bytes(&source, &id);
    case.raw.fill(0);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| independent_member_bytes(&source, &id)))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
    assert_eq!(source.to_bytes(), original);
}

fn independent_member_bytes(source: &ElectricalArchive, id: &ElectricalMemberId) -> Vec<u8> {
    source
        .edit_member(id, b"independent")
        .unwrap()
        .result()
        .to_bytes()
}
