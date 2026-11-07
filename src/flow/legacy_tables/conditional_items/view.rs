use super::*;

impl FlowConditionalItems {
    /// Borrows the complete source checkpoint; e.g. `let wire = items.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = items.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the normalized record grammar; e.g. `let format = items.format()`.
    pub fn format(&self) -> FlowConditionalFormat {
        self.format
    }
    /// Returns the outer version, including record128's raw prefix; e.g. `let version = items.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows complete decoded bytes; record128 excludes its raw prefix, e.g. `let body = items.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the admitted independent resource bounds; e.g. `let limit = items.options().maximum_items`.
    pub fn options(&self) -> FlowConditionalItemOptions {
        self.options
    }
    /// Enumerates every field, including empty vectors; e.g. `let count = items.fields().len()`.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FlowConditionalField<'_>> {
        self.fields
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowConditionalField {
                owner: &self.wire,
                ordinal,
                span,
            })
    }
    /// Resolves a physical field ordinal; e.g. `let version = items.field_at(0)`.
    pub fn field_at(&self, ordinal: usize) -> Option<FlowConditionalField<'_>> {
        Some(FlowConditionalField {
            owner: &self.wire,
            ordinal,
            span: self.fields.get(ordinal)?,
        })
    }
    /// Resolves only an original revision-bound field; e.g. `let field = items.field(&id)?`.
    pub fn field(&self, id: &FlowConditionalFieldId) -> Result<FlowConditionalField<'_>> {
        self.check_record(&id.record)?;
        self.field_at(id.ordinal).ok_or_else(|| {
            table_index_error(
                self.format.identifier(),
                "field",
                id.ordinal,
                self.fields.len(),
            )
        })
    }
    /// Enumerates complete scopes in parent-before-child order; e.g. `let count = items.scopes().len()`.
    pub fn scopes(&self) -> impl ExactSizeIterator<Item = FlowConditionalScope<'_>> {
        self.scopes
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowConditionalScope {
                owner: &self.wire,
                ordinal,
                span,
            })
    }
    /// Resolves one scope by physical descriptor ordinal; e.g. `let root = items.scope_at(0)`.
    pub fn scope_at(&self, ordinal: usize) -> Option<FlowConditionalScope<'_>> {
        Some(FlowConditionalScope {
            owner: &self.wire,
            ordinal,
            span: self.scopes.get(ordinal)?,
        })
    }
    /// Resolves only an original revision-bound scope; e.g. `let scope = items.scope(&id)?`.
    pub fn scope(&self, id: &FlowConditionalScopeId) -> Result<FlowConditionalScope<'_>> {
        self.check_record(&id.record)?;
        self.scope_at(id.ordinal).ok_or_else(|| {
            table_index_error(
                self.format.identifier(),
                "scope",
                id.ordinal,
                self.scopes.len(),
            )
        })
    }
    fn check_record(&self, record: &FlowRecordId) -> Result<()> {
        if *record == self.wire.record {
            return Ok(());
        }
        Err(conditional_invalid(
            self.format,
            format!(
                "identity record {record:?}; expected {:?}",
                self.wire.record
            ),
        ))
    }
    /// Returns complete framing/value accounting; e.g. `let bytes = items.coverage().decoded_bytes`.
    pub fn coverage(&self) -> FlowConditionalCoverage {
        FlowConditionalCoverage {
            mode: self.check_mode(),
            stored_bytes: self.wire.stored_bytes,
            stored_prefix_bytes: if self.format == FlowConditionalFormat::Record128 {
                4
            } else {
                0
            },
            decoded_bytes: self.wire.decoded.len(),
            wide_patterns_per_vector_element: self.scale,
            entries: self.totals[0],
            items: self.totals[1],
            groups: self.totals[2],
            subrecords: self.totals[3],
            vector_values: self.totals[4],
            values: self.totals[5],
            fields: self.fields.len(),
            scopes: self.scopes.len(),
            achieved_error: 0,
        }
    }
    fn check_mode(&self) -> FlowConditionalCheckMode {
        match self.format {
            FlowConditionalFormat::Record126 => FlowConditionalCheckMode::ExactRawPayload,
            FlowConditionalFormat::Record315 => FlowConditionalCheckMode::ExactZlibPayload,
            FlowConditionalFormat::Record128 => FlowConditionalCheckMode::ExactRawVersionZlibBody,
        }
    }
    /// Borrows native/engineering/codec exclusions; e.g. `let exclusions = items.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowConditionalDiagnostic] {
        use FlowConditionalDiagnostic::*;
        const EXCLUSIONS: [FlowConditionalDiagnostic; 5] = [
            NativeRuntimeNotReproduced,
            NativeInverseMapsNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
            CompressedRepresentationReencodedOnChange,
        ];
        &EXCLUSIONS[..if self.format == FlowConditionalFormat::Record126 {
            4
        } else {
            5
        }]
    }
}

impl FlowConditionalScope<'_> {
    /// Returns a revision-bound scope identity; e.g. `let id = scope.id()`.
    pub fn id(self) -> FlowConditionalScopeId {
        FlowConditionalScopeId {
            record: self.owner.record.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Returns the exact structural scope kind; e.g. `let kind = scope.kind()`.
    pub fn kind(self) -> FlowConditionalScopeKind {
        self.span.kind
    }
    /// Returns the complete decoded interval; e.g. `let interval = scope.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Returns physical position within its parent; e.g. `let ordinal = scope.ordinal_in_parent()`.
    pub fn ordinal_in_parent(self) -> usize {
        self.span.ordinal
    }
    /// Returns a checked parent identity, or None for the root; e.g. `let parent = scope.parent()`.
    pub fn parent(self) -> Option<FlowConditionalScopeId> {
        Some(FlowConditionalScopeId {
            record: self.owner.record.clone(),
            ordinal: self.span.parent?,
        })
    }
}

impl FlowConditionalItemEdit {
    /// Borrows the original complete view; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowConditionalItems {
        &self.source
    }
    /// Borrows the complete reparsed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowConditionalItems {
        &self.result
    }
    /// Borrows full carrier checkpoints and physical history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
