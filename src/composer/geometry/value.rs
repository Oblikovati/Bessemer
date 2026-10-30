use super::ComposerSceneText;
use crate::archive_codec::StoredDouble;
use crate::composer::arrays::{ComposerIndexBlock, ComposerVectorBlock};
use std::sync::Arc;

/// Explicit storage variants used by the complete scene grammar.
#[derive(Debug, Clone)]
pub enum ComposerSceneValue {
    /// Stored one-byte control/scalar value, without boolean normalization.
    Byte(u8),
    /// Stored unsigned four-byte word or uninterpreted binary32 pattern.
    Word(u32),
    /// Stored unsigned eight-byte word.
    Wide(u64),
    /// Stored binary64 version pattern.
    Double(StoredDouble),
    /// Terminated raw UTF-16 text.
    Text(ComposerSceneText),
    /// Fixed-length pattern array (matrices, colors and user-data vectors).
    Words(Arc<[u32]>),
    /// Complete packed vector with its actual caller context.
    Vector(ComposerVectorBlock),
    /// Complete unsigned index/length block with its actual caller context.
    Indices(ComposerIndexBlock),
    /// Only an explicitly bounded documented future/unknown field.
    SizedExtension(Arc<[u8]>),
}

impl PartialEq for ComposerSceneValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Byte(a), Self::Byte(b)) => a == b,
            (Self::Word(a), Self::Word(b)) => a == b,
            (Self::Wide(a), Self::Wide(b)) => a == b,
            (Self::Double(a), Self::Double(b)) => a == b,
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Words(a), Self::Words(b)) => a == b,
            (Self::SizedExtension(a), Self::SizedExtension(b)) => a == b,
            (Self::Vector(a), Self::Vector(b)) => vector_equal(a, b),
            (Self::Indices(a), Self::Indices(b)) => {
                a.count() == b.count() && a.domain() == b.domain() && a.values().eq(b.values())
            }
            _ => false,
        }
    }
}

impl Eq for ComposerSceneValue {}

pub(super) fn value_shape(value: &ComposerSceneValue) -> String {
    match value {
        ComposerSceneValue::Byte(word) => format!("byte {word}"),
        ComposerSceneValue::Word(word) => format!("word 0x{word:08x}"),
        ComposerSceneValue::Wide(word) => format!("wide 0x{word:016x}"),
        ComposerSceneValue::Double(word) => format!("double pattern 0x{:016x}", word.bits()),
        ComposerSceneValue::Text(text) => text_shape(text),
        ComposerSceneValue::Words(words) => format!("{} fixed patterns", words.len()),
        ComposerSceneValue::Vector(block) => {
            format!("vector {:?}, {:?}", block.layout(), block.packing())
        }
        ComposerSceneValue::Indices(block) => {
            format!("indices count {}, domain {}", block.count(), block.domain())
        }
        ComposerSceneValue::SizedExtension(bytes) => {
            format!("sized extension {} bytes", bytes.len())
        }
    }
}

fn text_shape(text: &ComposerSceneText) -> String {
    format!(
        "text {} units, {:?} terminator",
        text.units.len(),
        text.terminator
    )
}

fn vector_equal(first: &ComposerVectorBlock, second: &ComposerVectorBlock) -> bool {
    first.layout() == second.layout()
        && first.packing() == second.packing()
        && first.axes() == second.axes()
        && first.packed_bytes() == second.packed_bytes()
}
