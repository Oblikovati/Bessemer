//! Combined fresh BeamManager identities, candidate outcomes and typed construction (#146).
use bessemer::simulation::*;
use std::collections::BTreeSet;
#[path = "simulation_beam_inventory/families.rs"]
mod families;
#[allow(
    dead_code,
    reason = "Shared field/history helpers have dedicated family corpus tests."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{project_word, read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-beam-inventory/owned.bin");
const NATIVE_IDENTITIES: &[u8] =
    include_bytes!("../docs/features/simulation-beam-inventory/native-identities.bin");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PayloadFamily {
    Manager,
    Beam,
    Material,
    JointGroup,
    JointList,
    Joint,
}

impl PayloadFamily {
    fn from_index(index: u32) -> Self {
        match index {
            0 => Self::Manager,
            1 => Self::Beam,
            2 => Self::Material,
            3 => Self::JointGroup,
            4 => Self::JointList,
            5 => Self::Joint,
            _ => panic!("family {index}; expected family 0..=5"),
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Manager => 0,
            Self::Beam => 1,
            Self::Material => 2,
            Self::JointGroup => 3,
            Self::JointList => 4,
            Self::Joint => 5,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct InventoryTotals {
    streams: usize,
    logical_bytes: usize,
    arguments: usize,
    accepted: usize,
    failed: usize,
    families: [usize; 6],
}

struct PayloadView<'a> {
    family: PayloadFamily,
    document: &'a str,
    stream: &'a str,
    raw: &'a [u8],
}

struct CandidateArgument {
    version: SimulationPersistVersion,
    accepted: bool,
    expected_count: Option<usize>,
}

fn read_text<'a>(cursor: &mut &'a [u8]) -> &'a str {
    let size = read_word(cursor) as usize;
    std::str::from_utf8(take_projection(cursor, size)).unwrap()
}

fn project_text(output: &mut Vec<u8>, value: &str) {
    project_word(output, value.len());
    output.extend_from_slice(value.as_bytes());
}

fn read_payload<'a>(cursor: &mut &'a [u8]) -> PayloadView<'a> {
    let family = PayloadFamily::from_index(read_word(cursor));
    let document = read_text(cursor);
    let stream = read_text(cursor);
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let payload = PayloadView {
        family,
        document,
        stream,
        raw,
    };
    assert_family_path(&payload);
    payload
}

fn assert_family_path(payload: &PayloadView<'_>) {
    let suffix = payload
        .stream
        .split_once("/BeamManager/")
        .expect("expected BeamManager carrier")
        .1;
    let segments: Vec<&str> = suffix.split('/').collect();
    let expected = match segments.as_slice() {
        ["Property"] => PayloadFamily::Manager,
        ["Beams", child, "Property"] if !child.is_empty() => PayloadFamily::Beam,
        ["Beams", child, "Material"] if !child.is_empty() => PayloadFamily::Material,
        ["JointGroups", child, "JointGroup"] if !child.is_empty() => PayloadFamily::JointGroup,
        ["JointGroups", child, "JointList"] if !child.is_empty() => PayloadFamily::JointList,
        ["JointGroups", group, joint, "Joint"] if !group.is_empty() && !joint.is_empty() => {
            PayloadFamily::Joint
        }
        _ => panic!("{suffix}; expected a complete supported BeamManager payload path"),
    };
    assert_eq!(payload.family, expected);
}

fn read_argument(cursor: &mut &[u8]) -> CandidateArgument {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let accepted = read_word(cursor);
    assert!(accepted <= 1);
    let count = read_word(cursor) as i32;
    assert!(count >= -1);
    CandidateArgument {
        version,
        accepted: accepted != 0,
        expected_count: usize::try_from(count).ok(),
    }
}

fn project_argument(output: &mut Vec<u8>, argument: &CandidateArgument) {
    project_word(output, argument.version.value() as usize);
    project_word(output, usize::from(argument.accepted));
    output.extend_from_slice(
        &argument
            .expected_count
            .map(|count| count as i32)
            .unwrap_or(-1)
            .to_le_bytes(),
    );
}

fn replay_arguments(
    cursor: &mut &[u8],
    payload: &PayloadView<'_>,
    totals: &mut InventoryTotals,
) -> (Vec<u8>, Vec<u8>) {
    let count = read_word(cursor) as usize;
    let mut arguments = Vec::new();
    project_word(&mut arguments, count);
    let mut reconstructed = None;
    for _ in 0..count {
        let argument = read_argument(cursor);
        if let Some(bytes) = reconstruct_candidate(payload, &argument) {
            reconstructed = Some(bytes);
        }
        totals.arguments += 1;
        totals.accepted += usize::from(argument.accepted);
        totals.failed += usize::from(!argument.accepted);
        project_argument(&mut arguments, &argument);
    }
    (
        reconstructed.expect("every inventory occurrence must have a complete accepted argument"),
        arguments,
    )
}

