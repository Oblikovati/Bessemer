use bessemer::{Result, model_archives::ModelArchiveLimits, simulation::*};

const OUTER_BYTES: usize = 8_000_000;
const OUTER_FIELDS: usize = 1_000_000;
const OUTER_ENTRIES: usize = 2_000_000;
const EMBEDDED: SimulationThermalLimits = SimulationThermalLimits {
    maximum_bytes: OUTER_BYTES,
    maximum_fields: OUTER_FIELDS,
    maximum_entries: OUTER_ENTRIES,
    maximum_text_bytes: OUTER_BYTES,
    embedded: ModelArchiveLimits {
        maximum_bytes: OUTER_BYTES,
        maximum_fields: OUTER_FIELDS,
        maximum_entries: OUTER_ENTRIES,
        maximum_depth: 256,
        maximum_objects: 100_000,
    },
};

macro_rules! construct_complete {
    ($archive:ident, $raw:expr, $version:expr, $limits:expr) => {{
        let body = $archive::from_bytes($raw, $version, $limits)?;
        let rebuilt = $archive::from_fields(body.fields().to_vec(), $version, $limits)?;
        assert_eq!(rebuilt.fields(), body.fields());
        assert_eq!(rebuilt.field_offsets(), body.field_offsets());
        assert_eq!(body.coverage().checked_bytes, $raw.len());
        Ok(rebuilt.to_bytes())
    }};
}

macro_rules! outer_limits {
    ($limits:ident) => {
        $limits {
            maximum_bytes: OUTER_BYTES,
            maximum_fields: OUTER_FIELDS,
            maximum_entries: OUTER_ENTRIES,
            maximum_text_bytes: OUTER_BYTES,
        }
    };
}

pub(super) fn rebuild(
    family: u32,
    raw: &[u8],
    version: SimulationPersistVersion,
) -> Result<Vec<u8>> {
    let manager = outer_limits!(SimulationSolidManagerLimits);
    let material = outer_limits!(SimulationMaterialLimits);
    let surface = outer_limits!(SimulationSurfaceBodyLimits);
    match family {
        0 => construct_complete!(SimulationSolidManagerProperty, raw, version, manager),
        1 => component(raw, version),
        2 => construct_complete!(SimulationSolidBodyProperty, raw, version, EMBEDDED),
        3 => construct_complete!(SimulationSheetProperty, raw, version, EMBEDDED),
        4 => construct_complete!(SimulationStudyMaterial, raw, version, material),
        5 => construct_complete!(SimulationSurfaceBody, raw, version, surface),
        6 => construct_complete!(SimulationComponentPersistId, raw, version, EMBEDDED),
        _ => panic!("family {family}; expected explicit SolidManager family 0..=6"),
    }
}

fn component(raw: &[u8], version: SimulationPersistVersion) -> Result<Vec<u8>> {
    let kind = SimulationSolidChildKind::Component;
    let limits = outer_limits!(SimulationSolidChildLimits);
    let body = SimulationSolidChildProperty::from_bytes(raw, version, kind, limits)?;
    let rebuilt =
        SimulationSolidChildProperty::from_fields(body.fields().to_vec(), version, kind, limits)?;
    assert_eq!(rebuilt.fields(), body.fields());
    assert_eq!(rebuilt.field_offsets(), body.field_offsets());
    assert_eq!(body.coverage().checked_bytes, raw.len());
    Ok(rebuilt.to_bytes())
}
