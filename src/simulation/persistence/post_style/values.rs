use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use super::{
    SimulationPostStyleField, SimulationPostStyleLimits, SimulationPostStyleText,
    SimulationPostStyleValue,
};
use crate::{Result, simulation::invalid};

pub(super) use super::super::post_info::{
    info_require_kind as require_kind, info_to_wire_fields as to_wire_fields,
    info_wire_value as wire_value,
};

pub(super) fn from_wire_fields(
    fields: &[SimulationThermalField],
    limits: SimulationPostStyleLimits,
) -> Result<Vec<SimulationPostStyleField>> {
    fields
        .iter()
        .map(|field| {
            Ok(SimulationPostStyleField {
                path: field.path.clone(),
                value: typed_value(field, limits.maximum_text_bytes)?,
            })
        })
        .collect()
}

fn typed_value(
    field: &SimulationThermalField,
    maximum_bytes: usize,
) -> Result<SimulationPostStyleValue> {
    match &field.value {
        SimulationThermalValue::Signed(word) => Ok(SimulationPostStyleValue::Signed(*word)),
        SimulationThermalValue::Unsigned(bits) if is_float_path(&field.path) => {
            Ok(SimulationPostStyleValue::Float32Bits(*bits))
        }
        SimulationThermalValue::Unsigned(bits) => Ok(SimulationPostStyleValue::RawWordBits(*bits)),
        SimulationThermalValue::Bytes(raw) if field.path.as_ref() == "style.text" => {
            Ok(SimulationPostStyleValue::Text(
                SimulationPostStyleText::from_frame(raw, maximum_bytes)?,
            ))
        }
        value => Err(invalid(
            &field.path,
            format!(
                "{:?}; expected PostStyle signed/raw32/float32 or atomic style text",
                std::mem::discriminant(value)
            ),
        )),
    }
}

fn is_float_path(path: &str) -> bool {
    path.contains(".color_") || matches!(path, "legend.word_ac" | "legend.word_b0")
}
