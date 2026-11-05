use crate::model_archives::{ModelFieldHandle, ModelFieldValue};

/// Saved C-3341 settings, without inferred percentage, color or Boolean domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelBitmapAppearanceWords {
    /// Full signed raw64 matching-color-percentage setting.
    pub matching_color_percentage: i32,
    /// Full unsigned raw68 transparent-color setting; channel packing is unspecified.
    pub transparent_color: u32,
    /// Full signed raw80 ink-markup setting, absent before version0x2BD3.
    pub ink_markup_texture: Option<i32>,
}

/// An isolated saved setting replacement, without executing native refresh logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelBitmapAppearanceEdit {
    /// Replaces full signed raw64.
    MatchingColorPercentage(i32),
    /// Replaces full unsigned raw68.
    TransparentColor(u32),
    /// Replaces full signed raw80 only when the archive stores it.
    InkMarkupTexture(i32),
}

impl ModelBitmapAppearanceEdit {
    pub(super) fn replacement(self) -> (usize, ModelFieldValue) {
        match self {
            Self::MatchingColorPercentage(value) => (0, ModelFieldValue::I32(value)),
            Self::TransparentColor(value) => (1, ModelFieldValue::U32(value)),
            Self::InkMarkupTexture(value) => (2, ModelFieldValue::I32(value)),
        }
    }
}

/// Immutable saved appearance words and physical field identities for one revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBitmapAppearanceState {
    pub(super) words: ModelBitmapAppearanceWords,
    pub(super) source_fields: Vec<ModelFieldHandle>,
}

impl ModelBitmapAppearanceState {
    /// Copies the settings without normalization; e.g. `let words = appearance.words();`.
    pub fn words(&self) -> ModelBitmapAppearanceWords {
        self.words
    }

    /// Borrows raw64/raw68 and optional raw80 identities; e.g. `source.resolve_field(&appearance.source_fields()[0])?`.
    pub fn source_fields(&self) -> &[ModelFieldHandle] {
        &self.source_fields
    }
}
