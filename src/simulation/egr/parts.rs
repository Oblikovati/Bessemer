use super::{
    Result, SimulationEgrArchive, SimulationEgrLimits, SimulationEgrProfile, header, invalid,
    require,
};
use std::sync::Arc;

/// Complete original EGR regions, including unknown alignment and deleted-record bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationEgrParts {
    /// Exact complete 128-byte header, including unassigned words and remainder.
    pub header: [u8; 128],
    /// All loaded descriptors and their complete 128-byte alignment region.
    pub field_directory: Arc<[u8]>,
    /// Entire logical directory and its alignment bytes.
    pub row_directory: Arc<[u8]>,
    /// All allocated physical records in stored order, including deleted rows.
    pub allocated_records: Arc<[u8]>,
}

impl SimulationEgrArchive {
    /// Capture every original region, e.g. `let parts = table.parts()`; no writer assignments occur.
    pub fn parts(&self) -> SimulationEgrParts {
        let header = &self.layout.header;
        SimulationEgrParts {
            header: std::array::from_fn(|index| self.bytes[index]),
            field_directory: self.bytes[128..header.directory_start].into(),
            row_directory: self.bytes[header.directory_start..header.rows_start].into(),
            allocated_records: self.bytes[header.rows_start..].into(),
        }
    }
    /// Construct and fully validate exact supplied regions, e.g. `SimulationEgrArchive::from_parts(parts, profile, limits)?`.
    pub fn from_parts(
        parts: SimulationEgrParts,
        profile: SimulationEgrProfile,
        limits: SimulationEgrLimits,
    ) -> Result<Self> {
        let size = validate_parts(&parts, limits)?;
        let bytes = encode_parts(&parts, size)?;
        let archive = Self::from_bytes(&bytes, profile, limits)?;
        require(
            archive.parts() == parts,
            "EGR constructed regions",
            "supplied regions differ from decoded boundaries; expected exact region reconstruction",
        )?;
        Ok(archive)
    }
}

fn validate_parts(parts: &SimulationEgrParts, limits: SimulationEgrLimits) -> Result<usize> {
    let header = header::decode_header(&parts.header, limits)?;
    let actual = [
        parts.field_directory.len(),
        parts.row_directory.len(),
        parts.allocated_records.len(),
    ];
    let expected = [
        header.directory_start - 128,
        header.rows_start - header.directory_start,
        header.expected_bytes - header.rows_start,
    ];
    require(
        actual == expected,
        "EGR supplied regions",
        format!("lengths {actual:?}; expected {expected:?} from the validated header"),
    )?;
    Ok(header.expected_bytes)
}

fn encode_parts(parts: &SimulationEgrParts, size: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|error| {
        invalid(
            "EGR constructed extent",
            format!("{size} bytes, allocation {error}; expected available complete output storage"),
        )
    })?;
    for region in [
        &parts.header[..],
        &parts.field_directory,
        &parts.row_directory,
        &parts.allocated_records,
    ] {
        bytes.extend_from_slice(region);
    }
    Ok(bytes)
}
