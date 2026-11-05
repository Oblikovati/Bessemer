use super::*;
use crate::composer::ComposerLimits;

pub(super) fn read(bytes: &[u8], limits: ElectricalLimits) -> Result<ElectricalArchive> {
    validate_limits(limits)?;
    if bytes.len() > limits.maximum_bytes {
        return Err(failure(
            FailureClass::Invalid,
            "encoded bytes",
            format!(
                "bytes {}; expected <= {}",
                bytes.len(),
                limits.maximum_bytes
            ),
        ));
    }
    let mut adapted = b"SMG".to_vec();
    adapted.extend_from_slice(bytes);
    let backend =
        ComposerArchive::from_bytes(&adapted, backend_limits(limits)).map_err(electrical_error)?;
    Ok(ElectricalArchive { backend, limits })
}

fn validate_limits(limits: ElectricalLimits) -> Result<()> {
    let values = [
        limits.maximum_bytes,
        limits.maximum_members,
        limits.maximum_member_bytes,
        limits.maximum_expanded_bytes,
        limits.maximum_ini_records,
    ];
    if values.contains(&0) {
        return Err(failure(
            FailureClass::Invalid,
            "limits",
            format!("limits {limits:?}; expected every limit >= 1"),
        ));
    }
    Ok(())
}

fn backend_limits(limits: ElectricalLimits) -> ComposerLimits {
    ComposerLimits {
        maximum_bytes: limits.maximum_bytes.saturating_add(3),
        maximum_members: limits.maximum_members,
        maximum_member_bytes: limits.maximum_member_bytes,
        maximum_expanded_bytes: limits.maximum_expanded_bytes,
    }
}
