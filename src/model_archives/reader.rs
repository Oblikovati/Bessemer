use super::{ModelField, ModelFieldValue, invalid};
use crate::archive_codec::{ArchiveCursor, MaterialRead, ObjectReader, ObjectRecord};
use crate::header::ArchiveVersion;
use crate::{BessemerError, Result};

/// Explicit byte, field, collection, nesting and object budgets for one archive operation.
#[derive(Debug, Clone, Copy)]
pub struct ModelFieldLimits {
    /// Maximum input and reconstructed stream bytes.
    pub maximum_bytes: usize,
    /// Maximum typed stored fields.
    pub maximum_fields: usize,
    /// Maximum aggregate collection entries and kernel records.
    pub maximum_entries: usize,
    /// Maximum nested record scopes.
    pub maximum_depth: usize,
    /// Maximum newly loaded model objects.
    pub maximum_objects: usize,
}

pub(super) struct ModelDecoder<'a> {
    cursor: ArchiveCursor<'a>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    fields: Vec<ModelField>,
    scopes: Vec<String>,
    entries: usize,
    failure: Option<BessemerError>,
    loaded_objects: usize,
    context: ModelArchiveContext,
    scene_context: SceneArchiveContext,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
    kernel_expanded: usize,
    inherited_fields: Option<&'a [ModelField]>,
    verified_part_root: Option<super::part_archive::VerifiedPartRoot>,
    saved_helper_state: Option<super::configuration_manager::SavedHelperState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ModelArchiveContext {
    #[cfg(test)]
    Unspecified,
    OrdinaryPartExternalXml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SceneArchiveContext {
    Unselected,
    OwnedConfiguration,
}

impl<'a> ModelDecoder<'a> {
    pub fn referenced_count(&mut self, record: &ObjectRecord, label: &str) -> Result<usize> {
        let result =
            super::object_values::stored_object_count(self.object_fields(record), record, label);
        self.checked(result)
    }

    pub fn object_integer(&mut self, record: &ObjectRecord, label: &str) -> Result<i32> {
        let result =
            super::object_values::stored_object_integer(self.object_fields(record), record, label);
        self.checked(result)
    }

    pub fn section_size(&mut self, label: &str) -> Result<usize> {
        let size = self.i32(label)?;
        let remaining = self.remaining();
        let shape = if size >= 0 && size as usize <= remaining {
            Ok(size as usize)
        } else {
            Err(invalid(format!(
                "{label} section size {size}; expected 0..={remaining} available bytes"
            )))
        };
        self.checked(shape)
    }

    pub fn bounded<T>(
        &mut self,
        size: usize,
        read: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let span = self.scalar(|cursor| cursor.take(size))?;
        let outer = std::mem::replace(&mut self.cursor, ArchiveCursor::new(span));
        let result = read(self).and_then(|value| {
            self.cursor.finish().map_err(super::model_failure)?;
            Ok(value)
        });
        self.cursor = outer;
        self.checked(result)
    }

    #[cfg(test)]
    pub fn new(input: &'a [u8], version: ArchiveVersion, limits: ModelFieldLimits) -> Result<Self> {
        Self::new_in_context(input, version, limits, ModelArchiveContext::Unspecified)
    }

    pub fn new_in_context(
        input: &'a [u8],
        version: ArchiveVersion,
        limits: ModelFieldLimits,
        context: ModelArchiveContext,
    ) -> Result<Self> {
        require_limit(input.len(), limits.maximum_bytes, "model input bytes")?;
        Ok(Self {
            cursor: ArchiveCursor::new(input),
            version,
            limits,
            fields: Vec::new(),
            scopes: vec!["root".into()],
            entries: 0,
            failure: None,
            loaded_objects: 0,
            context,
            scene_context: SceneArchiveContext::Unselected,
            kernel: None,
            kernel_expanded: 0,
            inherited_fields: None,
            verified_part_root: None,
            saved_helper_state: None,
        })
    }

    pub fn with_inherited_fields(mut self, fields: &'a [ModelField]) -> Self {
        self.inherited_fields = Some(fields);
        self
    }

    pub(super) fn with_verified_part_root(
        mut self,
        root: super::part_archive::VerifiedPartRoot,
    ) -> Self {
        self.verified_part_root = Some(root);
        self
    }

    pub(super) fn require_current_part_unit(&mut self, component: &ObjectRecord) -> Result<()> {
        let result = super::component_detail_provenance::require_internal_unit(
            self.object_fields(component),
            component,
            self.verified_part_root,
        );
        self.checked(result)
    }

    pub(super) fn with_saved_helper_state(
        mut self,
        state: super::configuration_manager::SavedHelperState,
    ) -> Self {
        self.saved_helper_state = Some(state);
        self
    }

    pub(super) fn sheet_metal_extended(&self) -> bool {
        // C-182: the native helper constructor's default is zero outside a persisted handoff.
        self.saved_helper_state == Some(super::configuration_manager::SavedHelperState::Active)
    }

    pub(super) fn require_saved_helper_state(&mut self) -> Result<()> {
        if self.saved_helper_state.is_some() {
            return Ok(());
        }
        self.checked(Err(BessemerError::new(
            "model archive", "sheet-metal context", crate::FailureClass::Unsupported,
            format!("saved helper state {:?}; expected state from a complete validated configuration manager", self.saved_helper_state),
        )))
    }

    pub(super) fn configuration_scene<T>(
        &mut self,
        read: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        super::configuration_manager::require_current(self)?;
        let previous = self.scene_context;
        self.scene_context = SceneArchiveContext::OwnedConfiguration;
        let result = read(self);
        self.scene_context = previous;
        self.checked(result)
    }

    pub(super) fn require_configuration_scene(&mut self) -> Result<()> {
        if self.scene_context == SceneArchiveContext::OwnedConfiguration {
            return self.active();
        }
        self.checked(Err(BessemerError::new("configuration scene", "archive context", crate::FailureClass::Unsupported,
            format!("scene context {:?}; expected explicitly selected OwnedConfiguration condition-200 scope", self.scene_context))))
    }

    fn object_fields(&self, record: &ObjectRecord) -> &[ModelField] {
        let Some(inherited) = self.inherited_fields else {
            return &self.fields;
        };
        if super::object_values::contains_definition(&self.fields, record) {
            return &self.fields;
        }
        inherited
    }

    pub fn with_kernel(
        mut self,
        context: Option<super::kernel_frames::CompressedKernelContext>,
    ) -> Self {
        self.kernel = context;
        self
    }

    pub fn kernel_context(&self) -> Option<&super::kernel_frames::CompressedKernelContext> {
        self.kernel.as_ref()
    }

    pub fn remaining_entries(&self) -> usize {
        self.limits.maximum_entries.saturating_sub(self.entries)
    }

    pub fn kernel_allowance(&self) -> usize {
        self.kernel.as_ref().map_or(0, |context| {
            context
                .maximum_expanded
                .saturating_sub(self.kernel_expanded)
        })
    }

    pub fn charge_kernel(&mut self, expanded: usize, records: usize) -> Result<()> {
        let allowance = self.kernel_allowance();
        if expanded > allowance {
            return self.checked(Err(invalid(format!(
                "kernel expansion {expanded}; expected <= {allowance} remaining bytes"
            ))));
        }
        self.entries(records, "kernel transmit records")?;
        self.kernel_expanded += expanded;
        Ok(())
    }

    pub fn require_ordinary_part(&mut self) -> Result<()> {
        if self.context == ModelArchiveContext::OrdinaryPartExternalXml
            && self.version.value() > 0x27cb
        {
            return self.active();
        }
        let error = BessemerError::new(
            "model archive",
            "ordinary part context",
            crate::FailureClass::Unsupported,
            format!(
                "version {} context {:?}; expected > 0x27CB and OrdinaryPartExternalXml",
                self.version.value(),
                self.context
            ),
        );
        self.checked(Err(error))
    }

    pub fn field(&mut self, label: &str, value: ModelFieldValue) -> Result<()> {
        let budget = require_limit(
            self.fields.len() + 1,
            self.limits.maximum_fields,
            "model fields",
        );
        self.checked(budget)?;
        let path = format!("{}.{}", self.scopes.join("."), label).into();
        self.fields.push(ModelField { path, value });
        Ok(())
    }

    pub fn scope<T>(&mut self, name: &str, read: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        let budget = require_limit(
            self.scopes.len(),
            self.limits.maximum_depth,
            "model scope depth",
        );
        self.checked(budget)?;
        self.scopes.push(name.into());
        let result = read(self);
        self.scopes.pop();
        self.checked(result)
    }

    pub fn entries(&mut self, count: usize, label: &str) -> Result<()> {
        let total = self.entries.checked_add(count).ok_or_else(|| {
            invalid(format!(
                "{label} count {count}; expected aggregate within usize"
            ))
        });
        let total = self.checked(total)?;
        let budget = require_limit(total, self.limits.maximum_entries, label);
        self.checked(budget)?;
        self.entries = total;
        Ok(())
    }

    pub fn finish(self) -> Result<Vec<ModelField>> {
        self.active()?;
        self.cursor.finish().map_err(super::model_failure)?;
        Ok(self.fields)
    }

    pub fn archive_version(&self) -> ArchiveVersion {
        self.version
    }

    pub fn remaining(&self) -> usize {
        self.cursor.remaining()
    }

    pub fn object_tag(
        &mut self,
        label: &str,
        map: &mut ObjectReader,
        role: super::ModelObjectRole<'_>,
    ) -> Result<ObjectRecord> {
        self.active()?;
        let result = map
            .framed_object(&mut self.cursor)
            .map_err(super::model_failure);
        let framed = self.checked(result)?;
        let class_check = super::objects::require_role(&framed.record, role, label);
        self.checked(class_check)?;
        self.charge_new_object(&framed.record)?;
        let record = framed.record.clone();
        self.field(label, ModelFieldValue::ObjectTag(Box::new(framed)))?;
        Ok(record)
    }

    fn charge_new_object(&mut self, record: &ObjectRecord) -> Result<()> {
        if !matches!(record, ObjectRecord::New { .. }) {
            return self.active();
        }
        let count = self.loaded_objects.checked_add(1).ok_or_else(|| {
            invalid(format!(
                "model object count {}; expected increment within usize",
                self.loaded_objects
            ))
        });
        let count = self.checked(count)?;
        let budget = require_limit(count, self.limits.maximum_objects, "model objects");
        self.checked(budget)?;
        self.loaded_objects = count;
        Ok(())
    }

    pub fn scalar<T>(
        &mut self,
        read: impl FnOnce(&mut ArchiveCursor<'a>) -> Result<T>,
    ) -> Result<T> {
        self.active()?;
        let result = read(&mut self.cursor).map_err(super::model_failure);
        self.checked(result)
    }

    fn active(&self) -> Result<()> {
        self.failure.clone().map_or(Ok(()), Err)
    }

    pub(super) fn checked<T>(&mut self, result: Result<T>) -> Result<T> {
        self.active()?;
        if let Err(error) = &result {
            self.failure = Some(error.clone());
        }
        result
    }
}

fn require_limit(actual: usize, maximum: usize, label: &str) -> Result<()> {
    if actual > maximum {
        return Err(invalid(format!("{label} {actual}; expected <= {maximum}")));
    }
    Ok(())
}
