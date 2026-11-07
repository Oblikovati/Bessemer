//! Pin complete physical carriers and independent normalized 404 fields before the typed decoder.
#[path = "flow_groups/fixture.rs"]
mod fixture;
use bessemer::flow::*;
use std::io::Read;

fn carrier(case: &fixture::GroupCase) -> FlowCarrier {
    FlowCarrier::from_bytes(
        &case.wire,
        FlowLimits {
            maximum_wire_bytes: 2_000_000,
            maximum_records: 100,
            maximum_payload_bytes: 2_000_000,
        },
    )
    .unwrap()
}

#[test]
fn all_26_carriers_preserve_exact_bytes_and_explicit_payload_contexts() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 26);
    for case in cases {
        let source = carrier(&case);
        assert_eq!(source.as_bytes(), case.wire, "{}", case.name);
        let record = source.records().nth(case.ordinal).unwrap();
        assert_eq!(record.identifier, 404);
        let decoded = decoded_payload(record.payload, case.compressed);
        assert_eq!(decoded, case.decoded);
        assert_eq!(source.latest(404).unwrap().unwrap().id, record.id);
    }
}

fn decoded_payload(payload: &[u8], compressed: bool) -> Vec<u8> {
    if !compressed {
        return payload.to_vec();
    }
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(payload)
        .take(100_001)
        .read_to_end(&mut decoded)
        .unwrap();
    decoded
}

#[test]
fn normalized_fields_rebuild_every_frozen_raw_payload_and_group_span() {
    for case in fixture::cases() {
        let mut rebuilt = case.version.to_le_bytes().to_vec();
        rebuilt.extend(i32::try_from(case.groups.len()).unwrap().to_le_bytes());
        for group in &case.groups {
            assert_eq!(rebuilt.len(), group.start);
            assert_eq!(group.auxiliary.is_some(), case.version == 4);
            assert_eq!(group.flags_present, case.version >= 5);
            rebuilt.extend(i32::try_from(group.members.len()).unwrap().to_le_bytes());
            if let Some(auxiliary) = group.auxiliary {
                rebuilt.extend(auxiliary.to_le_bytes());
            }
            append_members(&mut rebuilt, group, case.version);
            assert_eq!(rebuilt.len(), group.end);
        }
        assert_eq!(rebuilt, case.decoded, "{}", case.name);
    }
}

fn append_members(output: &mut Vec<u8>, group: &fixture::ExpectedGroup, version: i32) {
    if version == 6 {
        for member in &group.members {
            output.extend(member.word.to_le_bytes());
            output.extend(member.wide.to_le_bytes());
            output.push(member.flag.unwrap());
        }
        return;
    }
    for member in &group.members {
        output.extend(member.word.to_le_bytes());
    }
    for member in &group.members {
        output.extend(member.wide.to_le_bytes());
    }
    for member in &group.members {
        assert_eq!(member.flag.is_some(), version == 5);
        if let Some(flag) = member.flag {
            output.push(flag);
        }
    }
}

#[test]
fn two_native_carriers_share_one_payload_variant_with_distinct_outer_files() {
    let cases = fixture::cases();
    let native = &cases[..2];
    assert_ne!(native[0].wire, native[1].wire);
    assert_eq!(native[0].decoded, native[1].decoded);
    for case in native {
        assert_eq!(case.version, 5);
        assert_eq!(case.groups.len(), 4056);
        assert_eq!(case.decoded.len(), 68960);
        assert!(case.groups.iter().all(|group| group.members.len() == 1));
        assert!(
            case.groups
                .iter()
                .all(|group| group.members[0].flag == Some(255))
        );
    }
}
