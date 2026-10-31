use super::*;

fn numeric_choice(field: &SimulationMeshPropertyField) -> Option<SimulationMeshPropertyValue> {
    match &field.value {
        SimulationMeshPropertyValue::Signed(word) if field.path.as_ref() == "mesh.word594" => {
            Some(SimulationMeshPropertyValue::Signed(word ^ 32))
        }
        SimulationMeshPropertyValue::DoubleBits(bits)
            if field.path.as_ref() == "mesh.double5a0" =>
        {
            Some(SimulationMeshPropertyValue::DoubleBits(
                bits ^ 0x8000000000000001,
            ))
        }
        _ => None,
    }
}

pub(super) fn choices(body: &SimulationMeshProperty) -> Vec<(usize, SimulationMeshPropertyValue)> {
    body.fields()
        .iter()
        .enumerate()
        .flat_map(|(index, field)| {
            field_choices(body, field)
                .into_iter()
                .map(move |value| (index, value))
        })
        .collect()
}

fn field_choices(
    body: &SimulationMeshProperty,
    field: &SimulationMeshPropertyField,
) -> Vec<SimulationMeshPropertyValue> {
    if let Some(value) = numeric_choice(field)
        .or_else(|| typed_load_values::text_or_blob_choice(field, LIMITS.maximum_text_bytes))
    {
        return vec![value];
    }
    archive_donors(body, field)
}

fn archive_donors(
    body: &SimulationMeshProperty,
    field: &SimulationMeshPropertyField,
) -> Vec<SimulationMeshPropertyValue> {
    let SimulationMeshPropertyValue::Embedded(original) = &field.value else {
        return Vec::new();
    };
    let mut encodings = std::collections::BTreeSet::from([original.to_bytes().unwrap()]);
    body.fields()
        .iter()
        .filter_map(|candidate| match &candidate.value {
            SimulationMeshPropertyValue::Embedded(donor)
                if encodings.insert(donor.to_bytes().unwrap()) =>
            {
                Some(candidate.value.clone())
            }
            _ => None,
        })
        .collect()
}
