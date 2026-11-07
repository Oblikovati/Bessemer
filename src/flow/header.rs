//! Exact GUID prefixes and typed edits; extension bytes and control-word semantics stay separate.
use super::*;
use std::fmt;

/// Exact Windows GUID wire bytes; binary equality includes all sixteen bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowGuid([u8; 16]);

impl FlowGuid {
    /// Retains exact wire order; e.g. `let nil = FlowGuid::from_bytes([0; 16])`.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
    /// Borrows the complete binary identity; e.g. `assert_eq!(nil.as_bytes(), &[0; 16])`.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for FlowGuid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0;
        let first = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        let second = u16::from_le_bytes([b[4], b[5]]);
        let third = u16::from_le_bytes([b[6], b[7]]);
        write!(
            formatter,
            "{first:08x}-{second:04x}-{third:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

/// Complete selected record-123 prefix, with the parent carrier and optional gated record 435.
#[derive(Clone, Debug)]
pub struct FlowHeaderMetadata {
    /// Complete original parent revision.
    pub source: FlowCarrier,
    /// Selected last record-123 occurrence.
    pub record: FlowRecordId,
    /// First wire GUID; consumer lookup role depends on requested result type.
    pub first_guid: FlowGuid,
    /// Second wire GUID, used by the documented CPT lookup consumer.
    pub second_guid: FlowGuid,
    /// Every byte after the established 32-byte prefix.
    pub extension: Arc<[u8]>,
    /// Present only when record 123 and a valid selected record 435 both exist.
    pub settings: Option<FlowSettingsHeader>,
}

/// Record-435 prefix, distinct from missing native defaults and retained unknown extensions.
#[derive(Clone, Debug)]
pub struct FlowSettingsHeader {
    /// Exact selected occurrence.
    pub record: FlowRecordId,
    /// Uninterpreted native control word, without an inferred enum domain.
    pub control_word: u32,
    /// Cached static-calculation-settings identity, including physically present nil.
    pub cfld_uuid: FlowGuid,
    /// Every byte after the established twenty-byte prefix.
    pub extension: Arc<[u8]>,
}

/// Exact known-prefix checks, with every extension retained without semantic credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowHeaderCoverage {
    /// Always exact for the selected known GUID prefixes, never a sampled check.
    pub mode: FlowHeaderCheckMode,
    /// One selected record 123 and optionally one selected record 435.
    pub checked_records: usize,
    /// Thirty-two bytes plus the optional twenty-byte settings prefix.
    pub checked_prefix_bytes: usize,
    /// Complete preserved extension bytes from the selected records.
    pub opaque_extension_bytes: usize,
    /// Exact byte disagreement; completed prefix operations report zero.
    pub achieved_byte_error: usize,
}

/// Header validation scope, separate from whole-carrier and settings-body validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowHeaderCheckMode {
    /// Exact selected record prefixes and retention of all extension bytes.
    ExactSelectedGuidPrefixes,
}

impl FlowHeaderMetadata {
    /// Reports complete selected-prefix checks; e.g. `let checked = header.coverage()`.
    pub fn coverage(&self) -> FlowHeaderCoverage {
        let settings = usize::from(self.settings.is_some());
        let extra = self
            .settings
            .as_ref()
            .map_or(0, |value| value.extension.len());
        FlowHeaderCoverage {
            mode: FlowHeaderCheckMode::ExactSelectedGuidPrefixes,
            checked_records: 1 + settings,
            checked_prefix_bytes: 32 + 20 * settings,
            opaque_extension_bytes: self.extension.len() + extra,
            achieved_byte_error: 0,
        }
    }
}

/// One established sixteen-byte field; its record is selected through the native header gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowHeaderGuidField {
    /// First GUID in record 123.
    First,
    /// Second GUID in record 123.
    Second,
    /// Static-settings GUID after the control word in record 435.
    StaticSettings,
}

impl FlowCarrier {
    /// Reads the complete selected prefixes and extensions; e.g. `let header = carrier.header_metadata()?`.
    pub fn header_metadata(&self) -> Result<Option<FlowHeaderMetadata>> {
        let Some(first) = self.latest(123)? else {
            return Ok(None);
        };
        let prefix = header_prefix(&first, 32)?;
        let settings = self.latest(435)?.map(settings_header).transpose()?;
        Ok(Some(FlowHeaderMetadata {
            source: self.clone(),
            record: first.id,
            first_guid: guid_prefix(&prefix[..16]),
            second_guid: guid_prefix(&prefix[16..32]),
            extension: Arc::from(&prefix[32..]),
            settings,
        }))
    }
    /// Replaces exactly one GUID through checked physical editing; e.g. `let edit = carrier.replace_header_guid(FlowHeaderGuidField::First, guid)?`.
    pub fn replace_header_guid(
        &self,
        field: FlowHeaderGuidField,
        guid: FlowGuid,
    ) -> Result<FlowCarrierEdit> {
        let header = self.header_metadata()?.ok_or_else(|| missing_header(123))?;
        let (id, offset) = selected_guid(&header, field)?;
        let record = self
            .records()
            .nth(id.ordinal())
            .ok_or_else(|| missing_header(123))?;
        let mut payload = record.payload.to_vec();
        payload[offset..offset + 16].copy_from_slice(guid.as_bytes());
        self.replace_payload(id, &payload)
    }
}

fn selected_guid(
    header: &FlowHeaderMetadata,
    field: FlowHeaderGuidField,
) -> Result<(&FlowRecordId, usize)> {
    match field {
        FlowHeaderGuidField::First => Ok((&header.record, 0)),
        FlowHeaderGuidField::Second => Ok((&header.record, 16)),
        FlowHeaderGuidField::StaticSettings => header
            .settings
            .as_ref()
            .map(|settings| (&settings.record, 4))
            .ok_or_else(|| missing_header(435)),
    }
}

fn settings_header(record: FlowRecord<'_>) -> Result<FlowSettingsHeader> {
    let prefix = header_prefix(&record, 20)?;
    let control_word = u32::from_le_bytes([prefix[0], prefix[1], prefix[2], prefix[3]]);
    Ok(FlowSettingsHeader {
        record: record.id,
        control_word,
        cfld_uuid: guid_prefix(&prefix[4..20]),
        extension: Arc::from(&prefix[20..]),
    })
}

fn header_prefix<'a>(record: &FlowRecord<'a>, minimum: usize) -> Result<&'a [u8]> {
    if record.payload.len() < minimum {
        return Err(failure(
            "GUID header",
            FailureClass::Invalid,
            format!(
                "record {} has {} bytes; expected at least {minimum}",
                record.identifier,
                record.payload.len()
            ),
        ));
    }
    Ok(record.payload)
}

fn guid_prefix(bytes: &[u8]) -> FlowGuid {
    let mut guid = [0; 16];
    guid.copy_from_slice(bytes);
    FlowGuid::from_bytes(guid)
}

fn missing_header(identifier: u32) -> BessemerError {
    failure(
        "GUID header",
        FailureClass::Missing,
        format!(
            "missing gated record {identifier}; expected complete record 123 and requested GUID prefix"
        ),
    )
}
