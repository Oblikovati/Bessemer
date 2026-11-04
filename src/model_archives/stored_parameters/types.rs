use super::*;
use std::sync::Arc;

/// Complete immutable archive that owns a projected parameter and its name dependencies.
#[derive(Debug, Clone)]
pub enum ModelParameterSource {
    /// Parameter physically defined in the complete configuration.
    Configuration(PartConfigurationArchive),
    /// Parameter physically defined in resolved features, with inherited configuration.
    ResolvedFeatures(PartResolvedFeaturesArchive),
}

/// Physical serialized class; no units or effective-value conversion is implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelParameterKind {
    /// moDensityParameter_c.
    Density,
    /// moLengthParameter_c.
    Length,
    /// moAngleParameter_c.
    Angle,
    /// moIntegerParameter_c.
    Integer,
    /// moScalerParameter_c attribute parameter.
    AttributeScalar,
    /// moStringParameter_c attribute parameter.
    AttributeString,
}

/// Exact primary or alternate stored value, retaining float bits and CString framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelParameterValue {
    /// Raw double bits, including NaN payloads and signed zero.
    Double(StoredDouble),
    /// Full signed integer domain.
    Integer(i32),
    /// Complete CString, including retained encoding and count width.
    Text(ArchiveString),
}

/// One name pointer and its resolved stored text, including inherited and shared definitions.
#[derive(Debug, Clone)]
pub struct ModelParameterName {
    pub(super) pointer: ModelFieldHandle,
    pub(super) tag: ModelObjectTag,
    pub(super) text: Option<ArchiveString>,
    pub(super) text_field: Option<ModelFieldHandle>,
}

/// Explicit replacement; changing presence must retain the existing serialized field shape.
#[derive(Debug, Clone)]
pub enum ModelParameterStoredEdit {
    /// Replace the primary value with the same stored type.
    Primary(ModelParameterValue),
    /// Replace an already serialized alternate or override value.
    Alternate(ModelParameterValue),
    /// Replace the raw presence byte without adding or removing its dependent value.
    AlternatePresence(u8),
}

/// Copied field relation with all physical source parents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelParameterFieldCopy {
    /// This stored projection has one physical parent per field.
    pub parents: [ModelFieldHandle; 1],
    /// Same-revision field identity in the projected result.
    pub result: ModelFieldHandle,
}

/// Semantic exclusions accompanying each complete stored parameter result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelParameterDiagnostic {
    /// Primary and alternate values have not been converted into an effective value.
    EffectiveValueAndUnitsNotEvaluated,
    /// No equation, native setter cascade or provider has been executed.
    EquationsAndNativeSettersNotExecuted,
    /// Geometry and external documents are outside stored parameter validation.
    GeometryAndExternalDocumentsNotChecked,
}

/// Immutable stored parameter, complete source, revision-bound fields and copied history.
#[derive(Debug, Clone)]
pub struct ModelParameterStoredState {
    pub(super) source: ModelParameterSource,
    pub(super) kind: ModelParameterKind,
    pub(super) names: [ModelParameterName; 2],
    pub(super) primary: ModelParameterValue,
    pub(super) alternate: Option<ModelParameterValue>,
    pub(super) presence: Option<u8>,
    pub(super) owner: ModelFieldHandle,
    pub(super) primary_field: ModelFieldHandle,
    pub(super) alternate_field: Option<ModelFieldHandle>,
    pub(super) presence_field: Option<ModelFieldHandle>,
    pub(super) copies: Arc<[ModelParameterFieldCopy]>,
}
