use crate::model_archives::ModelFieldHandle;

/// Distinct full signed saved keys; neither is a moNode ID or file-map index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelSketchReferenceStoredLookupKeys {
    /// Saved6C configuration-object lookup key; no runtime entity is resolved.
    pub configuration_object_key: i32,
    /// Saved70 configuration-owned RefWrapperConfigData record key.
    pub configuration_record_key: i32,
}

/// Explicit saved-word replacements; arbitrary signed values and sentinels remain intact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelSketchReferenceStoredLookupEdit {
    /// Replace only saved6C.
    ConfigurationObjectKey(i32),
    /// Replace only saved70.
    ConfigurationRecordKey(i32),
    /// Replace both independent saved keys in one archive operation.
    BothKeys {
        /// Full signed saved6C replacement.
        configuration_object_key: i32,
        /// Full signed saved70 replacement.
        configuration_record_key: i32,
    },
}

impl ModelSketchReferenceStoredLookupEdit {
    pub(super) fn lookup_replacement_keys(
        self,
        original: ModelSketchReferenceStoredLookupKeys,
    ) -> [i32; 2] {
        match self {
            Self::ConfigurationObjectKey(value) => [value, original.configuration_record_key],
            Self::ConfigurationRecordKey(value) => [original.configuration_object_key, value],
            Self::BothKeys {
                configuration_object_key,
                configuration_record_key,
            } => [configuration_object_key, configuration_record_key],
        }
    }
}

/// Immutable saved keys and physical field identities owned by one model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSketchReferenceStoredLookupState {
    pub(super) keys: ModelSketchReferenceStoredLookupKeys,
    pub(super) source_fields: [ModelFieldHandle; 2],
}

impl ModelSketchReferenceStoredLookupState {
    /// Copies both saved keys without lookup or normalization; e.g. `let keys = state.keys();`.
    pub fn keys(&self) -> ModelSketchReferenceStoredLookupKeys {
        self.keys
    }

    /// Borrows saved6C/saved70 revision identities; e.g. `source.resolve_field(&state.source_fields()[0])?`.
    pub fn source_fields(&self) -> &[ModelFieldHandle; 2] {
        &self.source_fields
    }
}
