use super::*;

fn numeric_choice(field: &SimulationMeshControlField) -> Option<SimulationMeshControlValue> {
    match &field.value {
        SimulationMeshControlValue::Signed(word) if field.path.ends_with(".base.integer") => {
            Some(SimulationMeshControlValue::Signed(word ^ 32))
        }
        SimulationMeshControlValue::Unsigned(word)
            if field.path.ends_with(".base.scalar_block.e8") =>
        {
            Some(SimulationMeshControlValue::Unsigned(word ^ 0x40000001))
        }
        SimulationMeshControlValue::DoubleBits(bits) if field.path.ends_with(".mesh.double370") => {
            Some(SimulationMeshControlValue::DoubleBits(
                bits ^ 0x8000000000000001,
            ))
        }
        _ => None,
    }
}

pub(super) fn choices(
    body: &SimulationMeshControlRecords,
) -> Vec<(usize, SimulationMeshControlValue)> {
    body.fields()
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            numeric_choice(field)
                .or_else(|| {
                    typed_load_values::text_or_blob_choice(field, LIMITS.maximum_text_bytes)
                })
                .or_else(|| typed_load_values::archive_choice(body.fields(), field))
                .map(|value| (index, value))
        })
        .collect()
}
