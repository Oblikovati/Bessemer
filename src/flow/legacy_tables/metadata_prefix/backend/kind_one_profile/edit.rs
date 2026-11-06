use super::*;
impl FlowMediumKindOneProfile {
    /// Replaces an existing first-table key, rejecting newly unresolved references; e.g. `let edit=profile.replace_first_key(0,key)?`.
    pub fn replace_first_key(
        &self,
        ordinal: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumProfileEdit> {
        self.patch(self.required_first(ordinal)?.full.0 + 4, key.as_bytes())
    }
    /// Replaces an existing second-table key with complete outer-reference checks; e.g. `let edit=profile.replace_reference_key(0,key)?`.
    pub fn replace_reference_key(
        &self,
        ordinal: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumProfileEdit> {
        self.patch(self.required_reference(ordinal)?.full.0 + 6, key.as_bytes())
    }
    /// Replaces a complete stored second-table state byte; e.g. `let edit=profile.replace_reference_state(0,255)?`.
    pub fn replace_reference_state(
        &self,
        ordinal: usize,
        state: u8,
    ) -> Result<FlowMediumProfileEdit> {
        self.patch(self.required_reference(ordinal)?.state, &[state])
    }
    /// Replaces one existing reference under fresh first-table membership checks; e.g. `let edit=profile.replace_reference(0,0,key)?`.
    pub fn replace_reference(
        &self,
        ordinal: usize,
        index: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumProfileEdit> {
        let reference = self.required_reference(ordinal)?;
        require_index(index, reference.references.len(), "reference key")?;
        self.patch(reference.references_start + index * 16, key.as_bytes())
    }
    /// Replaces one outer key under complete second-table membership checks; e.g. `let edit=profile.replace_outer_reference(0,key)?`.
    pub fn replace_outer_reference(
        &self,
        index: usize,
        key: FlowGuid,
    ) -> Result<FlowMediumProfileEdit> {
        require_index(index, self.outer.keys.len(), "outer key")?;
        self.patch(self.outer.start + index * 16, key.as_bytes())
    }
    /// Replaces a signed attribute without normalizing its bits; e.g. `let edit=profile.replace_outer_attribute(0,-1)?`.
    pub fn replace_outer_attribute(
        &self,
        index: usize,
        value: i32,
    ) -> Result<FlowMediumProfileEdit> {
        require_index(index, self.outer.attributes.len(), "outer attribute")?;
        self.patch(
            self.outer.attributes_start + index * 4,
            &value.to_le_bytes(),
        )
    }
    /// Validates a nested body edit as a complete record operation; e.g. `let edit=profile.apply_body_edit(0,&body_edit)?`.
    pub fn apply_body_edit(
        &self,
        ordinal: usize,
        edit: &FlowMediumBodyEdit,
    ) -> Result<FlowMediumProfileEdit> {
        let body = &self.required_first(ordinal)?.body;
        let before = edit.source();
        if before.backend_prefix().metadata_prefix().record() != self.record()
            || before.payload_interval() != body.payload_interval()
            || before.body_bytes() != body.body_bytes()
        {
            return Err(metadata_invalid(format!(
                "body edit source at {:?}; expected current profile occurrence {:?} and body {:?}",
                before.payload_interval(),
                self.record(),
                body.payload_interval()
            )));
        }
        self.finish_edit(edit.prefix_edit().clone())
    }
    /// Validates an existing metadata-only edit as a complete record operation; e.g. `let edit=profile.apply_metadata_edit(&metadata_edit)?`.
    pub fn apply_metadata_edit(
        &self,
        edit: &FlowMetadataPrefixEdit,
    ) -> Result<FlowMediumProfileEdit> {
        let before = edit.source();
        if before.record() != self.record()
            || before.payload_bytes() != self.payload_bytes()
            || before.opaque_tail() != edit.result().opaque_tail()
        {
            return Err(metadata_invalid(format!(
                "metadata edit source {:?}; expected current profile {:?} with unchanged backend bytes",
                before.record(),
                self.record()
            )));
        }
        self.finish_edit(edit.clone())
    }
    fn required_first(&self, ordinal: usize) -> Result<&FirstObject> {
        self.first.get(ordinal).ok_or_else(|| {
            table_index_error(424, "first profile object", ordinal, self.first.len())
        })
    }
    fn required_reference(&self, ordinal: usize) -> Result<&ReferenceObject> {
        self.second.get(ordinal).ok_or_else(|| {
            table_index_error(424, "profile reference object", ordinal, self.second.len())
        })
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowMediumProfileEdit> {
        self.finish_edit(self.metadata_prefix().patch(offset, bytes)?)
    }
    fn finish_edit(&self, prefix_edit: FlowMetadataPrefixEdit) -> Result<FlowMediumProfileEdit> {
        let prefix = prefix_edit.result();
        let result = prefix
            .source()
            .record_424_kind_one_profile(prefix.record(), self.options)?;
        self.check_layout(&result)?;
        Ok(FlowMediumProfileEdit {
            source: self.clone(),
            result,
            prefix_edit,
        })
    }
    fn check_layout(&self, after: &Self) -> Result<()> {
        if same_boundaries(self, after)
            && self
                .first
                .iter()
                .zip(after.first.iter())
                .all(|(a, b)| same_first(a, b))
            && self
                .second
                .iter()
                .zip(after.second.iter())
                .all(|(a, b)| same_reference(a, b))
        {
            return Ok(());
        }
        Err(profile_layout_error(self, after))
    }
}
fn require_index(index: usize, count: usize, label: &str) -> Result<()> {
    if index >= count {
        return Err(table_index_error(424, label, index, count));
    }
    Ok(())
}

fn same_first(a: &FirstObject, b: &FirstObject) -> bool {
    a.full == b.full && a.body.payload_interval() == b.body.payload_interval()
}
fn same_reference(a: &ReferenceObject, b: &ReferenceObject) -> bool {
    a.full == b.full
        && a.references_start == b.references_start
        && a.references.len() == b.references.len()
}
fn same_boundaries(before: &FlowMediumKindOneProfile, after: &FlowMediumKindOneProfile) -> bool {
    before.first.len() == after.first.len()
        && before.second.len() == after.second.len()
        && before.backend_end == after.backend_end
        && before.outer.start == after.outer.start
        && before.outer.attributes_start == after.outer.attributes_start
        && before.outer.keys.len() == after.outer.keys.len()
        && before.payload_bytes().len() == after.payload_bytes().len()
}
fn profile_layout_error(
    before: &FlowMediumKindOneProfile,
    after: &FlowMediumKindOneProfile,
) -> BessemerError {
    metadata_invalid(format!(
        "edited profile backend ends at {} with {} first/{} second entries; expected unchanged {} and {}/{} entry layouts",
        after.backend_end,
        after.first.len(),
        after.second.len(),
        before.backend_end,
        before.first.len(),
        before.second.len()
    ))
}
