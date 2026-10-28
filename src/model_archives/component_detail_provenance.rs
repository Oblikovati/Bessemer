use super::{ModelField, ModelFieldValue, object_values, part_archive::VerifiedPartRoot};
use crate::archive_codec::{ArchiveMapIndex, ObjectRecord};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn require_internal_unit(
    fields: &[ModelField],
    component: &ObjectRecord,
    root: Option<VerifiedPartRoot>,
) -> Result<()> {
    if matches!(component, ObjectRecord::Null) {
        return Ok(());
    }
    let root = require_root(root)?;
    require_unit_class(component)?;
    require_no_external_model(fields, component)?;
    require_owner_identity(fields, component, root)
}

fn require_owner_identity(
    fields: &[ModelField],
    component: &ObjectRecord,
    root: ArchiveMapIndex,
) -> Result<()> {
    let owner = owner_identity(fields, component)?;
    if owner != root {
        return Err(unsupported(format!(
            "unit owner {}, selected part {}; expected identical mapped owner",
            owner.value(),
            root.value()
        )));
    }
    Ok(())
}

fn require_root(root: Option<VerifiedPartRoot>) -> Result<ArchiveMapIndex> {
    root.map(VerifiedPartRoot::index).ok_or_else(|| {
        unsupported("current-part proof None; expected completed selected part archive".into())
    })
}

fn require_unit_class(component: &ObjectRecord) -> Result<()> {
    match component {
        ObjectRecord::New { class, .. } | ObjectRecord::Reference { class, .. }
            if class.name == "moUnitComponent_c" =>
        {
            Ok(())
        }
        actual => Err(unsupported(format!(
            "component {actual:?}; expected exact moUnitComponent_c"
        ))),
    }
}

fn require_no_external_model(fields: &[ModelField], component: &ObjectRecord) -> Result<()> {
    match object_values::stored_object_value(fields, component, "has_external_model")? {
        ModelFieldValue::U8(0) => Ok(()),
        actual => Err(unsupported(format!(
            "unit external-model field {actual:?}; expected stored U8(0)"
        ))),
    }
}

fn owner_identity(fields: &[ModelField], component: &ObjectRecord) -> Result<ArchiveMapIndex> {
    match object_values::stored_object_tag(fields, component, "owner")? {
        ObjectRecord::New { index, class } | ObjectRecord::Reference { index, class }
            if class.name == "moPart_c" =>
        {
            Ok(*index)
        }
        actual => Err(unsupported(format!(
            "unit owner {actual:?}; expected mapped moPart_c owner"
        ))),
    }
}

fn unsupported(cause: String) -> BessemerError {
    BessemerError::new(
        "model archive",
        "component-detail reference",
        FailureClass::Unsupported,
        cause,
    )
}
