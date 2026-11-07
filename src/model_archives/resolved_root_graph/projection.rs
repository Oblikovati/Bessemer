use super::*;
use crate::model_archives::{ModelFieldHandle, ModelFieldValue, ModelObjectId, ModelObjectRecord};
use crate::{BessemerError, FailureClass};
use std::collections::BTreeMap;

pub(super) fn invalid_root(actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "resolved root graph",
        "root.features",
        FailureClass::Invalid,
        format!(
            "{actual:?}; expected every canonical physical root and a unique loaded object definition or inherited identity"
        ),
    )
}

pub(super) fn project(source: &PartResolvedFeaturesArchive) -> Result<ModelResolvedRootGraph> {
    let definitions = local_definitions(source)?;
    let roots = collect_roots(source, &definitions)?;
    let copies = roots
        .iter()
        .map(|root| ModelResolvedRootCopy {
            parents: [root.field.clone()],
            result: root.field.clone(),
        })
        .collect::<Vec<_>>();
    Ok(ModelResolvedRootGraph {
        source: source.clone(),
        roots: roots.into(),
        copies: copies.into(),
    })
}

fn local_definitions(
    source: &PartResolvedFeaturesArchive,
) -> Result<BTreeMap<ModelObjectId, ModelFieldHandle>> {
    let mut definitions = BTreeMap::new();
    for (ordinal, field) in source.fields().iter().enumerate() {
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            continue;
        };
        let ModelObjectRecord::New { index, .. } = tag.record else {
            continue;
        };
        if definitions
            .insert(index, source.field_handle(ordinal)?)
            .is_some()
        {
            return Err(invalid_root((
                index,
                "duplicate physical object definition",
            )));
        }
    }
    Ok(definitions)
}

fn collect_roots(
    source: &PartResolvedFeaturesArchive,
    definitions: &BTreeMap<ModelObjectId, ModelFieldHandle>,
) -> Result<Vec<ModelResolvedRootOccurrence>> {
    let mut roots = Vec::with_capacity(source.root_count());
    let mut path = "root.features[0]".to_owned();
    for (index, field) in source.fields().iter().enumerate() {
        if field.path.as_ref() != path {
            continue;
        }
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            return Err(invalid_root(field));
        };
        let root = root_occurrence(source, index, roots.len(), tag, definitions)?;
        roots.push(root);
        path = format!("root.features[{}]", roots.len());
    }
    if roots.len() != source.root_count() {
        return Err(invalid_root((roots.len(), source.root_count())));
    }
    Ok(roots)
}

fn root_occurrence(
    source: &PartResolvedFeaturesArchive,
    index: usize,
    ordinal: usize,
    tag: &crate::model_archives::ModelObjectTag,
    definitions: &BTreeMap<ModelObjectId, ModelFieldHandle>,
) -> Result<ModelResolvedRootOccurrence> {
    Ok(ModelResolvedRootOccurrence {
        ordinal,
        field: source.field_handle(index)?,
        tag: tag.clone(),
        target: loaded_target(source.object_slots().0, &tag.record, definitions)?,
    })
}

fn loaded_target(
    inherited: usize,
    record: &ModelObjectRecord,
    definitions: &BTreeMap<ModelObjectId, ModelFieldHandle>,
) -> Result<ModelResolvedRootTarget> {
    let index = match record {
        ModelObjectRecord::Null => return Ok(ModelResolvedRootTarget::Null),
        ModelObjectRecord::New { index, .. } | ModelObjectRecord::Reference { index, .. } => index,
    };
    if (index.value() as usize) < inherited {
        return Ok(ModelResolvedRootTarget::Inherited(*index));
    }
    definitions
        .get(index)
        .cloned()
        .map(|definition| ModelResolvedRootTarget::Local { definition })
        .ok_or_else(|| invalid_root((index, "missing local physical definition")))
}
