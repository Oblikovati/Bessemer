use super::*;
#[test]
fn every_small_and_native_profile_prefix_and_every_appended_byte_is_rejected_as_incomplete_or_invalid()
 {
    for case in profile_fixture::cases() {
        if case.raw.len() <= 256 || case.name.starts_with("native-") {
            for end in 0..case.raw.len() {
                assert!(open(&case.raw[..end], options()).is_err());
            }
        }
        let mut extra = case.raw.clone();
        extra.push(0);
        assert!(open(&extra, options()).is_err());
    }
}
#[test]
fn all_global_profile_limits_accept_exact_totals_and_reject_each_one_below_or_zero() {
    for case in profile_fixture::cases() {
        let counts = totals(&case);
        let exact = limited(counts.map(|n| n.max(1)));
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..5 {
            let mut zero = counts.map(|n| n.max(1));
            zero[axis] = 0;
            assert!(open(&case.raw, limited(zero)).is_err());
            if counts[axis] == 0 {
                continue;
            }
            let mut below = counts.map(|n| n.max(1));
            below[axis] -= 1;
            assert!(open(&case.raw, limited(below)).is_err());
        }
    }
}
fn limited(counts: [usize; 5]) -> FlowMediumProfileOptions {
    FlowMediumProfileOptions {
        maximum_first_objects: counts[0],
        maximum_reference_objects: counts[1],
        maximum_references: counts[2],
        maximum_total_wide_words: counts[3],
        maximum_total_name_units: counts[4],
        ..options()
    }
}
#[test]
fn missing_keys_invalid_indices_wrong_foreign_stale_ids_and_foreign_body_edits_leave_sources_unchanged()
 {
    let case = profile_fixture::cases().remove(0);
    let profile = open(&case.raw, options()).unwrap();
    let missing = FlowGuid::from_bytes([0x55; 16]);
    assert!(profile.replace_reference(0, 0, missing).is_err());
    assert!(profile.replace_outer_reference(0, missing).is_err());
    check_indices(&profile, missing);
    check_identity(&profile, &case.raw);
    let foreign = open(&case.raw, options()).unwrap();
    let local = state_edit(&foreign);
    assert!(profile.apply_body_edit(0, &local).is_err());
    let local = state_edit(&profile);
    assert!(profile.apply_body_edit(1, &local).is_err());
    assert_eq!(profile.payload_bytes(), case.raw);
}
fn check_indices(profile: &FlowMediumKindOneProfile, key: FlowGuid) {
    assert!(profile.replace_first_key(usize::MAX, key).is_err());
    assert!(profile.replace_reference_key(usize::MAX, key).is_err());
    assert!(profile.replace_reference_state(usize::MAX, 0).is_err());
    assert!(profile.replace_reference(0, usize::MAX, key).is_err());
    assert!(profile.replace_outer_reference(usize::MAX, key).is_err());
    assert!(profile.replace_outer_attribute(usize::MAX, 0).is_err());
}
fn check_identity(profile: &FlowMediumKindOneProfile, raw: &[u8]) {
    let foreign = crate::api::carrier(raw);
    assert!(
        foreign
            .record_424_kind_one_profile(profile.record(), options())
            .is_err()
    );
    let wrong = foreign.latest(999).unwrap().unwrap().id;
    assert!(
        foreign
            .record_424_kind_one_profile(&wrong, options())
            .is_err()
    );
    let edit = profile.replace_reference_state(0, 255).unwrap();
    assert!(
        edit.result()
            .source()
            .record_424_kind_one_profile(profile.record(), options())
            .is_err()
    );
}
#[test]
fn unsupported_kinds_unresolved_references_versions_negative_counts_and_populated_final_collections_are_rejected()
 {
    let case = profile_fixture::cases().remove(0);
    for offset in [case.first[0].start as usize, case.second[0].start as usize] {
        let mut raw = case.raw.clone();
        raw[offset..offset + 4].copy_from_slice(&2i32.to_le_bytes());
        assert!(open(&raw, options()).is_err());
    }
    for offset in [case.raw.len() - 8, case.raw.len() - 4] {
        for count in [-1i32, 1] {
            let mut raw = case.raw.clone();
            raw[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(open(&raw, options()).is_err());
        }
    }
    check_versions_and_counts(&case);
}
fn check_versions_and_counts(case: &ProfileCase) {
    let start = case.backend_start as usize;
    let second = case.first.last().unwrap().end as usize;
    let reference = case.second[0].start as usize + 4;
    for offset in [start, start + 1, second, reference, reference + 1] {
        for version in 1..=255 {
            let mut raw = case.raw.clone();
            raw[offset] = version;
            assert!(open(&raw, options()).is_err());
        }
    }
    for offset in [
        start + 2,
        second + 1,
        reference + 19,
        case.backend_end as usize,
    ] {
        let mut raw = case.raw.clone();
        raw[offset..offset + 4].copy_from_slice(&(-1i32).to_le_bytes());
        assert!(open(&raw, options()).is_err());
    }
}
#[test]
fn profile_types_are_send_sync_and_eight_workers_match_all_complete_reads_and_nested_edits_after_input_mutation()
 {
    assert_send_sync::<FlowMediumProfileContext>();
    assert_send_sync::<FlowMediumProfileCheckMode>();
    assert_send_sync::<FlowMediumProfileOptions>();
    assert_send_sync::<FlowMediumProfileCoverage>();
    assert_send_sync::<FlowMediumProfileDiagnostic>();
    assert_send_sync::<FlowMediumKindOneProfile>();
    assert_send_sync::<FlowMediumProfileObject<'static>>();
    assert_send_sync::<FlowMediumProfileReference<'static>>();
    assert_send_sync::<FlowMediumProfileEdit>();
    let cases = profile_fixture::cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn assert_send_sync<T: Send + Sync>() {}
fn transcript(cases: &[ProfileCase]) -> Vec<u8> {
    let mut output = Vec::new();
    for case in cases {
        let mut raw = case.raw.clone();
        let profile = open(&raw, options()).unwrap();
        raw.fill(255);
        output.extend_from_slice(profile.payload_bytes());
        for object in profile.first_objects() {
            let local = object
                .body()
                .replace_state_byte(object.body().state_byte() ^ 255)
                .unwrap();
            let edit = profile.apply_body_edit(object.ordinal(), &local).unwrap();
            output.extend_from_slice(edit.result().source().as_bytes());
        }
        assert_eq!(profile.payload_bytes(), case.raw);
    }
    output
}

fn state_edit(profile: &FlowMediumKindOneProfile) -> FlowMediumBodyEdit {
    profile
        .first_object(0)
        .unwrap()
        .body()
        .replace_state_byte(255)
        .unwrap()
}

#[test]
fn nested_edit_provenance_rejects_other_physical_bodies_stale_revisions_and_backend_changes() {
    let case = profile_fixture::cases()
        .into_iter()
        .find(|case| case.name == "v3-duplicate_keys")
        .unwrap();
    let profile = open(&case.raw, options()).unwrap();
    let local = state_edit(&profile);
    assert!(profile.apply_body_edit(1, &local).is_err());
    assert!(profile.apply_metadata_edit(local.prefix_edit()).is_err());
    let changed = profile.apply_body_edit(0, &local).unwrap();
    assert!(changed.result().apply_body_edit(0, &local).is_err());
    let foreign = open(&case.raw, options()).unwrap();
    let metadata = foreign
        .metadata_prefix()
        .replace_version_byte(foreign.metadata_prefix().version_byte())
        .unwrap();
    assert!(profile.apply_metadata_edit(&metadata).is_err());
    assert_eq!(profile.payload_bytes(), case.raw);
}

#[test]
fn exhausted_global_name_budget_still_accepts_later_zero_unit_names() {
    let case = profile_fixture::cases()
        .into_iter()
        .find(|case| case.name == "v3-duplicate_keys")
        .unwrap();
    let raw = clear_later_names(&case);
    let limited = FlowMediumProfileOptions {
        maximum_total_name_units: 1,
        ..options()
    };
    let profile = open(&raw, limited).unwrap();
    assert_eq!(profile.coverage().name_units, 1);
    let units: Vec<_> = profile
        .first_objects()
        .map(|object| object.body().name().stored_units())
        .collect();
    assert_eq!(units, [1, 0, 0]);
    assert_eq!(profile.payload_bytes(), raw);
}

fn clear_later_names(case: &ProfileCase) -> Vec<u8> {
    let mut raw = case.raw.clone();
    for entry in case.first[1..].iter().rev() {
        let count = entry.start as usize + 38;
        raw[count..count + 4].copy_from_slice(&0i32.to_le_bytes());
        raw.drain(count + 4..count + 6);
    }
    raw
}
