use super::{Result, require, wire};
use crate::archive_codec::ArchiveRevision;
use crate::records::{dword, region};
use crate::simulation::invalid;

/// Independent raw constructor inputs; neither is implicitly a mesh-count constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfInitialParameters {
    /// Native header word 3, preserving all 32 source bits.
    pub parameter_a: u32,
    /// Native header word 5 and a later update's advertised record count.
    pub parameter_b: u32,
}

/// Immutable identity of one complete native initial image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfInitialRevision(ArchiveRevision);

/// Declared independent constructor parameter selected for an isolated edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfParameter {
    /// Header word 3.
    A,
    /// Header word 5.
    B,
}

/// Constructor parameter identity bound to its complete initial image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfParameterId {
    pub(super) revision: SimulationElfInitialRevision,
    pub(super) parameter: SimulationElfParameter,
}

/// Complete observed 1,224-byte initial image, including its zero continuation.
#[derive(Debug, Clone)]
pub struct SimulationElfInitialArchive {
    pub(super) parameters: SimulationElfInitialParameters,
    pub(super) revision: SimulationElfInitialRevision,
}

impl SimulationElfInitialArchive {
    /// Construct all native initial bytes from independent inputs, e.g. `SimulationElfInitialArchive::from_parameters(parameters)`.
    pub fn from_parameters(parameters: SimulationElfInitialParameters) -> Self {
        Self {
            parameters,
            revision: SimulationElfInitialRevision(ArchiveRevision::default()),
        }
    }

    /// Validate and reconstruct the complete native initializer profile, e.g. `SimulationElfInitialArchive::from_bytes(elf)?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        validate_image(bytes)?;
        let parameters = SimulationElfInitialParameters {
            parameter_a: dword(bytes, 16)?,
            parameter_b: dword(bytes, 24)?,
        };
        let archive = Self::from_parameters(parameters);
        require(
            archive.to_bytes() == bytes,
            "ELF initial reconstruction",
            format!(
                "{} source bytes differ; expected exact complete 1,224-byte initializer",
                bytes.len()
            ),
        )?;
        Ok(archive)
    }

    /// Exact complete initializer bytes, e.g. `initial.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1224);
        wire::append_patterns(&mut bytes, &[1]);
        wire::append_patterns(&mut bytes, &self.header_words());
        bytes.resize(1224, 0);
        bytes
    }

    /// Every stored header word, e.g. `initial.header_words()[3]`.
    pub fn header_words(&self) -> [u32; 7] {
        [
            199 / 61 + 3,
            4 * 61,
            100,
            self.parameters.parameter_a,
            0,
            self.parameters.parameter_b,
            1,
        ]
    }

    /// Both independent raw inputs, e.g. `initial.parameters().parameter_a`.
    pub fn parameters(&self) -> SimulationElfInitialParameters {
        self.parameters
    }

    /// Identity of this complete immutable initializer, e.g. `initial.revision()`.
    pub fn revision(&self) -> &SimulationElfInitialRevision {
        &self.revision
    }

    /// Bind a typed constructor parameter once, e.g. `initial.parameter_id(SimulationElfParameter::A)`.
    pub fn parameter_id(&self, parameter: SimulationElfParameter) -> SimulationElfParameterId {
        SimulationElfParameterId {
            revision: self.revision.clone(),
            parameter,
        }
    }

    /// Read exact bits through a revision-bound parameter, e.g. `initial.parameter(&id)?`.
    pub fn parameter(&self, id: &SimulationElfParameterId) -> Result<u32> {
        require(
            id.revision == self.revision,
            "ELF initial parameter",
            format!(
                "{:?} from foreign revision; expected this complete initializer",
                id.parameter
            ),
        )?;
        Ok(match id.parameter {
            SimulationElfParameter::A => self.parameters.parameter_a,
            SimulationElfParameter::B => self.parameters.parameter_b,
        })
    }
}

fn validate_image(bytes: &[u8]) -> Result<()> {
    require(
        bytes.len() == 1224,
        "ELF initial extent",
        format!(
            "{} bytes; expected complete 1,224-byte initializer",
            bytes.len()
        ),
    )?;
    for (offset, expected) in [(0, 1), (4, 6), (8, 244), (12, 100), (20, 0), (28, 1)] {
        let actual = dword(bytes, offset)?;
        require(
            actual == expected,
            "ELF initial header",
            format!("word {actual} at stream offset {offset}; expected {expected}"),
        )?;
    }
    validate_zero_continuation(bytes)
}

fn validate_zero_continuation(bytes: &[u8]) -> Result<()> {
    let continuation = region(bytes, 32, 1192)?;
    if let Some(index) = continuation.iter().position(|byte| *byte != 0) {
        return Err(invalid(
            "ELF initial continuation",
            format!(
                "byte {} at stream offset {}; expected zero-initialized continuation, populated images require separate checks",
                continuation[index],
                index + 32
            ),
        ));
    }
    Ok(())
}
