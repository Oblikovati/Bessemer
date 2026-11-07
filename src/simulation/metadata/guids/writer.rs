use super::{Result, SimulationGuidArchive, SimulationGuidParts, require};

/// Successful inspected writer projection, distinct from an executed COM stream creation.
#[derive(Debug, Clone)]
pub enum SimulationGuidWriterProjection {
    /// Third or fourth caller string is null/empty; no stream bytes are proposed.
    Suppressed,
    /// Complete four-field stream bytes are constructed; native I/O was not executed.
    StreamBytes(Box<SimulationGuidArchive>),
}

impl SimulationGuidWriterProjection {
    /// Apply NUL-free caller checks and the inspected creation gate, e.g. `SimulationGuidWriterProjection::from_caller_strings([None, Some(&[]), Some(&[65]), Some(&[66])], maximum)?`.
    pub fn from_caller_strings(strings: [Option<&[u16]>; 4], maximum: usize) -> Result<Self> {
        for (index, units) in strings.iter().enumerate() {
            require(
                !units.is_some_and(|units| units.contains(&0)),
                "TimeStampGUIDs caller string",
                format!(
                    "embedded NUL in field {index}; expected four NUL-free UTF-16 strings or null pointers"
                ),
            )?;
        }
        if strings[2].is_none_or(<[u16]>::is_empty) || strings[3].is_none_or(<[u16]>::is_empty) {
            return Ok(Self::Suppressed);
        }
        validate_created_extent(&strings, maximum)?;
        let parts = SimulationGuidParts {
            fields: strings.map(|units| units.map(Into::into)),
            tail: [].into(),
        };
        SimulationGuidArchive::from_parts(parts, maximum)
            .map(Box::new)
            .map(Self::StreamBytes)
    }

    /// Proposed complete stream when the gate permits it, e.g. `projection.stream()`; none does not mean a zero-byte stream.
    pub fn stream(&self) -> Option<&SimulationGuidArchive> {
        match self {
            Self::Suppressed => None,
            Self::StreamBytes(archive) => Some(archive.as_ref()),
        }
    }

    /// Always false: projection does not execute native storage or allocation, e.g. `projection.native_io_executed()`.
    pub fn native_io_executed(&self) -> bool {
        false
    }
}

fn validate_created_extent(strings: &[Option<&[u16]>; 4], maximum: usize) -> Result<()> {
    let mut total = 16;
    for units in strings {
        total += super::codec::field_length(*units)?;
    }
    require(
        total <= maximum,
        "TimeStampGUIDs caller output",
        format!("{total} bytes; expected at most {maximum} complete stream bytes"),
    )
}
