use super::*;
use crate::records::dword;

pub(super) fn fields(
    bytes: &[u8],
    layout: codec::ScalarLayout,
    revision: &SimulationPersistScalarRevision,
) -> Result<Arc<[SimulationPersistScalarField]>> {
    let mut fields = header_fields(layout, revision)?;
    for index in 0..layout.used {
        let offset = layout.header + index * layout.width;
        let value = if layout.width == 4 {
            SimulationPersistScalarValue::I32(dword(bytes, offset)? as i32)
        } else {
            SimulationPersistScalarValue::I64(codec::i64_word(bytes, offset)?)
        };
        append_field(
            &mut fields,
            revision,
            offset,
            SimulationPersistScalarRole::Entry(index),
            value,
        );
    }
    Ok(fields.into())
}

fn append_field(
    fields: &mut Vec<SimulationPersistScalarField>,
    revision: &SimulationPersistScalarRevision,
    stream_offset: usize,
    role: SimulationPersistScalarRole,
    value: SimulationPersistScalarValue,
) {
    fields.push(SimulationPersistScalarField {
        id: SimulationPersistScalarId {
            revision: revision.clone(),
            ordinal: fields.len(),
        },
        stream_offset,
        role,
        value,
    });
}

fn header_fields(
    layout: codec::ScalarLayout,
    revision: &SimulationPersistScalarRevision,
) -> Result<Vec<SimulationPersistScalarField>> {
    let mut fields = field_storage(layout)?;
    append_field(
        &mut fields,
        revision,
        0,
        SimulationPersistScalarRole::UsedCount,
        SimulationPersistScalarValue::I32(layout.used as i32),
    );
    if let Some(capacity) = layout.capacity {
        append_field(
            &mut fields,
            revision,
            4,
            SimulationPersistScalarRole::AllocationCapacity,
            SimulationPersistScalarValue::I32(capacity),
        );
    }
    Ok(fields)
}

fn field_storage(layout: codec::ScalarLayout) -> Result<Vec<SimulationPersistScalarField>> {
    let field_count = layout.used + layout.header / 4;
    let mut fields = Vec::new();
    fields.try_reserve_exact(field_count).map_err(|error| invalid("scalar field allocation", format!("{field_count} used/header fields, {error}; expected available complete field storage independent of capacity")))?;
    Ok(fields)
}

pub(super) fn coverage(
    archive: &SimulationPersistScalarArchive,
) -> SimulationPersistScalarCoverage {
    SimulationPersistScalarCoverage {
        profile: archive.profile,
        completion: if archive.layout.used == 0 {
            SimulationPersistScalarCompletion::CompletedEmpty
        } else {
            SimulationPersistScalarCompletion::CompletedValues
        },
        check_mode: SimulationPersistScalarCheckMode::ExactCompleteFrame,
        checked_bytes: archive.bytes.len(),
        checked_header_bytes: archive.layout.header,
        checked_entries: archive.layout.used,
        maximum_entries: archive.maximum_entries,
        diagnostics: [
            SimulationPersistScalarDiagnostic::NativeApplicationNotExecuted,
            SimulationPersistScalarDiagnostic::SemanticMappingNotInferred,
            SimulationPersistScalarDiagnostic::CapacityNotAllocated,
        ],
        requested_pattern_error_bits: 0,
    }
}
