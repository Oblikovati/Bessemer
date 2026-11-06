//! Exact section-14.53 boundary; populated object bodies remain opaque.
mod decode;
mod edit;
mod kind_one;
mod kind_one_profile;
mod kind_two;
mod medium_body;
mod profile_wire;
mod types;
mod view;
use super::*;
pub use kind_one::*;
pub use kind_one_profile::*;
pub use kind_two::*;
pub use types::*;
impl FlowMetadataPrefix {
    /// Reads the backend framing on an active prefix; e.g. `let backend = prefix.medium_backend_prefix()?`.
    pub fn medium_backend_prefix(&self) -> Result<FlowMediumBackendPrefix> {
        decode::read(self).map_err(|error| table_boundary(error, 424))
    }
}
