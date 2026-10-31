use super::thermal::{SimulationThermalField, SimulationThermalFieldId};
use crate::archive_codec::ArchiveRevision;
use crate::{Result, simulation::invalid};

pub(super) fn field_id(
    revision: &ArchiveRevision,
    ordinal: usize,
    count: usize,
    family: &str,
) -> Result<SimulationThermalFieldId> {
    if ordinal >= count {
        return Err(invalid(
            &format!("{family} field identity"),
            format!("ordinal {ordinal}; expected < {count}"),
        ));
    }
    Ok(SimulationThermalFieldId {
        revision: revision.clone(),
        ordinal,
    })
}

pub(super) fn field<'a>(
    fields: &'a [SimulationThermalField],
    revision: &ArchiveRevision,
    id: &SimulationThermalFieldId,
    family: &str,
) -> Result<&'a SimulationThermalField> {
    if id.revision != *revision {
        return Err(invalid(
            &format!("{family} field revision"),
            format!(
                "ordinal {} from another revision; expected this collection revision",
                id.ordinal
            ),
        ));
    }
    fields.get(id.ordinal).ok_or_else(|| {
        invalid(
            &format!("{family} field identity"),
            format!("ordinal {}; expected < {}", id.ordinal, fields.len()),
        )
    })
}
