use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    kind: FlowOwnerStateKind,
    options: FlowOwnerStateOptions,
) -> Result<FlowOwnerState> {
    read_inner(source, id, kind, options).map_err(|error| table_boundary(error, kind.identifier()))
}
fn read_inner(
    source: &FlowCarrier,
    id: &FlowRecordId,
    kind: FlowOwnerStateKind,
    options: FlowOwnerStateOptions,
) -> Result<FlowOwnerState> {
    let wire = wire::read(
        source,
        id,
        kind.identifier(),
        FlowLegacyTableTransport::Raw,
        options.maximum_payload_bytes,
    )?;
    if kind == FlowOwnerStateKind::Modern434 && ![0, 1].contains(&wire.version) {
        return Err(table_failure(
            434,
            FailureClass::Unsupported,
            format!("version {}; expected zero or one", wire.version),
        ));
    }
    validate_extent(&wire, kind)?;
    Ok(FlowOwnerState {
        wire,
        kind,
        options,
    })
}
fn validate_extent(wire: &TableWire, kind: FlowOwnerStateKind) -> Result<()> {
    let expected = match kind {
        FlowOwnerStateKind::Legacy122 => 200,
        FlowOwnerStateKind::Modern434 => 29,
    };
    if wire.decoded.len() == expected {
        return Ok(());
    }
    Err(table_failure(
        kind.identifier(),
        FailureClass::Invalid,
        format!(
            "payload {} bytes; expected exactly {expected} raw bytes for {kind:?}",
            wire.decoded.len()
        ),
    ))
}
impl FlowOwnerStateKind {
    pub(super) fn identifier(self) -> u32 {
        match self {
            Self::Legacy122 => 122,
            Self::Modern434 => 434,
        }
    }
    pub(super) fn words_start(self) -> usize {
        match self {
            Self::Legacy122 => 0,
            Self::Modern434 => 4,
        }
    }
    pub(super) fn word_count(self) -> usize {
        match self {
            Self::Legacy122 => 50,
            Self::Modern434 => 6,
        }
    }
}
