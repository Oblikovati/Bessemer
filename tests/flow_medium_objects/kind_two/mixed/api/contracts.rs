use super::*;
#[derive(Clone, Copy)]
enum Axis {
    Payload,
    First,
    Second,
    References,
    Wide,
    Names,
    BodyBytes,
    BodyWide,
    BodyNames,
    BodyWords,
    Words,
}
const AXES: [Axis; 11] = [
    Axis::Payload,
    Axis::First,
    Axis::Second,
    Axis::References,
    Axis::Wide,
    Axis::Names,
    Axis::BodyBytes,
    Axis::BodyWide,
    Axis::BodyNames,
    Axis::BodyWords,
    Axis::Words,
];
impl Axis {
    fn set(self, options: &mut FlowMediumMixedOptions, value: usize) {
        match self {
            Self::Payload => options.profile.metadata.maximum_payload_bytes = value,
            Self::First => options.profile.maximum_first_objects = value,
            Self::Second => options.profile.maximum_reference_objects = value,
            Self::References => options.profile.maximum_references = value,
            Self::Wide => options.profile.maximum_total_wide_words = value,
            Self::Names => options.profile.maximum_total_name_units = value,
            Self::BodyBytes => options.profile.body.maximum_body_bytes = value,
            Self::BodyWide => options.profile.body.maximum_wide_words = value,
            Self::BodyNames => options.profile.body.maximum_name_units = value,
            Self::BodyWords => options.maximum_body_word_patterns = value,
            Self::Words => options.maximum_total_word_patterns = value,
        }
    }
    fn exact(self, case: &MixedCase) -> usize {
        let (wide, words, names, refs) = totals(case);
        let body = body_maximums(case);
        match self {
            Self::Payload => case.raw.len(),
            Self::First => case.first.len(),
            Self::Second => case.second.len(),
            Self::References => refs,
            Self::Wide => wide,
            Self::Names => names,
            Self::BodyBytes => body[0],
            Self::BodyWide => body[1],
            Self::BodyNames => body[2],
            Self::BodyWords => body[3],
            Self::Words => words,
        }
    }
}
#[test]
fn every_independent_mixed_limit_accepts_exact_counts_and_rejects_one_below_or_zero() {
    for case in mixed_fixture::cases().iter().filter(|case| {
        case.name.starts_with("native-")
            || case.name == "arrays-tag5-32"
            || case.name == "v3-tag5-duplicates"
            || case.name == "edge-tag5-empty-both-references"
    }) {
        for axis in AXES {
            check_axis(case, axis);
        }
        let mut exact = options();
        for axis in AXES {
            axis.set(&mut exact, axis.exact(case).max(1));
        }
        open(&case.raw, exact).unwrap();
    }
}
fn check_axis(case: &MixedCase, axis: Axis) {
    let count = axis.exact(case);
    let mut bound = options();
    axis.set(&mut bound, count.max(1));
    open(&case.raw, bound).unwrap();
    if count > 1 {
        axis.set(&mut bound, count - 1);
        assert!(open(&case.raw, bound).is_err());
    }
    axis.set(&mut bound, 0);
    assert!(open(&case.raw, bound).is_err());
}
#[test]
fn complete_mixed_profiles_reject_all_compact_prefixes_extra_bytes_and_cross_kind_references() {
    for case in mixed_fixture::cases() {
        let mut extended = case.raw.clone();
        extended.push(0);
        assert!(open(&extended, options()).is_err());
        if case.name.starts_with("v3-tag") && !case.name.ends_with("duplicates") {
            check_prefixes(&case.raw);
        }
        if case.name == "v3-tag5-duplicates" {
            check_domains(&case);
            check_structural(&case);
        }
    }
}
fn check_prefixes(raw: &[u8]) {
    for end in 0..raw.len() {
        assert!(open(&raw[..end], options()).is_err(), "prefix {end}");
    }
}
fn check_domains(case: &MixedCase) {
    for entry in &case.second {
        let index = if entry.kind == 1 { 2 } else { 1 };
        let key = case
            .first
            .iter()
            .find(|first| first.kind == 3 - entry.kind && first.key != case.first[0].key)
            .unwrap();
        let mut raw = case.raw.clone();
        let start = entry.start as usize + if entry.kind == 1 { 27 } else { 26 };
        raw[start..start + 16].copy_from_slice(&key.key);
        assert_eq!(key.kind, index);
        assert!(open(&raw, options()).is_err());
    }
    let mut raw = case.raw.clone();
    let start = case.backend_end as usize + 4;
    raw[start..start + 16].fill(0x42);
    assert!(open(&raw, options()).is_err());
}
fn check_structural(case: &MixedCase) {
    for position in case
        .first
        .iter()
        .map(|entry| entry.start)
        .chain(case.second.iter().map(|entry| entry.start))
    {
        check_word_mutations(
            &case.raw,
            position as usize,
            &[i32::MIN, -1, 0, 3, i32::MAX],
        );
    }
    check_versions(case);
    check_counts(case);
}
fn check_versions(case: &MixedCase) {
    let second = case.first.last().unwrap().end as usize;
    for position in [
        case.backend_start as usize,
        case.backend_start as usize + 1,
        second,
    ] {
        for value in 1..=255 {
            let mut raw = case.raw.clone();
            raw[position] = value;
            assert!(open(&raw, options()).is_err());
        }
    }
}
fn check_counts(case: &MixedCase) {
    let second = case.first.last().unwrap().end as usize;
    for position in [
        case.backend_start as usize + 2,
        second + 1,
        case.backend_end as usize,
        case.raw.len() - 8,
        case.raw.len() - 4,
    ] {
        check_word_mutations(&case.raw, position, &[-1, i32::MAX]);
    }
    for entry in &case.second {
        check_word_mutations(
            &case.raw,
            entry.start as usize + if entry.kind == 1 { 23 } else { 22 },
            &[-1, i32::MAX],
        );
    }
}
fn check_word_mutations(raw: &[u8], position: usize, values: &[i32]) {
    for value in values {
        let mut changed = raw.to_vec();
        changed[position..position + 4].copy_from_slice(&value.to_le_bytes());
        assert!(open(&changed, options()).is_err());
    }
}
#[test]
fn mixed_profile_rejects_missing_state_indices_foreign_ids_and_stale_composition_without_source_mutation()
 {
    let cases = mixed_fixture::cases();
    let case = cases
        .iter()
        .find(|case| case.name == "v3-tag5-duplicates")
        .unwrap();
    let profile = open(&case.raw, options()).unwrap();
    check_missing_indices(&profile);
    check_stale_composition(&profile);
    assert_eq!(profile.payload_bytes(), case.raw);
    check_ids(&profile);
}
fn check_ids(profile: &FlowMediumMixedProfile) {
    let source = profile.source();
    let id = source.records().next().unwrap().id;
    assert!(source.record_424_mixed_profile(&id, options()).is_err());
    let mut raw = profile.payload_bytes().to_vec();
    raw[0] = 4;
    let foreign = crate::api::carrier(&raw);
    assert!(
        foreign
            .record_424_mixed_profile(profile.record(), options())
            .is_err()
    );
}
fn send_sync<T: Send + Sync>() {}
#[test]
fn all_mixed_public_types_are_send_sync_and_eight_workers_match_owned_read_and_edit_transcripts() {
    send_sync::<FlowMediumKind>();
    send_sync::<FlowMediumBody>();
    send_sync::<FlowMediumMixedOptions>();
    send_sync::<FlowMediumMixedCheckMode>();
    send_sync::<FlowMediumMixedCoverage>();
    send_sync::<FlowMediumMixedDiagnostic>();
    send_sync::<FlowMediumMixedEntry>();
    send_sync::<FlowMediumMixedReference>();
    send_sync::<FlowMediumMixedProfile>();
    send_sync::<FlowMediumMixedEdit>();
    let cases = mixed_fixture::cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn transcript(cases: &[MixedCase]) -> Vec<u8> {
    let mut result = Vec::new();
    for case in cases {
        let mut raw = case.raw.clone();
        let profile = open(&raw, options()).unwrap();
        raw.fill(255);
        result.extend_from_slice(profile.payload_bytes());
        if profile.outer_attributes().is_empty() {
            continue;
        }
        let edit = profile.replace_outer_attribute(0, 42).unwrap();
        result.extend_from_slice(edit.result().source().as_bytes());
    }
    result
}

fn body_maximums(case: &MixedCase) -> [usize; 4] {
    let mut maximums = [0; 4];
    for entry in &case.first {
        let (wide, words, names) = body_totals(&entry.body);
        for (maximum, value) in
            maximums
                .iter_mut()
                .zip([(entry.end - entry.start - 20) as usize, wide, names, words])
        {
            *maximum = (*maximum).max(value);
        }
    }
    maximums
}

fn check_missing_indices(profile: &FlowMediumMixedProfile) {
    let index = usize::MAX;
    let key = guid(&[0; 16]);
    assert!(profile.replace_first_key(index, key).is_err());
    assert!(profile.replace_reference_key(index, key).is_err());
    assert!(profile.replace_reference_state(1, 0).is_err());
    assert!(profile.replace_reference(0, index, key).is_err());
    assert!(profile.replace_outer_reference(index, key).is_err());
    assert!(profile.replace_outer_attribute(index, 0).is_err());
}

fn check_stale_composition(profile: &FlowMediumMixedProfile) {
    let body_edit = match profile.first_objects()[0].body() {
        FlowMediumBody::KindOne(body) => body.replace_base_byte(42).unwrap(),
        _ => panic!(),
    };
    assert!(profile.apply_kind_one_body_edit(1, &body_edit).is_err());
    assert!(
        profile
            .apply_metadata_edit(body_edit.prefix_edit())
            .is_err()
    );
    let changed = profile.apply_kind_one_body_edit(0, &body_edit).unwrap();
    assert!(
        changed
            .result()
            .apply_kind_one_body_edit(0, &body_edit)
            .is_err()
    );
    let metadata = profile.metadata_prefix().replace_version_byte(4).unwrap();
    assert!(changed.result().apply_metadata_edit(&metadata).is_err());
}
