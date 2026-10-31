use bessemer::simulation::*;
use std::sync::Arc;

pub(super) fn text_or_blob_choice(
    field: &SimulationThermalField,
    maximum_text_bytes: usize,
) -> Option<SimulationThermalValue> {
    match &field.value {
        SimulationThermalValue::Text(_) => Some(SimulationThermalValue::Text(
            SimulationPersistText::from_payload(Some(&[b'X', 0, 0]), maximum_text_bytes).unwrap(),
        )),
        SimulationThermalValue::Bytes(bytes) if !bytes.is_empty() => {
            let mut replacement = bytes.to_vec();
            replacement[0] ^= 0xff;
            Some(SimulationThermalValue::Bytes(Arc::from(replacement)))
        }
        _ => None,
    }
}

pub(super) fn archive_choice(
    fields: &[SimulationThermalField],
    field: &SimulationThermalField,
) -> Option<SimulationThermalValue> {
    let SimulationThermalValue::Embedded(original) = &field.value else {
        return None;
    };
    fields.iter().find_map(|candidate| match &candidate.value {
        SimulationThermalValue::Embedded(donor)
            if donor.to_bytes().unwrap() != original.to_bytes().unwrap() =>
        {
            Some(candidate.value.clone())
        }
        _ => None,
    })
}

pub(super) fn value_kind(value: &SimulationThermalValue) -> usize {
    match value {
        SimulationThermalValue::Signed(_) => 0,
        SimulationThermalValue::Unsigned(_) => 1,
        SimulationThermalValue::DoubleBits(_) => 2,
        SimulationThermalValue::Text(_) => 3,
        SimulationThermalValue::Bytes(_) => 4,
        SimulationThermalValue::Embedded(_) => 5,
    }
}
pub(super) fn wire(value: &SimulationThermalValue) -> Vec<u8> {
    match value {
        SimulationThermalValue::Signed(word) => word.to_le_bytes().to_vec(),
        SimulationThermalValue::Unsigned(word) => word.to_le_bytes().to_vec(),
        SimulationThermalValue::DoubleBits(bits) => bits.to_le_bytes().to_vec(),
        SimulationThermalValue::Text(text) => text.to_bytes(),
        SimulationThermalValue::Bytes(bytes) => bytes.to_vec(),
        SimulationThermalValue::Embedded(archive) => archive.to_bytes().unwrap(),
    }
}

pub(super) fn shifted_span(
    span: (usize, usize),
    target: (usize, usize),
    index: usize,
    changed: usize,
    width: usize,
) -> (usize, usize) {
    if index < changed {
        return span;
    }
    if index == changed {
        return (target.0, target.0 + width);
    }
    let delta = width as isize - (target.1 - target.0) as isize;
    (
        span.0.checked_add_signed(delta).unwrap(),
        span.1.checked_add_signed(delta).unwrap(),
    )
}