fn reconstruct_candidate(
    payload: &PayloadView<'_>,
    argument: &CandidateArgument,
) -> Option<Vec<u8>> {
    let result = families::reconstruct(payload, argument);
    assert_eq!(
        result.is_ok(),
        argument.accepted,
        "{} {} {:?}",
        payload.document,
        payload.stream,
        argument.version
    );
    match result {
        Ok(bytes) => {
            assert_eq!(bytes, payload.raw);
            Some(bytes)
        }
        Err(_) => None,
    }
}

fn project_payload(output: &mut Vec<u8>, payload: &PayloadView<'_>, raw: &[u8], arguments: &[u8]) {
    project_word(output, payload.family.index());
    project_text(output, payload.document);
    project_text(output, payload.stream);
    project_word(output, raw.len());
    output.extend_from_slice(raw);
    output.extend_from_slice(arguments);
}

fn identity_packet(mut cursor: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    while !cursor.is_empty() {
        let payload = read_payload(&mut cursor);
        project_word(&mut output, payload.family.index());
        project_text(&mut output, payload.document);
        project_text(&mut output, payload.stream);
        project_word(&mut output, payload.raw.len());
        let count = read_word(&mut cursor) as usize;
        take_projection(&mut cursor, count * 12);
    }
    output
}

fn assert_identities(packet: &[u8], expected: &[u8]) {
    assert!(
        identity_packet(packet) == expected,
        "expected exact inventory identity manifest"
    );
}

fn replay(mut cursor: &[u8]) -> (Vec<u8>, InventoryTotals) {
    let mut identities = BTreeSet::new();
    let mut output = Vec::new();
    let mut totals = InventoryTotals::default();
    while !cursor.is_empty() {
        let payload = read_payload(&mut cursor);
        assert!(
            identities.insert((payload.document, payload.stream)),
            "duplicate inventory identity: {} {}",
            payload.document,
            payload.stream
        );
        let (raw, arguments) = replay_arguments(&mut cursor, &payload, &mut totals);
        project_payload(&mut output, &payload, &raw, &arguments);
        totals.streams += 1;
        totals.logical_bytes += raw.len();
        totals.families[payload.family.index()] += 1;
    }
    (output, totals)
}

#[test]
fn owned_six_family_replay_preserves_every_argument_and_validated_constructed_byte() {
    let (output, totals) = replay(OWNED);
    assert_eq!(output, OWNED);
    assert_eq!(
        totals,
        InventoryTotals {
            streams: 6,
            logical_bytes: 4296,
            arguments: 8,
            accepted: 7,
            failed: 1,
            families: [1; 6]
        }
    );
}

#[test]
#[ignore = "requires complete freshly recovered native BeamManager inventory packet"]
fn all_native_identities_and_candidate_outcomes_replay_together_with_optional_export() {
    let path = std::env::var_os("BESSEMER_BEAM_INVENTORY_NATIVE_INPUT")
        .expect("set complete fresh native inventory packet");
    let packet = std::fs::read(path).unwrap();
    assert_identities(&packet, NATIVE_IDENTITIES);
    let (output, totals) = replay(&packet);
    assert_eq!(output, packet);
    assert_eq!(
        totals,
        InventoryTotals {
            streams: 1080,
            logical_bytes: 620654,
            arguments: 1114,
            accepted: 1084,
            failed: 30,
            families: [353, 163, 150, 36, 11, 367]
        }
    );
    if let Some(path) = std::env::var_os("BESSEMER_BEAM_INVENTORY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}

#[test]
#[should_panic(expected = "duplicate inventory identity")]
fn duplicate_inventory_identities_cannot_overwrite_or_inflate_the_denominator() {
    replay(&[OWNED, OWNED].concat());
}

#[test]
#[should_panic(expected = "expected family 0..=5")]
fn unknown_payload_families_fail_before_any_opaque_fallback() {
    let mut packet = OWNED.to_vec();
    packet[..4].copy_from_slice(&99u32.to_le_bytes());
    replay(&packet);
}

#[test]
#[should_panic(expected = "expected exact inventory identity manifest")]
fn renamed_identities_cannot_replace_expected_entries_with_equal_counts_and_sizes() {
    let expected = identity_packet(OWNED);
    let mut altered = OWNED.to_vec();
    altered[8] = b'z';
    assert_identities(&altered, &expected);
}
