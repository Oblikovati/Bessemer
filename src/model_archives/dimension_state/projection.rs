use super::ModelDimensionStoredState;
use crate::model_archives::{ModelField, ModelFieldHandle, ModelFieldValue};
use crate::{BessemerError, FailureClass, Result};

const DIMENSION_WORD_SUFFIXES: [&str; 4] = [
    ".am_object_id",
    ".is_design_table_dimension",
    ".mask",
    ".unknown_5c",
];

pub(super) fn project_dimension(
    fields: &[ModelField],
    handle: &ModelFieldHandle,
) -> Result<ModelDimensionStoredState> {
    let words = dimension_group(fields, handle.index)?;
    let prefix = words[2]
        .path
        .strip_suffix(".mask")
        .ok_or_else(|| dimension_error(handle.index, &words[2].path))?;
    let values = validate_dimension_group(&words, prefix, handle.index)?;
    let redundant = ModelFieldHandle {
        revision: handle.revision.clone(),
        index: handle.index + 1,
    };
    Ok(ModelDimensionStoredState {
        mask: values[2],
        redundant_id: values[3],
        source_fields: [handle.clone(), redundant],
        path: words[2].path.clone(),
    })
}

fn dimension_group(fields: &[ModelField], index: usize) -> Result<[&ModelField; 4]> {
    let start = index
        .checked_sub(2)
        .ok_or_else(|| dimension_error(index, "missing preceding dimension words"))?;
    let Some([object_id, table, mask, identifier]) =
        fields.get(start..).and_then(|tail| tail.get(..4))
    else {
        return Err(dimension_error(
            index,
            "incomplete physical dimension group",
        ));
    };
    Ok([object_id, table, mask, identifier])
}

fn validate_dimension_group(
    words: &[&ModelField; 4],
    prefix: &str,
    index: usize,
) -> Result<[i32; 4]> {
    if prefix.is_empty() {
        return Err(dimension_error(index, "empty dimension scope"));
    }
    let mut values = [0; 4];
    for (ordinal, (field, suffix)) in words.iter().zip(DIMENSION_WORD_SUFFIXES).enumerate() {
        let ModelFieldValue::I32(value) = field.value else {
            return Err(dimension_error(
                index,
                &format!("{} {:?}", field.path, field.value),
            ));
        };
        if field.path.strip_prefix(prefix) != Some(suffix) {
            return Err(dimension_error(index, &field.path));
        }
        values[ordinal] = value;
    }
    Ok(values)
}

fn dimension_error(index: usize, actual: &str) -> BessemerError {
    BessemerError::new(
        "model dimension stored state",
        actual,
        FailureClass::Invalid,
        format!(
            "field {index} at {actual:?}; expected adjacent same-scope I32 am_object_id, is_design_table_dimension, mask and unknown_5c dimension words"
        ),
    )
}
