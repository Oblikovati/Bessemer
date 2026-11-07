use super::*;
use crate::model_archives::{ModelFieldValue, PartResolvedFeaturesEdit};
use scope::equation_error;

impl PartResolvedFeaturesArchive {
    /// Replace one saved equation value through full archive validation; e.g. `source.edit_spline_equation(&state, ModelSplineEquationStoredEdit::LockStart(true))?`.
    pub fn edit_spline_equation(
        &self,
        state: &ModelSplineEquationStoredState,
        edit: ModelSplineEquationStoredEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        for field in state.source_fields() {
            self.resolve_field(field)?;
        }
        let (slot, replacement) = equation_replacement(state, edit);
        let selected = &state.fields[slot];
        let mut fields = self.fields().to_vec();
        let field = fields
            .get_mut(selected.index())
            .ok_or_else(|| equation_error("saved equation replacement", selected.index()))?;
        field.value = replacement;
        self.edit_fields(fields)
    }
}

fn equation_replacement(
    state: &ModelSplineEquationStoredState,
    edit: ModelSplineEquationStoredEdit,
) -> (usize, ModelFieldValue) {
    match edit {
        ModelSplineEquationStoredEdit::LockStart(enabled) => (
            0,
            ModelFieldValue::U32((state.flags & !0x10) | (u32::from(enabled) << 4)),
        ),
        ModelSplineEquationStoredEdit::LockEnd(enabled) => (
            0,
            ModelFieldValue::U32((state.flags & !0x20) | (u32::from(enabled) << 5)),
        ),
        ModelSplineEquationStoredEdit::SavedOption(value) => (8, ModelFieldValue::I32(value)),
        ModelSplineEquationStoredEdit::Text { role, value } => {
            (1 + role.slot(), ModelFieldValue::Text(value))
        }
    }
}
