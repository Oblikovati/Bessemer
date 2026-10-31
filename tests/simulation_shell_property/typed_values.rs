use super::*;

fn numeric_choice(field: &SimulationShellPropertyField) -> Option<SimulationShellPropertyValue> {
    match &field.value {
        SimulationShellPropertyValue::Signed(word)
            if field.path.as_ref() == "property.base.integer" =>
        {
            Some(SimulationShellPropertyValue::Signed(word ^ 32))
        }
        SimulationShellPropertyValue::Unsigned(word)
            if field.path.as_ref() == "shell.words338_33c[0]" =>
        {
            Some(SimulationShellPropertyValue::Unsigned(word ^ 0x40000001))
        }
        SimulationShellPropertyValue::DoubleBits(bits)
            if matches!(field.path.as_ref(), "shell.double328" | "shell.double360") =>
        {
            Some(SimulationShellPropertyValue::DoubleBits(
                bits ^ 0x8000000000000001,
            ))
        }
        _ => None,
    }
}

pub(super) fn choices(
    body: &SimulationShellProperty,
) -> Vec<(usize, SimulationShellPropertyValue)> {
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

pub(super) use super::typed_load_values::wire;
