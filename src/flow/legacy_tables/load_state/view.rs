use super::*;

impl FlowLoadState {
    /// Borrows the complete carrier checkpoint; e.g. `let bytes = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected physical occurrence; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the validated version18 prefix; e.g. `let version = state.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the declared external mode context; e.g. `let mode = state.mode()`.
    pub fn mode(&self) -> FlowLoadMode {
        self.options.mode
    }
    /// Returns the admitted bounds and mode; e.g. `let options = state.options()`.
    pub fn options(&self) -> FlowLoadStateOptions {
        self.options
    }
    /// Borrows the complete raw payload; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Enumerates every physical field, including empty arrays; e.g. `let count = state.fields().len()`.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FlowLoadField<'_>> {
        self.fields
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowLoadField {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Borrows one physical descriptor by ordinal; e.g. `let version = state.field_at(0)`.
    pub fn field_at(&self, ordinal: usize) -> Option<FlowLoadField<'_>> {
        Some(FlowLoadField {
            owner: self,
            ordinal,
            span: self.fields.get(ordinal)?,
        })
    }
    /// Resolves only a field from the same occurrence, revision and mode; e.g. `let field = state.field(&id)?`.
    pub fn field(&self, id: &FlowLoadFieldId) -> Result<FlowLoadField<'_>> {
        if id.record != self.wire.record || id.mode != self.options.mode {
            return Err(load_invalid(format!(
                "field identity {id:?}; expected record {:?} and mode {:?}",
                self.wire.record, self.options.mode
            )));
        }
        self.field_at(id.ordinal)
            .ok_or_else(|| table_index_error(11, "field", id.ordinal, self.fields.len()))
    }
    /// Enumerates complete scopes in physical parent-before-child order; e.g. `let count = state.scopes().len()`.
    pub fn scopes(&self) -> impl ExactSizeIterator<Item = FlowLoadScopeView> + '_ {
        self.scopes.iter().map(|span| FlowLoadScopeView {
            scope: span.scope,
            payload_interval: span.interval,
        })
    }
    /// Enumerates complete stored name/description views; e.g. `let count = state.texts().len()`.
    pub fn texts(&self) -> impl ExactSizeIterator<Item = FlowLoadText<'_>> {
        self.texts
            .iter()
            .map(|span| FlowLoadText { owner: self, span })
    }
    /// Borrows a physical text by ordinal; e.g. `let first = state.text_at(0)`.
    pub fn text_at(&self, ordinal: usize) -> Option<FlowLoadText<'_>> {
        Some(FlowLoadText {
            owner: self,
            span: self.texts.get(ordinal)?,
        })
    }
    /// Returns exact serialized accounting under the declared mode; e.g. `let items = state.coverage().items`.
    pub fn coverage(&self) -> FlowLoadCoverage {
        FlowLoadCoverage {
            check_mode: FlowLoadCheckMode::ExactRawDeclaredLoadMode,
            load_mode: self.options.mode,
            payload_bytes: self.wire.decoded.len(),
            items: self.totals[0],
            array_width: self.width,
            text_units: self.totals[1],
            array_values: self.totals[2],
            values: self.totals[3],
            fields: self.fields.len(),
            scopes: self.scopes.len(),
            texts: self.texts.len(),
            mode_tail_present: self.totals[0] > 0 && !self.options.mode.value(),
            achieved_error: 0,
        }
    }
    /// Borrows required context/runtime/engineering exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowLoadDiagnostic] {
        &[
            FlowLoadDiagnostic::LoadModeContextAssumed,
            FlowLoadDiagnostic::RuntimeTransformationsNotReproduced,
            FlowLoadDiagnostic::EngineeringRolesNotResolved,
            FlowLoadDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowLoadStateEdit {
    /// Borrows the original complete view; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowLoadState {
        &self.source
    }
    /// Borrows the complete reparsed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowLoadState {
        &self.result
    }
    /// Borrows full carrier checkpoints and physical history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
