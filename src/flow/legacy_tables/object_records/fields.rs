use super::*;
impl FlowObjectRow<'_> {
    /// Mints a revision-bound physically stored scalar; e.g. `let id = row.field_id(FlowObjectField::Member10)?`.
    pub fn field_id(&self, field: FlowObjectField) -> Result<FlowObjectFieldId> {
        let (offset, width) = self.field_span(field)?;
        Ok(self.owner.field_identity(self.start + offset, width))
    }
    fn field_span(&self, field: FlowObjectField) -> Result<(usize, usize)> {
        use FlowObjectField::*;
        let version = self.owner.version();
        match field {
            Member10 => Ok((0, 4)),
            DiscardedWord(index) if version < 5 => {
                Ok((wire::element_offset(304, 4, 4, index, 4)?, 4))
            }
            BlockByte(index) => Ok((
                wire::element_offset(304, self.block_offset(), 16, index, 1)?,
                1,
            )),
            Member7C if version >= 4 => Ok((self.block_offset() + 16, 4)),
            StateByte if version >= 6 => Ok((24, 1)),
            ExtensionByte(index) if version == 7 => {
                Ok((wire::element_offset(304, 25, 56, index, 1)?, 1))
            }
            _ => Err(self.missing_field(field)),
        }
    }
    fn missing_field(&self, field: FlowObjectField) -> BessemerError {
        table_failure(
            304,
            FailureClass::Missing,
            format!(
                "version {}, row {}, field {field:?}; expected a physically stored field",
                self.owner.version(),
                self.ordinal
            ),
        )
    }
}
impl FlowObjectTrailer<'_> {
    /// Mints a revision-bound trailer scalar; e.g. `let id = trailer.field_id(FlowObjectTrailerField::Member10)?`.
    pub fn field_id(&self, field: FlowObjectTrailerField) -> Result<FlowObjectFieldId> {
        let (offset, width) = match field {
            FlowObjectTrailerField::Member10 => (self.start, 4),
            FlowObjectTrailerField::BlockByte(index) => {
                (wire::element_offset(304, self.start + 4, 16, index, 1)?, 1)
            }
        };
        Ok(self.owner.field_identity(offset, width))
    }
}
impl FlowObjectFieldId {
    /// Borrows its physical checkpoint identity; e.g. `let ordinal = id.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns its exact serialized scalar interval; e.g. `let span = id.interval()`.
    pub fn interval(&self) -> (usize, usize) {
        (self.offset, self.offset + self.width)
    }
}
impl FlowObjectRecords {
    fn field_identity(&self, offset: usize, width: usize) -> FlowObjectFieldId {
        FlowObjectFieldId {
            record: self.wire.record.clone(),
            offset,
            width,
        }
    }
    pub(super) fn check_field(&self, id: &FlowObjectFieldId, width: usize) -> Result<()> {
        if id.record == self.wire.record && id.width == width {
            return Ok(());
        }
        Err(table_failure(
            304,
            FailureClass::Invalid,
            format!(
                "field offset {}, width {}, revision matches {}; expected current revision and width {width}",
                id.offset,
                id.width,
                id.record == self.wire.record
            ),
        ))
    }
}
