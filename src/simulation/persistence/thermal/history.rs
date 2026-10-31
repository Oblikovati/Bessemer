use super::*;
use crate::archive_codec::index_field_occurrences;
use crate::model_archives::{ModelField, SimulationEmbeddedArchive};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

type ThermalHistories = (
    Arc<[SimulationThermalHistory]>,
    Arc<[SimulationThermalNestedHistory]>,
);

pub(super) fn compose(
    source: &SimulationThermalCollection,
    result: &SimulationThermalCollection,
    requested: usize,
    replacement: &SimulationThermalValue,
    changed: bool,
) -> Result<ThermalHistories> {
    let mut outer = Vec::new();
    let mut nested = Vec::new();
    for ordinal in 0..source.fields().len() {
        let derived = changed && ordinal == requested;
        outer.push(outer_relation(source, result, ordinal, derived)?);
        if let Some(context) = nested_context(source, result, ordinal, replacement, derived)? {
            nested.extend(context.relations()?);
        }
    }
    Ok((outer.into(), nested.into()))
}

fn outer_relation(
    source: &SimulationThermalCollection,
    result: &SimulationThermalCollection,
    ordinal: usize,
    derived: bool,
) -> Result<SimulationThermalHistory> {
    let mut parents = vec![SimulationThermalHistoryParent::Source(
        source.field_id(ordinal)?,
    )];
    if derived {
        parents.push(SimulationThermalHistoryParent::Replacement);
    }
    Ok(SimulationThermalHistory {
        parents: parents.into(),
        result: result.field_id(ordinal)?,
        relation: if derived {
            SimulationThermalRelation::Derived
        } else {
            SimulationThermalRelation::Copied
        },
    })
}

struct NestedContext<'a> {
    source: &'a SimulationThermalCollection,
    result: &'a SimulationThermalCollection,
    ordinal: usize,
    before: &'a SimulationEmbeddedArchive,
    after: &'a SimulationEmbeddedArchive,
    replacement: Option<&'a SimulationEmbeddedArchive>,
}

fn nested_context<'a>(
    source: &'a SimulationThermalCollection,
    result: &'a SimulationThermalCollection,
    ordinal: usize,
    replacement: &'a SimulationThermalValue,
    derived: bool,
) -> Result<Option<NestedContext<'a>>> {
    let Some(before) = embedded_field(source, ordinal)? else {
        return Ok(None);
    };
    let after = required_embedded_field(result, ordinal)?;
    let replacement = match replacement {
        SimulationThermalValue::Embedded(archive) if derived => Some(archive.as_ref()),
        _ => None,
    };
    Ok(Some(NestedContext {
        source,
        result,
        ordinal,
        before,
        after,
        replacement,
    }))
}

fn required_embedded_field(
    collection: &SimulationThermalCollection,
    ordinal: usize,
) -> Result<&SimulationEmbeddedArchive> {
    embedded_field(collection, ordinal)?.ok_or_else(|| {
        invalid(
            "thermal nested result",
            format!(
                "ordinal {ordinal} has no embedded value; expected the original archive field role"
            ),
        )
    })
}

fn embedded_field(
    collection: &SimulationThermalCollection,
    ordinal: usize,
) -> Result<Option<&SimulationEmbeddedArchive>> {
    let field = collection.fields().get(ordinal).ok_or_else(|| {
        invalid(
            "thermal nested ordinal",
            format!(
                "ordinal {ordinal}; expected < {} fields",
                collection.fields().len()
            ),
        )
    })?;
    match &field.value {
        SimulationThermalValue::Embedded(archive) => Ok(Some(archive)),
        _ => Ok(None),
    }
}

impl NestedContext<'_> {
    fn relations(&self) -> Result<Vec<SimulationThermalNestedHistory>> {
        let before = index_field_occurrences(self.before.fields(), |field| field.path.as_ref());
        let after = index_field_occurrences(self.after.fields(), |field| field.path.as_ref());
        let mut original: Vec<_> = before.iter().collect();
        original.sort_by_key(|(_, (index, _))| *index);
        let mut relations = Vec::new();
        for (key, (index, field)) in original {
            let target = after.get(key).map(|(index, _)| *index);
            relations.push(self.row(field, Some(*index), target)?);
        }
        let mut created: Vec<_> = after
            .iter()
            .filter(|(key, _)| !before.contains_key(*key))
            .collect();
        created.sort_by_key(|(_, (index, _))| *index);
        for (_, (index, field)) in created {
            relations.push(self.row(field, None, Some(*index))?);
        }
        Ok(relations)
    }

    fn row(
        &self,
        field: &ModelField,
        original: Option<usize>,
        target: Option<usize>,
    ) -> Result<SimulationThermalNestedHistory> {
        Ok(SimulationThermalNestedHistory {
            outer_parent: self.source.field_id(self.ordinal)?,
            outer_result: self.result.field_id(self.ordinal)?,
            path: field.path.clone(),
            parents: self.parents(original, target)?,
            result: target
                .map(|index| self.after.field_handle(index))
                .transpose()?,
            relation: nested_relation(original, target, self.replacement.is_some())?,
        })
    }

    fn parents(
        &self,
        original: Option<usize>,
        target: Option<usize>,
    ) -> Result<Arc<[SimulationThermalNestedParent]>> {
        let mut parents = Vec::new();
        if let Some(index) = original {
            parents.push(SimulationThermalNestedParent::Source(
                self.before.field_handle(index)?,
            ));
        }
        if let (Some(replacement), Some(index)) = (self.replacement, target) {
            parents.push(SimulationThermalNestedParent::Replacement(
                replacement.field_handle(index)?,
            ));
        }
        if parents.is_empty() {
            return Err(invalid(
                "thermal nested history",
                format!(
                    "source {original:?}, result {target:?}; expected at least one source/replacement parent"
                ),
            ));
        }
        Ok(parents.into())
    }
}

fn nested_relation(
    original: Option<usize>,
    target: Option<usize>,
    replaced: bool,
) -> Result<SimulationThermalRelation> {
    use SimulationThermalRelation::{Copied, Created, Deleted, Derived};
    match (original, target) {
        (Some(_), Some(_)) => Ok(if replaced { Derived } else { Copied }),
        (Some(_), None) => Ok(Deleted),
        (None, Some(_)) => Ok(Created),
        (None, None) => Err(invalid(
            "thermal nested history",
            "source None, result None; expected at least one field occurrence",
        )),
    }
}
