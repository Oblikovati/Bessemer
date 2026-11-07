//! Fresh typed402/316 selection and complete headerless112 values, without native pairing.
mod context;
mod context_view;
mod edit;
mod read;
mod schema;
mod types;
mod view;
use super::*;
pub use schema::*;
pub use types::*;

impl FlowCarrier {
    /// Reads complete values under an explicit immutable context; e.g. `let values = carrier.record_112_selected_values(&id, &context, options)?`.
    pub fn record_112_selected_values(
        &self,
        id: &FlowRecordId,
        context: &FlowSelectionContext,
        options: FlowSelectedValueOptions,
    ) -> Result<FlowSelectedValues> {
        read::read(self, id, context, options).map_err(|error| table_boundary(error, 112))
    }
}
fn selected_invalid(cause: String) -> BessemerError {
    table_failure(112, FailureClass::Invalid, cause)
}
fn diagnostics() -> &'static [FlowSelectionDiagnostic] {
    use FlowSelectionDiagnostic::*;
    &[
        FreshMapAssumed,
        NativeOwnerBindingNotVerified,
        NativeRuntimeScatterNotReproduced,
        EngineeringRolesNotResolved,
        NativeApplicationNotChecked,
    ]
}
