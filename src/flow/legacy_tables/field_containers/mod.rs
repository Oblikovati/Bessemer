//! Complete unversioned105/113 words and raw155 conditional groups retain exact physical identities.
mod group155;
mod queries;
mod selection;
mod types;
mod word_view;
mod words;
use super::*;
pub use group155::*;
pub use selection::*;
pub use types::*;
fn invalid(identifier: u32, cause: String) -> BessemerError {
    table_failure(identifier, FailureClass::Invalid, cause)
}
fn bounded_payload<'a>(
    source: &'a FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
    maximum: usize,
) -> Result<&'a [u8]> {
    let raw = super::super::payload_codec::record_payload(
        source,
        id,
        identifier,
        "Flow legacy field containers",
    )?;
    if raw.len() <= maximum {
        return Ok(raw);
    }
    Err(invalid(
        identifier,
        format!(
            "payload bytes {}; expected at most configured bound {maximum}",
            raw.len()
        ),
    ))
}
fn replacement_record(
    original: &FlowRecordId,
    result: &FlowCarrier,
    identifier: u32,
) -> Result<FlowRecordId> {
    result
        .records()
        .nth(original.ordinal())
        .map(|record| record.id)
        .ok_or_else(|| {
            invalid(
                identifier,
                format!(
                    "replacement ordinal {}; expected an occurrence below {}",
                    original.ordinal(),
                    result.records().len()
                ),
            )
        })
}
impl FlowCarrier {
    /// Reads exactly 320 unversioned raw patterns; e.g. `let fields = carrier.record_105_fields(&id, options)?`.
    pub fn record_105_fields(
        &self,
        id: &FlowRecordId,
        options: FlowLegacyFieldWordOptions,
    ) -> Result<FlowLegacyWordFields> {
        words::read(self, id, FlowLegacyWordProfile::Fixed105, options)
            .map_err(|error| table_boundary(error, 105))
    }
    /// Reads one complete unversioned counted vector; e.g. `let fields = carrier.record_113_fields(&id, options)?`.
    pub fn record_113_fields(
        &self,
        id: &FlowRecordId,
        options: FlowLegacyFieldWordOptions,
    ) -> Result<FlowLegacyWordFields> {
        words::read(self, id, FlowLegacyWordProfile::Counted113, options)
            .map_err(|error| table_boundary(error, 113))
    }
    /// Reads the complete raw zero-version fallback grammar; e.g. `let fields = carrier.record_155_fields(&id, options)?`.
    pub fn record_155_fields(
        &self,
        id: &FlowRecordId,
        options: FlowConditionalItemOptions,
    ) -> Result<FlowLegacyGroup155> {
        group155::read(self, id, options).map_err(|error| table_boundary(error, 155))
    }
}
