use super::*;
use std::sync::Arc;

/// Signed native region-type byte, normalized before selecting a width source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFieldRegionType(i8);

impl TryFrom<i32> for FlowFieldRegionType {
    type Error = BessemerError;
    fn try_from(value: i32) -> Result<Self> {
        i8::try_from(value).map(Self).map_err(|_| {
            field_invalid(format!(
                "region type {value}; expected signed byte -128 through 127"
            ))
        })
    }
}

impl FlowFieldRegionType {
    /// Returns the admitted signed code; e.g. `let code = region_type.code()`.
    pub fn code(self) -> i8 {
        self.0
    }
}

/// Raw supplied numeric width input, checked only if the region actually selects it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFieldWidthInput {
    /// Exact signed integer context.
    Signed(i64),
    /// Raw binary64 input; NaN payloads are not normalized on capture.
    Binary64(u64),
}

/// Explicit isolated width inputs, retaining missing and unused context.
#[derive(Clone, Debug)]
pub struct FlowFieldWidthRequest {
    /// Validated signed native type byte.
    pub region_type: FlowFieldRegionType,
    /// Raw cached value, used only by type -1.
    pub cached_width: Option<i64>,
    /// Raw signed index, used only by type one.
    pub region_index: Option<i32>,
    /// Selected runtime numeric field values, without an inferred global record-102 binding.
    pub numeric_widths: Option<Arc<[FlowFieldWidthInput]>>,
}

/// Exact branch that produced a completed width, independently of runtime provenance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFieldWidthOrigin {
    /// Type -1 uses the supplied cached width.
    Cached,
    /// Type one selects an in-range numeric position.
    SelectedNumeric(usize),
    /// Type one's index is outside the supplied collection.
    MissingRegionZero,
    /// Other signed type codes return zero without inspecting unused context.
    TypeDefaultZero,
}

/// Completed strict-profile width calculation with immutable input checkpoint.
#[derive(Clone, Debug)]
pub struct FlowFieldWidthProjection {
    /// Admitted nonnegative signed width.
    pub width: FlowFieldCount,
    /// Selected width source.
    pub origin: FlowFieldWidthOrigin,
    /// Exact count conversion error within this profile.
    pub achieved_error: u32,
    request: FlowFieldWidthRequest,
}

impl FlowFieldWidthProjection {
    /// Borrows the immutable supplied context; e.g. `let request = projection.request()`.
    pub fn request(&self) -> &FlowFieldWidthRequest {
        &self.request
    }
    /// Reports that isolated arithmetic does not execute native storage or floating-point exception state; e.g. `let exclusions = projection.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowFieldWidthDiagnostic] {
        &[
            FlowFieldWidthDiagnostic::SelectedNumericBindingNotEstablished,
            FlowFieldWidthDiagnostic::NativeFloatingPointStateNotReproduced,
            FlowFieldWidthDiagnostic::NativeContextNotExecuted,
        ]
    }
}

/// Exclusions from the isolated strict-profile width arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFieldWidthDiagnostic {
    /// Supplied numeric context is not automatically bound to a global record.
    SelectedNumericBindingNotEstablished,
    /// Native floating-point exception state is not simulated.
    NativeFloatingPointStateNotReproduced,
    /// Native runtime context was not executed.
    NativeContextNotExecuted,
}

/// Calculates the isolated documented width without guessing context; e.g. `let width = project_flow_region_width(&request)?.width`.
pub fn project_flow_region_width(
    request: &FlowFieldWidthRequest,
) -> Result<FlowFieldWidthProjection> {
    let (width, origin) = match request.region_type.code() {
        -1 => (cached_count(request)?, FlowFieldWidthOrigin::Cached),
        1 => selected_numeric(request)?,
        _ => (
            FlowFieldCount::try_from(0)?,
            FlowFieldWidthOrigin::TypeDefaultZero,
        ),
    };
    Ok(FlowFieldWidthProjection {
        width,
        origin,
        achieved_error: 0,
        request: request.clone(),
    })
}

fn cached_count(request: &FlowFieldWidthRequest) -> Result<FlowFieldCount> {
    let value = request.cached_width.ok_or_else(|| {
        field_invalid(
            "cached width absent; expected a native nonnegative signed count for type -1".into(),
        )
    })?;
    FlowFieldCount::try_from(value)
}

fn selected_numeric(
    request: &FlowFieldWidthRequest,
) -> Result<(FlowFieldCount, FlowFieldWidthOrigin)> {
    let index = request.region_index.ok_or_else(|| {
        field_invalid(
            "region index absent; expected signed index and numeric-width collection for type one"
                .into(),
        )
    })?;
    let numbers = request.numeric_widths.as_deref().ok_or_else(|| field_invalid(format!("numeric-width collection absent at region index {index}; expected explicit selected runtime values")))?;
    if index < 0 || index as usize >= numbers.len() {
        return Ok((
            FlowFieldCount::try_from(0)?,
            FlowFieldWidthOrigin::MissingRegionZero,
        ));
    }
    let count = numeric_count(numbers[index as usize])?;
    Ok((count, FlowFieldWidthOrigin::SelectedNumeric(index as usize)))
}

fn numeric_count(value: FlowFieldWidthInput) -> Result<FlowFieldCount> {
    match value {
        FlowFieldWidthInput::Signed(value) => FlowFieldCount::try_from(value),
        FlowFieldWidthInput::Binary64(bits) => {
            let numeric = f64::from_bits(bits);
            let truncated = numeric.trunc();
            if !numeric.is_finite() || !(0.0..=f64::from(i32::MAX)).contains(&truncated) {
                return Err(field_invalid(format!(
                    "numeric width bits {bits:#018x}, value {numeric}; expected finite truncation in zero through {}",
                    i32::MAX
                )));
            }
            FlowFieldCount::try_from(truncated as i64)
        }
    }
}
