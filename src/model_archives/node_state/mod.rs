//! Stored node projections over already validated complete archive fields (#178).
mod accessors;
mod archive_ops;
mod projection;
#[cfg(test)]
#[path = "../../../tests/model_node_state/projection_unit.rs"]
mod projection_tests;

use super::{ModelFieldHandle, ModelFieldValue};
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;

/// One documented stored bit, independent of native setter cascades or effective predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelNodeStoredFlag {
    /// Stored needs-update bit; native needsUpdate can also consult virtual state.
    NeedsUpdate,
    /// Stored update-error bit.
    UpdateError,
    /// Stored update-warning bit.
    UpdateWarning,
    /// Stored SAT-not-solved bit, retaining the native name.
    SatNotSolved,
    /// Stored update-locked bit.
    UpdateLocked,
    /// Stored update-pending bit.
    UpdatePending,
    /// Stored update-security bit, retaining the native name.
    UpdateSecurity,
    /// Stored ID-assignment request; reading does not assign an ID.
    NeedsIdToBeSet,
    /// Parameter-only stored read-only bit; no owner pointer is allocated.
    ReadOnly,
    /// Parameter-only stored shared bit, independent of runtime reference count.
    SharedParameter,
    /// Parameter-only stored global-variable bit, without equation evaluation.
    GlobalVariableParameter,
    /// Parameter-only shared-thickness bit in the optional second word.
    SharedThickness,
}

/// Owned stored node words and revision-bound source fields from a complete part archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelNodeStoredState {
    update_state: i32,
    flags_28: u32,
    flags_2c: Option<u32>,
    parameter_class: Option<&'static str>,
    source_fields: [ModelFieldHandle; 2],
    second_field: Option<ModelFieldHandle>,
    path: Arc<str>,
}

#[derive(Clone, Copy)]
enum NodeWord {
    Update,
    First,
    Second,
}

impl ModelNodeStoredFlag {
    fn specification(self) -> (NodeWord, u32, bool) {
        use ModelNodeStoredFlag::*;
        match self {
            NeedsUpdate => (NodeWord::Update, 1, false),
            UpdateError => (NodeWord::Update, 2, false),
            UpdateWarning => (NodeWord::Update, 4, false),
            SatNotSolved => (NodeWord::Update, 0x100, false),
            UpdateLocked => (NodeWord::Update, 0x4000, false),
            UpdatePending => (NodeWord::Update, 0x8000, false),
            UpdateSecurity => (NodeWord::Update, 0x20000, false),
            NeedsIdToBeSet => (NodeWord::First, 0x40000000, false),
            ReadOnly => (NodeWord::First, 0x10000000, true),
            SharedParameter => (NodeWord::First, 0x20000000, true),
            GlobalVariableParameter => (NodeWord::First, 0x80000000, true),
            SharedThickness => (NodeWord::Second, 1, true),
        }
    }
}

impl ModelNodeStoredState {
    fn selected_word(&self, flag: ModelNodeStoredFlag) -> Result<(u32, &ModelFieldHandle, u32)> {
        let (word, mask, parameter_only) = flag.specification();
        if parameter_only && self.parameter_class.is_none() {
            return Err(self.flag_error(
                flag,
                FailureClass::Unsupported,
                "generic node; expected a verified parameter-class definition",
            ));
        }
        match word {
            NodeWord::Update => Ok((self.update_state as u32, &self.source_fields[0], mask)),
            NodeWord::First => Ok((self.flags_28, &self.source_fields[1], mask)),
            NodeWord::Second => self
                .second_word(flag)
                .map(|(word, handle)| (word, handle, mask)),
        }
    }

    fn second_word(&self, flag: ModelNodeStoredFlag) -> Result<(u32, &ModelFieldHandle)> {
        self.flags_2c
            .zip(self.second_field.as_ref())
            .ok_or_else(|| {
                self.flag_error(
                    flag,
                    FailureClass::Missing,
                    "second word absent; expected a serialized second flags word",
                )
            })
    }

    fn replacement(
        &self,
        flag: ModelNodeStoredFlag,
        enabled: bool,
    ) -> Result<(&ModelFieldHandle, ModelFieldValue)> {
        let (before, handle, mask) = self.selected_word(flag)?;
        let after = if enabled {
            before | mask
        } else {
            before & !mask
        };
        let value = if handle == &self.source_fields[0] {
            ModelFieldValue::I32(after as i32)
        } else {
            ModelFieldValue::U32(after)
        };
        Ok((handle, value))
    }

    fn flag_error(
        &self,
        flag: ModelNodeStoredFlag,
        class: FailureClass,
        cause: &str,
    ) -> BessemerError {
        BessemerError::new(
            "model node stored flag",
            self.path.as_ref(),
            class,
            format!(
                "flag {flag:?}, parameter class {:?}: {cause}",
                self.parameter_class
            ),
        )
    }
}
