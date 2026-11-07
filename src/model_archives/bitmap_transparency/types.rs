use crate::model_archives::ModelFieldHandle;

/// Full signed saved words; no Boolean, percentage or closed enum domain is imposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelBitmapTransparencyWords {
    /// Full signed raw70 has-transparency setting.
    pub has_transparency_setting: i32,
    /// Full signed raw60 level parameter, without unit conversion.
    pub level_parameter: i32,
    /// Full signed raw6c type selector, without native classification.
    pub type_selector: i32,
}

/// One isolated saved-word replacement; no native setter or classifier executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelBitmapTransparencyEdit {
    /// Replaces only raw70.
    HasTransparencySetting(i32),
    /// Replaces only raw60.
    LevelParameter(i32),
    /// Replaces only raw6c.
    TypeSelector(i32),
}

impl ModelBitmapTransparencyEdit {
    pub(super) fn replacement(self) -> (usize, i32) {
        match self {
            Self::HasTransparencySetting(value) => (0, value),
            Self::LevelParameter(value) => (1, value),
            Self::TypeSelector(value) => (2, value),
        }
    }
}

/// Immutable settings and physical identities owned by one complete model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBitmapTransparencyState {
    pub(super) words: ModelBitmapTransparencyWords,
    pub(super) source_fields: [ModelFieldHandle; 3],
}

impl ModelBitmapTransparencyState {
    /// Copies all three signed settings without normalization; e.g. `let words = setting.words();`.
    pub fn words(&self) -> ModelBitmapTransparencyWords {
        self.words
    }

    /// Borrows has-transparency/level/type revision identities; e.g. `source.resolve_field(&setting.source_fields()[0])?`.
    pub fn source_fields(&self) -> &[ModelFieldHandle; 3] {
        &self.source_fields
    }
}
