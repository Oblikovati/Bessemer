use super::{SimulationPostInfo, SimulationPostInfoValue};
use crate::{Result, simulation::invalid};

/// Raw header words and bounded accessor meanings, without active-slot inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPostInfoHeaderView {
    /// All seven stored words, including unresolved header members.
    pub raw_words: [u32; 7],
    /// Original signed ContourType code, without inferred visual meaning.
    pub contour_type: i32,
    /// Exact forward preference conversion, which is not invertible.
    pub preference_integer: i32,
    /// Original section-count word; all six section slots remain physically present.
    pub section_active_count: i32,
    /// Original iso-count word; all six iso slots remain physically present.
    pub iso_active_count: i32,
}

impl SimulationPostInfo {
    /// Inspect stored header roles without applying defaults; e.g. `info.header_view()?.preference_integer`.
    pub fn header_view(&self) -> Result<SimulationPostInfoHeaderView> {
        let words = [
            header_word(self, 0)?,
            header_word(self, 1)?,
            header_word(self, 2)?,
            header_word(self, 3)?,
            header_word(self, 4)?,
            header_word(self, 5)?,
            header_word(self, 6)?,
        ];
        Ok(SimulationPostInfoHeaderView {
            raw_words: words,
            contour_type: words[2] as i32,
            preference_integer: contour_preference(words[2] as i32),
            section_active_count: words[5] as i32,
            iso_active_count: words[6] as i32,
        })
    }
}

fn header_word(info: &SimulationPostInfo, ordinal: usize) -> Result<u32> {
    match info.fields().get(ordinal).map(|field| &field.value) {
        Some(SimulationPostInfoValue::RawWordBits(bits)) => Ok(*bits),
        Some(SimulationPostInfoValue::Signed(word)) => Ok(*word as u32),
        value => Err(invalid(
            "PostInfo header view",
            format!("field {ordinal}: {value:?}; expected raw32 or signed32 header word"),
        )),
    }
}

fn contour_preference(code: i32) -> i32 {
    match code {
        0 => 0,
        1 => 1,
        4 => 2,
        5 | 6 => 3,
        _ => 6,
    }
}
