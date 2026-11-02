use super::ModelNodeStoredState;
use crate::archive_codec::ObjectRecord;
use crate::model_archives::{ModelField, ModelFieldHandle, ModelFieldValue};
use crate::{BessemerError, FailureClass, Result};

const PARAMETER_CLASSES: &[&str] = &[
    "moDensityParameter_c",
    "moLengthParameter_c",
    "moAngleParameter_c",
    "moIntegerParameter_c",
    "moParameter_c",
    "moScalerParameter_c",
    "moStringParameter_c",
];

pub(super) fn project_node(
    fields: &[ModelField],
    handle: &ModelFieldHandle,
) -> Result<ModelNodeStoredState> {
    let Some([state, flags, id, tail @ ..]) = fields.get(handle.index..) else {
        return Err(invalid_node(handle.index, "missing physical fields"));
    };
    let (prefix, update_state, flags_28) = node_words(state, flags, id, handle.index)?;
    let (flags_2c, second_field) = optional_second(tail, prefix, handle)?;
    Ok(ModelNodeStoredState {
        update_state,
        flags_28,
        flags_2c,
        parameter_class: parameter_owner(fields, handle.index, prefix),
        source_fields: [handle.clone(), sibling_handle(handle, 1)],
        second_field,
        path: state.path.clone(),
    })
}

fn node_words<'a>(
    state: &'a ModelField,
    flags: &ModelField,
    id: &ModelField,
    index: usize,
) -> Result<(&'a str, i32, u32)> {
    let prefix = state
        .path
        .strip_suffix(".unknown_0c")
        .ok_or_else(|| invalid_node(index, &state.path))?;
    let (
        ModelFieldValue::I32(update_state),
        ModelFieldValue::U32(flags_28),
        ModelFieldValue::I32(_),
    ) = (&state.value, &flags.value, &id.value)
    else {
        return Err(invalid_node(index, &state.path));
    };
    require_sibling(flags, prefix, ".flags_28", index)?;
    require_sibling(id, prefix, ".id", index)?;
    Ok((prefix, *update_state, *flags_28))
}

fn require_sibling(field: &ModelField, prefix: &str, suffix: &str, index: usize) -> Result<()> {
    if field.path.strip_prefix(prefix) == Some(suffix) {
        return Ok(());
    }
    Err(invalid_node(index, &field.path))
}

fn optional_second(
    tail: &[ModelField],
    prefix: &str,
    handle: &ModelFieldHandle,
) -> Result<(Option<u32>, Option<ModelFieldHandle>)> {
    let Some(field) = tail
        .first()
        .filter(|field| field.path.strip_prefix(prefix) == Some(".unknown_2c"))
    else {
        return Ok((None, None));
    };
    match field.value {
        ModelFieldValue::U32(word) => Ok((Some(word), Some(sibling_handle(handle, 3)))),
        _ => Err(invalid_node(handle.index, &field.path)),
    }
}

fn sibling_handle(handle: &ModelFieldHandle, delta: usize) -> ModelFieldHandle {
    ModelFieldHandle {
        revision: handle.revision.clone(),
        index: handle.index + delta,
    }
}

fn parameter_owner(fields: &[ModelField], index: usize, prefix: &str) -> Option<&'static str> {
    let body = prefix.strip_suffix(".node")?;
    let owner = fields[..index]
        .iter()
        .rev()
        .find(|field| field.path.as_ref() == body)?;
    let ModelFieldValue::ObjectTag(tag) = &owner.value else {
        return None;
    };
    let ObjectRecord::New { class, .. } = &tag.record else {
        return None;
    };
    PARAMETER_CLASSES
        .iter()
        .copied()
        .find(|name| *name == class.name)
}

fn invalid_node(index: usize, actual: &str) -> BessemerError {
    BessemerError::new(
        "model node stored state",
        actual,
        FailureClass::Invalid,
        format!(
            "field {index} at {actual:?}; expected adjacent I32 unknown_0c, U32 flags_28 and I32 id in one decoded node scope"
        ),
    )
}
