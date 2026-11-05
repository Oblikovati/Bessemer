use super::*;
use std::io::Write;

fn stored(decoded: &[u8], compressed: bool) -> Vec<u8> {
    if !compressed {
        return decoded.to_vec();
    }
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(decoded).unwrap();
    encoder.finish().unwrap()
}

fn payload_carrier(identifier: u32, payload: &[u8]) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
    carrier(&wire)
}

fn read_payload(payload: &[u8], compressed: bool) -> bessemer::Result<FlowGroupedRecord> {
    let source = payload_carrier(404, payload);
    let id = source.records().next().unwrap().id;
    source.record_404_groups(&id, options(compressed))
}

#[test]
fn exact_and_one_below_global_budgets_and_wrong_contexts_are_explicit() {
    for case in fixture::cases() {
        let source = carrier(&case.wire);
        let id = source.records().nth(case.ordinal).unwrap().id;
        let exact = exact_options(&case);
        assert_eq!(
            source
                .record_404_groups(&id, exact)
                .unwrap()
                .decoded_bytes(),
            case.decoded
        );
        assert_undersized(&source, &id, exact);
        let wrong = options(!case.compressed);
        assert!(source.record_404_groups(&id, wrong).is_err());
        assert_eq!(source.as_bytes(), case.wire);
    }
}

fn exact_options(case: &fixture::GroupCase) -> FlowGroupOptions {
    let members = case
        .groups
        .iter()
        .map(|group| group.members.len())
        .sum::<usize>();
    FlowGroupOptions {
        maximum_decoded_bytes: case.decoded.len(),
        maximum_groups: case.groups.len().max(1),
        maximum_members: members.max(1),
        ..options(case.compressed)
    }
}

fn assert_undersized(source: &FlowCarrier, id: &FlowRecordId, exact: FlowGroupOptions) {
    for smaller in [
        FlowGroupOptions {
            maximum_decoded_bytes: exact.maximum_decoded_bytes - 1,
            ..exact
        },
        FlowGroupOptions {
            maximum_groups: exact.maximum_groups - 1,
            ..exact
        },
        FlowGroupOptions {
            maximum_members: exact.maximum_members - 1,
            ..exact
        },
    ] {
        assert!(source.record_404_groups(id, smaller).is_err());
    }
}

#[test]
fn every_owned_prefix_and_trailing_byte_fails_without_a_partial_result() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "v6-mixed-none")
        .unwrap();
    for compressed in [false, true] {
        for end in 0..case.decoded.len() {
            assert!(read_payload(&stored(&case.decoded[..end], compressed), compressed).is_err());
        }
        let trailing = [&case.decoded[..], &[0]].concat();
        assert!(read_payload(&stored(&trailing, compressed), compressed).is_err());
    }
    let encoded = stored(&case.decoded, true);
    for end in 0..encoded.len() {
        assert!(read_payload(&encoded[..end], true).is_err());
    }
    assert_bad_zlib(&encoded);
}

fn assert_bad_zlib(encoded: &[u8]) {
    let mut corrupt = encoded.to_vec();
    let end = corrupt.len() - 1;
    corrupt[end] ^= 1;
    let trailing = [encoded, &[0]].concat();
    let concatenated = [encoded, encoded].concat();
    let gzip = include_bytes!("../fixtures/flow-arrays/wrong-wrapper.gz");
    for bytes in [
        &corrupt[..],
        &trailing,
        &concatenated,
        &encoded[2..encoded.len() - 4],
        gzip.as_slice(),
    ] {
        assert_eq!(
            read_payload(bytes, true).unwrap_err().class,
            FailureClass::Integrity
        );
    }
}

