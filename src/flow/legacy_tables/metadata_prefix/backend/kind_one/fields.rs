use super::*;
impl FlowMediumKindOne {
    /// Borrows existing raw patterns under one typed role; e.g. `let scalars = body.wide_words(FlowMediumWideField::FinalScalars)?`.
    pub fn wide_words(&self, field: FlowMediumWideField) -> Result<FlowTableWideArray<'_>> {
        let (start, end) = self.wide_span(field)?;
        Ok(FlowTableWideArray {
            bytes: &self.raw()[start..end],
            start,
        })
    }
    pub(super) fn wide_span(&self, field: FlowMediumWideField) -> Result<(usize, usize)> {
        match field {
            FlowMediumWideField::CurveScalar { curve } => {
                let span = self.required_curve(curve)?;
                Ok((span.scalar, span.scalar + 8))
            }
            FlowMediumWideField::CurveArray { curve, array } => self.curve_array(curve, array),
            FlowMediumWideField::SharedArray { array } => self
                .span
                .arrays
                .get(array)
                .copied()
                .ok_or_else(|| table_index_error(424, "medium shared array", array, 6)),
            FlowMediumWideField::FinalScalars => Ok(self.span.scalars),
        }
    }
    pub(super) fn required_curve(&self, ordinal: usize) -> Result<&CurveSpan> {
        self.span
            .curves
            .get(ordinal)
            .ok_or_else(|| table_index_error(424, "medium curve", ordinal, 7))
    }
    fn curve_array(&self, ordinal: usize, role: FlowMediumCurveArray) -> Result<(usize, usize)> {
        let curve = self.required_curve(ordinal)?;
        medium_body::curve_array(self.raw(), curve, role).ok_or_else(|| {
            metadata_invalid(format!(
                "curve {ordinal} array {role:?}; expected active extra-array flag"
            ))
        })
    }
}
