use crate::{BessemerError, FailureClass};
pub(super) fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "display-list archive",
        entity: "Contents/DisplayLists".into(),
        class: FailureClass::Invalid,
        cause: cause.into(),
    }
}
pub(super) fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "display-list archive",
        entity: "Contents/DisplayLists".into(),
        class: FailureClass::Unsupported,
        cause: cause.into(),
    }
}
