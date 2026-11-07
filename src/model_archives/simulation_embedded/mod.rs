//! Explicit Simulation helper context; ordinary model class dispatch stays independent (#134).
mod archive;
mod boundaries;
mod bounds;
mod chains;
mod component_boundaries;
mod components;
mod decoder;
mod edit;
mod reference_wrappers;
mod representations;
mod sheet_components;
mod wrappers;

pub use archive::SimulationEmbeddedArchive;
pub use decoder::SimulationEmbeddedProfile;
pub(super) use decoder::read_object;
pub use edit::SimulationEmbeddedEdit;

fn unsupported(entity: &str, cause: impl Into<String>) -> crate::BessemerError {
    crate::BessemerError::new(
        "Simulation embedded archive",
        entity,
        crate::FailureClass::Unsupported,
        cause,
    )
}
