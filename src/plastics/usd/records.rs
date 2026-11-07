use super::{Result, UsdCharacterWidth, framing, invalid};
use std::sync::Arc;

/// Immutable complete counted record, with exact source offsets and framing retained.
#[derive(Debug, Clone)]
pub struct UsdStringRecord {
    pub(super) bytes: Arc<[u8]>,
    pub(super) span: framing::UsdRecordSpan,
}

impl UsdStringRecord {
    /// Require exactly one complete supplied record, e.g. `UsdStringRecord::from_bytes(encoded, maximum)?`.
    pub fn from_bytes(encoded: &[u8], maximum_payload_bytes: usize) -> Result<Self> {
        let span = framing::decode_record(encoded, 0, maximum_payload_bytes)?;
        if span.end != encoded.len() {
            return Err(invalid(
                "standalone record",
                format!(
                    "end {}, encoded length {}; expected exactly one complete record",
                    span.end,
                    encoded.len()
                ),
            ));
        }
        Ok(Self {
            bytes: encoded.into(),
            span,
        })
    }
    /// Construct shortest documented UTF-16 framing without normalizing units, e.g. `UsdStringRecord::from_units(&[0xd800, 0], maximum)?`.
    pub fn from_units(units: &[u16], maximum_payload_bytes: usize) -> Result<Self> {
        let size = units.len().checked_mul(2).ok_or_else(|| {
            invalid(
                "UTF-16 input",
                format!(
                    "{} units; expected representable two-byte payload",
                    units.len()
                ),
            )
        })?;
        require_payload(size, maximum_payload_bytes)?;
        let mut encoded =
            framing::encode_count(units.len() as u64, UsdCharacterWidth::Utf16CodeUnits);
        encoded.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
        Self::from_bytes(&encoded, maximum_payload_bytes)
    }
    /// Construct opaque narrow bytes without guessing a code page, e.g. `UsdStringRecord::from_narrow_bytes(&[0x80, 0], maximum)?`.
    pub fn from_narrow_bytes(payload: &[u8], maximum_payload_bytes: usize) -> Result<Self> {
        require_payload(payload.len(), maximum_payload_bytes)?;
        let mut encoded =
            framing::encode_count(payload.len() as u64, UsdCharacterWidth::NarrowOpaque);
        encoded.extend_from_slice(payload);
        Self::from_bytes(&encoded, maximum_payload_bytes)
    }
    /// Exact original encoded record, e.g. `record.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes[self.span.offset..self.span.end].to_vec()
    }
    /// Exact original length header, e.g. `record.header()`.
    pub fn header(&self) -> &[u8] {
        &self.bytes[self.span.offset..self.span.payload_offset]
    }
    /// Exact counted payload, including NULs and residual code units, e.g. `record.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.bytes[self.span.payload_offset..self.span.end]
    }
    /// Raw UTF-16 units; opaque narrow input returns None, e.g. `record.utf16_units()`.
    pub fn utf16_units(&self) -> Option<Vec<u16>> {
        if self.span.width != UsdCharacterWidth::Utf16CodeUnits {
            return None;
        }
        Some(
            self.payload()
                .as_chunks::<2>()
                .0
                .iter()
                .map(|pair| u16::from_le_bytes(*pair))
                .collect(),
        )
    }
    /// Retained u64 count without narrowing, e.g. `record.declared_count()`.
    pub fn declared_count(&self) -> u64 {
        self.span.count
    }
    /// Opaque narrow versus raw UTF-16 representation, e.g. `record.character_width()`.
    pub fn character_width(&self) -> UsdCharacterWidth {
        self.span.width
    }
    /// Original record/header, payload and end positions, e.g. `record.source_offsets()`.
    pub fn source_offsets(&self) -> (usize, usize, usize) {
        (self.span.offset, self.span.payload_offset, self.span.end)
    }
}

fn require_payload(size: usize, maximum: usize) -> Result<()> {
    if size > maximum {
        return Err(invalid(
            "caller payload",
            format!("{size} bytes; expected at most {maximum}"),
        ));
    }
    Ok(())
}
