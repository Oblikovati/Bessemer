//! Independent complete ID/state contracts and explicitly partial metadata-prefix accounting.
#[path = "flow_metadata/api.rs"]
mod api;
#[path = "flow_metadata/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_metadata/rebuild.rs"]
mod rebuild;
#[path = "flow_metadata/types.rs"]
mod types;

#[test]
fn every_id_state_row_and_ordered_assignment_chain_matches_independent_wire_and_maps() {
    let cases = fixture::state_cases();
    assert_eq!(cases.len(), 1880);
    for case in cases {
        assert_eq!(rebuild::state_wire(&case), case.raw, "{}", case.name);
        rebuild::apply_states(&case);
    }
}

#[test]
fn every_known_metadata_prefix_raw_string_projection_and_opaque_tail_is_preserved() {
    let cases = fixture::prefix_cases();
    assert_eq!(cases.len(), 537);
    for case in cases {
        assert_eq!(rebuild::prefix_wire(&case), case.raw, "{}", case.name);
    }
}

#[test]
fn installed_states_are_complete_while_metadata_stops_at_its_explicit_backend_boundary() {
    let states = fixture::state_cases().remove(0);
    assert_eq!((states.version_word, states.end), (0, 18));
    let entries: Vec<_> = states
        .entries
        .iter()
        .map(|entry| (entry.identifier, entry.state))
        .collect();
    assert_eq!(entries, [(410, 1), (413, 1)]);
    check_native_prefix();
}

fn check_native_prefix() {
    let prefix = fixture::prefix_cases().remove(0);
    assert_eq!(
        (prefix.signed_version, prefix.known, prefix.unknown),
        (2, 112, 13079)
    );
    assert_eq!(prefix.raw.len(), 13191);
    assert_eq!(prefix.objects[0].first.units, 33);
    assert_eq!(
        prefix.objects[0].second.native_text,
        "Water"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>()
    );
    assert!(prefix.requires_backend);
    assert!(!prefix.full_schema);
}
