use super::*;
/// Complete raw record155 with its real version-zero identity and shared conditional-item ledger.
#[derive(Clone, Debug)]
pub struct FlowLegacyGroup155 {
    wire: TableWire,
    options: FlowConditionalItemOptions,
    body: conditional_items::ConditionalBody,
}
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowConditionalItemOptions,
) -> Result<FlowLegacyGroup155> {
    let wire = read_wire(source, id, options)?;
    if wire.version != 0 {
        return Err(table_failure(
            155,
            FailureClass::Unsupported,
            format!("version {}; expected stored version zero", wire.version),
        ));
    }
    let body = conditional_items::read_raw155(&wire, options)?;
    Ok(FlowLegacyGroup155 {
        wire,
        options,
        body,
    })
}
fn read_wire(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowConditionalItemOptions,
) -> Result<TableWire> {
    bounded_payload(source, id, 155, options.maximum_stored_bytes)?;
    wire::read(
        source,
        id,
        155,
        FlowLegacyTableTransport::Raw,
        options.maximum_decoded_bytes,
    )
}
fn check_structure(
    before: &FlowLegacyGroup155,
    after: &FlowLegacyGroup155,
    id: &FlowConditionalFieldId,
    value: FlowConditionalValue,
) -> Result<()> {
    if before.body.same_structure(&after.body) {
        return Ok(());
    }
    Err(invalid(
        155,
        format!(
            "field {id:?} value {value:?}; expected identical complete field and scope structure"
        ),
    ))
}
impl FlowLegacyGroup155 {
    /// Borrows the immutable physical checkpoint; e.g. `let raw = fields.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the exact record155 occurrence; e.g. `let ordinal = fields.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the admitted stored version; e.g. `assert_eq!(fields.version(), 0)`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows the complete raw payload including its real outer version; e.g. `let raw = fields.payload_bytes()`.
    pub fn payload_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the original independent grammar limits; e.g. `let limit = fields.options().maximum_items`.
    pub fn options(&self) -> FlowConditionalItemOptions {
        self.options
    }
    /// Enumerates every existing field in wire order; e.g. `let count = fields.fields().len()`.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FlowConditionalField<'_>> {
        self.body.fields(&self.wire)
    }
    /// Resolves a physical descriptor ordinal; e.g. `let field = fields.field_at(0)`.
    pub fn field_at(&self, ordinal: usize) -> Option<FlowConditionalField<'_>> {
        self.body.field_at(&self.wire, ordinal)
    }
    /// Resolves an exact original field identity; e.g. `let field = fields.field(&id)?`.
    pub fn field(&self, id: &FlowConditionalFieldId) -> Result<FlowConditionalField<'_>> {
        self.body.field(&self.wire, id)
    }
    /// Enumerates complete nested ancestry; e.g. `let scopes = fields.scopes().len()`.
    pub fn scopes(&self) -> impl ExactSizeIterator<Item = FlowConditionalScope<'_>> {
        self.body.scopes(&self.wire)
    }
    /// Resolves one physical scope ordinal; e.g. `let root = fields.scope_at(0)`.
    pub fn scope_at(&self, ordinal: usize) -> Option<FlowConditionalScope<'_>> {
        self.body.scope_at(&self.wire, ordinal)
    }
    /// Resolves an exact original scope identity; e.g. `let scope = fields.scope(&id)?`.
    pub fn scope(&self, id: &FlowConditionalScopeId) -> Result<FlowConditionalScope<'_>> {
        self.body.scope(&self.wire, id)
    }
    /// Reports complete raw grammar and exact aggregate counts; e.g. `let proof = fields.coverage()`.
    pub fn coverage(&self) -> FlowConditionalCoverage {
        self.body.coverage(&self.wire)
    }
    /// Borrows unresolved native and engineering qualifications; e.g. `let exclusions = fields.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowLegacyFieldDiagnostic] {
        use FlowLegacyFieldDiagnostic::*;
        &[
            EngineeringMeaningsNotAssigned,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked,
            NativeInverseMapsNotReproduced,
            NativeRecord155PayloadNotVerified,
        ]
    }
    /// Edits an existing nonstructural value with complete raw revalidation; e.g. `let edit = fields.replace_field(&id, 0, FlowConditionalValue::WordBits(7))?`.
    pub fn replace_field(
        &self,
        id: &FlowConditionalFieldId,
        index: usize,
        value: FlowConditionalValue,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let field = self.field(id)?;
        let offset = self.editable_offset(field, index)?;
        let bytes = wire::scalar_bytes(155, field.encoding(), value)?;
        let mut raw = self.wire.decoded.to_vec();
        raw[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = wire::edited(&self.wire, &raw)?;
        let record = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = read(carrier_edit.result(), &record, self.options)?;
        check_structure(self, &result, id, value)?;
        wire::check_decoded(&result.wire, &raw)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn editable_offset(&self, field: FlowConditionalField<'_>, index: usize) -> Result<usize> {
        if !field.is_editable() {
            return Err(invalid(
                155,
                format!(
                    "field {:?}; expected an existing nonstructural field",
                    field.kind()
                ),
            ));
        }
        field.element_offset(index).ok_or_else(|| {
            table_index_error(155, "field element", index, field.layout().element_count)
        })
    }
}
