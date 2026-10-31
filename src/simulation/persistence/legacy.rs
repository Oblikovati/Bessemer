use super::{
    Result, SimulationPersistCheckMode, SimulationPersistCoverage, SimulationPersistReadMode,
    SimulationPersistVersion, SimulationPrimaryVersion, coverage::coverage, invalid, require,
};
use crate::simulation::unsupported;
use std::sync::Arc;

/// Exact observed 128-byte legacy version profile with all 114 opaque suffix bytes.
#[derive(Debug, Clone)]
pub struct SimulationLegacyVersion {
    bytes: Arc<[u8]>,
    numeric: SimulationPersistVersion,
}

impl SimulationLegacyVersion {
    /// Decode only the observed six-digit profile, e.g. `SimulationLegacyVersion::from_bytes(bytes)?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 128 || bytes[12..14] != [0, 0] {
            return Err(unsupported(
                "legacy version profile",
                format!(
                    "{} bytes, prefix {:?}; expected 128 bytes with a zero UTF-16 unit at 12..14",
                    bytes.len(),
                    bytes.get(..14)
                ),
            ));
        }
        let numeric = legacy_numeric(&bytes[..12])?;
        Ok(Self {
            bytes: bytes.into(),
            numeric,
        })
    }

    /// Construct the exact observed profile, e.g. `SimulationLegacyVersion::from_parts(version, &[0; 114])?`; no native save executes.
    pub fn from_parts(numeric: SimulationPersistVersion, opaque_suffix: &[u8]) -> Result<Self> {
        validate_legacy_parts(numeric, opaque_suffix)?;
        let mut bytes = Vec::with_capacity(128);
        bytes.extend(
            format!("{:06}", numeric.value())
                .encode_utf16()
                .flat_map(|unit| unit.to_le_bytes()),
        );
        bytes.extend_from_slice(&[0, 0]);
        bytes.extend_from_slice(opaque_suffix);
        Self::from_bytes(&bytes)
    }

    /// Six exact stored ASCII code units, e.g. `legacy.text_units()`.
    pub fn text_units(&self) -> [u16; 6] {
        std::array::from_fn(|index| {
            u16::from_le_bytes([self.bytes[index * 2], self.bytes[index * 2 + 1]])
        })
    }
}

fn validate_legacy_parts(numeric: SimulationPersistVersion, opaque_suffix: &[u8]) -> Result<()> {
    require(
        numeric.value() <= 999999,
        "legacy numeric version",
        format!(
            "{}; expected six ASCII decimal digits in 0..=999999",
            numeric.value()
        ),
    )?;
    require(
        opaque_suffix.len() == 114,
        "legacy suffix",
        format!(
            "{} bytes; expected exactly 114 opaque bytes",
            opaque_suffix.len()
        ),
    )?;
    Ok(())
}

impl SimulationLegacyVersion {
    /// Once-normalized six-digit value, e.g. `legacy.numeric_version()`.
    pub fn numeric_version(&self) -> SimulationPersistVersion {
        self.numeric
    }
    /// Exact fourteen-byte prefix including its terminator, e.g. `legacy.prefix()`.
    pub fn prefix(&self) -> &[u8] {
        &self.bytes[..14]
    }
    /// All 114 uninterpreted bytes, e.g. `legacy.opaque_suffix()`.
    pub fn opaque_suffix(&self) -> &[u8] {
        &self.bytes[14..]
    }
    /// Exact complete original carrier, e.g. `legacy.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Complete observed framing with explicit opaque suffix coverage, e.g. `legacy.coverage()`.
    pub fn coverage(&self) -> SimulationPersistCoverage {
        coverage(
            SimulationPersistCheckMode::ExactLegacyVersionProfile,
            128,
            128,
            114,
            false,
        )
    }
}

fn legacy_numeric(prefix: &[u8]) -> Result<SimulationPersistVersion> {
    let mut numeric = 0;
    for pair in prefix.as_chunks::<2>().0 {
        let unit = u16::from_le_bytes(*pair);
        if !(48..=57).contains(&unit) {
            return Err(unsupported(
                "legacy version digits",
                format!("code unit {unit:#06x}; expected six UTF-16LE ASCII decimal digits"),
            ));
        }
        numeric = numeric * 10 + i32::from(unit - 48);
    }
    SimulationPersistVersion::new(numeric)
}

/// Availability-based selection retaining its complete decoded carrier, never a largest-number rule.
#[derive(Debug, Clone)]
pub enum SimulationSelectedVersion {
    /// Available primary carrier; malformed primary errors never fall back.
    Primary(SimulationPrimaryVersion),
    /// Available legacy carrier, selected only when primary is unavailable.
    Legacy(SimulationLegacyVersion),
    /// Both carriers unavailable; compatibility constant 500070, without a fabricated record.
    CompatibilityDefault,
}

impl SimulationSelectedVersion {
    /// Apply explicit availability and context, e.g. `SimulationSelectedVersion::select(None, None, None, SimulationPersistReadMode::Full, 0x100000)?`.
    pub fn select(
        primary: Option<&[u8]>,
        legacy: Option<&[u8]>,
        numeric: Option<SimulationPersistVersion>,
        mode: SimulationPersistReadMode,
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        if let Some(bytes) = primary {
            let numeric = numeric.ok_or_else(|| invalid("primary numeric context", "None for an available primary carrier; expected an explicit normalized nonnegative signed-i32 version"))?;
            return SimulationPrimaryVersion::from_bytes(bytes, numeric, mode, maximum_text_bytes)
                .map(Self::Primary);
        }
        if let Some(bytes) = legacy {
            return SimulationLegacyVersion::from_bytes(bytes).map(Self::Legacy);
        }
        Ok(Self::CompatibilityDefault)
    }

    /// Selected value with its original typed provenance retained, e.g. `selected.numeric_version()`.
    pub fn numeric_version(&self) -> SimulationPersistVersion {
        match self {
            Self::Primary(record) => record.numeric_version(),
            Self::Legacy(record) => record.numeric_version(),
            Self::CompatibilityDefault => SimulationPersistVersion::COMPATIBILITY,
        }
    }
}
