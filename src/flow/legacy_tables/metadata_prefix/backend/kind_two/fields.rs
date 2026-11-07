use super::*;
impl FlowMediumKindTwo {
    /// Borrows an existing raw wide field; e.g. `let words=body.wide_words(FlowMediumKindTwoWideField::BodyScalar)?`.
    pub fn wide_words(&self, field: FlowMediumKindTwoWideField) -> Result<FlowTableWideArray<'_>> {
        let (start, end) = self.wide_span(field)?;
        Ok(FlowTableWideArray {
            bytes: &self.raw()[start..end],
            start,
        })
    }
    pub(super) fn wide_span(&self, field: FlowMediumKindTwoWideField) -> Result<(usize, usize)> {
        match field {
            FlowMediumKindTwoWideField::CurveScalar { scope, curve } => {
                let span = self.required_curve(scope, curve)?;
                Ok((span.scalar, span.scalar + 8))
            }
            FlowMediumKindTwoWideField::CurveArray {
                scope,
                curve,
                array,
            } => medium_body::curve_array(self.raw(), self.required_curve(scope, curve)?, array)
                .ok_or_else(|| self.field_error(field)),
            FlowMediumKindTwoWideField::SharedArray { array } => self.shared_array(array),
            FlowMediumKindTwoWideField::BodyScalar => Ok(self.span.scalar),
            _ => self.model_wide_span(field),
        }
    }
    fn shared_array(&self, array: usize) -> Result<(usize, usize)> {
        self.span
            .prefix
            .arrays
            .get(array)
            .copied()
            .ok_or_else(|| self.field_error(FlowMediumKindTwoWideField::SharedArray { array }))
    }
    fn model_wide_span(&self, field: FlowMediumKindTwoWideField) -> Result<(usize, usize)> {
        let span = match (&self.span.model.payload, field) {
            (ModelPayload::Curves { scalars, .. }, FlowMediumKindTwoWideField::ModelScalars)
            | (ModelPayload::Scalars(scalars), FlowMediumKindTwoWideField::ModelScalars) => {
                Some(*scalars)
            }
            (
                ModelPayload::Switched { first, .. },
                FlowMediumKindTwoWideField::ModelFirstScalars,
            ) => Some(*first),
            (ModelPayload::Switched { last, .. }, FlowMediumKindTwoWideField::ModelLastScalar) => {
                Some(*last)
            }
            (ModelPayload::Tables(table), _) => table_wide_span(table, field),
            _ => None,
        };
        span.ok_or_else(|| self.field_error(field))
    }
    pub(super) fn curve_spans(&self, scope: FlowMediumCurveScope) -> &[CurveSpan] {
        match scope {
            FlowMediumCurveScope::Body => &self.span.prefix.curves,
            FlowMediumCurveScope::Model => match &self.span.model.payload {
                ModelPayload::Curves { curves, .. } => curves,
                _ => &[],
            },
        }
    }
    pub(super) fn required_curve(
        &self,
        scope: FlowMediumCurveScope,
        ordinal: usize,
    ) -> Result<&CurveSpan> {
        let spans = self.curve_spans(scope);
        spans.get(ordinal).ok_or_else(|| {
            table_index_error(
                424,
                &format!("{scope:?} kind-two curve"),
                ordinal,
                spans.len(),
            )
        })
    }
    pub(super) fn table(&self) -> Result<&TableModelSpan> {
        match &self.span.model.payload {
            ModelPayload::Tables(value) => Ok(value),
            _ => Err(metadata_invalid(format!(
                "model {:?}; expected table model five",
                self.model_kind()
            ))),
        }
    }
    pub(super) fn model_states_span(&self) -> (usize, usize) {
        match &self.span.model.payload {
            ModelPayload::Curves { states, .. } => *states,
            ModelPayload::Tables(table) => table.states,
            ModelPayload::Switched { state, .. } => (*state, *state + 1),
            ModelPayload::Scalars(_) => (self.span.model.end, self.span.model.end),
        }
    }
    fn field_error(&self, field: FlowMediumKindTwoWideField) -> BessemerError {
        metadata_invalid(format!(
            "model {:?}, field {field:?}; expected an existing wide field with its declared presence",
            self.model_kind()
        ))
    }
}
fn table_wide_span(
    table: &TableModelSpan,
    field: FlowMediumKindTwoWideField,
) -> Option<(usize, usize)> {
    match field {
        FlowMediumKindTwoWideField::ModelScalars => Some(table.scalars),
        FlowMediumKindTwoWideField::ModelFirstArray { array } => table.first.get(array).copied(),
        FlowMediumKindTwoWideField::ModelSecondArray => Some(table.second),
        FlowMediumKindTwoWideField::ModelThirdArray => Some(table.third),
        _ => None,
    }
}
