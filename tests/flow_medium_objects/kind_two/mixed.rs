use super::*;
#[path = "mixed/fixture.rs"]
mod mixed_fixture;
#[path = "mixed/rebuild.rs"]
mod mixed_rebuild;
#[path = "mixed/types.rs"]
mod mixed_types;
use mixed_types::*;
#[test]
fn every_mixed_profile_rebuilds_all_nested_fields_typed_references_and_exact_eof() {
    let cases = mixed_fixture::cases();
    assert_eq!(cases.len(), 3753);
    for case in cases {
        assert_eq!(mixed_rebuild::profile(&case), case.raw, "{}", case.name);
    }
}
#[test]
fn mixed_profiles_cover_both_orders_all_models_states_and_physical_duplicates() {
    let cases = mixed_fixture::cases();
    let mut models = std::collections::BTreeSet::new();
    let mut states = std::collections::BTreeSet::new();
    for entry in cases.iter().flat_map(|case| &case.first) {
        if let MixedBody::Two(body) = &entry.body {
            models.insert(body.model.kind);
            states.insert(body.footer.state);
        }
    }
    assert_eq!(models, [0, 1, 2, 3, 4, 5].into());
    assert_eq!(states.len(), 256);
    check_orders_and_duplicates(&cases);
}
fn check_orders_and_duplicates(cases: &[MixedCase]) {
    for (suffix, first_kind, count) in [
        ("one-first", 1, 4),
        ("two-first", 2, 4),
        ("duplicates", 1, 6),
    ] {
        let case = cases
            .iter()
            .find(|case| case.name == format!("v3-tag5-{suffix}"))
            .unwrap();
        assert_eq!(case.first.len(), count);
        assert_eq!(case.first[0].kind, first_kind);
        assert_eq!(case.second[0].kind, first_kind);
        assert_eq!(case.footer.attributes, [i32::MIN, 0, i32::MAX]);
        assert_eq!(case.footer.keys, vec![case.second[0].key.clone(); 3]);
    }
}
