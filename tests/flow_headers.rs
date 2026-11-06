//! Native GUID text oracles and adversarial selected-prefix/header-gate cases.
#[path = "flow_carriers/fixture.rs"]
mod fixture;
use bessemer::{FailureClass, flow::*};

const FIRST_NATIVE_GUIDS: [&str; 3] = [
    "294f3380-4314-4882-9451-d36e4562451d",
    "00000000-0000-0000-0000-000000000000",
    "f285ac33-b61a-43e0-aace-3315aab7711a",
];
const SETTINGS_NATIVE_GUIDS: [&str; 3] = [
    "b74082b0-db86-4729-8b64-856d8dd015ae",
    "00000000-0000-0000-0000-000000000000",
    "b74082b0-db86-4729-8b64-856d8dd015ae",
];

fn assert_native_header_coverage(header: &FlowHeaderMetadata) {
    assert!(header.extension.is_empty());
    assert!(header.settings.as_ref().unwrap().extension.is_empty());
    let coverage = header.coverage();
    assert_eq!(
        coverage.mode,
        FlowHeaderCheckMode::ExactSelectedGuidPrefixes
    );
    assert_eq!(coverage.checked_records, 2);
    assert_eq!(coverage.checked_prefix_bytes, 52);
    assert_eq!(coverage.opaque_extension_bytes, 0);
    assert_eq!(coverage.achieved_byte_error, 0);
}

fn assert_missing_headers() {
    let nil = FlowGuid::from_bytes([0; 16]);
    let orphan = owned(&[(435, b"")]);
    assert!(orphan.header_metadata().unwrap().is_none());
    let error = orphan
        .replace_header_guid(FlowHeaderGuidField::First, nil)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    let only_first = owned(&[(123, &[0; 32])]);
    let header = only_first.header_metadata().unwrap().unwrap();
    assert!(header.settings.is_none());
    assert_eq!(header.coverage().checked_prefix_bytes, 32);
    assert!(
        only_first
            .replace_header_guid(FlowHeaderGuidField::StaticSettings, nil)
            .is_err()
    );
}

fn assert_all_guid_edits(carrier: &FlowCarrier) {
    let before = carrier.header_metadata().unwrap().unwrap();
    for field in [
        FlowHeaderGuidField::First,
        FlowHeaderGuidField::Second,
        FlowHeaderGuidField::StaticSettings,
    ] {
        for guid in [
            selected_value(&before, field),
            FlowGuid::from_bytes([0x5a; 16]),
        ] {
            assert_guid_edit(carrier, field, guid);
        }
    }
}

fn settings_guid(carrier: &FlowCarrier) -> FlowGuid {
    let header = carrier.header_metadata().unwrap().unwrap();
    header.settings.unwrap().cfld_uuid
}

fn assert_parallel_headers(carrier: &FlowCarrier, guid: FlowGuid) {
    std::thread::scope(|scope| {
        let joins: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    carrier
                        .replace_header_guid(FlowHeaderGuidField::StaticSettings, guid)
                        .unwrap()
                })
            })
            .collect();
        for join in joins {
            let edit = join.join().unwrap();
            assert_eq!(settings_guid(edit.result()), guid);
        }
    });
}

#[test]
fn all_native_guid_roles_match_independent_reviewed_windows_text() {
    for (index, case) in fixture::cases().into_iter().take(3).enumerate() {
        let carrier = FlowCarrier::from_bytes(&case.raw, fixture::limits()).unwrap();
        let header = carrier.header_metadata().unwrap().unwrap();
        assert_eq!(carrier.records().len(), case.records.len(), "{}", case.name);
        assert_eq!(header.first_guid.to_string(), FIRST_NATIVE_GUIDS[index]);
        assert_eq!(
            header.second_guid.to_string(),
            "dd9387fc-29c8-4ad5-b3c8-805e110c86dd"
        );
        let settings = header.settings.as_ref().unwrap();
        assert_eq!(settings.cfld_uuid.to_string(), SETTINGS_NATIVE_GUIDS[index]);
        assert_eq!(settings.control_word, 1);
        assert_eq!(header.source.as_bytes(), case.raw);
        assert_native_header_coverage(&header);
    }
}

fn owned(records: &[(u32, &[u8])]) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    for &(identifier, payload) in records {
        wire.extend(identifier.to_le_bytes());
        wire.extend((payload.len() as u64).to_le_bytes());
        wire.extend(payload);
    }
    FlowCarrier::from_bytes(&wire, fixture::limits()).unwrap()
}

