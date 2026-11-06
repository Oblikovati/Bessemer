use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "replay.rs"]
mod replay;

fn options() -> FlowMessageListOptions {
    FlowMessageListOptions {
        string_mode: Some(FlowMetadataStringMode::NativeUtf16),
        maximum_payload_bytes: 2_000_000,
        maximum_items: 100_000,
        maximum_strings: 100_000,
        maximum_text_bytes: 2_000_000,
        maximum_values: 2_000_000,
        maximum_scopes: 500_000,
    }
}
fn open(raw: &[u8], bounds: FlowMessageListOptions) -> bessemer::Result<FlowMessageLists> {
    let carrier = state_carrier::carrier(301, raw);
    let id = carrier.records().nth(1).unwrap().id;
    carrier.record_301_message_lists(&id, bounds)
}
#[test]
fn complete_typed_messages_match_every_owned_source_string_and_field() {
    for case in fixture::cases() {
        let messages = open(&case.raw, options()).unwrap();
        check_header(&case, &messages);
        for (expected, text) in case.texts.iter().zip(messages.texts()) {
            check_string(&case, expected, text);
            check_identity(&messages, text);
        }
        for item in messages.items() {
            check_item(&case, item);
        }
        assert!(messages.item(usize::MAX).is_none());
        assert!(messages.text_at(usize::MAX).is_none());
    }
}
fn check_identity(messages: &FlowMessageLists, text: FlowMessageText<'_>) {
    assert_eq!(
        messages.text(&text.id()).unwrap().raw_bytes(),
        text.raw_bytes()
    );
    assert_eq!(text.id().record(), messages.record());
    assert_eq!(
        messages.text_at(text.id().ordinal()).unwrap().role(),
        text.role()
    );
}
fn check_header(case: &MessageCase, messages: &FlowMessageLists) {
    let proof = messages.coverage();
    assert_eq!(messages.decoded_bytes(), case.raw);
    assert_eq!(messages.version(), case.version);
    assert_eq!(messages.options(), options());
    assert_eq!(proof.items, case.items as usize);
    assert_eq!(proof.strings, case.texts.len());
    assert_eq!(proof.scopes, case.scopes.len());
    assert_eq!(proof.payload_bytes, case.raw.len());
    assert_eq!(
        proof.stored_values,
        case.fields.iter().map(|field| field.count as usize).sum()
    );
    let transferred: usize = case
        .texts
        .iter()
        .map(|text| (text.end - text.start - 4) as usize)
        .sum();
    assert_eq!(proof.text_bytes, transferred);
    assert_eq!(proof.achieved_error, 0);
    assert_eq!(proof.applied_string_mode.is_none(), case.version == 1);
}
fn check_string(case: &MessageCase, expected: &MessageText, text: FlowMessageText<'_>) {
    assert_eq!(
        text.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(text.stored_count(), expected.count as usize);
    assert_eq!(text.raw_bytes(), text_field(case, expected).raw);
    assert_eq!(text.prefix_bytes(), expected.prefix.as_deref());
    assert_eq!(
        text.prefix_units(),
        expected.prefix.as_ref().map(|raw| raw.len() / 2)
    );
    assert_eq!(
        text.encoding().element_width(),
        if expected.legacy { 1 } else { 2 }
    );
    for index in 0..text.stored_count() {
        check_element(text, index, expected.legacy);
    }
    assert!(text.unit(usize::MAX).is_none() && text.byte(usize::MAX).is_none());
}
fn check_element(text: FlowMessageText<'_>, index: usize, legacy: bool) {
    if legacy {
        assert_eq!(text.byte(index), Some(text.raw_bytes()[index]));
        assert!(text.unit(index).is_none());
        return;
    }
    assert_eq!(
        text.unit(index),
        Some(u16::from_le_bytes(
            text.raw_bytes()[index * 2..index * 2 + 2]
                .try_into()
                .unwrap()
        ))
    );
    assert!(text.byte(index).is_none());
}
fn check_item(case: &MessageCase, item: FlowMessageItem<'_>) {
    let path = format!("[\"items\",{},\"member_8_pattern\"]", item.ordinal());
    let field = case.fields.iter().find(|field| field.path == path).unwrap();
    assert_eq!(item.member_pattern().to_le_bytes().as_slice(), field.raw);
    assert_eq!(item.payload_interval().0, field.offset as usize);
    assert_eq!(
        item.primary().role(),
        FlowMessageTextRole::Primary(item.ordinal())
    );
    check_list(item.first_list(), item.ordinal(), false);
    assert_eq!(item.second_list().is_some(), case.version == 3);
    if let Some(list) = item.second_list() {
        check_list(list, item.ordinal(), true);
    }
}
fn check_list(list: FlowMessageStringList<'_>, item: usize, second: bool) {
    assert_eq!(list.string_count(), list.texts().len());
    assert!(list.payload_interval().1 >= list.payload_interval().0 + 4);
    for (index, text) in list.texts().enumerate() {
        let role = if second {
            FlowMessageTextRole::SecondList(item, index)
        } else {
            FlowMessageTextRole::FirstList(item, index)
        };
        assert_eq!(text.role(), role);
        assert_eq!(list.text(index).unwrap().id(), text.id());
    }
    assert!(list.text(list.string_count()).is_none() && list.text(usize::MAX).is_none());
}
#[test]
fn stored_members_and_selected_text_elements_edit_reopen_and_reverse_exactly() {
    for case in fixture::cases() {
        let messages = open(&case.raw, options()).unwrap();
        for item in messages.items() {
            check_member_edit(&messages, item);
        }
        for text in messages.texts() {
            check_text_edits(&messages, text);
        }
    }
}
fn check_text_edits(messages: &FlowMessageLists, text: FlowMessageText<'_>) {
    let count = text.stored_count();
    let indices: BTreeSet<_> = [0, count / 2, count.saturating_sub(1)]
        .into_iter()
        .filter(|index| *index < count)
        .collect();
    for index in indices {
        let edit = changed_text(messages, text, index);
        state_carrier::check_history(edit.carrier_edit());
        assert_eq!(edit.source().decoded_bytes(), messages.decoded_bytes());
        let reverse = reverse_text(&edit, text, index);
        assert_eq!(
            reverse.result().source().as_bytes(),
            messages.source().as_bytes()
        );
    }
}
fn reverse_text(
    edit: &FlowMessageListEdit,
    text: FlowMessageText<'_>,
    index: usize,
) -> FlowMessageListEdit {
    let current = edit.result().text_at(text.id().ordinal()).unwrap();
    match text.encoding() {
        FlowMessageTextEncoding::LegacyBytes => edit
            .result()
            .replace_text_byte(&current.id(), index, text.byte(index).unwrap())
            .unwrap(),
        FlowMessageTextEncoding::Utf16Units => edit
            .result()
            .replace_text_unit(&current.id(), index, text.unit(index).unwrap())
            .unwrap(),
    }
}
fn changed_text(
    messages: &FlowMessageLists,
    text: FlowMessageText<'_>,
    index: usize,
) -> FlowMessageListEdit {
    let expected = replacement_bytes(text, index);
    let edit = match text.encoding() {
        FlowMessageTextEncoding::LegacyBytes => messages
            .replace_text_byte(&text.id(), index, expected[0])
            .unwrap(),
        FlowMessageTextEncoding::Utf16Units => messages
            .replace_text_unit(
                &text.id(),
                index,
                u16::from_le_bytes([expected[0], expected[1]]),
            )
            .unwrap(),
    };
    let offset = text.payload_interval().0 + 4 + index * text.encoding().element_width();
    check_patch(messages, &edit, offset, &expected);
    edit
}
fn replacement_bytes(text: FlowMessageText<'_>, index: usize) -> Vec<u8> {
    if text.encoding() == FlowMessageTextEncoding::LegacyBytes {
        return vec![!text.byte(index).unwrap()];
    }
    let value = if index + 1 == text.stored_count() {
        0
    } else {
        !text.unit(index).unwrap()
    };
    value.to_le_bytes().to_vec()
}

fn check_patch(
    messages: &FlowMessageLists,
    edit: &FlowMessageListEdit,
    offset: usize,
    expected: &[u8],
) {
    let mut raw = messages.decoded_bytes().to_vec();
    raw[offset..offset + expected.len()].copy_from_slice(expected);
    assert_eq!(edit.result().decoded_bytes(), raw);
}

fn check_member_edit(messages: &FlowMessageLists, item: FlowMessageItem<'_>) {
    let edit = messages
        .replace_member(item.ordinal(), !item.member_pattern())
        .unwrap();
    check_patch(
        messages,
        &edit,
        item.payload_interval().0,
        &(!item.member_pattern()).to_le_bytes(),
    );
    state_carrier::check_history(edit.carrier_edit());
    let reverse = edit
        .result()
        .replace_member(item.ordinal(), item.member_pattern())
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        messages.source().as_bytes()
    );
}
