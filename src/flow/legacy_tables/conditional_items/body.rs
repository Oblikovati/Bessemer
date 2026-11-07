use super::*;
/// Internal complete ledger shares grammar and borrowed views without extending public formats.
#[derive(Clone, Debug)]
pub(in crate::flow::legacy_tables) struct ConditionalBody {
    pub(super) fields: Arc<[ConditionalFieldSpan]>,
    pub(super) scopes: Arc<[ConditionalScopeSpan]>,
    pub(super) totals: [usize; 6],
}
impl ConditionalBody {
    pub(in crate::flow::legacy_tables) fn fields<'a>(
        &'a self,
        wire: &'a TableWire,
    ) -> impl ExactSizeIterator<Item = FlowConditionalField<'a>> {
        self.fields
            .iter()
            .enumerate()
            .map(|(ordinal, span)| view::field_view(wire, ordinal, span))
    }
    pub(in crate::flow::legacy_tables) fn field_at<'a>(
        &'a self,
        wire: &'a TableWire,
        ordinal: usize,
    ) -> Option<FlowConditionalField<'a>> {
        self.fields
            .get(ordinal)
            .map(|span| view::field_view(wire, ordinal, span))
    }
    pub(in crate::flow::legacy_tables) fn field<'a>(
        &'a self,
        wire: &'a TableWire,
        id: &FlowConditionalFieldId,
    ) -> Result<FlowConditionalField<'a>> {
        view::check_record_wire(wire, &id.record)?;
        self.field_at(wire, id.ordinal).ok_or_else(|| {
            table_index_error(wire.identifier, "field", id.ordinal, self.fields.len())
        })
    }
    pub(in crate::flow::legacy_tables) fn scopes<'a>(
        &'a self,
        wire: &'a TableWire,
    ) -> impl ExactSizeIterator<Item = FlowConditionalScope<'a>> {
        self.scopes
            .iter()
            .enumerate()
            .map(|(ordinal, span)| view::scope_view(wire, ordinal, span))
    }
    pub(in crate::flow::legacy_tables) fn scope_at<'a>(
        &'a self,
        wire: &'a TableWire,
        ordinal: usize,
    ) -> Option<FlowConditionalScope<'a>> {
        self.scopes
            .get(ordinal)
            .map(|span| view::scope_view(wire, ordinal, span))
    }
    pub(in crate::flow::legacy_tables) fn scope<'a>(
        &'a self,
        wire: &'a TableWire,
        id: &FlowConditionalScopeId,
    ) -> Result<FlowConditionalScope<'a>> {
        view::check_record_wire(wire, &id.record)?;
        self.scope_at(wire, id.ordinal).ok_or_else(|| {
            table_index_error(wire.identifier, "scope", id.ordinal, self.scopes.len())
        })
    }
    pub(in crate::flow::legacy_tables) fn coverage(
        &self,
        wire: &TableWire,
    ) -> FlowConditionalCoverage {
        FlowConditionalCoverage {
            mode: FlowConditionalCheckMode::ExactRawPayload,
            stored_bytes: wire.stored_bytes,
            stored_prefix_bytes: 0,
            decoded_bytes: wire.decoded.len(),
            wide_patterns_per_vector_element: 1,
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
    pub(in crate::flow::legacy_tables) fn same_structure(&self, other: &Self) -> bool {
        self.fields == other.fields && self.scopes == other.scopes
    }
}