#[test]
fn unsupported_versions_negative_counts_and_impossible_counts_fail_before_allocation() {
    for version in [i32::MIN, -1, 0, 1, 2, 7, i32::MAX] {
        let decoded = [version.to_le_bytes(), 0_i32.to_le_bytes()].concat();
        assert_eq!(
            read_payload(&decoded, false).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
    for words in [
        [5_i32, -1, 0],
        [5, i32::MAX, 0],
        [3, 1, -1],
        [3, 1, i32::MAX],
        [4, 1, 0],
    ] {
        let decoded: Vec<_> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
        assert_eq!(
            read_payload(&decoded, false).unwrap_err().class,
            FailureClass::Invalid
        );
    }
}

#[test]
fn foreign_occurrences_wrong_identifiers_missing_fields_and_empty_indexes_fail() {
    let payload = stored(
        &[
            3_i32.to_le_bytes(),
            1_i32.to_le_bytes(),
            0_i32.to_le_bytes(),
        ]
        .concat(),
        false,
    );
    let source = payload_carrier(404, &payload);
    let foreign = payload_carrier(404, &payload).records().next().unwrap().id;
    assert!(source.record_404_groups(&foreign, options(false)).is_err());
    let wrong = payload_carrier(433, &payload);
    let id = wrong.records().next().unwrap().id;
    assert!(wrong.record_404_groups(&id, options(false)).is_err());
    let id = source.records().next().unwrap().id;
    let record = source.record_404_groups(&id, options(false)).unwrap();
    assert_missing_fields(&record);
}

fn assert_missing_fields(record: &FlowGroupedRecord) {
    assert_eq!(record.groups().len(), 1);
    assert_eq!(
        record
            .replace_field(0, FlowGroupFieldEdit::Auxiliary(0))
            .unwrap_err()
            .class,
        FailureClass::Missing
    );
    assert!(
        record
            .replace_field(
                0,
                FlowGroupFieldEdit::Word {
                    member: 0,
                    value: 0
                }
            )
            .is_err()
    );
}

#[test]
fn duplicate_record_ids_keep_exact_occurrences_and_edit_only_the_selected_one() {
    let source = duplicated_carrier();
    for (item, version) in source.records().zip([3, 6]) {
        let record = source.record_404_groups(&item.id, options(false)).unwrap();
        assert_eq!(record.version().wire_value(), version);
        let request = FlowGroupFieldEdit::Word {
            member: 1,
            value: 19,
        };
        let edited = record.replace_field(1, request).unwrap();
        assert_eq!(edited.carrier_edit().history().len(), 2);
    }
}

fn duplicated_carrier() -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    let cases = fixture::cases();
    for case in cases
        .iter()
        .filter(|case| case.name == "v3-mixed-none" || case.name == "v6-mixed-none")
    {
        let source = carrier(&case.wire);
        let payload = source.records().nth(case.ordinal).unwrap();
        wire.extend_from_slice(&payload_carrier(404, payload.payload).as_bytes()[2..]);
    }
    carrier(&wire)
}

#[test]
fn absent_legacy_wire_flags_cannot_be_synthesized_by_an_edit() {
    for name in ["v3-mixed-none", "v4-mixed-none"] {
        let case = fixture::cases()
            .into_iter()
            .find(|case| case.name == name)
            .unwrap();
        let record = view(&case);
        let request = FlowGroupFieldEdit::Flag {
            member: 0,
            value: 255,
        };
        assert_eq!(
            record.replace_field(1, request).unwrap_err().class,
            FailureClass::Missing
        );
        assert_eq!(record.source().as_bytes(), case.wire);
    }
}

#[test]
fn actual_compressed_growth_is_rejected_at_the_original_wire_bound() {
    let mut decoded = [
        5_i32.to_le_bytes(),
        1_i32.to_le_bytes(),
        1024_i32.to_le_bytes(),
    ]
    .concat();
    decoded.resize(12 + 1024 * 13, 0);
    let source = payload_carrier(404, &stored(&decoded, true));
    let id = source.records().next().unwrap().id;
    let record = source.record_404_groups(&id, options(true)).unwrap();
    let request = FlowGroupFieldEdit::Word {
        member: 0,
        value: 0x11223344,
    };
    let edit = record.replace_field(0, request).unwrap();
    assert!(edit.result().source().as_bytes().len() > source.as_bytes().len());
    assert_growth_failure(source.as_bytes(), request);
}

fn assert_growth_failure(wire: &[u8], request: FlowGroupFieldEdit) {
    let bounded = FlowCarrier::from_bytes(
        wire,
        FlowLimits {
            maximum_wire_bytes: wire.len(),
            maximum_records: 10,
            maximum_payload_bytes: wire.len(),
        },
    )
    .unwrap();
    let id = bounded.records().next().unwrap().id;
    let record = bounded.record_404_groups(&id, options(true)).unwrap();
    assert!(record.replace_field(0, request).is_err());
    assert_eq!(record.source().as_bytes(), wire);
}
