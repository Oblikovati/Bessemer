use bessemer::{Result, model_archives::ModelArchiveLimits, simulation::*};

#[path = "../simulation_solid_inventory/families.rs"]
mod solid_families;

const BODY_BYTES: usize = 8_000_000;
const BODY_FIELDS: usize = 1_000_000;
const BODY_ENTRIES: usize = 2_000_000;
const EMBEDDED: SimulationThermalLimits = SimulationThermalLimits {
    maximum_bytes: BODY_BYTES,
    maximum_fields: BODY_FIELDS,
    maximum_entries: BODY_ENTRIES,
    maximum_text_bytes: BODY_BYTES,
    embedded: ModelArchiveLimits {
        maximum_bytes: BODY_BYTES,
        maximum_fields: BODY_FIELDS,
        maximum_entries: BODY_ENTRIES,
        maximum_depth: 256,
        maximum_objects: 100_000,
    },
};

macro_rules! construct_profile {
    ($archive:ident, $raw:expr, $version:expr, $context:expr, $limits:expr) => {{
        let context = $context;
        let body = $archive::from_bytes($raw, $version, context.clone(), $limits)?;
        let rebuilt = $archive::from_fields(body.fields().to_vec(), $version, context, $limits)?;
        assert_eq!(body.to_bytes(), $raw);
        assert_eq!(rebuilt.fields(), body.fields());
        assert_eq!(rebuilt.field_offsets(), body.field_offsets());
        assert_eq!(body.coverage().checked_bytes, $raw.len());
        Ok(rebuilt.to_bytes())
    }};
}

macro_rules! plain_limits {
    ($limits:ident) => {
        $limits {
            maximum_bytes: BODY_BYTES,
            maximum_fields: BODY_FIELDS,
            maximum_entries: BODY_ENTRIES,
            maximum_text_bytes: BODY_BYTES,
        }
    };
}

fn wire_profile(profile: Option<u32>) -> SimulationShellWireProfile {
    match profile {
        Some(0) => SimulationShellWireProfile::Reader,
        Some(1) => SimulationShellWireProfile::Writer,
        other => panic!("profile {other:?}; expected explicit reader 0 or writer 1"),
    }
}

pub(super) fn rebuild(
    family: u32,
    raw: &[u8],
    version: SimulationPersistVersion,
    profile: Option<u32>,
) -> Result<Vec<u8>> {
    let wire = wire_profile(profile);
    match family {
        0 | 1 => manager(family, raw, version, wire),
        2 | 3 => property(family, raw, version, wire),
        4 => material(raw, version, wire),
        5 | 6 => composite(family, raw, version, wire),
        _ => panic!("family {family}; expected explicit ShellManager family 0..=6"),
    }
}

fn manager(
    family: u32,
    raw: &[u8],
    version: SimulationPersistVersion,
    wire: SimulationShellWireProfile,
) -> Result<Vec<u8>> {
    let header = plain_limits!(SimulationShellManagerLimits);
    let group = plain_limits!(SimulationShellGroupLimits);
    match family {
        0 => construct_profile!(SimulationShellManagerProperty, raw, version, wire, header),
        1 => construct_profile!(SimulationShellGroup, raw, version, wire, group),
        other => panic!("family {other}; expected header 0 or group 1"),
    }
}

fn property(
    family: u32,
    raw: &[u8],
    version: SimulationPersistVersion,
    wire: SimulationShellWireProfile,
) -> Result<Vec<u8>> {
    assert_eq!(wire, SimulationShellWireProfile::Reader);
    let context = SimulationShellPropertyContext {
        kind: if family == 2 {
            SimulationShellPropertyKind::Shells
        } else {
            SimulationShellPropertyKind::MidSurfShells
        },
        layer_presence: SimulationShellLayerPresence::Unknown,
    };
    construct_profile!(SimulationShellProperty, raw, version, context, EMBEDDED)
}

fn material(
    raw: &[u8],
    version: SimulationPersistVersion,
    wire: SimulationShellWireProfile,
) -> Result<Vec<u8>> {
    assert_eq!(wire, SimulationShellWireProfile::Reader);
    solid_families::rebuild(4, raw, version)
}

fn composite(
    family: u32,
    raw: &[u8],
    version: SimulationPersistVersion,
    wire: SimulationShellWireProfile,
) -> Result<Vec<u8>> {
    let profile = match (family, wire) {
        (5, SimulationShellWireProfile::Reader) => SimulationShellCompositeProfile::FlipTurnReader,
        (5, SimulationShellWireProfile::Writer) => SimulationShellCompositeProfile::FlipTurnWriter,
        (6, SimulationShellWireProfile::Reader) => SimulationShellCompositeProfile::ExtraReader,
        (6, SimulationShellWireProfile::Writer) => SimulationShellCompositeProfile::ExtraWriter,
        other => panic!("{other:?}; expected composite family 5 or 6 with explicit profile"),
    };
    construct_profile!(SimulationShellComposite, raw, version, profile, EMBEDDED)
}
