use super::admission::{reference_error, reference_scope, require_reference_owner};
use super::{ModelIntersectionReferenceStoredState, ModelIntersectionReferenceStoredWords};
use crate::Result;
use crate::model_archives::{ModelField, ModelFieldHandle, ModelFieldValue};
use std::sync::Arc;

pub(super) fn project_intersection_reference(
    fields: &[ModelField],
    leading: &ModelFieldHandle,
) -> Result<ModelIntersectionReferenceStoredState> {
    let Some([state, ordinal, span, backward]) =
        fields.get(leading.index..).and_then(|tail| tail.get(..4))
    else {
        return Err(reference_error("intersection reference", leading.index));
    };
    let scope = reference_scope(&state.path)?;
    require_reference_owner(&fields[..leading.index], &scope)?;
    let words = reference_words([state, ordinal, span, backward], scope.group)?;
    let source_fields = [0, 1, 2, 3].map(|relative| ModelFieldHandle {
        revision: leading.revision.clone(),
        index: leading.index + relative,
    });
    Ok(ModelIntersectionReferenceStoredState {
        words,
        source_fields,
        path: Arc::from(scope.group),
    })
}

fn reference_words(
    fields: [&ModelField; 4],
    prefix: &str,
) -> Result<ModelIntersectionReferenceStoredWords> {
    for (field, suffix) in fields
        .iter()
        .zip([".raw_00", ".raw_20", ".raw_24", ".raw_28"])
    {
        if field.path.strip_prefix(prefix) != Some(suffix) {
            return Err(reference_error(prefix, &field.path));
        }
    }
    let ModelFieldValue::U16(raw_state) = fields[0].value else {
        return Err(reference_error(prefix, &fields[0].value));
    };
    Ok(ModelIntersectionReferenceStoredWords {
        raw_state,
        raw_word_20: reference_i32(fields[1])?,
        span_count_state: reference_i32(fields[2])?,
        backward_offset_state: reference_i32(fields[3])?,
    })
}

fn reference_i32(field: &ModelField) -> Result<i32> {
    match field.value {
        ModelFieldValue::I32(value) => Ok(value),
        ref other => Err(reference_error(&field.path, other)),
    }
}
