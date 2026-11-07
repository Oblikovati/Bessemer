use super::thermal::{SimulationThermalField, SimulationThermalValue};
use crate::{
    Result,
    simulation::{SimulationPersistText, invalid},
};
use std::sync::Arc;

pub(super) struct BinaryLoadParts {
    pub bytes: Arc<[u8]>,
    pub fields: Vec<SimulationThermalField>,
    pub offsets: Vec<(usize, usize)>,
}

pub(super) fn rebase_prefix(
    bytes: Arc<[u8]>,
    mut fields: Vec<SimulationThermalField>,
    offsets: Vec<(usize, usize)>,
    maximum_text: usize,
) -> Result<BinaryLoadParts> {
    if fields.len() != offsets.len() {
        return Err(invalid(
            "prefix field spans",
            format!(
                "{} fields, {} spans; expected equal physical counts",
                fields.len(),
                offsets.len()
            ),
        ));
    }
    for (field, span) in fields.iter_mut().zip(&offsets) {
        rebase_text(field, *span, bytes.clone(), maximum_text)?;
    }
    Ok(BinaryLoadParts {
        bytes,
        fields,
        offsets,
    })
}

fn rebase_text(
    field: &mut SimulationThermalField,
    span: (usize, usize),
    parent: Arc<[u8]>,
    maximum: usize,
) -> Result<()> {
    if !matches!(field.value, SimulationThermalValue::Text(_)) {
        return Ok(());
    }
    let text = SimulationPersistText::read_shared(parent, span.0, maximum)?;
    if text.source_offsets().2 != span.1 {
        return Err(invalid(
            &field.path,
            format!(
                "text ends at {}, span {:?}; expected matching body boundary",
                text.source_offsets().2,
                span
            ),
        ));
    }
    field.value = SimulationThermalValue::Text(text);
    Ok(())
}
