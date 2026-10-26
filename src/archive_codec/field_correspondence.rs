#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FieldCorrespondence {
    Derived { before: usize, after: usize },
    Deleted { before: usize },
    Created { after: usize },
}

pub(crate) fn field_correspondences<'a, T: PartialEq>(
    source: &'a [T],
    result: &'a [T],
    path: impl Fn(&'a T) -> &'a str,
) -> (Vec<FieldCorrespondence>, usize) {
    let before = super::field_occurrences::index_field_occurrences(source, &path);
    let after = super::field_occurrences::index_field_occurrences(result, &path);
    let mut changes: Vec<_> = before
        .iter()
        .filter_map(|(path, (index, field))| match after.get(path) {
            Some((_, target)) if field == target => None,
            Some((target, _)) => Some(FieldCorrespondence::Derived {
                before: *index,
                after: *target,
            }),
            None => Some(FieldCorrespondence::Deleted { before: *index }),
        })
        .collect();
    let unchanged = before.len() - changes.len();
    changes.extend(
        after
            .iter()
            .filter(|(path, _)| !before.contains_key(*path))
            .map(|(_, (index, _))| FieldCorrespondence::Created { after: *index }),
    );
    (changes, unchanged)
}

#[cfg(test)]
mod tests {
    use super::{FieldCorrespondence, field_correspondences};

    #[test]
    fn changed_created_deleted_and_repeated_fields_have_total_order() {
        let source = [("z", 1), ("a", 2), ("z", 3), ("deleted", 4)];
        let result = [("z", 1), ("created", 5), ("a", 6)];
        let (changes, unchanged) = field_correspondences(&source, &result, |field| field.0);
        assert_eq!(unchanged, 1);
        assert_eq!(
            changes,
            vec![
                FieldCorrespondence::Derived {
                    before: 1,
                    after: 2
                },
                FieldCorrespondence::Deleted { before: 3 },
                FieldCorrespondence::Deleted { before: 2 },
                FieldCorrespondence::Created { after: 1 }
            ]
        );
        assert_eq!(
            field_correspondences(&result, &result, |field| field.0),
            (Vec::new(), 3)
        );
        assert_eq!(
            field_correspondences::<(&str, i32)>(&[], &[], |field| field.0),
            (Vec::new(), 0)
        );
    }
}
