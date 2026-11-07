use super::*;
#[path = "../support/binary_load_mutations.rs"]
mod load_mutations;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-shell-manager/group-malformed.bin");

#[test]
fn all_targeted_count_mfc_and_scalar_mutations_match_actual_source_including_valid_changed_controls()
 {
    let mut cursor = MALFORMED;
    let mut totals = (0, 0);
    while !cursor.is_empty() {
        let group = read_group(&mut cursor);
        let (accepted, rejected) =
            load_mutations::check_mutations(&mut cursor, &group.to_bytes(), |raw| {
                SimulationShellGroup::from_bytes(raw, group.version(), group.wire_profile(), LIMITS)
                    .is_ok()
            });
        totals.0 += accepted;
        totals.1 += rejected;
    }
    assert_eq!(totals, (192, 684));
}

#[test]
fn every_owned_group_rejects_every_proper_prefix_and_trailing_byte() {
    let mut cursor = EXPECTED;
    let mut count = 0;
    while !cursor.is_empty() {
        let group = read_group(&mut cursor);
        projection::consume_fields(&mut cursor);
        assert_group_prefixes(&group);
        count += 1;
    }
    assert_eq!(count, 72);
}

#[test]
fn oversized_cstring_lengths_fail_before_decoding_with_exact_field_context() {
    let mut raw = hardcaps::empty_string_records(1, 890015, PROFILES[0]);
    raw[4..11].copy_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    let version = SimulationPersistVersion::new(890015).unwrap();
    let error = SimulationShellGroup::from_bytes(&raw, version, PROFILES[0], LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.operation, "decode ShellManagerGroup");
    assert_eq!(error.entity, "record0.string0");
    assert!(error.cause.contains("4294967295") && error.cause.contains("expected <= 8000000"));
}

#[test]
fn invalid_utf16_remains_a_typed_failure_for_the_owning_cstring_field() {
    let group = contracts::owned_groups().remove(1);
    let ordinal = group
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == "record0.string1")
        .unwrap();
    let (_, end) = group.field_offsets()[ordinal];
    let mut raw = group.to_bytes();
    raw[end - 6..end - 4].copy_from_slice(&[0, 0xd8]);
    let error =
        SimulationShellGroup::from_bytes(&raw, group.version(), group.wire_profile(), LIMITS)
            .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.entity, "record0.string1");
    assert!(error.cause.contains("d800") && error.cause.contains("expected valid UTF-16LE"));
}

fn assert_group_prefixes(group: &SimulationShellGroup) {
    let raw = group.to_bytes();
    for end in 0..raw.len() {
        assert!(
            SimulationShellGroup::from_bytes(
                &raw[..end],
                group.version(),
                group.wire_profile(),
                LIMITS
            )
            .is_err()
        );
    }
    let appended = [raw.as_slice(), &[0]].concat();
    assert!(
        SimulationShellGroup::from_bytes(&appended, group.version(), group.wire_profile(), LIMITS)
            .is_err()
    );
}
