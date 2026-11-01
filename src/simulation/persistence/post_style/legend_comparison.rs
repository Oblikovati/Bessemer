//! Conditional CDspLegend::DiffOGLRender control flow, specification 14.489/C-871 (#166).
use super::super::rgba::SimulationRgbaColor;
use super::SimulationLegendComparisonState;
use super::legend_state::{COLOR_OFFSETS, WORD_OFFSETS, comparison_failure};
use crate::{FailureClass, Result};

impl SimulationLegendComparisonState {
    /// Compare only the named native dependencies under explicit FP assumptions; e.g. `left.different_masked_exceptions_daz_off(&right)?`.
    pub fn different_masked_exceptions_daz_off(&self, other: &Self) -> Result<bool> {
        if self.words_differ(other, &[0, 1, 2, 3])? {
            return Ok(true);
        }
        let count = self.palette_count()?;
        for index in 0..count {
            if self.color_differs(other, index)? {
                return Ok(true);
            }
        }
        self.position_differs(other)
    }

    fn position_differs(&self, other: &Self) -> Result<bool> {
        if self.words_differ(other, &[4])? {
            return Ok(true);
        }
        for index in [14, 15] {
            if scalar_differs(self.word(index)?, other.word(index)?) {
                return Ok(true);
            }
        }
        if self.words_differ(other, &[5, 6, 7])? {
            return Ok(true);
        }
        self.suffix_differs(other)
    }

    fn suffix_differs(&self, other: &Self) -> Result<bool> {
        let format_slots: &[usize] = if self.word(6)? == 0 { &[9, 10] } else { &[8] };
        if self.words_differ(other, format_slots)? {
            return Ok(true);
        }
        if self.words_differ(other, &[11])? {
            return Ok(true);
        }
        if self.word(11)? != 0 && self.color_differs(other, 9)? {
            return Ok(true);
        }
        if self.words_differ(other, &[12, 13])? {
            return Ok(true);
        }
        if self.word(12)? != 0 && self.color_differs(other, 10)? {
            return Ok(true);
        }
        Ok(self.word(13)? != 0 && self.color_differs(other, 11)?)
    }

    fn words_differ(&self, other: &Self, slots: &[usize]) -> Result<bool> {
        for slot in slots {
            if self.word(*slot)? != other.word(*slot)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn word(&self, slot: usize) -> Result<u32> {
        self.words.get(slot).copied().flatten().ok_or_else(|| {
            let path = WORD_OFFSETS.get(slot).map_or_else(
                || format!("legend slot {slot}"),
                |offset| format!("legend.word_{offset:x}"),
            );
            comparison_failure(
                &path,
                FailureClass::Missing,
                "None; expected an explicit loaded raw32 word",
            )
        })
    }

    fn palette_count(&self) -> Result<usize> {
        let count = self.word(2)? as i32;
        if !(0..=9).contains(&count) {
            return Err(comparison_failure(
                "legend.word_10",
                FailureClass::Invalid,
                &format!("{count}; expected active palette count in 0..=9"),
            ));
        }
        Ok(count as usize)
    }

    fn color_differs(&self, other: &Self, index: usize) -> Result<bool> {
        Ok(self
            .color(index)?
            .different_masked_exceptions_daz_off(other.color(index)?))
    }

    fn color(&self, index: usize) -> Result<SimulationRgbaColor> {
        let mut bits = [0; 4];
        for (component, value) in bits.iter_mut().enumerate() {
            *value = self
                .channels
                .get(index * 4 + component)
                .copied()
                .flatten()
                .ok_or_else(|| missing_color(index))?;
        }
        Ok(SimulationRgbaColor::from_bits(bits))
    }
}

fn missing_color(index: usize) -> crate::BessemerError {
    let path = COLOR_OFFSETS.get(index).map_or_else(
        || format!("legend color {index}"),
        |offset| format!("legend.color_{offset:x}"),
    );
    comparison_failure(
        &path,
        FailureClass::Missing,
        "absent channel; expected an explicit loaded 16-byte RGBA color",
    )
}

fn scalar_differs(left: u32, right: u32) -> bool {
    SimulationRgbaColor::from_bits([left, 0, 0, 0])
        .different_masked_exceptions_daz_off(SimulationRgbaColor::from_bits([right, 0, 0, 0]))
}
