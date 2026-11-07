use super::SimulationShellLayerPresence;
use crate::{BessemerError, FailureClass, Result, simulation::invalid};

pub(super) fn selected(
    presence: &SimulationShellLayerPresence,
    count: usize,
    version: i32,
) -> Result<Option<&[bool]>> {
    if version < 891106 || count == 0 {
        return Ok(None);
    }
    match presence {
        SimulationShellLayerPresence::Known(values) if values.len() == count => Ok(Some(values)),
        SimulationShellLayerPresence::Known(values) => Err(invalid(
            "layer_presence",
            format!(
                "{} booleans; expected exactly {count} for nonempty newest shell layers",
                values.len()
            ),
        )),
        SimulationShellLayerPresence::Unknown => Err(missing_presence(count)),
    }
}

fn missing_presence(count: usize) -> BessemerError {
    BessemerError::new(
        "decode shell Property",
        "layer_presence",
        FailureClass::Unsupported,
        format!(
            "Unknown; incomplete runtime context, expected Known with {count} layer lookup booleans"
        ),
    )
}

pub(super) fn lookup(presence: Option<&[bool]>, index: usize) -> Result<Option<bool>> {
    let Some(values) = presence else {
        return Ok(None);
    };
    values.get(index).copied().map(Some).ok_or_else(|| {
        invalid(
            "layer_presence",
            format!(
                "index {index}; expected within normalized vector of {} booleans",
                values.len()
            ),
        )
    })
}
