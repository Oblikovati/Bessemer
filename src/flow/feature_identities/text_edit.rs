use super::*;

pub(super) fn payload(
    list: &FlowFeatureIdentities,
    object: &FlowFeatureIdentity,
    units: &[u16],
) -> Result<Vec<u8>> {
    let size = replacement_size(list, object, units)?;
    let count_start = object.start + 12;
    let text_end = object.end - 1;
    let mut payload = Vec::with_capacity(size);
    payload.extend_from_slice(&list.payload[..count_start]);
    payload.extend_from_slice(&(units.len() as i32).to_le_bytes());
    payload.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
    payload.extend_from_slice(&list.payload[text_end..]);
    Ok(payload)
}

fn replacement_size(
    list: &FlowFeatureIdentities,
    object: &FlowFeatureIdentity,
    units: &[u16],
) -> Result<usize> {
    validate_units(list, object, units)?;
    let size = (list.payload.len() - object.text.len() * 2)
        .checked_add(units.len() * 2)
        .ok_or_else(|| {
            feature_failure(
                FailureClass::Invalid,
                format!(
                    "replacement text {} units; expected payload size fitting usize",
                    units.len()
                ),
            )
        })?;
    check_wire_bound(list, size)?;
    Ok(size)
}

fn check_wire_bound(list: &FlowFeatureIdentities, size: usize) -> Result<()> {
    let limits = list.source.revision.limits;
    let remaining = limits.maximum_wire_bytes - (list.source.as_bytes().len() - list.payload.len());
    let maximum = remaining.min(limits.maximum_payload_bytes);
    if size <= maximum {
        return Ok(());
    }
    Err(feature_failure(
        FailureClass::Invalid,
        format!(
            "replacement payload {size} bytes; expected at most {maximum} within original carrier/payload limits"
        ),
    ))
}

fn validate_units(
    list: &FlowFeatureIdentities,
    object: &FlowFeatureIdentity,
    units: &[u16],
) -> Result<()> {
    let remaining = list.options.maximum_text_units - (list.total_text_units - object.text.len());
    if units.len() > remaining || units.len() > i32::MAX as usize {
        return Err(feature_failure(
            FailureClass::Invalid,
            format!(
                "replacement text has {} units; expected at most {} remaining units and signed-i32 count",
                units.len(),
                remaining
            ),
        ));
    }
    super::decode::validate_text(units)?;
    Ok(())
}
