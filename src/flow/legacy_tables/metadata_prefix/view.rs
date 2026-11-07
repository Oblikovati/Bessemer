use super::*;

impl FlowMetadataPrefix {
    /// Borrows the immutable carrier checkpoint; e.g. `let bytes = prefix.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the revision-bound selected occurrence; e.g. `let ordinal = prefix.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows the complete raw payload including its tail; e.g. `let raw = prefix.payload_bytes()`.
    pub fn payload_bytes(&self) -> &[u8] {
        &self.raw
    }
    /// Returns the original explicit parsing options; e.g. `let options = prefix.options()`.
    pub fn options(&self) -> FlowMetadataPrefixOptions {
        self.options
    }
    /// Returns every stored version bit; e.g. `let version = prefix.version_byte()`.
    pub fn version_byte(&self) -> u8 {
        self.raw[0]
    }
    /// Returns the signed gate value; e.g. `let signed = prefix.signed_version()`.
    pub fn signed_version(&self) -> i8 {
        self.raw[0] as i8
    }
    /// Distinguishes the active native branch; e.g. `let active = prefix.body_active()`.
    pub fn body_active(&self) -> bool {
        self.signed_version() > 1
    }
    /// Borrows all undecoded backend or inactive bytes; e.g. `let tail = prefix.opaque_tail()`.
    pub fn opaque_tail(&self) -> &[u8] {
        &self.raw[self.end..]
    }
    /// Enumerates stored vectors in order; e.g. `let count = prefix.vectors().len()`.
    pub fn vectors(&self) -> impl ExactSizeIterator<Item = FlowMetadataVector<'_>> {
        (0..self.vectors.len()).map(|ordinal| FlowMetadataVector {
            owner: self,
            ordinal,
        })
    }
    /// Borrows one stored vector; e.g. `let first = prefix.vector(0)`.
    pub fn vector(&self, ordinal: usize) -> Option<FlowMetadataVector<'_>> {
        (ordinal < self.vectors.len()).then_some(FlowMetadataVector {
            owner: self,
            ordinal,
        })
    }
    /// Enumerates stored objects in order; e.g. `let count = prefix.objects().len()`.
    pub fn objects(&self) -> impl ExactSizeIterator<Item = FlowMetadataObject<'_>> {
        (0..self.objects.len()).map(|ordinal| FlowMetadataObject {
            owner: self,
            ordinal,
        })
    }
    /// Borrows one stored object; e.g. `let first = prefix.object(0)`.
    pub fn object(&self, ordinal: usize) -> Option<FlowMetadataObject<'_>> {
        (ordinal < self.objects.len()).then_some(FlowMetadataObject {
            owner: self,
            ordinal,
        })
    }
    /// Reports exact prefix coverage and explicit full-record incompleteness; e.g. `let scope = prefix.coverage()`.
    pub fn coverage(&self) -> FlowMetadataPrefixCoverage {
        FlowMetadataPrefixCoverage {
            payload_bytes: self.raw.len(),
            known_prefix_bytes: self.end,
            unknown_bytes: self.raw.len() - self.end,
            full_payload_schema: false,
            achieved_error: 0,
        }
    }
    /// Identifies the exact stopping branch; e.g. `let stop = prefix.stop()`.
    pub fn stop(&self) -> FlowMetadataPrefixStop {
        if self.body_active() {
            FlowMetadataPrefixStop::RequiresBackend
        } else {
            FlowMetadataPrefixStop::InactiveVersionBranch
        }
    }
    /// Borrows the native and backend exclusions; e.g. `let exclusions = prefix.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMetadataPrefixDiagnostic] {
        &[
            FlowMetadataPrefixDiagnostic::BackendNotDecoded,
            FlowMetadataPrefixDiagnostic::NativeRuntimeNotReproduced,
            FlowMetadataPrefixDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
impl FlowMetadataPrefixEdit {
    /// Borrows the unchanged typed input; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowMetadataPrefix {
        &self.source
    }
    /// Borrows the reparsed typed output; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowMetadataPrefix {
        &self.result
    }
    /// Borrows complete physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
