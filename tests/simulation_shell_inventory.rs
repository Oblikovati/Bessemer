//! Combined ShellManager identity and original-profile interoperability (#160).
#[path = "simulation_shell_inventory/families.rs"]
mod families;
#[path = "support/inventory_packet.rs"]
mod inventory_packet;
#[allow(
    dead_code,
    reason = "Shared projection helpers also serve complete field/history corpus tests."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use inventory_packet::{InventoryTotals, InventoryWire};

const WIRE: InventoryWire = InventoryWire {
    storage: "ShellManager",
    has_profile: true,
};
const NATIVE_IDENTITIES: &[u8] =
    include_bytes!("../docs/features/simulation-shell-inventory/native-identities.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-shell-inventory/owned.bin");
const NATIVE_TOTALS: InventoryTotals = InventoryTotals {
    streams: 936,
    logical_bytes: 1648966,
    arguments: 1008,
    accepted: 962,
    failed: 46,
    incomplete: 1,
    families: [361, 69, 234, 15, 237, 11, 9],
};

#[test]
fn owned_all_seven_families_preserve_asymmetric_profiles_and_incomplete_occurrences() {
    let totals = inventory_packet::replay_packet(OWNED, WIRE, families::rebuild);
    assert_eq!(
        totals,
        InventoryTotals {
            streams: 9,
            logical_bytes: 1550,
            arguments: 13,
            accepted: 10,
            failed: 3,
            incomplete: 1,
            families: [1, 1, 1, 1, 1, 1, 3],
        }
    );
}

#[test]
#[should_panic(expected = "duplicate ShellManager identity")]
fn duplicate_shell_occurrences_cannot_inflate_completion() {
    inventory_packet::replay_packet(&[OWNED, OWNED].concat(), WIRE, families::rebuild);
}

#[test]
#[should_panic(expected = "expected explicit ShellManager family 0..=6")]
fn unknown_shell_family_fails_before_codec_dispatch() {
    let mut packet = OWNED.to_vec();
    packet[..4].copy_from_slice(&99u32.to_le_bytes());
    inventory_packet::replay_packet(&packet, WIRE, families::rebuild);
}

#[test]
#[should_panic(expected = "expected exact complete manifest")]
fn renamed_shell_identity_cannot_replace_an_equal_count_inventory() {
    let manifest = inventory_packet::identity_manifest(OWNED, WIRE);
    let mut packet = OWNED.to_vec();
    packet[8] = b'z';
    inventory_packet::assert_manifest(&packet, &manifest, WIRE);
}

fn first_profile_offset() -> usize {
    let mut cursor = &OWNED[4..];
    for _ in 0..3 {
        let size = load_projection::read_word(&mut cursor) as usize;
        load_projection::take_projection(&mut cursor, size);
    }
    load_projection::read_word(&mut cursor);
    OWNED.len() - cursor.len() + 4
}

#[test]
#[should_panic(expected = "expected reader 0 or writer 1")]
fn unknown_profile_fails_before_codec_dispatch() {
    let mut packet = OWNED.to_vec();
    let offset = first_profile_offset();
    packet[offset..offset + 4].copy_from_slice(&2u32.to_le_bytes());
    inventory_packet::replay_packet(&packet, WIRE, families::rebuild);
}

#[test]
#[ignore = "requires complete freshly recovered native ShellManager inventory packet"]
fn native_all_identities_preserve_original_profiles_and_construct_complete_fields() {
    let path = std::env::var_os("BESSEMER_SHELL_INVENTORY_NATIVE_INPUT")
        .expect("set complete fresh ShellManager packet");
    let packet = std::fs::read(path).unwrap();
    inventory_packet::assert_manifest(&packet, NATIVE_IDENTITIES, WIRE);
    let (totals, rebuilt) =
        inventory_packet::replay_packet_with_output(&packet, WIRE, families::rebuild);
    assert_eq!(totals, NATIVE_TOTALS);
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_INVENTORY_OUTPUT") {
        std::fs::write(path, rebuilt).unwrap();
    }
}
