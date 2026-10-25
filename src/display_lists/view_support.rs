use super::DisplayViewLimits;
use crate::{BessemerError, FailureClass, Result};

pub(super) struct ViewBudget {
    pub limits: DisplayViewLimits,
    nested: usize,
}
impl ViewBudget {
    pub fn new(limits: DisplayViewLimits) -> Self {
        Self { limits, nested: 0 }
    }
    pub fn nested(&mut self, count: usize, label: &str) -> Result<()> {
        let total = self.nested.checked_add(count).ok_or_else(|| {
            invalid(format!(
                "{label} accumulated count {} + {count}; expected usize",
                self.nested
            ))
        })?;
        require_size(total, self.limits.maximum_nested_entries, label)?;
        self.nested = total;
        Ok(())
    }
}

pub(super) fn require_size(actual: usize, maximum: usize, label: &str) -> Result<()> {
    if actual <= maximum {
        return Ok(());
    }
    Err(invalid(format!(
        "{label} count {actual}; expected at most {maximum}"
    )))
}

pub(super) fn require_presence(actual: bool, expected: bool, label: &str) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "{label} presence {actual}; expected {expected} for this version and parent fields"
    )))
}

pub(super) fn invalid(cause: String) -> BessemerError {
    BessemerError {
        operation: "display-list views",
        entity: "view collection".into(),
        class: FailureClass::Invalid,
        cause,
    }
}
