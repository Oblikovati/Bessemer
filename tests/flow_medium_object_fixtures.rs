//! Independent complete serialized kind-one bodies, without engineering interpretation.
#[path = "flow_medium_objects/api.rs"]
mod api;
#[path = "flow_medium_objects/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_medium_objects/rebuild.rs"]
mod rebuild;
#[path = "flow_medium_objects/types.rs"]
mod types;
#[test]
fn every_complete_kind_one_body_reconstructs_all_bits_intervals_and_following_tail() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 912);
    for case in cases {
        assert_eq!(rebuild::body(&case), case.raw, "{}", case.name);
    }
}
#[test]
fn native_body_retains_all_seven_curve_arrays_and_its_exact_following_backend_bytes() {
    let cases = fixture::cases();
    let native = &cases[0];
    let exact = &cases[1];
    assert_eq!(
        (
            native.raw.len(),
            native.footer.end,
            native.footer.tail.len()
        ),
        (13053, 12973, 80)
    );
    assert_eq!(exact.raw.len(), 12973);
    assert!(exact.footer.tail.is_empty());
    assert_eq!(&native.raw[..native.footer.end as usize], exact.raw);
    assert_eq!((native.first_count, native.second_count), (50, 100));
    check_native_curves(native);
    check_native_text_and_state(native);
}
#[test]
fn every_base_state_and_flag_byte_and_raw_wide_pattern_has_owned_independent_coverage() {
    let cases = fixture::cases();
    let bases: std::collections::BTreeSet<_> = cases.iter().map(|case| case.base).collect();
    let states: std::collections::BTreeSet<_> =
        cases.iter().map(|case| case.footer.state).collect();
    let flags: std::collections::BTreeSet<_> = cases
        .iter()
        .flat_map(|case| case.curves.iter().map(|curve| curve.flag))
        .collect();
    assert_eq!(bases.len(), 256);
    assert_eq!(states.len(), 256);
    assert_eq!(flags.len(), 256);
    let words: std::collections::BTreeSet<_> = cases
        .iter()
        .flat_map(|case| case.curves.iter().map(|curve| curve.scalar))
        .collect();
    for bit in 0..64 {
        assert!(words.contains(&(1u64 << bit)));
        assert!(words.contains(&!(1u64 << bit)));
    }
}

fn check_native_curves(native: &types::BodyCase) {
    let curves: Vec<_> = native
        .curves
        .iter()
        .map(|curve| (curve.count, curve.flag))
        .collect();
    assert_eq!(
        curves,
        [
            (50, 1),
            (50, 0),
            (50, 0),
            (50, 0),
            (100, 0),
            (100, 0),
            (100, 0)
        ]
    );
}

fn check_native_text_and_state(native: &types::BodyCase) {
    assert_eq!(
        (
            native.text.units,
            native.footer.state,
            native.footer.loaded_state
        ),
        (6, 1, 1)
    );
}
