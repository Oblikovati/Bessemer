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

fn text_or_blob_choice(
    field: &SimulationShellPropertyField,
) -> Option<SimulationShellPropertyValue> {
    match &field.value {
        SimulationShellPropertyValue::Text(_) => Some(SimulationShellPropertyValue::Text(
            SimulationPersistText::from_payload(Some(&[b'X', 0, 0]), LIMITS.maximum_text_bytes)
                .unwrap(),
        )),
        SimulationShellPropertyValue::Bytes(bytes) if !bytes.is_empty() => {
            let mut replacement = bytes.to_vec();
            replacement[0] ^= 0xff;
            Some(SimulationShellPropertyValue::Bytes(Arc::from(replacement)))
        }
        _ => None,
    }
}

fn archive_choice(
    body: &SimulationShellProperty,
    field: &SimulationShellPropertyField,
) -> Option<SimulationShellPropertyValue> {
    let SimulationShellPropertyValue::Embedded(original) = &field.value else {
        return None;
    };
    body.fields()
        .iter()
        .find_map(|candidate| match &candidate.value {
            SimulationShellPropertyValue::Embedded(donor)
                if donor.to_bytes().unwrap() != original.to_bytes().unwrap() =>
            {
                Some(candidate.value.clone())
            }
            _ => None,
        })
}

pub(super) fn choices(
    body: &SimulationShellProperty,
) -> Vec<(usize, SimulationShellPropertyValue)> {
    body.fields()
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            numeric_choice(field)
                .or_else(|| text_or_blob_choice(field))
                .or_else(|| archive_choice(body, field))
                .map(|value| (index, value))
        })
        .collect()
}

pub(super) fn wire(value: &SimulationShellPropertyValue) -> Vec<u8> {
    match value {
        SimulationShellPropertyValue::Signed(word) => word.to_le_bytes().to_vec(),
        SimulationShellPropertyValue::Unsigned(word) => word.to_le_bytes().to_vec(),
        SimulationShellPropertyValue::DoubleBits(bits) => bits.to_le_bytes().to_vec(),
        SimulationShellPropertyValue::Text(text) => text.to_bytes(),
        SimulationShellPropertyValue::Bytes(bytes) => bytes.to_vec(),
        SimulationShellPropertyValue::Embedded(archive) => archive.to_bytes().unwrap(),
    }
}
