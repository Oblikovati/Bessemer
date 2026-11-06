use super::*;
impl FlowMediumMixedProfile {
    /// Replaces an existing first-table key with typed membership revalidation; e.g. `let edit=profile.replace_first_key(0,key)?`.
    pub fn replace_first_key(&self, ordinal: usize, key: FlowGuid) -> Result<FlowMediumMixedEdit> {
        self.patch(self.required_first(ordinal)?.full.0 + 4, key.as_bytes())
    }
    /// Replaces an existing second-table key and checks all outer references; e.g. `let edit=profile.replace_reference_key(0,key)?`.
    pub fn replace_reference_key(
        &self,
        ordinal: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumMixedEdit> {
        self.patch(self.required_reference(ordinal)?.full.0 + 6, key.as_bytes())
    }
    /// Replaces stored kind-one reference state; kind two has no such field; e.g. `let edit=profile.replace_reference_state(0,255)?`.
    pub fn replace_reference_state(
        &self,
        ordinal: usize,
        state: u8,
    ) -> Result<FlowMediumMixedEdit> {
        let entry = self.required_reference(ordinal)?;
        let (offset, _) = entry.state.ok_or_else(|| {
            metadata_invalid(format!(
                "reference {ordinal} kind {:?}; expected kind-one stored state field",
                entry.kind
            ))
        })?;
        self.patch(offset, &[state])
    }
    /// Replaces an ordered reference in its selected kind's map; e.g. `let edit=profile.replace_reference(0,0,key)?`.
    pub fn replace_reference(
        &self,
        ordinal: usize,
        index: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumMixedEdit> {
        let entry = self.required_reference(ordinal)?;
        require_index(index, entry.references.len(), "mixed typed reference")?;
        self.patch(entry.references_start + index * 16, key.as_bytes())
    }
    /// Replaces an outer reference with complete second-table membership checks; e.g. `let edit=profile.replace_outer_reference(0,key)?`.
    pub fn replace_outer_reference(
        &self,
        index: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumMixedEdit> {
        require_index(index, self.outer.keys.len(), "mixed outer reference")?;
        self.patch(self.outer.start + index * 16, key.as_bytes())
    }
    /// Replaces a signed outer attribute without numeric conversion; e.g. `let edit=profile.replace_outer_attribute(0,-1)?`.
    pub fn replace_outer_attribute(&self, index: usize, value: i32) -> Result<FlowMediumMixedEdit> {
        require_index(index, self.outer.attributes.len(), "mixed outer attribute")?;
        self.patch(
            self.outer.attributes_start + index * 4,
            &value.to_le_bytes(),
        )
    }
    /// Composes a selected kind-one body edit and validates the complete record; e.g. `let edit=profile.apply_kind_one_body_edit(0,&body_edit)?`.
    pub fn apply_kind_one_body_edit(
        &self,
        ordinal: usize,
        edit: &FlowMediumBodyEdit,
    ) -> Result<FlowMediumMixedEdit> {
        let before = edit.source();
        self.check_body_source(
            ordinal,
            FlowMediumKind::One,
            before.backend_prefix().metadata_prefix().record(),
            before.payload_interval(),
            before.body_bytes(),
        )?;
        self.finish_edit(edit.prefix_edit().clone())
    }
    /// Composes a selected kind-two body edit and validates the complete record; e.g. `let edit=profile.apply_kind_two_body_edit(0,&body_edit)?`.
    pub fn apply_kind_two_body_edit(
        &self,
        ordinal: usize,
        edit: &FlowMediumKindTwoEdit,
    ) -> Result<FlowMediumMixedEdit> {
        let before = edit.source();
        self.check_body_source(
            ordinal,
            FlowMediumKind::Two,
            before.backend_prefix().metadata_prefix().record(),
            before.payload_interval(),
            before.body_bytes(),
        )?;
        self.finish_edit(edit.prefix_edit().clone())
    }
    /// Composes an existing metadata-only edit with unchanged backend bytes; e.g. `let edit=profile.apply_metadata_edit(&metadata_edit)?`.
    pub fn apply_metadata_edit(
        &self,
        edit: &FlowMetadataPrefixEdit,
    ) -> Result<FlowMediumMixedEdit> {
        let before = edit.source();
        if before.record() != self.record()
            || before.payload_bytes() != self.payload_bytes()
            || before.opaque_tail() != edit.result().opaque_tail()
        {
            return Err(metadata_invalid(format!(
                "metadata edit source {:?}; expected current mixed profile {:?} and unchanged backend",
                before.record(),
                self.record()
            )));
        }
        self.finish_edit(edit.clone())
    }
    fn check_body_source(
        &self,
        ordinal: usize,
        kind: FlowMediumKind,
        record: &FlowRecordId,
        interval: (usize, usize),
        raw: &[u8],
    ) -> Result<()> {
        let entry = self.required_first(ordinal)?;
        if record == self.record()
            && kind == entry.body.kind()
            && interval == entry.body.payload_interval()
            && raw == entry.body.bytes()
        {
            return Ok(());
        }
        Err(metadata_invalid(format!(
            "body edit source {record:?}, kind {kind:?}, interval {interval:?}; expected current {:?}, {:?} at {:?}",
            self.record(),
            entry.body.kind(),
            entry.body.payload_interval()
        )))
    }
    fn required_first(&self, ordinal: usize) -> Result<&FlowMediumMixedEntry> {
        self.first
            .get(ordinal)
            .ok_or_else(|| table_index_error(424, "mixed first object", ordinal, self.first.len()))
    }
    fn required_reference(&self, ordinal: usize) -> Result<&FlowMediumMixedReference> {
        self.second.get(ordinal).ok_or_else(|| {
            table_index_error(424, "mixed reference object", ordinal, self.second.len())
        })
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowMediumMixedEdit> {
        self.finish_edit(self.metadata_prefix().patch(offset, bytes)?)
    }
    fn finish_edit(&self, prefix_edit: FlowMetadataPrefixEdit) -> Result<FlowMediumMixedEdit> {
        let prefix = prefix_edit.result();
        let result = prefix
            .source()
            .record_424_mixed_profile(prefix.record(), self.options)?;
        if !same_layout(self, &result) {
            return Err(metadata_invalid(format!(
                "edited mixed backend end {}; expected {} and unchanged entry boundaries",
                result.backend_end, self.backend_end
            )));
        }
        Ok(FlowMediumMixedEdit {
            source: self.clone(),
            result,
            prefix_edit,
        })
    }
}
fn require_index(index: usize, count: usize, label: &str) -> Result<()> {
    if index >= count {
        return Err(table_index_error(424, label, index, count));
    }
    Ok(())
}
fn same_layout(before: &FlowMediumMixedProfile, after: &FlowMediumMixedProfile) -> bool {
    before.backend_interval() == after.backend_interval()
        && before.payload_bytes().len() == after.payload_bytes().len()
        && before.outer_references_interval() == after.outer_references_interval()
        && before.outer_attributes_interval() == after.outer_attributes_interval()
        && before.first.len() == after.first.len()
        && before.second.len() == after.second.len()
        && before.first.iter().zip(after.first.iter()).all(|(a, b)| {
            a.full == b.full
                && a.body.kind() == b.body.kind()
                && a.body.payload_interval() == b.body.payload_interval()
        })
        && before.second.iter().zip(after.second.iter()).all(|(a, b)| {
            a.full == b.full
                && a.kind == b.kind
                && a.state.map(|(offset, _)| offset) == b.state.map(|(offset, _)| offset)
                && a.references_interval() == b.references_interval()
        })
}
