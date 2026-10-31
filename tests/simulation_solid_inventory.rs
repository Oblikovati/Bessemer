//! Fresh complete SolidManager inventory replay, retaining incomplete occurrences (#154).
use bessemer::simulation::SimulationPersistVersion;
#[path = "simulation_solid_inventory/families.rs"]
mod families;
#[path = "support/inventory_packet.rs"]
mod inventory_packet;
#[allow(
    dead_code,
    reason = "Shared projection helpers also serve complete field/history corpus tests."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use inventory_packet::{InventoryTotals as SolidReplayTotals, InventoryWire};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-solid-inventory/owned.bin");
const NATIVE_IDENTITIES: &[u8] =
    include_bytes!("../docs/features/simulation-solid-inventory/native-identities.bin");

const WIRE: InventoryWire = InventoryWire {
    storage: "SolidManager",
    has_profile: false,
};

fn solid_identity_manifest(cursor: &[u8]) -> Vec<u8> {
    inventory_packet::identity_manifest(cursor, WIRE)
}

fn assert_solid_manifest(packet: &[u8], expected: &[u8]) {
    inventory_packet::assert_manifest(packet, expected, WIRE);
}

fn rebuild_solid(
    family: u32,
    raw: &[u8],
    version: SimulationPersistVersion,
    profile: Option<u32>,
) -> bessemer::Result<Vec<u8>> {
    assert_eq!(profile, None);
    families::rebuild(family, raw, version)
}

fn replay_solid_packet(cursor: &[u8]) -> SolidReplayTotals {
    inventory_packet::replay_packet(cursor, WIRE, rebuild_solid)
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
