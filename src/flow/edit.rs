use super::*;

impl FlowCarrier {
    /// Replaces one occurrence on a copy; e.g. `let edit = carrier.replace_payload(&id, b"new")?`.
    pub fn replace_payload(&self, id: &FlowRecordId, payload: &[u8]) -> Result<FlowCarrierEdit> {
        let span = selected_span(self, id)?;
        let result = if &self.as_bytes()[span.start + 12..span.end] == payload {
            self.clone()
        } else {
            replaced_carrier(self, span, payload)?
        };
        let history = edit_history(self, &result, id.ordinal);
        Ok(FlowCarrierEdit {
            source: self.clone(),
            result,
            history,
        })
    }
}

fn selected_span(carrier: &FlowCarrier, id: &FlowRecordId) -> Result<FlowSpan> {
    if !Arc::ptr_eq(&carrier.revision, &id.revision) {
        return Err(identity_failure(id.ordinal, carrier.revision.spans.len()));
    }
    carrier
        .revision
        .spans
        .get(id.ordinal)
        .copied()
        .ok_or_else(|| identity_failure(id.ordinal, carrier.revision.spans.len()))
}

fn identity_failure(ordinal: usize, count: usize) -> BessemerError {
    failure(
        "record identity",
        FailureClass::Invalid,
        format!(
            "foreign, stale or invalid ordinal {ordinal}; expected this immutable revision and ordinal below {count}"
        ),
    )
}

fn replaced_carrier(carrier: &FlowCarrier, span: FlowSpan, payload: &[u8]) -> Result<FlowCarrier> {
    let size = replacement_size(carrier, span, payload.len())?;
    let mut wire = Vec::with_capacity(size);
    wire.extend_from_slice(&carrier.as_bytes()[..span.start + 4]);
    wire.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    wire.extend_from_slice(payload);
    wire.extend_from_slice(&carrier.as_bytes()[span.end..]);
    FlowCarrier::from_bytes(&wire, carrier.revision.limits)
}

fn replacement_size(carrier: &FlowCarrier, span: FlowSpan, payload: usize) -> Result<usize> {
    let limits = carrier.revision.limits;
    let unchanged = carrier.as_bytes().len() - (span.end - span.start - 12);
    let available = limits.maximum_wire_bytes - unchanged;
    if payload > limits.maximum_payload_bytes || payload > available {
        return Err(failure(
            "replacement payload",
            FailureClass::Invalid,
            format!(
                "{payload} bytes; expected at most {} payload bytes and {available} remaining carrier bytes",
                limits.maximum_payload_bytes
            ),
        ));
    }
    Ok(unchanged + payload)
}

fn edit_history(
    source: &FlowCarrier,
    result: &FlowCarrier,
    selected: usize,
) -> Vec<FlowRecordHistory> {
    let unchanged = Arc::ptr_eq(&source.revision, &result.revision);
    source
        .records()
        .zip(result.records())
        .enumerate()
        .map(|(index, (before, after))| FlowRecordHistory {
            source: before.id,
            result: after.id,
            relation: if index == selected && !unchanged {
                FlowPayloadRelation::Derived
            } else {
                FlowPayloadRelation::Copied
            },
        })
        .collect()
}
