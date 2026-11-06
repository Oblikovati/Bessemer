#[cfg(test)]
#[path = "compute_tests.rs"]
mod tests;
use super::*;
pub(super) fn evaluate(
    body: &FlowMediumKindOne,
    accessor: FlowMediumCurveAccessor,
    query: f64,
) -> Result<FlowMediumCurveEvaluation> {
    finite_number(query.to_bits(), "query", 0)?;
    let curve = body.required_curve(accessor.as_i32() as usize)?;
    let (value, branch) = curve_value(body, accessor, query)?;
    Ok(FlowMediumCurveEvaluation {
        record: body.backend_prefix().metadata_prefix().record().clone(),
        body: body.payload_interval(),
        curve: (curve.start, curve.end),
        accessor,
        query: query.to_bits(),
        bits: value.to_bits(),
        class: classification(value),
        branch,
        points: curve.count,
    })
}
fn curve_value(
    body: &FlowMediumKindOne,
    accessor: FlowMediumCurveAccessor,
    query: f64,
) -> Result<(f64, FlowMediumEvaluationBranch)> {
    let ordinal = accessor.as_i32() as usize;
    let curve = body.required_curve(ordinal)?;
    if curve.count == 0 {
        let scalar = finite_number(wire::wide_at(body.raw(), curve.scalar), "scalar", 0)?;
        return Ok((scalar, FlowMediumEvaluationBranch::Constant));
    }
    let coordinates = body.wide_words(FlowMediumWideField::CurveArray {
        curve: ordinal,
        array: FlowMediumCurveArray::First,
    })?;
    let outputs = body.wide_words(FlowMediumWideField::CurveArray {
        curve: ordinal,
        array: FlowMediumCurveArray::Second,
    })?;
    ValidatedTable::new(coordinates, outputs, curve.count)?.evaluate(query)
}
fn finite_number(bits: u64, role: &str, index: usize) -> Result<f64> {
    let value = f64::from_bits(bits);
    if !value.is_finite() {
        return Err(table_failure(
            424,
            FailureClass::Invalid,
            format!("evaluation {role}[{index}] bits 0x{bits:016x}; expected finite binary64"),
        ));
    }
    Ok(value)
}
fn word_bits(words: FlowTableWideArray<'_>, index: usize) -> Result<u64> {
    words
        .raw_bits(index)
        .ok_or_else(|| table_index_error(424, "evaluation numeric word", index, words.word_count()))
}
fn raw_number(words: FlowTableWideArray<'_>, index: usize) -> Result<f64> {
    word_bits(words, index).map(f64::from_bits)
}
fn finite_at(words: FlowTableWideArray<'_>, index: usize, role: &str) -> Result<f64> {
    finite_number(word_bits(words, index)?, role, index)
}
struct ValidatedTable<'a> {
    coordinates: FlowTableWideArray<'a>,
    outputs: FlowTableWideArray<'a>,
    count: usize,
}
impl<'a> ValidatedTable<'a> {
    fn new(
        coordinates: FlowTableWideArray<'a>,
        outputs: FlowTableWideArray<'a>,
        count: usize,
    ) -> Result<Self> {
        if count == 0 || coordinates.word_count() != count || outputs.word_count() != count {
            return Err(metadata_invalid(format!(
                "evaluation arrays {}/{} and count {count}; expected equally sized nonempty arrays",
                coordinates.word_count(),
                outputs.word_count()
            )));
        }
        validate_selected_arrays(coordinates, outputs, count)?;
        Ok(Self {
            coordinates,
            outputs,
            count,
        })
    }
    fn evaluate(&self, query: f64) -> Result<(f64, FlowMediumEvaluationBranch)> {
        if query <= raw_number(self.coordinates, 0)? {
            return Ok((
                raw_number(self.outputs, 0)?,
                FlowMediumEvaluationBranch::LowerClamp,
            ));
        }
        if raw_number(self.coordinates, self.count - 1)? <= query {
            return Ok((
                raw_number(self.outputs, self.count - 1)?,
                FlowMediumEvaluationBranch::UpperClamp,
            ));
        }
        let (lower, upper) = self.bracket(query)?;
        Ok((
            self.interpolate(lower, upper, query)?,
            FlowMediumEvaluationBranch::Linear { lower, upper },
        ))
    }
    fn bracket(&self, query: f64) -> Result<(usize, usize)> {
        let mut lower = 0;
        let mut upper = self.count;
        while upper - lower > 1 {
            let middle = lower + (upper - lower) / 2;
            if raw_number(self.coordinates, middle)? <= query {
                lower = middle;
            } else {
                upper = middle;
            }
        }
        Ok((lower, upper))
    }
    fn interpolate(&self, lower: usize, upper: usize, query: f64) -> Result<f64> {
        let x0 = raw_number(self.coordinates, lower)?;
        let x1 = raw_number(self.coordinates, upper)?;
        let y0 = raw_number(self.outputs, lower)?;
        let y1 = raw_number(self.outputs, upper)?;
        // #241: retain the documented separately rounded operation order, including exact interior knots.
        let slope = (y1 - y0) / (x1 - x0);
        let product = slope * (query - x0);
        Ok(product + y0)
    }
}
fn coordinate_order(
    words: FlowTableWideArray<'_>,
    count: usize,
) -> Result<Option<(usize, f64, f64)>> {
    let mut previous = finite_at(words, 0, "coordinate")?;
    let mut nonordered = None;
    for index in 1..count {
        let current = finite_at(words, index, "coordinate")?;
        if previous >= current && nonordered.is_none() {
            nonordered = Some((index, previous, current));
        }
        previous = current;
    }
    Ok(nonordered)
}
fn classification(value: f64) -> FlowMediumArithmeticClass {
    if value.is_nan() {
        return FlowMediumArithmeticClass::NotANumber;
    }
    if value.is_infinite() {
        return if value.is_sign_positive() {
            FlowMediumArithmeticClass::PositiveInfinity
        } else {
            FlowMediumArithmeticClass::NegativeInfinity
        };
    }
    FlowMediumArithmeticClass::Finite
}

fn validate_selected_arrays(
    coordinates: FlowTableWideArray<'_>,
    outputs: FlowTableWideArray<'_>,
    count: usize,
) -> Result<()> {
    let nonordered = coordinate_order(coordinates, count)?;
    for index in 0..count {
        finite_at(outputs, index, "output")?;
    }
    if let Some((index, previous, current)) = nonordered {
        return Err(metadata_invalid(format!(
            "evaluation coordinates[{}..={index}] {previous:?}/{current:?}; expected strictly increasing finite values",
            index - 1
        )));
    }
    Ok(())
}
