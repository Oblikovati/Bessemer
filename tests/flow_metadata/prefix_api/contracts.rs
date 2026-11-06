use super::*;
#[test]
fn every_small_known_prefix_and_all_missing_terminators_fail_without_consuming_opaque_tail() {
    for case in fixture::prefix_cases() {
        if case.known <= 4096 {
            for length in 0..case.known as usize {
                assert!(open(&case.raw[..length], options()).is_err());
            }
        }
        for object in &case.objects {
            check_terminators(&case.raw, object);
        }
        let view = open(&case.raw[..case.known as usize], options()).unwrap();
        assert!(view.opaque_tail().is_empty());
        assert!(!view.coverage().full_payload_schema);
    }
}
fn check_terminators(raw: &[u8], object: &PrefixObject) {
    for text in [
        Some(&object.first),
        Some(&object.second),
        Some(&object.third),
        object.fourth.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        if text.units == 0 {
            continue;
        }
        let mut changed = raw.to_vec();
        changed[text.end as usize - 2] = 1;
        assert!(open(&changed, options()).is_err());
    }
}
#[test]
fn all_independent_limits_accept_exact_counts_and_reject_one_below_and_zero() {
    for case in fixture::prefix_cases() {
        let counts = counts(&case);
        let exact = limited(counts.map(|value| value.max(1)));
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..5 {
            let mut zero = counts.map(|value| value.max(1));
            zero[axis] = 0;
            assert!(open(&case.raw, limited(zero)).is_err());
            if counts[axis] == 0 {
                continue;
            }
            let mut below = counts.map(|value| value.max(1));
            below[axis] -= 1;
            assert!(open(&case.raw, limited(below)).is_err());
        }
    }
}
fn counts(case: &PrefixCase) -> [usize; 5] {
    let values = case.vectors.iter().map(|vector| vector.values.len()).sum();
    let units = case.objects.iter().map(object_units).sum();
    [
        case.raw.len(),
        case.vectors.len(),
        case.objects.len(),
        values,
        units,
    ]
}
fn limited(counts: [usize; 5]) -> FlowMetadataPrefixOptions {
    FlowMetadataPrefixOptions {
        maximum_payload_bytes: counts[0],
        maximum_vectors: counts[1],
        maximum_objects: counts[2],
        maximum_values: counts[3],
        maximum_string_units: counts[4],
        ..options()
    }
}
#[test]
fn wrong_foreign_stale_indices_absent_strings_and_structural_version_changes_are_rejected() {
    let case = fixture::prefix_cases().remove(0);
    let view = open(&case.raw, options()).unwrap();
    let wrong = carrier(&case.raw, 423);
    let id = wrong.latest(423).unwrap().unwrap().id;
    assert!(wrong.record_424_metadata_prefix(&id, options()).is_err());
    assert!(
        wrong
            .record_424_metadata_prefix(view.record(), options())
            .is_err()
    );
    check_invalid_edits(&view);
    let changed = view.replace_object_state(0, 255).unwrap();
    assert!(
        changed
            .result()
            .source()
            .record_424_metadata_prefix(view.record(), options())
            .is_err()
    );
    assert_eq!(view.payload_bytes(), case.raw);
}
#[test]
fn inactive_versions_can_change_inside_the_same_signed_branch_and_inputs_are_isolated() {
    let mut raw = vec![128, 255, 0, 1];
    let view = open(&raw, options()).unwrap();
    raw.fill(0);
    let edit = view.replace_version_byte(255).unwrap();
    assert_eq!(view.payload_bytes(), [128, 255, 0, 1]);
    assert_eq!(edit.result().payload_bytes(), [255, 255, 0, 1]);
    assert_eq!(edit.result().opaque_tail(), [255, 0, 1]);
    assert!(view.replace_version_byte(2).is_err());
    let mut negative = vec![2];
    negative.extend_from_slice(&(-1i32).to_le_bytes());
    assert!(open(&negative, options()).is_err());
    let mut impossible = vec![2];
    impossible.extend_from_slice(&i32::MAX.to_le_bytes());
    assert!(open(&impossible, options()).is_err());
}
#[test]
fn all_public_prefix_types_are_send_sync_and_eight_workers_match_complete_transcripts() {
    assert_send_sync::<FlowMetadataStringMode>();
    assert_send_sync::<FlowMetadataPrefixOptions>();
    assert_send_sync::<FlowMetadataStringRole>();
    assert_send_sync::<FlowMetadataPrefixCoverage>();
    assert_send_sync::<FlowMetadataPrefixStop>();
    assert_send_sync::<FlowMetadataPrefixDiagnostic>();
    assert_send_sync::<FlowMetadataPrefix>();
    assert_send_sync::<FlowMetadataVector<'static>>();
    assert_send_sync::<FlowMetadataObject<'static>>();
    assert_send_sync::<FlowMetadataText<'static>>();
    assert_send_sync::<FlowMetadataPrefixEdit>();
    let cases = fixture::prefix_cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn assert_send_sync<T: Send + Sync>() {}
fn transcript(cases: &[PrefixCase]) -> Vec<u8> {
    let mut output = Vec::new();
    for case in cases {
        let view = open(&case.raw, options()).unwrap();
        output.extend_from_slice(&(view.coverage().known_prefix_bytes as u64).to_le_bytes());
        for object in view.objects() {
            output.extend_from_slice(object.native_fourth_text_bytes());
            output.push(object.state_byte());
            let edit = view
                .replace_object_state(object.ordinal(), object.state_byte() ^ 255)
                .unwrap();
            output.extend_from_slice(edit.result().source().as_bytes());
        }
        output.extend_from_slice(view.opaque_tail());
    }
    output
}

fn object_units(object: &PrefixObject) -> usize {
    [
        Some(&object.first),
        Some(&object.second),
        Some(&object.third),
        object.fourth.as_ref(),
    ]
    .into_iter()
    .flatten()
    .map(|text| text.units as usize)
    .sum()
}

fn check_invalid_edits(view: &FlowMetadataPrefix) {
    assert!(view.replace_vector_value(usize::MAX, 0, 0).is_err());
    assert!(view.replace_member_c4(usize::MAX, 0).is_err());
    assert!(view.replace_object_state(usize::MAX, 0).is_err());
    assert!(
        view.replace_string_unit(0, FlowMetadataStringRole::Fourth, 0, 0)
            .is_err()
    );
    assert!(
        view.replace_string_unit(0, FlowMetadataStringRole::First, usize::MAX, 0)
            .is_err()
    );
    assert!(view.replace_version_byte(1).is_err());
}

#[test]
fn string_edits_recompute_first_nul_and_native_fallback_and_reject_terminal_changes() {
    let case = fixture::prefix_cases().remove(0);
    let view = open(&case.raw, options()).unwrap();
    let text = view
        .object(0)
        .unwrap()
        .string(FlowMetadataStringRole::First)
        .unwrap();
    let terminated = view
        .replace_string_unit(0, FlowMetadataStringRole::First, 0, 0)
        .unwrap();
    let object = terminated.result().object(0).unwrap();
    assert!(object.native_fourth_text_bytes().is_empty());
    assert!(object.fourth_text_is_derived());
    assert!(
        view.replace_string_unit(0, FlowMetadataStringRole::First, text.stored_units() - 1, 1)
            .is_err()
    );
    check_surrogate_edit(&view);
    assert_eq!(view.payload_bytes(), case.raw);
}
#[test]
fn version_changes_between_active_layouts_require_identical_stored_string_presence() {
    let empty = open(&[3, 0, 0, 0, 0, 0, 0, 0, 0], options()).unwrap();
    let changed = empty.replace_version_byte(127).unwrap();
    assert_eq!(changed.result().version_byte(), 127);
    assert_eq!(
        changed.result().stop(),
        FlowMetadataPrefixStop::RequiresBackend
    );
    assert_eq!(changed.result().coverage().known_prefix_bytes, 9);
    let case = fixture::prefix_cases().remove(0);
    assert!(
        open(&case.raw, options())
            .unwrap()
            .replace_version_byte(3)
            .is_err()
    );
}

fn check_surrogate_edit(view: &FlowMetadataPrefix) {
    let surrogate = view
        .replace_string_unit(0, FlowMetadataStringRole::First, 0, 0xd800)
        .unwrap();
    assert_eq!(
        &surrogate
            .result()
            .object(0)
            .unwrap()
            .native_fourth_text_bytes()[..2],
        &[0, 0xd8]
    );
}
