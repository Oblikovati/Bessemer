use super::{Result, invalid, require};
use crate::records::{dword, region};
use std::sync::Arc;

/// Immutable counted text retaining its entire source parent and exact frame boundary.
#[derive(Debug, Clone)]
pub struct SimulationPersistText {
    parent: Arc<[u8]>,
    offset: usize,
    payload_offset: usize,
    end: usize,
    declared_bytes: u32,
}

impl SimulationPersistText {
    /// Read one bounded frame in its complete parent, e.g. `SimulationPersistText::read_from(bytes, 0, 0x100000)?`.
    pub fn read_from(bytes: &[u8], offset: usize, maximum_payload: usize) -> Result<Self> {
        let extent = text_extent(bytes, offset, maximum_payload)?;
        Ok(Self::owned_record(bytes.into(), offset, extent))
    }

    pub(super) fn read_shared(parent: Arc<[u8]>, offset: usize, maximum: usize) -> Result<Self> {
        let extent = text_extent(&parent, offset, maximum)?;
        Ok(Self::owned_record(parent, offset, extent))
    }

    fn owned_record(parent: Arc<[u8]>, offset: usize, extent: (usize, usize, u32)) -> Self {
        let (payload_offset, end, declared_bytes) = extent;
        Self {
            parent,
            offset,
            payload_offset,
            end,
            declared_bytes,
        }
    }

    /// Construct exactly one raw frame, e.g. `SimulationPersistText::from_payload(Some(&[0, 0]), 16)?`; None is null.
    pub fn from_payload(payload: Option<&[u8]>, maximum_payload: usize) -> Result<Self> {
        require(
            payload.is_none_or(|bytes| !bytes.is_empty()),
            "persistence payload",
            "Some(empty); expected None for null or a complete nonzero terminated payload",
        )?;
        let payload = payload.unwrap_or(&[]);
        validate_payload(payload, maximum_payload)?;
        let count = u32::try_from(payload.len()).map_err(|_| {
            invalid(
                "persistence payload",
                format!("{} bytes; expected a u32 count", payload.len()),
            )
        })?;
        let mut bytes = reserve_frame(payload.len())?;
        bytes.extend_from_slice(&count.to_le_bytes());
        bytes.extend_from_slice(payload);
        Self::read_from(&bytes, 0, maximum_payload)
    }

    /// Construct null or complete raw UTF-16 units, e.g. `SimulationPersistText::from_units(Some(&[0xd800, 0]), 32)?`.
    pub fn from_units(units: Option<&[u16]>, maximum_payload: usize) -> Result<Self> {
        let Some(units) = units else {
            return Self::from_payload(None, maximum_payload);
        };
        let count = terminated_units_size(units.len())?;
        require(
            count <= maximum_payload && u32::try_from(count).is_ok(),
            "persistence units",
            format!("{count} bytes; expected a u32 count at most {maximum_payload}"),
        )?;
        let mut payload = reserve_frame(count)?;
        payload.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
        payload.extend_from_slice(&[0, 0]);
        Self::from_payload(Some(&payload), maximum_payload)
    }

    /// Entire immutable input, including bytes outside this frame, e.g. `record.source_parent()`.
    pub fn source_parent(&self) -> &[u8] {
        &self.parent
    }

    /// Original header, payload and end offsets, e.g. `record.source_offsets()`.
    pub fn source_offsets(&self) -> (usize, usize, usize) {
        (self.offset, self.payload_offset, self.end)
    }

    /// Exact original four-byte little-endian count, e.g. `record.header()`.
    pub fn header(&self) -> &[u8] {
        &self.parent[self.offset..self.payload_offset]
    }

    /// Complete counted payload including its terminator, e.g. `record.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.parent[self.payload_offset..self.end]
    }

    /// Stored byte count including the terminator, e.g. `record.declared_bytes()`.
    pub fn declared_bytes(&self) -> u32 {
        self.declared_bytes
    }

    /// Distinguish null from allocated empty, e.g. `record.is_null()`.
    pub fn is_null(&self) -> bool {
        self.declared_bytes == 0
    }

    /// Raw units excluding exactly the final terminator; null and odd-length frames return None, e.g. `record.utf16_units()`.
    pub fn utf16_units(&self) -> Option<Vec<u16>> {
        if self.is_null() || !self.declared_bytes.is_multiple_of(2) {
            return None;
        }
        let content = &self.payload()[..self.payload().len() - 2];
        Some(
            content
                .as_chunks::<2>()
                .0
                .iter()
                .map(|pair| u16::from_le_bytes(*pair))
                .collect(),
        )
    }

    /// Exact complete encoded frame without unrelated parent bytes, e.g. `record.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.encoded().to_vec()
    }

    pub(super) fn encoded(&self) -> &[u8] {
        &self.parent[self.offset..self.end]
    }

    pub(super) fn require_limit(&self, maximum: usize) -> Result<()> {
        require(
            self.payload().len() <= maximum,
            "persistence text limit",
            format!(
                "{} payload bytes; expected at most {maximum}",
                self.payload().len()
            ),
        )
    }

    /// Exact frame coverage within its complete retained parent, e.g. `record.coverage()`.
    pub fn coverage(&self) -> super::SimulationPersistCoverage {
        let framed = self.end - self.offset;
        super::coverage::coverage(
            super::SimulationPersistCheckMode::ExactTextFrame,
            self.parent.len(),
            framed,
            self.parent.len() - framed,
            false,
        )
    }
}

pub(super) fn text_extent(
    bytes: &[u8],
    offset: usize,
    maximum: usize,
) -> Result<(usize, usize, u32)> {
    let count = dword(bytes, offset)?;
    require(
        count as usize <= maximum,
        "persistence text count",
        format!("{count} bytes; expected at most {maximum}"),
    )?;
    let start = offset.checked_add(4).ok_or_else(|| {
        invalid(
            "persistence text offset",
            format!("{offset}+4; expected representable header endpoint"),
        )
    })?;
    let payload = region(bytes, start, count as usize)?;
    validate_payload(payload, maximum)?;
    Ok((start, start + payload.len(), count))
}

fn validate_payload(payload: &[u8], maximum: usize) -> Result<()> {
    require(
        payload.len() <= maximum,
        "persistence text length",
        format!("{} bytes; expected at most {maximum}", payload.len()),
    )?;
    require(
        payload.is_empty() || (payload.len() >= 2 && payload.ends_with(&[0, 0])),
        "persistence text payload",
        format!(
            "{} bytes ending {:?}; expected null or at least two bytes with a final zero UTF-16 unit",
            payload.len(),
            payload.get(payload.len().saturating_sub(2)..)
        ),
    )
}

fn reserve_frame(payload_count: usize) -> Result<Vec<u8>> {
    let size = payload_count.checked_add(4).ok_or_else(|| {
        invalid(
            "persistence frame",
            format!("{payload_count}+4; expected representable byte extent"),
        )
    })?;
    super::reserve_bytes(size)
}

fn terminated_units_size(count: usize) -> Result<usize> {
    count
        .checked_mul(2)
        .and_then(|size| size.checked_add(2))
        .ok_or_else(|| {
            invalid(
                "persistence units",
                format!("{count} units; expected representable terminated byte count"),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::terminated_units_size;

    #[test]
    fn terminated_unit_size_checks_multiplication_and_terminator_overflow() {
        assert_eq!(terminated_units_size(0).unwrap(), 2);
        assert_eq!(terminated_units_size(3).unwrap(), 8);
        assert!(terminated_units_size(usize::MAX).is_err());
        assert!(terminated_units_size(usize::MAX / 2).is_err());
    }
}
