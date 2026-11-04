//! Explicit comparison inputs retain absent runtime state instead of applying guessed migrations (#166).
use super::{SimulationPostStyle, SimulationPostStyleField, SimulationPostStyleValue};
use crate::{BessemerError, FailureClass, Result};

pub(super) const WORD_OFFSETS: [u32; 16] = [
    8, 12, 16, 20, 0xa8, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xd4, 0xe8, 0xfc, 0xac, 0xb0,
];
pub(super) const COLOR_OFFSETS: [u32; 12] = [
    0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78, 0x88, 0x98, 0xd8, 0xec, 0x100,
];

/// Immutable explicit inputs; equality compares optional raw words, while missing runtime inputs remain missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegendComparisonState {
    pub(super) words: [Option<u32>; 16],
    pub(super) channels: [Option<u32>; 48],
}

impl SimulationLegendComparisonState {
    /// Capture explicit named runtime inputs without defaults; e.g. `Self::from_fields(style.fields())?`.
    pub fn from_fields(fields: &[SimulationPostStyleField]) -> Result<Self> {
        Self::from_word_reader(|path| comparison_input(fields, path))
    }

    pub(super) fn from_word_reader<F>(mut read_word: F) -> Result<Self>
    where
        F: FnMut(&str) -> Result<Option<u32>>,
    {
        let mut state = Self {
            words: [None; 16],
            channels: [None; 48],
        };
        for (slot, offset) in state.words.iter_mut().zip(WORD_OFFSETS) {
            *slot = read_word(&format!("legend.word_{offset:x}"))?;
        }
        for (index, slot) in state.channels.iter_mut().enumerate() {
            let offset = COLOR_OFFSETS[index / 4];
            *slot = read_word(&format!("legend.color_{offset:x}.component_{}", index % 4))?;
        }
        state.require_whole_colors()?;
        Ok(state)
    }

    fn require_whole_colors(&self) -> Result<()> {
        for (index, channels) in self.channels.as_chunks::<4>().0.iter().enumerate() {
            let present = channels.iter().filter(|channel| channel.is_some()).count();
            if present != 0 && present != 4 {
                return Err(comparison_failure(
                    &format!("legend.color_{:x}", COLOR_OFFSETS[index]),
                    FailureClass::Invalid,
                    &format!(
                        "{present} components; expected absent or complete four-component loaded RGBA"
                    ),
                ));
            }
        }
        Ok(())
    }
}

impl SimulationPostStyle {
    /// Project stored inputs while leaving absent initialized fields unknown; e.g. `style.legend_comparison_state()?`.
    pub fn legend_comparison_state(&self) -> Result<SimulationLegendComparisonState> {
        SimulationLegendComparisonState::from_fields(self.fields())
    }
}

fn comparison_input(fields: &[SimulationPostStyleField], path: &str) -> Result<Option<u32>> {
    let mut matching = fields.iter().filter(|field| field.path.as_ref() == path);
    let Some(field) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() {
        return Err(comparison_failure(
            path,
            FailureClass::Invalid,
            "duplicate fields; expected at most one explicit raw32 input",
        ));
    }
    comparison_word_value(&field.value, path).map(Some)
}

fn comparison_word_value(value: &SimulationPostStyleValue, path: &str) -> Result<u32> {
    match value {
        SimulationPostStyleValue::Signed(word) => Ok(*word as u32),
        SimulationPostStyleValue::RawWordBits(bits)
        | SimulationPostStyleValue::Float32Bits(bits) => Ok(*bits),
        value => Err(comparison_failure(
            path,
            FailureClass::Invalid,
            &format!("{value:?}; expected explicit raw32 input"),
        )),
    }
}

pub(super) fn comparison_failure(path: &str, class: FailureClass, cause: &str) -> BessemerError {
    BessemerError::new("legend comparison", path, class, cause)
}
