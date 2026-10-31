use super::SimulationShellGroupLimits;
use crate::{Result, archive_codec::ArchiveString, simulation::invalid};
use std::sync::Arc;

/// Complete primitive representation, preserving CString prefix widths and raw double bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationShellGroupValue {
    /// Exact signed four-byte pattern.
    Signed(i32),
    /// Exact eight-byte pattern, including NaN payloads and signed zero.
    DoubleBits(u64),
    /// Existing MFC CString encoding and retained length-prefix width, without locale conversion.
    Text(ArchiveString),
}

/// One immutable physical field in complete stored order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationShellGroupField {
    /// Grammar-derived role, retaining every record ordinal.
    pub path: Arc<str>,
    /// Complete selected stored representation.
    pub value: SimulationShellGroupValue,
}

pub(super) fn value_size(value: &SimulationShellGroupValue) -> Result<usize> {
    match value {
        SimulationShellGroupValue::Signed(_) => Ok(4),
        SimulationShellGroupValue::DoubleBits(_) => Ok(8),
        SimulationShellGroupValue::Text(text) => text.encoded_size(),
    }
}

pub(super) fn encode_value(value: &SimulationShellGroupValue, output: &mut Vec<u8>) -> Result<()> {
    match value {
        SimulationShellGroupValue::Signed(value) => output.extend_from_slice(&value.to_le_bytes()),
        SimulationShellGroupValue::DoubleBits(value) => {
            output.extend_from_slice(&value.to_le_bytes())
        }
        SimulationShellGroupValue::Text(text) => return text.encode(output),
    }
    Ok(())
}

fn require_encoded_size(size: usize, maximum: usize) -> Result<()> {
    if size <= maximum {
        return Ok(());
    }
    Err(invalid(
        "ShellManagerGroup output",
        format!("{size} bytes; expected <= {maximum}"),
    ))
}

pub(super) fn value_bytes(value: &SimulationShellGroupValue, maximum: usize) -> Result<Vec<u8>> {
    let size = value_size(value)?;
    require_encoded_size(size, maximum)?;
    let mut bytes = super::super::reserve_bytes(size)?;
    encode_value(value, &mut bytes)?;
    Ok(bytes)
}

fn fields_size(fields: &[SimulationShellGroupField], maximum: usize) -> Result<usize> {
    let mut size = 0usize;
    for field in fields {
        let next = value_size(&field.value)?;
        size = size.checked_add(next).ok_or_else(|| {
            invalid(
                "ShellManagerGroup output",
                format!("{size} + {next} bytes; expected representable total"),
            )
        })?;
        require_encoded_size(size, maximum)?;
    }
    Ok(size)
}

pub(super) fn encode_fields(
    fields: &[SimulationShellGroupField],
    limits: SimulationShellGroupLimits,
) -> Result<Vec<u8>> {
    if fields.len() > limits.maximum_fields {
        return Err(invalid(
            "ShellManagerGroup construction",
            format!(
                "{} fields; expected <= {}",
                fields.len(),
                limits.maximum_fields
            ),
        ));
    }
    let size = fields_size(fields, limits.maximum_bytes)?;
    let mut bytes = super::super::reserve_bytes(size)?;
    for field in fields {
        encode_value(&field.value, &mut bytes)?;
    }
    Ok(bytes)
}
