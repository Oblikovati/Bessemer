//! Pure diagnostic projections from stored arrays, specification 14.35.
use super::*;

/// Native diagnostic label; enum domains and engineering units remain unknown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsField {
    /// Native EXTERNAL diagnostic.
    External,
    /// Native LAMTURB diagnostic.
    Lamturb,
    /// Native CHT diagnostic.
    Cht,
    /// Native US_DIM diagnostic.
    UsDim,
    /// Native IT diagnostic.
    It,
    /// Native NT diagnostic.
    Nt,
    /// Native NZ diagnostic.
    Nz,
}

const FIELD_ORDER: [FlowSettingsField; 7] = [
    FlowSettingsField::External,
    FlowSettingsField::Lamturb,
    FlowSettingsField::Cht,
    FlowSettingsField::UsDim,
    FlowSettingsField::It,
    FlowSettingsField::Nt,
    FlowSettingsField::Nz,
];

impl FlowSettingsField {
    /// Returns the exact diagnostic spelling; e.g. `assert_eq!(field.label(), "CHT")`.
    pub fn label(self) -> &'static str {
        match self {
            Self::External => "EXTERNAL",
            Self::Lamturb => "LAMTURB",
            Self::Cht => "CHT",
            Self::UsDim => "US_DIM",
            Self::It => "IT",
            Self::Nt => "NT",
            Self::Nz => "NZ",
        }
    }
    /// Returns the zero-based record-102 slot; e.g. `let offset = field.slot_index() * 8`.
    pub fn slot_index(self) -> usize {
        match self {
            Self::External => 114,
            Self::Lamturb => 115,
            Self::Cht => 117,
            Self::UsDim => 119,
            Self::It => 100,
            Self::Nt => 101,
            Self::Nz => 103,
        }
    }
    fn default_value(self) -> Option<i32> {
        match self {
            Self::It => Some(6),
            Self::Nt => Some(2),
            Self::Nz => Some(1),
            _ => None,
        }
    }
}

/// Derived integer or a specific missing prerequisite, never a guessed fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsFieldValue {
    /// Completed derivation within the established numeric domain.
    Derived(i32),
    /// IT/NT/NZ require a physically stored record-7 slot 88.
    RequiresIntegerSlot88,
    /// Native floating-point exception state lies outside this projection.
    RequiresFloatingPointContext,
}

/// One diagnostic projection retaining its exact binary64 source pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsFieldProjection {
    field: FlowSettingsField,
    stored_bits: u64,
    value: FlowSettingsFieldValue,
}

impl FlowSettingsFieldProjection {
    /// Returns the source-bound label and slot; e.g. `let slot = projection.field().slot_index()`.
    pub fn field(&self) -> FlowSettingsField {
        self.field
    }
    /// Returns the untouched binary64 bits; e.g. `let stored = f64::from_bits(projection.stored_bits())`.
    pub fn stored_bits(&self) -> u64 {
        self.stored_bits
    }
    /// Returns the derivation or missing prerequisite; e.g. `let value = projection.value()`.
    pub fn value(&self) -> FlowSettingsFieldValue {
        self.value
    }
}

/// All seven diagnostic projections with retained source arrays and exact checkpoint.
#[derive(Clone, Debug)]
pub struct FlowSettingsFields {
    numeric: FlowFixedSettings,
    integer: Option<FlowFixedSettings>,
    guard: Option<i32>,
    fields: [FlowSettingsFieldProjection; 7],
}

/// Exact projection check scope, separate from normalization and native execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsFieldsCheckMode {
    /// All seven stored fields and their explicit prerequisite states were checked.
    ExactStoredFieldProjection,
}

/// Counts distinguish derived results from unresolved required context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsFieldsCoverage {
    /// Explicit check scope.
    pub mode: FlowSettingsFieldsCheckMode,
    /// Established integer results.
    pub derived_fields: usize,
    /// Fields without required integer slot 88.
    pub missing_guard_fields: usize,
    /// Fields requiring native floating-point context.
    pub floating_point_context_fields: usize,
    /// Exact disagreement with retained stored bits, zero for completed views.
    pub achieved_byte_error: usize,
}

impl FlowCarrier {
    /// Projects seven diagnostics without normalizing arrays; e.g. `let fields = carrier.settings_fields()?`.
    pub fn settings_fields(&self) -> Result<Option<FlowSettingsFields>> {
        let Some(numeric) = self.fixed_settings(FlowFixedSettingsKind::Numeric)? else {
            return Ok(None);
        };
        let integer = self.fixed_settings(FlowFixedSettingsKind::Integer)?;
        let guard = integer
            .as_ref()
            .and_then(|view| view.slot(88))
            .and_then(FlowFixedSlot::signed);
        let fields = project_fields(&numeric, guard)?;
        Ok(Some(FlowSettingsFields {
            numeric,
            integer,
            guard,
            fields,
        }))
    }
}

