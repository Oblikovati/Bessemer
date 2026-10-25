use super::{
    DisplayArchive, DisplayField, DisplayFieldChange, DisplayFieldHandle, DisplayFieldRelation,
    archive_error::invalid,
};
use crate::Result;
use std::collections::BTreeMap;
type IndexedArchiveFields<'a> = BTreeMap<(&'a str, usize), (usize, &'a DisplayField)>;

impl DisplayArchive {
    /// Resolves a history field only in its owning revision, including equivalent clones.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::display_lists::DisplayArchive, edit: &bessemer::display_lists::DisplayArchiveEdit) -> bessemer::Result<()> {
    /// if let Some(change) = edit.changes().first() {
    ///     for parent in change.parents() { let field = source.resolve_field(parent)?; }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn resolve_field(&self, handle: &DisplayFieldHandle) -> Result<&DisplayField> {
        if self.revision != handle.revision {
            return Err(invalid(format!(
                "field position {}; expected a handle from this archive revision",
                handle.position
            )));
        }
        self.contents().fields.get(handle.position).ok_or_else(|| {
            invalid(format!(
                "field position {}; expected below {}",
                handle.position,
                self.contents().fields.len()
            ))
        })
    }

    pub(super) fn field_identity(&self, position: usize) -> DisplayFieldHandle {
        DisplayFieldHandle {
            revision: self.revision.clone(),
            position,
        }
    }
}

pub(super) fn edit_history(
    source: &DisplayArchive,
    result: &DisplayArchive,
) -> (Vec<DisplayFieldChange>, usize) {
    let before = indexed_fields(&source.contents().fields);
    let after = indexed_fields(&result.contents().fields);
    let mut changes = source_relations(source, result, &before, &after);
    let unchanged = before.len() - changes.len();
    append_created_fields(&mut changes, result, &before, &after);
    (changes, unchanged)
}

fn source_relations(
    source: &DisplayArchive,
    result: &DisplayArchive,
    before: &IndexedArchiveFields<'_>,
    after: &IndexedArchiveFields<'_>,
) -> Vec<DisplayFieldChange> {
    before
        .iter()
        .filter_map(|(path, (index, _))| {
            source_change(
                source,
                result,
                *index,
                after.get(path).map(|(index, _)| *index),
            )
        })
        .collect()
}

fn indexed_fields(fields: &[DisplayField]) -> IndexedArchiveFields<'_> {
    let mut occurrences = BTreeMap::new();
    let mut indexed = BTreeMap::new();
    for (index, field) in fields.iter().enumerate() {
        // A path alone must not collapse repeated physical fields into one history entry.
        let occurrence = occurrences.entry(field.path.as_ref()).or_insert(0);
        indexed.insert((field.path.as_ref(), *occurrence), (index, field));
        *occurrence += 1;
    }
    indexed
}

fn source_change(
    source: &DisplayArchive,
    result: &DisplayArchive,
    before: usize,
    after: Option<usize>,
) -> Option<DisplayFieldChange> {
    if after
        .is_some_and(|index| source.contents().fields[before] == result.contents().fields[index])
    {
        return None;
    }
    Some(changed_field(source, result, before, after))
}

fn append_created_fields(
    changes: &mut Vec<DisplayFieldChange>,
    result: &DisplayArchive,
    before: &IndexedArchiveFields<'_>,
    after: &IndexedArchiveFields<'_>,
) {
    changes.extend(
        after
            .iter()
            .filter(|(path, _)| !before.contains_key(*path))
            .map(|(_, (index, _))| created_field(result, *index)),
    );
}

fn changed_field(
    source: &DisplayArchive,
    result: &DisplayArchive,
    before: usize,
    after: Option<usize>,
) -> DisplayFieldChange {
    DisplayFieldChange {
        path: source.contents().fields[before].path.clone(),
        relation: if after.is_some() {
            DisplayFieldRelation::Derived
        } else {
            DisplayFieldRelation::Deleted
        },
        parents: vec![source.field_identity(before)],
        result: after.map(|index| result.field_identity(index)),
    }
}

fn created_field(result: &DisplayArchive, index: usize) -> DisplayFieldChange {
    DisplayFieldChange {
        path: result.contents().fields[index].path.clone(),
        relation: DisplayFieldRelation::Created,
        parents: Vec::new(),
        result: Some(result.field_identity(index)),
    }
}

impl DisplayFieldChange {
    /// Returns the parser-derived wire path; handles distinguish repeated occurrences.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::display_lists::DisplayFieldChange) {
    /// let path = change.path();
    /// # }
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns creation, deletion or derivation without inferring geometry merges or splits.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::display_lists::DisplayFieldChange) {
    /// let relation = change.relation();
    /// # }
    /// ```
    pub fn relation(&self) -> DisplayFieldRelation {
        self.relation
    }

    /// Borrows every source field parent; created fields have none.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::display_lists::DisplayFieldChange) {
    /// let parents = change.parents();
    /// # }
    /// ```
    pub fn parents(&self) -> &[DisplayFieldHandle] {
        &self.parents
    }

    /// Borrows the result field identity; deleted fields have none.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::display_lists::DisplayFieldChange) {
    /// let result = change.result();
    /// # }
    /// ```
    pub fn result(&self) -> Option<&DisplayFieldHandle> {
        self.result.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_lists::DisplayFieldValue;

    #[test]
    fn repeated_paths_keep_each_physical_field_in_occurrence_order() {
        let fields: Vec<_> = [11, 22, 33]
            .into_iter()
            .map(|value| DisplayField {
                path: "root.repeated".into(),
                value: DisplayFieldValue::I32(value),
            })
            .collect();
        let indexed = indexed_fields(&fields);
        assert_eq!(indexed.len(), 3);
        for (occurrence, expected) in fields.iter().enumerate() {
            assert_eq!(
                indexed[&("root.repeated", occurrence)],
                (occurrence, expected)
            );
        }
    }
}
