use std::collections::BTreeMap;

pub(crate) type IndexedFields<'a, T> = BTreeMap<(&'a str, usize), (usize, &'a T)>;

pub(crate) fn index_field_occurrences<'a, T>(
    fields: &'a [T],
    path: impl Fn(&'a T) -> &'a str,
) -> IndexedFields<'a, T> {
    let mut occurrences = BTreeMap::new();
    let mut indexed = BTreeMap::new();
    for (index, field) in fields.iter().enumerate() {
        let name = path(field);
        let occurrence = occurrences.entry(name).or_insert(0);
        indexed.insert((name, *occurrence), (index, field));
        *occurrence += 1;
    }
    indexed
}

#[cfg(test)]
mod tests {
    use super::index_field_occurrences;

    #[test]
    fn duplicate_paths_keep_every_physical_position() {
        let fields = [("z", 1), ("a", 2), ("z", 3)];
        let indexed = index_field_occurrences(&fields, |field| field.0);
        assert_eq!(indexed[&("z", 0)], (0, &fields[0]));
        assert_eq!(indexed[&("z", 1)], (2, &fields[2]));
        assert_eq!(indexed[&("a", 0)], (1, &fields[1]));
    }
}
