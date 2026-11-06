use super::*;

#[test]
fn every_frozen_field_interval_raw_word_and_mapping_context_matches_complete_typed_view() {
    for case in cases() {
        let view = read_case(&case);
        check_headers(&view, &case);
        assert_eq!(view.groups().len(), 2);
        for group in &case.groups {
            check_group(
                view.group(group.ordinal as usize).unwrap(),
                group,
                case.version,
            );
        }
        assert!(view.group(2).is_none());
        check_coverage(&view, &case);
    }
}

fn check_group(actual: FlowReferenceGroup<'_>, expected: &ReferenceGroup, version: i32) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.entry_count(), expected.entries.len());
    assert_eq!(actual.entries().len(), expected.entries.len());
    for (ordinal, entry) in expected.entries.iter().enumerate() {
        let actual_entry = actual.entry(ordinal).unwrap();
        assert_eq!(actual_entry.path(), (actual.ordinal(), ordinal));
        assert_eq!(
            actual_entry.payload_interval(),
            (entry.start as usize, entry.end as usize)
        );
        slots::check_entry(actual_entry, entry, version);
    }
    assert!(actual.entry(expected.entries.len()).is_none());
    check_group_arrays(actual, expected);
}

fn check_group_arrays(actual: FlowReferenceGroup<'_>, expected: &ReferenceGroup) {
    assert_eq!(actual.refinement_level(), expected.level);
    assert_eq!(actual.mapping_words().is_some(), expected.level.is_some());
    if let Some(array) = actual.mapping_words() {
        assert_eq!(array.word_count(), expected.mapping.len());
        for (index, value) in expected.mapping.iter().enumerate() {
            assert_eq!(array.raw_word(index), Some(*value as u32));
        }
        assert_eq!(array.raw_word(expected.mapping.len()), None);
    }
    assert_eq!(actual.wide_words().is_some(), !expected.wide.is_empty());
    if let Some(array) = actual.wide_words() {
        assert_eq!(array.word_count(), expected.wide.len());
        for (index, bits) in expected.wide.iter().enumerate() {
            assert_eq!(array.raw_bits(index), Some(*bits));
        }
        assert_eq!(array.raw_bits(expected.wide.len()), None);
    }
}

fn check_coverage(view: &FlowReferenceGroups, case: &ReferenceCase) {
    let coverage = view.coverage();
    assert_eq!(coverage.stored_bytes, case.frame.stored.len());
    assert_eq!(coverage.decoded_bytes, case.frame.decoded.len());
    assert_eq!(coverage.achieved_error, 0);
    let totals = expected_totals(case);
    assert_eq!(
        [
            coverage.entries,
            coverage.reference_words,
            coverage.first_words,
            coverage.second_words,
            coverage.mapping_words,
            coverage.wide_words
        ],
        totals
    );
    check_diagnostics(view);
    check_mode(coverage.mode, case.frame.compressed);
}

fn expected_totals(case: &ReferenceCase) -> [usize; 6] {
    let mut totals = [0; 6];
    for group in &case.groups {
        totals[0] += group.entries.len();
        totals[4] += group.mapping.len();
        totals[5] += group.wide.len();
        for entry in &group.entries {
            totals[1] += entry.references.len();
            totals[2] += entry.first.len();
            totals[3] += entry.second.len();
        }
    }
    totals
}

fn check_headers(view: &FlowReferenceGroups, case: &ReferenceCase) {
    assert_eq!(view.version(), case.version);
    assert_eq!(view.decoded_bytes(), case.frame.decoded);
    assert_eq!(
        view.header(FlowReferenceHeaderField::Discarded),
        case.discarded_header
    );
    assert_eq!(
        view.header(FlowReferenceHeaderField::Stored),
        case.stored_header
    );
    assert_eq!(view.native_header_default(), case.native_header_default);
    assert_eq!(
        view.options().first_group_mapping_enabled,
        case.frame.mapping
    );
}

fn check_diagnostics(view: &FlowReferenceGroups) {
    assert_eq!(view.record().ordinal(), 1);
    assert_eq!(view.source().records().len(), 3);
    assert_eq!(view.diagnostics().len(), 7);
    assert!(
        view.diagnostics()
            .contains(&FlowReferenceGroupDiagnostic::ReferenceFixupsNotReproduced)
    );
    assert!(
        view.diagnostics()
            .contains(&FlowReferenceGroupDiagnostic::RefinementPostprocessingNotReproduced)
    );
}

fn check_mode(actual: FlowLegacyTableCheckMode, compressed: bool) {
    assert_eq!(
        actual,
        if compressed {
            FlowLegacyTableCheckMode::ExactZlibTable
        } else {
            FlowLegacyTableCheckMode::ExactRawTable
        }
    );
}
