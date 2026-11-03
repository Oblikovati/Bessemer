//! Stored entity visual roles, preserving raw words independently of rendering and layer resolution.
mod edit;
mod projection;
mod roles;
#[cfg(test)]
#[path = "../../tests/entity_visual_state/words.rs"]
mod tests;
use crate::tessellation::StoredFloat;
pub use roles::{EntityVisualByLayerRole, EntityVisualCacheRole};

/// Owned physical values; modifying this copy never changes its source archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityVisualStoredWords {
    /// Leading layer-number word, including all sentinel values.
    pub layer_number: u16,
    /// Complete ByLayer word, including unresolved high bits.
    pub by_layer_flags: u16,
    /// Complete stored layer-attribute cache byte.
    pub layer_attribute_cache: u8,
    /// Complete color word; the high byte retains its unresolved meaning.
    pub color_word: u32,
    /// Line-style word, distinct from the leading layer number.
    pub line_style: u16,
    /// Signed stored width selector, without unit conversion.
    pub standard_width: i16,
    /// Raw float bits only when that field physically exists.
    pub custom_width: Option<StoredFloat>,
}

/// One explicit stored-role request; native setter cascades are outside this operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityVisualStoredEdit {
    /// Replace the complete layer number.
    LayerNumber(u16),
    /// Replace the complete ByLayer word.
    ByLayerFlags(u16),
    /// Change one documented ByLayer bit, preserving all other bits.
    ByLayerRole(EntityVisualByLayerRole, bool),
    /// Replace the complete cache byte.
    LayerAttributeCache(u8),
    /// Change one stored cache bit, independently of effective visibility.
    CacheRole(EntityVisualCacheRole, bool),
    /// Replace every color bit, including the unresolved high byte.
    ColorWord(u32),
    /// Replace low-byte red, green and blue, preserving the high byte.
    ColorRgb([u8; 3]),
    /// Replace the complete line-style word.
    LineStyle(u16),
    /// Change only the signed width selector.
    StandardWidth(i16),
    /// Change only an existing custom-width field; absent fields fail explicitly.
    CustomWidth(StoredFloat),
    /// Change the complete width pair with the same physical float presence.
    LineWidth {
        /// Signed stored width selector.
        standard: i16,
        /// Stored float presence must match the source profile.
        custom: Option<StoredFloat>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VisualScalar {
    U16(u16),
    U8(u8),
    U32(u32),
    I16(i16),
    F32(StoredFloat),
}

pub(crate) fn visual_role_error(
    path: &str,
    observed: impl std::fmt::Debug,
) -> crate::BessemerError {
    crate::BessemerError::new(
        "stored entity visual roles",
        path,
        crate::FailureClass::Invalid,
        format!(
            "{observed:?}; expected a complete same-scope U16/U16/U8/U32/U16/I16 visual record with an optional stored F32"
        ),
    )
}
