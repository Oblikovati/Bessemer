//! C-885 initialization and raw transfers stay distinct from file state and later consumer roles (#166).
mod views;
use crate::{BessemerError, FailureClass, Result};

/// Words with C-885 initializer/preference provenance; `None` means absent from the supplied state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStyleRawWords {
    /// Raw member 1F0.
    pub word_1f0: Option<u32>,
    /// Raw member 204.
    pub word_204: Option<u32>,
    /// Raw member 208.
    pub word_208: Option<u32>,
    /// Raw member 230.
    pub word_230: Option<u32>,
    /// Raw member 234.
    pub word_234: Option<u32>,
    /// Raw member 238.
    pub word_238: Option<u32>,
    /// Raw member 23C.
    pub word_23c: Option<u32>,
    /// Raw member 240.
    pub word_240: Option<u32>,
    /// Raw member 28C.
    pub word_28c: Option<u32>,
    /// Raw member 290.
    pub word_290: Option<u32>,
    /// Raw member 294.
    pub word_294: Option<u32>,
    /// Raw member 298.
    pub word_298: Option<u32>,
}

/// Four raw preference transfers with exact member identities and distinct runtime structure offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStylePreferenceWords {
    /// Preference 15B4 to style204 to plotAdvPreference8C.
    pub word_204: u32,
    /// Preference 15BC to style208 to plotAdvPreference94.
    pub word_208: u32,
    /// Preference1574 to style230 to plotAdvPreference4C.
    pub word_230: u32,
    /// Preference152C to style240 to plotAdvPreference04.
    pub word_240: u32,
}

impl SimulationStyleRawWords {
    /// Return only the twelve bound Init assignments, never file defaults; e.g. `SimulationStyleRawWords::initializer_words()`.
    pub fn initializer_words() -> Self {
        Self {
            word_1f0: Some(0),
            word_204: Some(0),
            word_208: Some(0),
            word_230: Some(1),
            word_234: Some(0),
            word_238: Some(0),
            word_23c: Some(0),
            word_240: Some(1),
            word_28c: Some(200),
            word_290: Some(50),
            word_294: Some(0),
            word_298: Some(1),
        }
    }

    /// Apply only C-885's four raw transfers; `None` means a known null pointer; e.g. `words.after_local_preference_set(incoming, Some(73))`.
    pub fn after_local_preference_set(
        self,
        incoming: SimulationStylePreferenceWords,
        pointed_plot_word: Option<i32>,
    ) -> Self {
        if pointed_plot_word == Some(73) {
            return self;
        }
        Self {
            word_204: Some(incoming.word_204),
            word_208: Some(incoming.word_208),
            word_230: Some(incoming.word_230),
            word_240: Some(incoming.word_240),
            ..self
        }
    }

    /// Model only the raw234/238 local assignments after the inherited call; e.g. `words.after_local_preference_copy(7, 9, None)`.
    pub fn after_local_preference_copy(
        self,
        word_234: u32,
        word_238: u32,
        pointed_plot_word: Option<i32>,
    ) -> Self {
        let clear = matches!(pointed_plot_word, Some(70 | 73));
        Self {
            word_234: Some(if clear { 0 } else { word_234 }),
            word_238: Some(if clear { 0 } else { word_238 }),
            ..self
        }
    }

    /// Export only four known raw words, preserving required missing state; e.g. `words.local_preference_export()?`.
    pub fn local_preference_export(self) -> Result<SimulationStylePreferenceWords> {
        Ok(SimulationStylePreferenceWords {
            word_204: required_preference_word(self.word_204, "style.word_204")?,
            word_208: required_preference_word(self.word_208, "style.word_208")?,
            word_230: required_preference_word(self.word_230, "style.word_230")?,
            word_240: required_preference_word(self.word_240, "style.word_240")?,
        })
    }
}

fn required_preference_word(word: Option<u32>, path: &str) -> Result<u32> {
    word.ok_or_else(|| {
        BessemerError::new(
            "local raw style preference export",
            path,
            FailureClass::Missing,
            "word None; expected an explicit known raw uint32 value",
        )
    })
}
