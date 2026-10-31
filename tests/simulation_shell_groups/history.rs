use super::*;

const EXPECTED_HISTORY: &[u8] =
    include_bytes!("../fixtures/simulation-shell-manager/group-history.bin");

struct GroupEditCase<'a> {
    profile: SimulationShellWireProfile,
    version: SimulationPersistVersion,
    ordinal: usize,
    replacement: SimulationShellGroupValue,
    replacement_wire: &'a [u8],
    changed: bool,
    original: &'a [u8],
    expected: &'a [u8],
}

fn read_replacement<'a>(cursor: &mut &'a [u8]) -> (SimulationShellGroupValue, &'a [u8]) {
    let tag = read_word(cursor);
    let wire = projection::framed_bytes(cursor);
    let value = match tag {
        0 => SimulationShellGroupValue::Signed(i32::from_le_bytes(wire.try_into().unwrap())),
        2 => SimulationShellGroupValue::DoubleBits(u64::from_le_bytes(wire.try_into().unwrap())),
        3 => {
            let encoding = if read_word(cursor) == 1 {
                TextEncoding::Utf16
            } else {
                TextEncoding::Latin1
            };
            let text = std::str::from_utf8(projection::framed_bytes(cursor)).unwrap();
            SimulationShellGroupValue::Text(ArchiveString::new(text, encoding).unwrap())
        }
        _ => panic!("replacement tag {tag}; expected Signed/DoubleBits/MFC CString"),
    };
    (value, wire)
}

fn read_case<'a>(cursor: &mut &'a [u8]) -> GroupEditCase<'a> {
    let profile = PROFILES[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let ordinal = read_word(cursor) as usize;
    let (replacement, replacement_wire) = read_replacement(cursor);
    let changed = read_word(cursor) != 0;
    let original_size = read_word(cursor) as usize;
    let expected_size = read_word(cursor) as usize;
    let original = take_projection(cursor, original_size);
    let expected = take_projection(cursor, expected_size);
    GroupEditCase {
        profile,
        version,
        ordinal,
        replacement,
        replacement_wire,
        changed,
        original,
        expected,
    }
}

fn make_edit(case: &GroupEditCase<'_>) -> SimulationShellGroupEdit {
    let group = SimulationShellGroup::from_bytes(case.original, case.version, case.profile, LIMITS)
        .unwrap();
    let parent = group.field_id(case.ordinal).unwrap();
    let edit = group.edit_field(&parent, case.replacement.clone()).unwrap();
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.replacement(), &case.replacement);
    assert_eq!(edit.parent(), parent);
    assert_eq!(
        edit.result(),
        edit.archive().field_id(case.ordinal).unwrap()
    );
    let (start, end) = edit.archive().field_offsets()[case.ordinal];
    assert_eq!(
        &edit.archive().to_bytes()[start..end],
        case.replacement_wire
    );
    assert_eq!(edit.result() != parent, case.changed);
    assert_edit_scope(&edit, case);
    edit
}

fn assert_edit_scope(edit: &SimulationShellGroupEdit, case: &GroupEditCase<'_>) {
    let coverage = edit.coverage();
    let expected = if case.changed {
        SimulationShellGroupCompletion::Changed
    } else {
        SimulationShellGroupCompletion::Unchanged
    };
    assert_eq!(coverage.completion, expected);
    assert_eq!(
        coverage.check_mode,
        SimulationShellGroupCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(coverage.version, case.version);
    assert_eq!(coverage.wire_profile, case.profile);
    assert_eq!(coverage.checked_bytes, case.expected.len());
    assert_eq!(coverage.checked_fields, edit.archive().fields().len());
    assert_eq!(coverage.text_bytes, edit.archive().coverage().text_bytes);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
}

fn project_case_prefix(output: &mut Vec<u8>, case: &GroupEditCase<'_>) {
    project_word(
        output,
        usize::from(case.profile == SimulationShellWireProfile::Writer),
    );
    project_word(output, case.version.value() as usize);
    project_word(output, case.ordinal);
    project_replacement_value(output, case);
    project_word(output, usize::from(case.changed));
    project_word(output, case.original.len());
    project_word(output, case.expected.len());
}

fn project_replacement_value(output: &mut Vec<u8>, case: &GroupEditCase<'_>) {
    project_word(
        output,
        match case.replacement {
            SimulationShellGroupValue::Signed(_) => 0,
            SimulationShellGroupValue::DoubleBits(_) => 2,
            SimulationShellGroupValue::Text(_) => 3,
        },
    );
    project_word(output, case.replacement_wire.len());
    output.extend_from_slice(case.replacement_wire);
    if let SimulationShellGroupValue::Text(text) = &case.replacement {
        project_word(output, usize::from(text.encoding() == TextEncoding::Utf16));
        project_word(output, text.text().len());
        output.extend_from_slice(text.text().as_bytes());
    }
}

fn project_histories(cursor: &mut &[u8], output: &mut Vec<u8>, edit: &SimulationShellGroupEdit) {
    let source_ids: Vec<_> = (0..edit.source().fields().len())
        .map(|index| edit.source().field_id(index).unwrap())
        .collect();
    let result_ids: Vec<_> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    let mut history = Vec::new();
    load_projection::project_outer_history(&mut history, edit.history(), &source_ids, &result_ids);
    assert_eq!(take_projection(cursor, history.len()), history);
    output.extend_from_slice(&history);
    assert_eq!(read_word(cursor), 0);
    project_word(output, 0);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let case = read_case(cursor);
    let edit = make_edit(&case);
    project_case_prefix(output, &case);
    output.extend_from_slice(&edit.source().to_bytes());
    output.extend_from_slice(&edit.archive().to_bytes());
    projection::project_fields(output, edit.archive());
    projection::consume_fields(cursor);
    project_histories(cursor, output, &edit);
}

#[test]
fn every_owned_signed_double_text_and_prefix_edit_matches_complete_independent_fields_and_parents()
{
    project_packet(EXPECTED_HISTORY, 312, project_case);
}

#[test]
#[ignore = "requires complete independent native group no-op field/history expectations"]
fn every_native_empty_group_noop_matches_complete_history_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_GROUP_NATIVE_HISTORY_INPUT")
        .expect("set complete native group histories");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(EXPECTED_HISTORY, 312, project_case);
    packet.extend(project_packet(&native, 79, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_GROUP_HISTORY_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
