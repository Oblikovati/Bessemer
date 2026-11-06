use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "replay.rs"]
mod replay;

fn options() -> FlowObjectRecordOptions {
    FlowObjectRecordOptions {
        maximum_payload_bytes: 2_000_000,
        maximum_rows: 100_000,
    }
}
fn open(raw: &[u8], options: FlowObjectRecordOptions) -> bessemer::Result<FlowObjectRecords> {
    let source = state_carrier::carrier(304, raw);
    let id = source.records().nth(1).unwrap().id;
    source.record_304_objects(&id, options)
}
fn check_case(row: &ObjectCase) -> FlowObjectRecords {
    let view = open(&row.raw, options()).unwrap();
    assert_eq!(view.version(), row.version);
    assert_eq!(view.stored_count(), row.items.len());
    assert_eq!(view.decoded_bytes(), row.raw);
    assert_eq!(view.options(), options());
    for expected in &row.items {
        check_row(&view, expected);
    }
    check_trailer(&view, row);
    check_coverage(&view);
    assert!(view.row(row.items.len()).is_err() && view.row(usize::MAX).is_err());
    view
}
fn check_row(view: &FlowObjectRecords, expected: &ObjectItem) {
    let row = view.row(expected.ordinal as usize).unwrap();
    assert_eq!(row.ordinal(), expected.ordinal as usize);
    assert_eq!(
        row.interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(row.member_10(), expected.member);
    check_optional_fields(&row, expected);
    for field in row_fields(view.version()) {
        let id = row.field_id(field).unwrap();
        assert_eq!(id.interval(), expected_interval(expected, field));
        check_identity(view, &id);
    }
}
fn expected_interval(row: &ObjectItem, field: FlowObjectField) -> (usize, usize) {
    let block = 4 + row.legacy.as_ref().map_or(0, Vec::len);
    let extra = block + row.block.len();
    let state = extra + row.extra.as_ref().map_or(0, Vec::len);
    let extension = state + row.state.as_ref().map_or(0, Vec::len);
    let (relative, width) = match field {
        FlowObjectField::Member10 => (0, 4),
        FlowObjectField::DiscardedWord(index) => (4 + index * 4, 4),
        FlowObjectField::BlockByte(index) => (block + index, 1),
        FlowObjectField::Member7C => (extra, 4),
        FlowObjectField::StateByte => (state, 1),
        FlowObjectField::ExtensionByte(index) => (extension + index, 1),
    };
    (
        row.start as usize + relative,
        row.start as usize + relative + width,
    )
}
fn check_optional_fields(row: &FlowObjectRow<'_>, expected: &ObjectItem) {
    let legacy = expected.legacy.as_ref().map(|raw| {
        std::array::from_fn(|index| {
            i32::from_le_bytes(raw[index * 4..index * 4 + 4].try_into().unwrap())
        })
    });
    assert_eq!(row.discarded_words(), legacy);
    assert_eq!(row.block(), expected.block);
    assert_eq!(
        row.member_7c(),
        expected
            .extra
            .as_ref()
            .map(|raw| i32::from_le_bytes(raw.as_slice().try_into().unwrap()))
    );
    assert_eq!(row.state_byte(), expected.state.as_ref().map(|raw| raw[0]));
    assert_eq!(row.native_state_byte(), expected.native_state);
    assert_eq!(row.extension(), expected.extension.as_deref());
}

fn check_trailer(view: &FlowObjectRecords, expected: &ObjectCase) {
    assert_eq!(view.legacy_trailer().is_some(), expected.trailer.is_some());
    let Some(trailer) = view.legacy_trailer() else {
        return;
    };
    let expected = expected.trailer.as_ref().unwrap();
    assert_eq!(trailer.member_10(), expected.member);
    assert_eq!(trailer.block(), expected.block);
    assert_eq!(trailer.native_appended_object(), expected.appended);
    assert_eq!(
        trailer.interval(),
        (view.decoded_bytes().len() - 20, view.decoded_bytes().len())
    );
    check_trailer_identities(view, &trailer);
}
fn check_trailer_identities(view: &FlowObjectRecords, trailer: &FlowObjectTrailer<'_>) {
    let start = view.decoded_bytes().len() - 20;
    let member = trailer.field_id(FlowObjectTrailerField::Member10).unwrap();
    assert_eq!(member.interval(), (start, start + 4));
    check_identity(view, &member);
    for index in 0..16 {
        let id = trailer
            .field_id(FlowObjectTrailerField::BlockByte(index))
            .unwrap();
        assert_eq!(id.interval(), (start + 4 + index, start + 5 + index));
        check_identity(view, &id);
    }
}

fn check_identity(view: &FlowObjectRecords, id: &FlowObjectFieldId) {
    let (start, end) = id.interval();
    assert_eq!(id.record(), view.record());
    assert_eq!(id.clone(), *id);
    if end - start == 1 {
        assert_eq!(view.byte(id).unwrap(), view.decoded_bytes()[start]);
        assert!(view.signed(id).is_err());
        return;
    }
    let expected = i32::from_le_bytes(view.decoded_bytes()[start..end].try_into().unwrap());
    assert_eq!(view.signed(id).unwrap(), expected);
    assert!(view.byte(id).is_err());
}
fn row_fields(version: i32) -> Vec<FlowObjectField> {
    use FlowObjectField::*;
    let mut fields = vec![Member10];
    if version < 5 {
        fields.extend((0..4).map(DiscardedWord));
    }
    fields.extend((0..16).map(BlockByte));
    if version >= 4 {
        fields.push(Member7C);
    }
    if version >= 6 {
        fields.push(StateByte);
    }
    if version == 7 {
        fields.extend((0..56).map(ExtensionByte));
    }
    fields
}
fn check_coverage(view: &FlowObjectRecords) {
    let coverage = view.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, view.decoded_bytes().len());
    assert_eq!(coverage.stored_rows, view.stored_count());
    assert_eq!(coverage.legacy_trailer, view.legacy_trailer().is_some());
    assert_eq!(coverage.achieved_error, 0);
    use FlowObjectRecordDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            NativeIdentityBindingNotVerified,
            NativeRuntimePopulationNotReproduced,
            LegacyFlagLifecycleNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_owned_field_optional_presence_interval_and_projection_matches_the_typed_view() {
    for row in cases() {
        check_case(&row);
    }
}
