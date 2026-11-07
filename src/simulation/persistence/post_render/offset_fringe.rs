//! Named offset and fringe decisions keep modeled runtime changes separate from persisted bytes (#166).
use super::super::{SimulationPostDialog, load_snapshot::BinaryLoadSnapshot};
use super::views::render_word;
use crate::{BessemerError, FailureClass, Result};

/// Explicit NeedOffset inputs, including genuinely absent older runtime word48.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationOffsetRequestInputs {
    /// Original signed style1B8 offset-request mode.
    pub style_1b8: i32,
    /// Explicit runtime word48, if available.
    pub word_48: Option<i32>,
}

impl SimulationOffsetRequestInputs {
    /// Evaluate the decisive mode before requiring word48; e.g. `inputs.needs_offset()?`.
    pub fn needs_offset(self) -> Result<bool> {
        if matches!(self.style_1b8, 1 | 2) {
            return Ok(true);
        }
        self.word_48.map(|word| word == 1).ok_or_else(|| {
            BessemerError::new(
                "post offset request",
                "moving_objects.word_48",
                FailureClass::Missing,
                format!(
                    "mode {}, word48 None; expected explicit runtime word48 unless mode is 1 or 2",
                    self.style_1b8
                ),
            )
        })
    }
}

/// One modeled GetFringeOption call, without changing stored dialog state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationFringeOptionView {
    /// Original raw dialog14 control.
    pub stored_word_14: i32,
    /// Original raw dialog48 control.
    pub stored_word_48: i32,
    /// Word48 after the modeled runtime call, distinct from the original wire value.
    pub runtime_word_48_after: i32,
    /// Native numeric return code; UI labels remain unproven.
    pub fringe_option_result: i32,
}

impl SimulationFringeOptionView {
    /// Model the getter and its local side effect while retaining both inputs; e.g. `Self::from_stored(1, 1)`.
    pub fn from_stored(word_14: i32, word_48: i32) -> Self {
        let (after, result) = if word_14 == 0 {
            (word_48, zero_fringe(word_48))
        } else {
            nonzero_fringe(word_48)
        };
        Self {
            stored_word_14: word_14,
            stored_word_48: word_48,
            runtime_word_48_after: after,
            fringe_option_result: result,
        }
    }
}

fn zero_fringe(word_48: i32) -> i32 {
    match word_48 {
        0 => 0,
        1 => 1,
        4 => 2,
        _ => 3,
    }
}

fn nonzero_fringe(word_48: i32) -> (i32, i32) {
    if word_48 as u32 <= 1 || word_48 == 5 {
        return (6, 1);
    }
    (word_48, i32::from(word_48 != 4))
}

impl SimulationPostDialog {
    /// Preserve optional stored runtime state for the named offset decision; e.g. `dialog.offset_request_inputs()?.needs_offset()?`.
    pub fn offset_request_inputs(&self) -> Result<SimulationOffsetRequestInputs> {
        Ok(SimulationOffsetRequestInputs {
            style_1b8: render_word(self, "style.word_1b8")?,
            word_48: optional_word48(self)?,
        })
    }

    /// Model the fringe call without editing the file or assuming older runtime defaults; e.g. `dialog.fringe_option_view()?`.
    pub fn fringe_option_view(&self) -> Result<SimulationFringeOptionView> {
        Ok(SimulationFringeOptionView::from_stored(
            render_word(self, "fourteen_words0c_28_30_3c_518_51c[2]")?,
            render_word(self, "moving_objects.word_48")?,
        ))
    }
}

fn optional_word48<T: BinaryLoadSnapshot>(snapshot: &T) -> Result<Option<i32>> {
    match render_word(snapshot, "moving_objects.word_48") {
        Ok(word) => Ok(Some(word)),
        Err(error) if error.class == FailureClass::Missing => Ok(None),
        Err(error) => Err(error),
    }
}
