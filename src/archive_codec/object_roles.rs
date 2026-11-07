use super::{ArchiveClass, ObjectRecord};
use crate::{BessemerError, Result};

#[derive(Clone, Copy)]
pub(crate) enum ArchiveObjectRole<'a> {
    Declared(&'a [&'a str]),
    Any,
}

pub(crate) fn require_object_role(
    record: &ObjectRecord,
    role: ArchiveObjectRole<'_>,
    label: &str,
    invalid: impl Fn(String) -> BessemerError,
) -> Result<()> {
    let (ArchiveObjectRole::Declared(expected), Some(class)) = (role, role_runtime_class(record))
    else {
        return Ok(());
    };
    // #31: map validity alone does not prove that an alias satisfies its declared pointer role.
    if expected.contains(&class.name.as_str()) {
        return Ok(());
    }
    Err(invalid(format!(
        "object role {label:?} has runtime class {:?}; expected one of {expected:?}",
        class.name
    )))
}

fn role_runtime_class(record: &ObjectRecord) -> Option<&ArchiveClass> {
    match record {
        ObjectRecord::Null => None,
        ObjectRecord::New { class, .. } | ObjectRecord::Reference { class, .. } => Some(class),
    }
}
