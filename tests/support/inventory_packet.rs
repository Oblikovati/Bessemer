use crate::load_projection::{project_word, read_word, take_projection};
use bessemer::{Result, simulation::SimulationPersistVersion};
use std::collections::BTreeSet;

pub(crate) type InventoryRebuild =
    fn(u32, &[u8], SimulationPersistVersion, Option<u32>) -> Result<Vec<u8>>;

#[derive(Clone, Copy)]
pub(crate) struct InventoryWire {
    pub(crate) storage: &'static str,
    pub(crate) has_profile: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct InventoryTotals {
    pub(crate) streams: usize,
    pub(crate) logical_bytes: usize,
    pub(crate) arguments: usize,
    pub(crate) accepted: usize,
    pub(crate) failed: usize,
    pub(crate) incomplete: usize,
    pub(crate) families: [usize; 7],
}

struct InventoryPayload<'a> {
    family: u32,
    document: &'a [u8],
    stream: &'a [u8],
    raw: &'a [u8],
}

fn framed_bytes<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let size = read_word(cursor) as usize;
    take_projection(cursor, size)
}

fn inventory_payload<'a>(cursor: &mut &'a [u8], wire: InventoryWire) -> InventoryPayload<'a> {
    let family = read_word(cursor);
    assert!(
        family <= 6,
        "family {family}; expected explicit {} family 0..=6",
        wire.storage
    );
    let document = framed_bytes(cursor);
    let stream = framed_bytes(cursor);
    let raw = framed_bytes(cursor);
    InventoryPayload {
        family,
        document,
        stream,
        raw,
    }
}

fn append_identity(output: &mut Vec<u8>, payload: &InventoryPayload<'_>) {
    project_word(output, payload.family as usize);
    for text in [payload.document, payload.stream] {
        project_word(output, text.len());
        output.extend_from_slice(text);
    }
    project_word(output, payload.raw.len());
}

pub(crate) fn identity_manifest(mut cursor: &[u8], wire: InventoryWire) -> Vec<u8> {
    let mut manifest = Vec::new();
    while !cursor.is_empty() {
        append_identity(&mut manifest, &inventory_payload(&mut cursor, wire));
        let count = read_word(&mut cursor) as usize;
        take_projection(&mut cursor, count * if wire.has_profile { 12 } else { 8 });
    }
    manifest
}

pub(crate) fn assert_manifest(packet: &[u8], expected: &[u8], wire: InventoryWire) {
    assert!(
        identity_manifest(packet, wire) == expected,
        "{} identities; expected exact complete manifest",
        wire.storage
    );
}

fn read_profile(cursor: &mut &[u8], wire: InventoryWire) -> Option<u32> {
    if !wire.has_profile {
        return None;
    }
    let profile = read_word(cursor);
    assert!(
        profile <= 1,
        "profile {profile}; expected reader 0 or writer 1"
    );
    Some(profile)
}

fn assert_argument(
    payload: &InventoryPayload<'_>,
    version: SimulationPersistVersion,
    accepted: u32,
    rebuilt: Result<Vec<u8>>,
) -> Vec<u8> {
    assert_eq!(
        rebuilt.is_ok(),
        accepted == 1,
        "{} {} version {}",
        String::from_utf8_lossy(payload.document),
        String::from_utf8_lossy(payload.stream),
        version.value()
    );
    match rebuilt {
        Ok(bytes) => {
            assert_eq!(bytes, payload.raw);
            bytes
        }
        Err(_) => Vec::new(),
    }
}

fn append_rebuilt(
    output: &mut Vec<u8>,
    payload: &InventoryPayload<'_>,
    version: SimulationPersistVersion,
    profile: Option<u32>,
    accepted: u32,
    bytes: &[u8],
) {
    for word in [
        payload.family,
        version.value() as u32,
        profile.unwrap_or(u32::MAX),
        accepted,
    ] {
        project_word(output, word as usize);
    }
    for frame in [payload.document, payload.stream, bytes] {
        project_word(output, frame.len());
        output.extend_from_slice(frame);
    }
}

fn replay_argument(
    cursor: &mut &[u8],
    payload: &InventoryPayload<'_>,
    totals: &mut InventoryTotals,
    wire: InventoryWire,
    rebuild: InventoryRebuild,
    output: &mut Vec<u8>,
) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let profile = read_profile(cursor, wire);
    let accepted = read_word(cursor);
    assert!(
        accepted <= 1,
        "accepted {accepted}; expected Boolean 0 or 1"
    );
    let bytes = assert_argument(
        payload,
        version,
        accepted,
        rebuild(payload.family, payload.raw, version, profile),
    );
    append_rebuilt(output, payload, version, profile, accepted, &bytes);
    totals.arguments += 1;
    totals.accepted += accepted as usize;
    totals.failed += usize::from(accepted == 0);
    accepted == 1
}

fn replay_payload(
    cursor: &mut &[u8],
    payload: &InventoryPayload<'_>,
    totals: &mut InventoryTotals,
    wire: InventoryWire,
    rebuild: InventoryRebuild,
    output: &mut Vec<u8>,
) {
    let count = read_word(cursor);
    assert!(
        count > 0,
        "argument count {count}; expected at least one original argument"
    );
    let mut complete = false;
    for _ in 0..count {
        complete |= replay_argument(cursor, payload, totals, wire, rebuild, output);
    }
    totals.incomplete += usize::from(!complete);
    totals.streams += 1;
    totals.logical_bytes += payload.raw.len();
    totals.families[payload.family as usize] += 1;
}

pub(crate) fn replay_packet(
    cursor: &[u8],
    wire: InventoryWire,
    rebuild: InventoryRebuild,
) -> InventoryTotals {
    replay_packet_with_output(cursor, wire, rebuild).0
}

pub(crate) fn replay_packet_with_output(
    mut cursor: &[u8],
    wire: InventoryWire,
    rebuild: InventoryRebuild,
) -> (InventoryTotals, Vec<u8>) {
    let mut identities = BTreeSet::new();
    let mut totals = InventoryTotals::default();
    let mut output = Vec::new();
    while !cursor.is_empty() {
        let payload = inventory_payload(&mut cursor, wire);
        assert!(
            identities.insert((payload.document, payload.stream)),
            "duplicate {} identity",
            wire.storage
        );
        replay_payload(
            &mut cursor,
            &payload,
            &mut totals,
            wire,
            rebuild,
            &mut output,
        );
    }
    (totals, output)
}
