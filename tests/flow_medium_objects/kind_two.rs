#[path = "kind_two/api.rs"]
mod api;
#[path = "kind_two/mixed.rs"]
mod mixed;
use super::{fixture, packet::*, rebuild, types::*};
#[path = "kind_two/fixture.rs"]
mod kind_fixture;
#[path = "kind_two/rebuild.rs"]
mod kind_rebuild;
#[path = "kind_two/types.rs"]
mod kind_types;
use kind_types::*;
#[test]
fn every_generated_kind_two_body_rebuilds_all_common_and_model_fields_with_exact_tail() {
    let cases = kind_fixture::cases();
    assert_eq!(cases.len(), 2492);
    for case in cases {
        assert_eq!(kind_rebuild::body(&case), case.raw, "{}", case.name);
    }
}
#[test]
fn all_six_model_tags_and_raw_model_states_are_covered_without_installed_native_credit() {
    let cases = kind_fixture::cases();
    let kinds: std::collections::BTreeSet<_> = cases.iter().map(|case| case.model.kind).collect();
    assert_eq!(kinds, [0, 1, 2, 3, 4, 5].into());
    let base: std::collections::BTreeSet<_> = cases.iter().map(|case| case.prefix.base).collect();
    assert_eq!(base.len(), 256);
    let state: std::collections::BTreeSet<_> = cases.iter().map(|case| case.footer.state).collect();
    assert_eq!(state.len(), 256);
    for kind in [1, 4, 5] {
        check_model_states(&cases, kind);
    }
    for case in cases {
        assert!(!case.footer.installed);
        assert!(!case.footer.runtime);
        assert!(case.footer.complete);
    }
}
fn check_model_states(cases: &[KindTwoCase], kind: i32) {
    let states: std::collections::BTreeSet<_> = cases
        .iter()
        .filter(|case| case.model.kind == kind)
        .flat_map(|case| model_states(&case.model.payload))
        .collect();
    assert_eq!(states.len(), 256);
}
fn model_states(model: &ModelPayload) -> Vec<u8> {
    match model {
        ModelPayload::Curves(value) => value.states.clone(),
        ModelPayload::Switched(value) => vec![value.state],
        ModelPayload::Table(value) => value.states.clone(),
        ModelPayload::Scalars(_) => Vec::new(),
    }
}
#[test]
fn table_model_preserves_every_raw_word_bit_and_all_signed_member_boundaries() {
    let cases = kind_fixture::cases();
    let tables: Vec<_> = cases
        .iter()
        .filter_map(|case| match &case.model.payload {
            ModelPayload::Table(value) => Some(value),
            _ => None,
        })
        .collect();
    let words: std::collections::BTreeSet<_> = tables
        .iter()
        .flat_map(|value| value.word_patterns.iter().copied())
        .collect();
    for bit in 0..32 {
        assert!(words.contains(&(1u32 << bit)));
        assert!(words.contains(&!(1u32 << bit)));
    }
    let members: std::collections::BTreeSet<_> = tables.iter().map(|value| value.member).collect();
    for value in [i32::MIN, -1, 0, i32::MAX] {
        assert!(members.contains(&value));
    }
}
