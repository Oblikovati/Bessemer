use super::*;
use bessemer::flow::*;
fn link_options(compressed: bool) -> FlowLinkRecordOptions {
    FlowLinkRecordOptions {
        compression: if compressed {
            FlowLegacyTableTransport::Zlib
        } else {
            FlowLegacyTableTransport::Raw
        },
        global_pointer_count: None,
        maximum_decoded_bytes: 2_000_000,
        maximum_entries: 100_000,
        maximum_array_elements: 2_000_000,
    }
}
pub(super) fn links(row: &SelectionCase) -> FlowLinkRecords {
    let carrier = state_carrier::carrier(402, &row.inputs.links_stored);
    let id = carrier.records().nth(1).unwrap().id;
    carrier
        .record_402_links(&id, link_options(row.inputs.compressed_links))
        .unwrap()
}
pub(super) fn states(row: &SelectionCase) -> FlowByteStates {
    let carrier = state_carrier::carrier(316, &row.inputs.states_stored);
    let id = carrier.records().nth(1).unwrap().id;
    carrier
        .record_316_byte_states(
            &id,
            FlowByteStateOptions {
                maximum_decoded_bytes: 2_000_000,
                maximum_values: 1_000_000,
            },
        )
        .unwrap()
}
#[test]
fn complete_original_input_records_match_the_existing_typed_link_and_byte_state_views() {
    for row in fixture::cases() {
        let links = links(&row);
        assert_eq!(
            links.decoded_bytes(),
            row.inputs.links_decoded,
            "{}",
            row.name
        );
        assert_eq!(links.version(), row.inputs.version);
        assert_eq!(links.entries().len(), row.inputs.entries.len());
        for (entry, expected) in links.entries().zip(&row.inputs.entries) {
            check_entry(entry, expected);
        }
        check_states(&row);
    }
}
fn check_entry(entry: FlowLinkEntry<'_>, expected: &SelectionEntry) {
    assert_eq!(entry.ordinal(), expected.ordinal as usize);
    assert_eq!(
        entry.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(entry.code_byte(), expected.code);
    assert_eq!(entry.native_code_byte(), expected.native_code);
}
fn check_states(row: &SelectionCase) {
    let states = states(row);
    assert_eq!(states.decoded_bytes(), row.inputs.states_decoded);
    assert_eq!(states.raw_bytes(), &row.inputs.states_decoded[8..]);
    assert_eq!(
        states.header(),
        i32::from_le_bytes(row.inputs.states_decoded[..4].try_into().unwrap())
    );
    assert_eq!(states.stored_count(), row.inputs.states_decoded.len() - 8);
    let noop = states.replace_header(states.header()).unwrap();
    state_carrier::check_history(noop.carrier_edit());
    assert_eq!(
        noop.result().source().as_bytes(),
        states.source().as_bytes()
    );
}
