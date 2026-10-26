use super::object_values::{
    stored_object_count, stored_object_integer, stored_object_tag, stored_object_value,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::{ArchiveMapIndex, ObjectRecord};
use crate::{BessemerError, FailureClass, Result};
use std::collections::BTreeSet;

pub(super) fn validate(fields: &[ModelField], manager: &ObjectRecord) -> Result<()> {
    let selected = identity(stored_object_tag(fields, manager, "configuration_358")?)?;
    let alias = identity(stored_object_tag(fields, manager, "configuration_360")?)?;
    if selected != alias {
        return Err(unsupported(format!(
            "selected aliases {selected:?} and {alias:?}"
        )));
    }
    let roots = roots(fields, manager)?;
    let complete = traverse(fields, roots)?;
    if !complete.contains(&selected) {
        return Err(unsupported(format!(
            "selected configuration {selected:?} is outside the root graph"
        )));
    }
    Ok(())
}

fn roots<'a>(fields: &'a [ModelField], manager: &ObjectRecord) -> Result<Vec<&'a ObjectRecord>> {
    let count = stored_object_count(fields, manager, "configurations.count")?;
    let mut roots = Vec::new();
    let mut unique = BTreeSet::new();
    for index in 0..count {
        let node = stored_object_tag(fields, manager, &format!("configurations[{index}]"))?;
        if !unique.insert(identity(node)?) {
            return Err(unsupported(format!(
                "duplicate configuration root {node:?}"
            )));
        }
        roots.push(node);
    }
    if roots.is_empty() {
        return Err(unsupported("configuration root count 0".into()));
    }
    Ok(roots)
}

fn traverse<'a>(
    fields: &'a [ModelField],
    roots: Vec<&'a ObjectRecord>,
) -> Result<BTreeSet<ArchiveMapIndex>> {
    let mut active = BTreeSet::new();
    let mut complete = BTreeSet::new();
    let mut pending: Vec<_> = roots.into_iter().rev().map(|node| (node, false)).collect();
    while let Some((node, exit)) = pending.pop() {
        let index = identity(node)?;
        if exit {
            active.remove(&index);
            complete.insert(index);
            continue;
        }
        if active.contains(&index) {
            return Err(unsupported(format!("configuration cycle at {index:?}")));
        }
        if complete.contains(&index) {
            continue;
        }
        active.insert(index);
        pending.push((node, true));
        schedule_children(fields, node, &mut pending)?;
    }
    Ok(complete)
}

fn schedule_children<'a>(
    fields: &'a [ModelField],
    node: &ObjectRecord,
    pending: &mut Vec<(&'a ObjectRecord, bool)>,
) -> Result<()> {
    let parent = stored_object_tag(fields, node, "parent_698")?;
    let conversion = stored_object_integer(fields, node, "raw_77c")?;
    let feature_text = stored_object_value(fields, node, "feature.has_unknown_1e0")?;
    if !matches!(parent, ObjectRecord::Null)
        || conversion != 0
        || *feature_text != ModelFieldValue::U8(0)
    {
        return Err(unsupported(format!(
            "configuration {:?} parent {parent:?}, conversion {conversion}, feature text {feature_text:?}",
            identity(node)?
        )));
    }
    let count = stored_object_count(fields, node, "children_6a0.count")?;
    for index in (0..count).rev() {
        pending.push((
            stored_object_tag(fields, node, &format!("children_6a0[{index}]"))?,
            false,
        ));
    }
    Ok(())
}

fn identity(record: &ObjectRecord) -> Result<ArchiveMapIndex> {
    match record {
        ObjectRecord::New { index, class } | ObjectRecord::Reference { index, class }
            if class.name == "moPartConfiguration_c" =>
        {
            Ok(*index)
        }
        actual => Err(unsupported(format!("configuration tag {actual:?}"))),
    }
}

fn unsupported(cause: String) -> BessemerError {
    BessemerError::new(
        "configuration context",
        "Contents/CMgr",
        FailureClass::Unsupported,
        format!(
            "{cause}; expected matching selected aliases reachable from distinct nonnull part roots in an acyclic graph, with null parents, zero conversion and absent feature text"
        ),
    )
}
