use super::thermal::SimulationThermalFieldId;
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

pub(super) fn field<'a, T>(
    fields: &'a [T],
    revision: &ArchiveRevision,
    id: &SimulationThermalFieldId,
    family: &str,
) -> Result<&'a T> {
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

pub(super) fn outer_field_relation(
    source: SimulationThermalFieldId,
    result: SimulationThermalFieldId,
    derived: bool,
) -> super::thermal::SimulationThermalHistory {
    use super::thermal::{
        SimulationThermalHistory, SimulationThermalHistoryParent, SimulationThermalRelation,
    };
    let mut parents = vec![SimulationThermalHistoryParent::Source(source)];
    if derived {
        parents.push(SimulationThermalHistoryParent::Replacement);
    }
    SimulationThermalHistory {
        parents: parents.into(),
        result,
        relation: if derived {
            SimulationThermalRelation::Derived
        } else {
            SimulationThermalRelation::Copied
        },
    }
}