impl FlowSettingsFields {
    /// Borrows the exact numeric source array; e.g. `let raw = fields.numeric().as_bytes()`.
    pub fn numeric(&self) -> &FlowFixedSettings {
        &self.numeric
    }
    /// Borrows the optional guard array; e.g. `let present = fields.integer().is_some()`.
    pub fn integer(&self) -> Option<&FlowFixedSettings> {
        self.integer.as_ref()
    }
    /// Returns the signed stored guard without a default; e.g. `let guard = fields.integer_slot_88()`.
    pub fn integer_slot_88(&self) -> Option<i32> {
        self.guard
    }
    /// Borrows all seven fields in native diagnostic order; e.g. `let first = &fields.fields()[0]`.
    pub fn fields(&self) -> &[FlowSettingsFieldProjection; 7] {
        &self.fields
    }
    /// Reports all derived and unresolved prerequisites; e.g. `let count = fields.coverage().derived_fields`.
    pub fn coverage(&self) -> FlowSettingsFieldsCoverage {
        let mut coverage = FlowSettingsFieldsCoverage {
            mode: FlowSettingsFieldsCheckMode::ExactStoredFieldProjection,
            derived_fields: 0,
            missing_guard_fields: 0,
            floating_point_context_fields: 0,
            achieved_byte_error: 0,
        };
        for field in self.fields {
            match field.value {
                FlowSettingsFieldValue::Derived(_) => coverage.derived_fields += 1,
                FlowSettingsFieldValue::RequiresIntegerSlot88 => coverage.missing_guard_fields += 1,
                FlowSettingsFieldValue::RequiresFloatingPointContext => {
                    coverage.floating_point_context_fields += 1
                }
            }
        }
        coverage
    }
    /// Borrows the raw-profile execution exclusions; e.g. `let exclusions = fields.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowFixedSettingsDiagnostic] {
        self.numeric.diagnostics()
    }
}

fn project_fields(
    numeric: &FlowFixedSettings,
    guard: Option<i32>,
) -> Result<[FlowSettingsFieldProjection; 7]> {
    let mut result = [FlowSettingsFieldProjection {
        field: FlowSettingsField::External,
        stored_bits: 0,
        value: FlowSettingsFieldValue::RequiresFloatingPointContext,
    }; 7];
    for (destination, field) in result.iter_mut().zip(FIELD_ORDER) {
        let bits = numeric_field_bits(numeric, field)?;
        *destination = FlowSettingsFieldProjection {
            field,
            stored_bits: bits,
            value: derive_field(field, f64::from_bits(bits), guard),
        };
    }
    Ok(result)
}

fn derive_field(
    field: FlowSettingsField,
    stored: f64,
    guard: Option<i32>,
) -> FlowSettingsFieldValue {
    use FlowSettingsFieldValue::*;
    if field == FlowSettingsField::Cht {
        return if stored.is_nan() {
            RequiresFloatingPointContext
        } else {
            Derived(i32::from(stored > 0.0))
        };
    }
    let Some(converted) = truncate_i32(stored) else {
        return RequiresFloatingPointContext;
    };
    if field == FlowSettingsField::External {
        return Derived(i32::from(converted == 0));
    }
    derive_guarded(field, converted, guard)
}

fn derive_guarded(
    field: FlowSettingsField,
    converted: i32,
    guard: Option<i32>,
) -> FlowSettingsFieldValue {
    use FlowSettingsFieldValue::*;
    let Some(default) = field.default_value() else {
        return Derived(converted);
    };
    match guard {
        None => RequiresIntegerSlot88,
        Some(0) => Derived(if converted == 0 { default } else { converted }),
        Some(_) => Derived(0),
    }
}

fn truncate_i32(value: f64) -> Option<i32> {
    if !value.is_finite() {
        return None;
    }
    let truncated = value.trunc();
    if truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX) {
        return None;
    }
    // The explicit representability check preserves native-domain truncation rather than saturation.
    Some(truncated as i32)
}

fn numeric_field_bits(numeric: &FlowFixedSettings, field: FlowSettingsField) -> Result<u64> {
    if let Some(FlowFixedSlot::Numeric(bits)) = numeric.slot(field.slot_index()) {
        return Ok(bits);
    }
    Err(BessemerError::new(
        "Flow settings fields",
        field.label(),
        FailureClass::Invalid,
        format!(
            "missing numeric slot {}; expected validated 320-slot record 102",
            field.slot_index()
        ),
    ))
}
