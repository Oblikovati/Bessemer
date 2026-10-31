//! Fresh complete SolidManager inventory replay, retaining incomplete occurrences (#154).
use bessemer::simulation::SimulationPersistVersion;
use std::collections::BTreeSet;
#[path = "simulation_solid_inventory/families.rs"]
mod families;
#[allow(
    dead_code,
    reason = "Shared projection helpers also serve complete field/history corpus tests."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{project_word, read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-solid-inventory/owned.bin");
const NATIVE_IDENTITIES: &[u8] =
    include_bytes!("../docs/features/simulation-solid-inventory/native-identities.bin");

#[derive(Debug, Default, PartialEq, Eq)]
struct SolidReplayTotals {
    streams: usize,
    logical_bytes: usize,
    arguments: usize,
    accepted: usize,
    failed: usize,
    incomplete: usize,
    families: [usize; 7],
}

struct SolidPayload<'a> {
    family: u32,
    document: &'a [u8],
    stream: &'a [u8],
    raw: &'a [u8],
}

fn framed_bytes<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let size = read_word(cursor) as usize;
    take_projection(cursor, size)
}

fn solid_payload<'a>(cursor: &mut &'a [u8]) -> SolidPayload<'a> {
    let family = read_word(cursor);
    assert!(
        family <= 6,
        "family {family}; expected explicit SolidManager family 0..=6"
    );
    let document = framed_bytes(cursor);
    let stream = framed_bytes(cursor);
    let raw = framed_bytes(cursor);
    SolidPayload {
        family,
        document,
        stream,
        raw,
    }
}

fn append_identity(output: &mut Vec<u8>, payload: &SolidPayload<'_>) {
    project_word(output, payload.family as usize);
    for text in [payload.document, payload.stream] {
        project_word(output, text.len());
        output.extend_from_slice(text);
    }
    project_word(output, payload.raw.len());
}

fn solid_identity_manifest(mut cursor: &[u8]) -> Vec<u8> {
    let mut manifest = Vec::new();
    while !cursor.is_empty() {
        append_identity(&mut manifest, &solid_payload(&mut cursor));
        let count = read_word(&mut cursor) as usize;
        take_projection(&mut cursor, count * 8);
    }
    manifest
}

fn assert_solid_manifest(packet: &[u8], expected: &[u8]) {
    assert!(
        solid_identity_manifest(packet) == expected,
        "SolidManager identities; expected exact complete manifest"
    );
}

fn replay_solid_argument(
    cursor: &mut &[u8],
    payload: &SolidPayload<'_>,
    totals: &mut SolidReplayTotals,
) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let accepted = read_word(cursor);
    assert!(accepted <= 1);
    let rebuilt = families::rebuild(payload.family, payload.raw, version);
    assert_eq!(
        rebuilt.is_ok(),
        accepted == 1,
        "{} {} version {}",
        String::from_utf8_lossy(payload.document),
        String::from_utf8_lossy(payload.stream),
        version.value()
    );
    if let Ok(bytes) = rebuilt {
        assert_eq!(bytes, payload.raw);
    }
    totals.arguments += 1;
    totals.accepted += accepted as usize;
    totals.failed += usize::from(accepted == 0);
    accepted == 1
}

fn replay_solid_packet(mut cursor: &[u8]) -> SolidReplayTotals {
    let mut identities = BTreeSet::new();
    let mut totals = SolidReplayTotals::default();
    while !cursor.is_empty() {
        let payload = solid_payload(&mut cursor);
        assert!(
            identities.insert((payload.document, payload.stream)),
            "duplicate SolidManager identity"
        );
        let count = read_word(&mut cursor);
        assert!(count > 0);
        let mut complete = false;
        for _ in 0..count {
            complete |= replay_solid_argument(&mut cursor, &payload, &mut totals);
        }
        totals.incomplete += usize::from(!complete);
        totals.streams += 1;
        totals.logical_bytes += payload.raw.len();
        totals.families[payload.family as usize] += 1;
    }
    totals
}

#[test]
fn owned_seven_families_construct_completely_and_keep_failed_occurrences_incomplete() {
    let totals = replay_solid_packet(OWNED);
    assert_eq!(totals.streams, 15);
    assert_eq!(
        (
            totals.arguments,
            totals.accepted,
            totals.failed,
            totals.incomplete
        ),
        (15, 14, 1, 1)
    );
    assert_eq!(totals.families, [2, 2, 2, 2, 2, 3, 2]);
}

#[test]
#[ignore = "requires complete freshly recovered native SolidManager inventory packet"]
fn native_full_inventory_replays_all_original_arguments_without_incomplete_success() {
    let path = std::env::var_os("BESSEMER_SOLID_INVENTORY_NATIVE_INPUT")
        .expect("set complete fresh SolidManager packet");
    let packet = std::fs::read(path).unwrap();
    assert_solid_manifest(&packet, NATIVE_IDENTITIES);
    let totals = replay_solid_packet(&packet);
    assert_eq!(
        totals,
        SolidReplayTotals {
            streams: 3439,
            logical_bytes: 4329610,
            arguments: 3730,
            accepted: 3491,
            failed: 239,
            incomplete: 6,
            families: [361, 685, 680, 238, 837, 232, 406]
        }
    );
    if let Some(path) = std::env::var_os("BESSEMER_SOLID_INVENTORY_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
#[should_panic(expected = "duplicate SolidManager identity")]
fn duplicate_solid_occurrences_cannot_overwrite_or_inflate_inventory() {
    replay_solid_packet(&[OWNED, OWNED].concat());
}

#[test]
#[should_panic(expected = "expected explicit SolidManager family 0..=6")]
fn unknown_solid_family_fails_before_dispatch() {
    let mut packet = OWNED.to_vec();
    packet[..4].copy_from_slice(&99u32.to_le_bytes());
    replay_solid_packet(&packet);
}

#[test]
#[should_panic(expected = "expected exact complete manifest")]
fn renamed_solid_occurrence_cannot_replace_an_equal_count_inventory() {
    let expected = solid_identity_manifest(OWNED);
    let mut packet = OWNED.to_vec();
    packet[8] = b'z';
    assert_solid_manifest(&packet, &expected);
}
