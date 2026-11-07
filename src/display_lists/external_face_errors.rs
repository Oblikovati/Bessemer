use crate::{BessemerError, FailureClass};

pub(super) fn link_error(class: FailureClass, entity: &str, cause: String) -> BessemerError {
    BessemerError {
        operation: "check external face links",
        entity: entity.into(),
        class,
        cause,
    }
}

pub(super) fn missing_component(path: &str, identity: &str) -> BessemerError {
    link_error(
        FailureClass::Missing,
        path,
        format!("component {identity:?}; expected a matching directory declaration"),
    )
}

pub(super) fn component_target_error(path: &str, identity: &str, ambiguous: bool) -> BessemerError {
    let (class, cause) = if ambiguous {
        (
            FailureClass::Unsupported,
            "names multiple cache targets; expected one unambiguous block/index/hash target",
        )
    } else {
        (
            FailureClass::Missing,
            "has an empty block name; expected a named block for its requested facet",
        )
    };
    link_error(class, path, format!("component {identity:?} {cause}"))
}
