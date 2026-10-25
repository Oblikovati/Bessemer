use super::{
    DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits, DisplayField,
    DisplayFieldValue, archive_error::invalid,
};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, ObjectReader},
};

pub(super) struct DisplayDecoder<'a> {
    pub cursor: ArchiveCursor<'a>,
    pub input: &'a [u8],
    pub context: DisplayArchiveContext,
    pub limits: DisplayArchiveLimits,
    pub objects: ObjectReader,
    pub external_component: Option<(std::sync::Arc<crate::archive_codec::ArchiveString>, u32)>,
    pub restoring_configurations: bool,
    pub drawing_buckets: bool,
    fields: Vec<DisplayField>,
    scopes: Vec<String>,
    loaded_objects: usize,
    entries: usize,
}

impl<'a> DisplayDecoder<'a> {
    pub fn new(
        input: &'a [u8],
        context: DisplayArchiveContext,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        super::view_support::require_size(
            input.len(),
            limits.maximum_bytes,
            "display-list input bytes",
        )?;
        Ok(Self {
            cursor: ArchiveCursor::new(input),
            input,
            context,
            limits,
            objects: ObjectReader::new(),
            external_component: None,
            restoring_configurations: false,
            drawing_buckets: false,
            fields: Vec::new(),
            scopes: vec!["root".into()],
            loaded_objects: 0,
            entries: 0,
        })
    }

    pub fn field(&mut self, name: &str, value: DisplayFieldValue) -> Result<()> {
        self.view_entries(&value)?;
        super::view_support::require_size(
            self.fields.len() + 1,
            self.limits.maximum_fields,
            "display-list fields",
        )?;
        let path = format!("{}.{}", self.scopes.join("."), name).into();
        self.fields.push(DisplayField { path, value });
        Ok(())
    }

    pub fn scope<T>(&mut self, name: &str, read: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        super::view_support::require_size(
            self.scopes.len(),
            self.limits.maximum_depth,
            "display-list scope depth",
        )?;
        self.scopes.push(name.into());
        let result = read(self);
        self.scopes.pop();
        result
    }

    pub fn charge_object(&mut self) -> Result<()> {
        self.loaded_objects = self
            .loaded_objects
            .checked_add(1)
            .ok_or_else(|| invalid("object count overflow; expected usize"))?;
        super::view_support::require_size(
            self.loaded_objects,
            self.limits.maximum_objects,
            "display-list objects",
        )
    }

    pub fn entries(&mut self, count: usize, label: &str) -> Result<()> {
        self.entries = self
            .entries
            .checked_add(count)
            .ok_or_else(|| invalid(format!("{label} total overflow; expected usize")))?;
        super::view_support::require_size(self.entries, self.limits.maximum_entries, label)
    }

    pub fn available_entries(&self) -> usize {
        self.limits.maximum_entries.saturating_sub(self.entries)
    }

    pub fn finish(self) -> Result<DisplayArchiveContents> {
        self.cursor.finish()?;
        Ok(DisplayArchiveContents {
            fields: self.fields,
        })
    }
}
