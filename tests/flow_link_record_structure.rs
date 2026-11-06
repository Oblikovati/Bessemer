//! A valid alternate parse must not make a scalar edit a structural operation.
use bessemer::flow::*;
use bessemer::FailureClass;

#[test]
fn member_edit_rejects_an_alternate_complete_parse_with_different_tail_spans() {
    let original = ambiguous_branch_record();
    let before = link_view(&original);
    let mut alternate = original.clone();
    alternate[32..36].copy_from_slice(&0_i32.to_le_bytes());
    let parsed = link_view(&alternate);
    check_alternate_counts(&before, &parsed);
    let field = FlowLinkRecordField::TailMember { index: 6 };
    let error = before
        .replace_field(field, FlowLinkRecordValue::Signed32(0))
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("changed parsed intervals"));
    assert_eq!(before.decoded_bytes(), original);
}

fn ambiguous_branch_record() -> Vec<u8> {
    let mut raw = 6_i32.to_le_bytes().to_vec();
    raw.extend_from_slice(&0_i32.to_le_bytes());
    let mut members = [0_i32; 11];
    members[6] = 2;
    raw.extend(members.into_iter().flat_map(i32::to_le_bytes));
    // With no conditional values, their first word becomes a valid numeric count.
    raw.extend([1_i32, 1, 0x1234, 0].into_iter().flat_map(i32::to_le_bytes));
    raw
}

fn link_view(payload: &[u8]) -> FlowLinkRecords {
    let source = single_link_carrier(payload);
    let options = FlowLinkRecordOptions {
        compression: FlowLegacyTableTransport::Raw,
        global_pointer_count: None,
        maximum_decoded_bytes: 1000,
        maximum_entries: 1,
        maximum_array_elements: 3,
    };
    source
        .record_402_links(&source.latest(402).unwrap().unwrap().id, options)
        .unwrap()
}

fn single_link_carrier(payload: &[u8]) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    raw.extend_from_slice(&402_u32.to_le_bytes());
    raw.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    raw.extend_from_slice(payload);
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: 1000,
            maximum_payload_bytes: 1000,
            maximum_records: 1,
        },
    )
    .unwrap()
}

fn check_alternate_counts(before: &FlowLinkRecords, parsed: &FlowLinkRecords) {
    assert_eq!(before.tail().conditional_integers().word_count(), 2);
    assert_eq!(parsed.tail().conditional_integers().word_count(), 0);
    assert_eq!(
        before.tail().numeric().unwrap().wide_words().word_count(),
        0
    );
    assert_eq!(
        parsed.tail().numeric().unwrap().wide_words().word_count(),
        1
    );
}
