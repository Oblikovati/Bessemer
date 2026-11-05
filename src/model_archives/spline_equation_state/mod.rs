//! Exact saved spline equation roles, distinct from native API/evaluation behavior (#199).
mod accessors;
mod edit;
mod projection;
mod scope;
mod types;
use super::{ModelFieldHandle, PartResolvedFeaturesArchive};
use crate::Result;
pub use types::*;

impl PartResolvedFeaturesArchive {
    /// Enumerate physical equation-bearing splines in wire order; e.g. `source.spline_equation_states()?`.
    pub fn spline_equation_states(&self) -> Result<Vec<ModelSplineEquationStoredState>> {
        self.fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| field.path.ends_with(".flags_32c"))
            .filter(|(index, _)| projection::is_equation_candidate(self.fields(), *index))
            .map(|(index, _)| self.spline_equation_state(&self.field_handle(index)?))
            .collect()
    }

    /// Select the physical flags32C field of an equation spline; e.g. `source.spline_equation_state(&flags)?`.
    pub fn spline_equation_state(
        &self,
        flags: &ModelFieldHandle,
    ) -> Result<ModelSplineEquationStoredState> {
        self.resolve_field(flags)?;
        projection::project_state(self, flags).map_err(|error| {
            super::resolved_failure::selected_feature_error(error, self.configuration())
        })
    }
}
