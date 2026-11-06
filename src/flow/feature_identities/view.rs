use super::*;

impl FlowFeatureIdentities {
    /// Borrows the complete original checkpoint; e.g. `let raw = list.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the revision-bound selected occurrence; e.g. `let ordinal = list.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows every selected payload byte; e.g. `let payload = list.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    /// Returns the validated list version, independent of object words; e.g. `let version = list.version()`.
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Borrows all physical entries in wire order; e.g. `let count = list.objects().len()`.
    pub fn objects(&self) -> &[FlowFeatureIdentity] {
        &self.objects
    }
    /// Borrows stored or virtual tail state; e.g. `let tail = list.tail()`.
    pub fn tail(&self) -> &FlowFeatureTail {
        &self.tail
    }
    /// Returns the accepted bounds and explicit context; e.g. `let context = list.options().legacy_tail_count`.
    pub fn options(&self) -> FlowFeatureOptions {
        self.options
    }
    /// Reports complete stored-wire checks; e.g. `let checked = list.coverage()`.
    pub fn coverage(&self) -> FlowFeatureCoverage {
        let tail = match &self.tail {
            FlowFeatureTail::Stored(values) => values.len(),
            _ => 0,
        };
        FlowFeatureCoverage {
            mode: FlowFeatureCheckMode::ExactRawIdentityList,
            checked_bytes: self.payload.len(),
            checked_objects: self.objects.len(),
            checked_text_units: self.total_text_units,
            checked_tail_values: tail,
            achieved_byte_error: 0,
        }
    }
    /// Borrows execution exclusions; e.g. `let exclusions = list.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowFeatureDiagnostic] {
        &[
            FlowFeatureDiagnostic::RuntimeObjectMatchingNotExecuted,
            FlowFeatureDiagnostic::AlternateStringModesNotChecked,
            FlowFeatureDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowFeatureIdentity {
    /// Returns the exact payload-relative entry interval; e.g. `let (start, end) = object.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    /// Returns the independent stored header word; e.g. `let header = object.header_version()`.
    pub fn header_version(&self) -> u32 {
        self.header_version
    }
    /// Returns all original factory-code bits; e.g. `let code = object.type_code()`.
    pub fn type_code(&self) -> u32 {
        self.type_code
    }
    /// Returns the independent identity-body word; e.g. `let body = object.body_version()`.
    pub fn body_version(&self) -> u32 {
        self.body_version
    }
    /// Returns the native factory class without constructing an object; e.g. `let class = object.native_class()`.
    pub fn native_class(&self) -> FlowFeatureClass {
        match self.type_code {
            0 => FlowFeatureClass::BoundaryCondition,
            5 => FlowFeatureClass::VSource,
            9 => FlowFeatureClass::SSSource,
            10 => FlowFeatureClass::Fan,
            _ => FlowFeatureClass::Unknown,
        }
    }
    /// Borrows every wire code unit, including suffixes and terminal NUL; e.g. `let raw = object.wire_text_units()`.
    pub fn wire_text_units(&self) -> &[u16] {
        &self.text
    }
    /// Borrows identity units up to the first NUL, preserving surrogate units; e.g. `let key = object.identity_units()`.
    pub fn identity_units(&self) -> &[u16] {
        let end = self
            .text
            .iter()
            .position(|unit| *unit == 0)
            .unwrap_or(self.text.len());
        &self.text[..end]
    }
    /// Returns the untouched state byte; e.g. `let raw = object.state_byte()`.
    pub fn state_byte(&self) -> u8 {
        self.state
    }
    /// Applies the established native nonzero conversion; e.g. `let enabled = object.state_boolean()`.
    pub fn state_boolean(&self) -> bool {
        self.state != 0
    }
}

impl FlowFeatureClass {
    /// Returns the native class spelling; e.g. `let name = object.native_class().native_name()`.
    pub fn native_name(self) -> &'static str {
        match self {
            Self::BoundaryCondition => "SBoundaryCondition",
            Self::VSource => "SVSource",
            Self::SSSource => "SSSource",
            Self::Fan => "SFan",
            Self::Unknown => "SUnknownFeature",
        }
    }
}

impl FlowFeatureEdit {
    /// Borrows the full earlier typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowFeatureIdentities {
        &self.source
    }
    /// Borrows the fully reparsed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowFeatureIdentities {
        &self.result
    }
    /// Borrows the complete physical edit and occurrence history; e.g. `let parents = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