#[test]
fn gate_duplicate_selection_and_missing_versus_present_nil_are_explicit() {
    assert_missing_headers();
    let both = owned(&[
        (123, b"bad earlier"),
        (123, &[0; 32]),
        (435, b"bad earlier"),
        (435, &[0; 20]),
    ]);
    let header = both.header_metadata().unwrap().unwrap();
    assert_eq!(header.record.ordinal(), 1);
    let selected = header.settings.unwrap();
    assert_eq!(selected.record.ordinal(), 3);
    assert_eq!(selected.cfld_uuid.as_bytes(), &[0; 16]);
}

#[test]
fn every_short_selected_prefix_fails_without_fallback_or_defaulting() {
    for size in 0..32 {
        let raw = vec![0; size];
        let carrier = owned(&[(123, &[0; 32]), (123, &raw)]);
        let error = carrier.header_metadata().unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains(&size.to_string()));
    }
    for size in 0..20 {
        let raw = vec![0; size];
        let carrier = owned(&[(123, &[0; 32]), (435, &[0; 20]), (435, &raw)]);
        assert_eq!(
            carrier.header_metadata().unwrap_err().class,
            FailureClass::Invalid
        );
    }
}

#[test]
fn guid_edits_preserve_control_extensions_and_all_other_carrier_bytes() {
    let mut cases: Vec<_> = fixture::cases()
        .into_iter()
        .take(3)
        .map(|case| FlowCarrier::from_bytes(&case.raw, fixture::limits()).unwrap())
        .collect();
    cases.push(owned(&[
        (123, &[0; 35]),
        (435, &[255; 24]),
        (19, b"opaque"),
    ]));
    for carrier in cases {
        assert_all_guid_edits(&carrier);
    }
}

fn selected_value(header: &FlowHeaderMetadata, field: FlowHeaderGuidField) -> FlowGuid {
    match field {
        FlowHeaderGuidField::First => header.first_guid,
        FlowHeaderGuidField::Second => header.second_guid,
        FlowHeaderGuidField::StaticSettings => header.settings.as_ref().unwrap().cfld_uuid,
    }
}

fn assert_guid_edit(carrier: &FlowCarrier, field: FlowHeaderGuidField, guid: FlowGuid) {
    let before = carrier.header_metadata().unwrap().unwrap();
    let edit = carrier.replace_header_guid(field, guid).unwrap();
    let after = edit.result().header_metadata().unwrap().unwrap();
    assert_eq!(selected_value(&after, field), guid);
    assert_extensions(&before, &after);
    assert_record_bytes(carrier, &edit, field);
    let reversed = edit
        .result()
        .replace_header_guid(field, selected_value(&before, field))
        .unwrap();
    assert_eq!(reversed.result().as_bytes(), carrier.as_bytes());
    assert_eq!(edit.source().as_bytes(), carrier.as_bytes());
}

fn assert_extensions(before: &FlowHeaderMetadata, after: &FlowHeaderMetadata) {
    assert_eq!(before.extension, after.extension);
    assert_eq!(before.coverage(), after.coverage());
    let first = before.settings.as_ref().unwrap();
    let second = after.settings.as_ref().unwrap();
    assert_eq!(first.control_word, second.control_word);
    assert_eq!(first.extension, second.extension);
    assert_eq!(
        before.coverage().opaque_extension_bytes,
        before.extension.len() + first.extension.len()
    );
}

fn assert_record_bytes(carrier: &FlowCarrier, edit: &FlowCarrierEdit, field: FlowHeaderGuidField) {
    let (identifier, offset) = match field {
        FlowHeaderGuidField::First => (123, 0),
        FlowHeaderGuidField::Second => (123, 16),
        FlowHeaderGuidField::StaticSettings => (435, 4),
    };
    let selected = carrier.latest(identifier).unwrap().unwrap();
    let start = selected.header_offset + 12 + offset;
    assert_eq!(
        &carrier.as_bytes()[..start],
        &edit.result().as_bytes()[..start]
    );
    assert_eq!(
        &carrier.as_bytes()[start + 16..],
        &edit.result().as_bytes()[start + 16..]
    );
}

#[test]
fn binary_guid_endianness_and_shared_header_ownership_are_stable() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlowHeaderMetadata>();
    assert_send_sync::<FlowGuid>();
    assert_send_sync::<FlowHeaderGuidField>();
    assert_send_sync::<FlowHeaderCoverage>();
    let guid = FlowGuid::from_bytes(std::array::from_fn(|index| index as u8));
    assert_eq!(guid.to_string(), "03020100-0504-0706-0809-0a0b0c0d0e0f");
    let nil = FlowGuid::from_bytes([0; 16]);
    assert_eq!(nil.to_string(), "00000000-0000-0000-0000-000000000000");
    let carrier = owned(&[(123, &[0; 32]), (435, &[0; 20])]);
    assert_parallel_headers(&carrier, guid);
    assert_eq!(settings_guid(&carrier), nil);
}
