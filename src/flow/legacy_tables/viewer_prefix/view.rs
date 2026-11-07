use super::*;
impl FlowViewerPrefix {
    /// Borrows the immutable carrier checkpoint; e.g. `let raw = prefix.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected occurrence identity; e.g. `let ordinal = prefix.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the admitted signed outer version; e.g. `let version = prefix.outer_version()`.
    pub fn outer_version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the original resource policy and explicit profile; e.g. `let profile = prefix.options().profile`.
    pub fn options(&self) -> FlowViewerPrefixOptions {
        self.options
    }
    /// Borrows all decoded bytes, including opaque continuation; e.g. `let raw = prefix.decoded_body()`.
    pub fn decoded_body(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the preserved ignored initial word; e.g. `let ignored = prefix.ignored_initial()`.
    pub fn ignored_initial(&self) -> i32 {
        wire::signed_at(&self.wire.decoded, 0)
    }
    /// Returns the physical second word before native normalization; e.g. `let stored = prefix.stored_second()`.
    pub fn stored_second(&self) -> i32 {
        wire::signed_at(&self.wire.decoded, 4)
    }
    /// Returns the final signed word of the known prefix; e.g. `let final_word = prefix.final_word()`.
    pub fn final_word(&self) -> i32 {
        wire::signed_at(&self.wire.decoded, self.layout.triplet_start + 24)
    }
    /// Returns the two documented native metadata outputs; e.g. `let outputs = prefix.native_metadata()`.
    pub fn native_metadata(&self) -> FlowViewerMetadata {
        let second = self.stored_second();
        FlowViewerMetadata {
            first_i32: self.final_word(),
            second_i32: if second == 1 { 2 } else { second },
        }
    }
    /// Borrows a checked raw array without engineering interpretation; e.g. `let array = prefix.array(0)?`.
    pub fn array(&self, ordinal: usize) -> Result<FlowTableWideArray<'_>> {
        let span = self
            .layout
            .arrays
            .get(ordinal)
            .ok_or_else(|| table_index_error(121, "viewer array", ordinal, 3))?;
        Ok(FlowTableWideArray {
            bytes: &self.wire.decoded[span.start..span.end],
            start: span.start,
        })
    }
    /// Borrows all three otherwise discarded raw patterns; e.g. `let bits = prefix.discarded_triplet().raw_bits(0)`.
    pub fn discarded_triplet(&self) -> FlowTableWideArray<'_> {
        let start = self.layout.triplet_start;
        FlowTableWideArray {
            bytes: &self.wire.decoded[start..start + 24],
            start,
        }
    }
    /// Borrows every opaque decoded continuation byte; e.g. `let opaque = prefix.opaque_tail()`.
    pub fn opaque_tail(&self) -> &[u8] {
        &self.wire.decoded[self.layout.prefix_end..]
    }
    /// Reports exact prefix scope and incomplete full-body validation separately; e.g. `let coverage = prefix.coverage()`.
    pub fn coverage(&self) -> FlowViewerPrefixCoverage {
        FlowViewerPrefixCoverage {
            mode: FlowViewerPrefixCheckMode::ExactVersionZlibAndKnownPrefix,
            profile: self.options.profile,
            stored_payload_bytes: self.wire.stored_bytes,
            raw_outer_prefix_bytes: 4,
            decoded_body_bytes: self.wire.decoded.len(),
            known_decoded_prefix_bytes: self.layout.prefix_end,
            opaque_decoded_bytes: self.opaque_tail().len(),
            known_wide_values: self.layout.wide_values,
            body_validation: FlowViewerBodyValidation::Incomplete,
            achieved_prefix_error: 0,
        }
    }
    /// Borrows profile, incomplete-body, native and interpretation exclusions; e.g. `let exclusions = prefix.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowViewerPrefixDiagnostic] {
        use FlowViewerPrefixDiagnostic::*;
        &[
            ViewerQueryProfileAssumed,
            OpaqueBodyNotValidated,
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked,
        ]
    }
}
impl FlowViewerPrefixEdit {
    /// Borrows the validated source prefix checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowViewerPrefix {
        &self.source
    }
    /// Borrows the reopened prefix result with preserved opaque continuation; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowViewerPrefix {
        &self.result
    }
    /// Borrows complete physical carrier history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
    /// Reports exact prefix checks with incomplete body status; e.g. `let coverage = edit.coverage()`.
    pub fn coverage(&self) -> FlowViewerPrefixCoverage {
        self.result.coverage()
    }
    /// Borrows retained profile/body and native acceptance exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowViewerPrefixDiagnostic] {
        self.result.diagnostics()
    }
}
