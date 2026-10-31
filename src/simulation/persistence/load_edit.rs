use super::load_snapshot::BinaryLoadSnapshot;
use super::thermal::{SimulationThermalField, SimulationThermalValue};
use crate::{Result, simulation::invalid};

pub(super) fn replaced_field<S: BinaryLoadSnapshot>(
    source: &S,
    ordinal: usize,
    replacement: &SimulationThermalValue,
    reparse: impl FnOnce(Vec<SimulationThermalField>) -> Result<S>,
) -> Result<S> {
    let original = source.fields().get(ordinal).ok_or_else(|| {
        invalid(
            &format!("{} replacement", source.subject()),
            format!("ordinal {ordinal}; expected < {}", source.fields().len()),
        )
    })?;
    if original.value == *replacement {
        return Ok(source.clone());
    }
    let mut fields = source.fields().to_vec();
    let target = fields.get_mut(ordinal).ok_or_else(|| {
        invalid(
            &format!("{} replacement", source.subject()),
            format!("ordinal {ordinal}; expected an existing copied field"),
        )
    })?;
    target.value = replacement.clone();
    let result = reparse(fields)?;
    verify_preservation(source, &result, ordinal)?;
    Ok(result)
}

pub(super) fn require_kind(
    original: &SimulationThermalValue,
    replacement: &SimulationThermalValue,
    path: &str,
) -> Result<()> {
    if std::mem::discriminant(original) == std::mem::discriminant(replacement) {
        return Ok(());
    }
    Err(invalid(
        path,
        format!(
            "replacement kind {}; expected {}",
            kind_name(replacement),
            kind_name(original)
        ),
    ))
}

fn kind_name(value: &SimulationThermalValue) -> &'static str {
    match value {
        SimulationThermalValue::Signed(_) => "Signed",
        SimulationThermalValue::Unsigned(_) => "Unsigned",
        SimulationThermalValue::DoubleBits(_) => "DoubleBits",
        SimulationThermalValue::Text(_) => "Text",
        SimulationThermalValue::Bytes(_) => "Bytes",
        SimulationThermalValue::Embedded(_) => "Embedded",
    }
}

fn verify_preservation<S: BinaryLoadSnapshot>(
    source: &S,
    result: &S,
    ordinal: usize,
) -> Result<()> {
    let (start, end) = field_span(source, ordinal)?;
    let (next_start, next_end) = field_span(result, ordinal)?;
    if start == next_start
        && source.wire_bytes()[..start] == result.wire_bytes()[..next_start]
        && source.wire_bytes()[end..] == result.wire_bytes()[next_end..]
    {
        return Ok(());
    }
    Err(invalid(
        &format!("{} edit preservation", source.subject()),
        format!(
            "source span {start}..{end}, result {next_start}..{next_end}; expected identical prefix and suffix"
        ),
    ))
}

fn field_span<S: BinaryLoadSnapshot>(collection: &S, ordinal: usize) -> Result<(usize, usize)> {
    collection
        .field_offsets()
        .get(ordinal)
        .copied()
        .ok_or_else(|| {
            invalid(
                &format!("{} field span", collection.subject()),
                format!(
                    "ordinal {ordinal}; expected < {} complete field spans",
                    collection.fields().len()
                ),
            )
        })
}
