use super::*;

impl FlowPrimaryTable {
    /// Borrows the original complete carrier; e.g. `let bytes = table.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = table.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the parsed supported version; e.g. `let version = table.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows all raw payload bytes through exact EOF; e.g. `let raw = table.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the admitted class and resource bounds; e.g. `let class = table.options().profile`.
    pub fn options(&self) -> FlowPrimaryTableOptions {
        self.options
    }
    /// Enumerates fields in physical parse order, with columns kept strided; e.g. `let count = table.fields().len()`.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FlowPrimaryField<'_>> {
        self.fields
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowPrimaryField {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Borrows a descriptor by ordinal; e.g. `let version = table.field_at(0)`.
    pub fn field_at(&self, ordinal: usize) -> Option<FlowPrimaryField<'_>> {
        Some(FlowPrimaryField {
            owner: self,
            ordinal,
            span: self.fields.get(ordinal)?,
        })
    }
    /// Resolves an identity only in its original checkpoint; e.g. `let field = table.field(&id)?`.
    pub fn field(&self, id: &FlowPrimaryFieldId) -> Result<FlowPrimaryField<'_>> {
        if id.record != self.wire.record {
            return Err(primary_invalid(format!(
                "field identity {id:?}; expected record {:?}",
                self.wire.record
            )));
        }
        self.field_at(id.ordinal)
            .ok_or_else(|| table_index_error(439, "field", id.ordinal, self.fields.len()))
    }
    /// Enumerates complete scopes in parent-before-child order; e.g. `let scopes = table.scopes().len()`.
    pub fn scopes(&self) -> impl ExactSizeIterator<Item = FlowPrimaryScopeView> + '_ {
        self.scopes.iter().map(|span| FlowPrimaryScopeView {
            scope: span.scope,
            payload_interval: span.interval,
        })
    }
    /// Returns complete exact raw accounting; e.g. `let count = table.coverage().items`.
    pub fn coverage(&self) -> FlowPrimaryCoverage {
        FlowPrimaryCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.stored_bytes,
            items: self.totals[0],
            elements: self.totals[1],
            rows: self.totals[2],
            wide_blocks: self.totals[3],
            values: self.totals[4],
            scopes: self.scopes.len(),
            fields: self.fields.len(),
            achieved_error: 0,
        }
    }
    /// Borrows required class/runtime/engineering exclusions; e.g. `let exclusions = table.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowPrimaryDiagnostic] {
        &[
            FlowPrimaryDiagnostic::PrimaryClassAssumed,
            FlowPrimaryDiagnostic::PopulatedNativeNotChecked,
            FlowPrimaryDiagnostic::NativeRuntimeNotReproduced,
            FlowPrimaryDiagnostic::EngineeringRolesNotResolved,
            FlowPrimaryDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowPrimaryTableEdit {
    /// Borrows the original validated view; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowPrimaryTable {
        &self.source
    }
    /// Borrows the complete reparsed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowPrimaryTable {
        &self.result
    }
    /// Borrows physical history and full carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
