use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::optional_render_word_bits,
};
use super::SimulationStyleRawWords;
use crate::Result;

impl SimulationPostDialog {
    /// Read raw words with initializer/preference provenance without filling absent fields; e.g. `dialog.raw_style_words()?`.
    pub fn raw_style_words(&self) -> Result<SimulationStyleRawWords> {
        raw_style_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve optional raw fields from the canonical snapshot; e.g. `style.raw_style_words()?`.
    pub fn raw_style_words(&self) -> Result<SimulationStyleRawWords> {
        raw_style_from_snapshot(self)
    }
}

fn raw_style_from_snapshot<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<SimulationStyleRawWords> {
    Ok(SimulationStyleRawWords {
        word_1f0: optional_render_word_bits(snapshot, "style.word_1f0")?,
        word_204: optional_render_word_bits(snapshot, "style.word_204")?,
        word_208: optional_render_word_bits(snapshot, "style.word_208")?,
        word_230: optional_render_word_bits(snapshot, "style.word_230")?,
        word_234: optional_render_word_bits(snapshot, "style.word_234")?,
        word_238: optional_render_word_bits(snapshot, "style.word_238")?,
        word_23c: optional_render_word_bits(snapshot, "style.word_23c")?,
        word_240: optional_render_word_bits(snapshot, "style.word_240")?,
        word_28c: optional_render_word_bits(snapshot, "style.word_28c")?,
        word_290: optional_render_word_bits(snapshot, "style.word_290")?,
        word_294: optional_render_word_bits(snapshot, "style.word_294")?,
        word_298: optional_render_word_bits(snapshot, "style.word_298")?,
    })
}
