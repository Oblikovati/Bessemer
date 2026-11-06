mod fields;
use super::*;

impl FlowBlockExtras {
    /// Borrows the complete original carrier checkpoint; e.g. `let raw = extras.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = extras.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows every decoded byte; e.g. `let raw = extras.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed legacy version; e.g. `let version = extras.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns original typed context and limits; e.g. `let count = extras.options().leaf_count`.
    pub fn options(&self) -> FlowBlockExtraOptions {
        self.options
    }
    /// Enumerates every field descriptor in physical parse order; e.g. `let count = extras.fields().len()`.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FlowBlockExtraField<'_>> {
        self.fields
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowBlockExtraField {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Borrows an existing field by ordinal; e.g. `let version_field = extras.field_at(0)`.
    pub fn field_at(&self, ordinal: usize) -> Option<FlowBlockExtraField<'_>> {
        Some(FlowBlockExtraField {
            owner: self,
            ordinal,
            span: self.fields.get(ordinal)?,
        })
    }
    /// Resolves a field only within its original revision and count context; e.g. `let field = extras.field(&id)?`.
    pub fn field(&self, id: &FlowBlockExtraFieldId) -> Result<FlowBlockExtraField<'_>> {
        if id.record != self.wire.record || id.leaf_count != self.options.leaf_count {
            return Err(extra_invalid(format!(
                "field identity {id:?}; expected record {:?} and leaf context {:?}",
                self.wire.record, self.options.leaf_count
            )));
        }
        self.field_at(id.ordinal)
            .ok_or_else(|| table_index_error(408, "field", id.ordinal, self.fields.len()))
    }
    /// Enumerates every complete scope in deterministic parent-before-child order; e.g. `let count = extras.scopes().len()`.
    pub fn scopes(&self) -> impl ExactSizeIterator<Item = FlowBlockExtraScope<'_>> {
        self.entities
            .iter()
            .map(|span| FlowBlockExtraScope { owner: self, span })
    }
    /// Reports exact wire coverage and caller assumptions; e.g. `let coverage = extras.coverage()`.
    pub fn coverage(&self) -> FlowBlockExtraCoverage {
        FlowBlockExtraCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            leaf_count: self.options.leaf_count,
            enabled: self.options.enabled,
            entities: self.entities.len(),
            fields: self.fields.len(),
            values: self.values,
            achieved_error: 0,
        }
    }
    /// Borrows runtime, reference, tree-order and application exclusions; e.g. `let exclusions = extras.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowBlockExtraDiagnostic] {
        &[
            FlowBlockExtraDiagnostic::EnabledContextNotReconstructed,
            FlowBlockExtraDiagnostic::TreeOrderNotAssigned,
            FlowBlockExtraDiagnostic::NativeObjectReferencesNotResolved,
            FlowBlockExtraDiagnostic::EngineeringRolesNotResolved,
            FlowBlockExtraDiagnostic::NativeRuntimeNotReproduced,
            FlowBlockExtraDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowBlockExtraScope<'_> {
    /// Returns all physical parent ordinals; e.g. `let entity = scope.entity()`.
    pub fn entity(self) -> FlowBlockExtraEntity {
        self.span.entity
    }
    /// Returns the complete decoded scope interval; e.g. `let interval = scope.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns a stored or full derived nested index, otherwise None; e.g. `let index = scope.native_index()`.
    pub fn native_index(self) -> Option<i32> {
        match self.span.index? {
            ExtraIndexSource::Derived(value) => Some(value),
            ExtraIndexSource::Stored(offset) => {
                Some(wire::signed_at(&self.owner.wire.decoded, offset))
            }
        }
    }
    /// Distinguishes stored nested indices from runtime defaults and non-index scopes; e.g. `let stored = scope.index_is_stored()`.
    pub fn index_is_stored(self) -> Option<bool> {
        self.span
            .index
            .map(|index| matches!(index, ExtraIndexSource::Stored(_)))
    }
}

impl FlowBlockExtraEdit {
    /// Borrows the original immutable extra view; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowBlockExtras {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowBlockExtras {
        &self.result
    }
    /// Borrows full physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
