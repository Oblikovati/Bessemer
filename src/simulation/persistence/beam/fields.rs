use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Explicit beam scalar width alongside the existing complete shared wire values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationBeamValue {
    /// Signed two-byte physical field, never promoted to a four-byte word.
    SignedShort(i16),
    /// Shared word, raw double, text, blob or complete embedded archive value.
    Shared(SimulationThermalValue),
}

/// One immutable typed physical beam field in stored order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationBeamField {
    /// Complete grammar-derived physical role.
    pub path: Arc<str>,
    /// Complete width-preserving stored value.
    pub value: SimulationBeamValue,
}

pub(super) fn short_path(path: &str) -> bool {
    matches!(path, "record0.beam.shorte2" | "record0.beam.shorte0")
}

pub(super) fn project_field(field: &SimulationThermalField) -> Result<SimulationBeamField> {
    let value = if short_path(&field.path) {
        project_short(field)?
    } else {
        SimulationBeamValue::Shared(field.value.clone())
    };
    Ok(SimulationBeamField {
        path: field.path.clone(),
        value,
    })
}

pub(super) fn wire_value(value: &SimulationBeamValue) -> SimulationThermalValue {
    match value {
        SimulationBeamValue::SignedShort(value) => {
            SimulationThermalValue::Bytes(value.to_le_bytes().as_slice().into())
        }
        SimulationBeamValue::Shared(value) => value.clone(),
    }
}

pub(super) fn wire_field(field: &SimulationBeamField) -> SimulationThermalField {
    SimulationThermalField {
        path: field.path.clone(),
        value: wire_value(&field.value),
    }
}

fn project_short(field: &SimulationThermalField) -> Result<SimulationBeamValue> {
    let SimulationThermalValue::Bytes(raw) = &field.value else {
        return Err(invalid(
            &field.path,
            format!(
                "value {:?}; expected a complete two-byte short frame",
                field.value
            ),
        ));
    };
    let bytes: [u8; 2] = raw.as_ref().try_into().map_err(|_| {
        invalid(
            &field.path,
            format!("{} bytes; expected exactly 2", raw.len()),
        )
    })?;
    Ok(SimulationBeamValue::SignedShort(i16::from_le_bytes(bytes)))
}
