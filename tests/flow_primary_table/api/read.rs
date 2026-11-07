use super::*;

#[test]
fn every_complete_primary_table_matches_all_source_leaf_values_scopes_and_exact_byte_coverage() {
    for case in fixture::cases() {
        let table = open(&case.raw, options()).unwrap();
        assert_eq!(table.decoded_bytes(), case.raw, "{}", case.name);
        assert_eq!(table.version(), case.version);
        assert_eq!(table.scopes().collect::<Vec<_>>(), expected::scopes(&case));
        check_values(&table, &case);
        check_coverage(&table, &case);
    }
}

fn check_values(table: &FlowPrimaryTable, case: &PrimaryCase) {
    let mut expected = expected::leaves(case);
    let mut covered = vec![false; case.raw.len()];
    for field in table.fields() {
        assert_eq!(table.field(&field.id()).unwrap().layout(), field.layout());
        assert_eq!(field.id().record(), table.record());
        assert_eq!(
            table.field_at(field.id().ordinal()).unwrap().kind(),
            field.kind()
        );
        for index in 0..field.layout().element_count {
            check_element(field, index, &mut expected, &mut covered);
        }
        assert!(field.value(field.layout().element_count).is_none());
        assert!(field.element_bytes(usize::MAX).is_none());
    }
    assert!(expected.is_empty(), "{}", case.name);
    assert!(covered.into_iter().all(|value| value));
    assert!(table.field_at(table.fields().len()).is_none());
}

fn check_element(
    field: FlowPrimaryField<'_>,
    index: usize,
    expected: &mut expected::Leaves,
    covered: &mut [bool],
) {
    let offset = field.element_offset(index).unwrap();
    let bytes = field.element_bytes(index).unwrap();
    assert_eq!(bytes.len(), field.encoding().width());
    for present in &mut covered[offset..offset + bytes.len()] {
        assert!(!*present);
        *present = true;
    }
    assert_eq!(
        Some((field.value(index).unwrap(), field.scope(), field.kind())),
        expected.remove(&offset)
    );
    if field.kind() == FlowPrimaryFieldKind::StoredFlag {
        assert_eq!(field.native_flag(), Some(bytes[0] == 1));
    } else {
        assert_eq!(field.native_flag(), None);
    }
}

fn check_coverage(table: &FlowPrimaryTable, case: &PrimaryCase) {
    let coverage = table.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, case.raw.len());
    assert_eq!(coverage.items, case.items.len());
    assert_eq!(coverage.scopes, table.scopes().len());
    assert_eq!(coverage.fields, table.fields().len());
    assert_eq!(coverage.fields, expected::field_count(case));
    assert_eq!(
        coverage.values,
        table
            .fields()
            .map(|field| field.layout().element_count)
            .sum()
    );
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(table.options(), options());
    assert_eq!(table.diagnostics().len(), 5);
    assert_eq!(table.record().ordinal(), 1);
    assert_eq!(table.source().records().len(), 3);
    check_collection_counts(table, case);
}

fn check_collection_counts(table: &FlowPrimaryTable, case: &PrimaryCase) {
    let elements: Vec<_> = case
        .items
        .iter()
        .flat_map(|item| &item.groups)
        .filter_map(|group| group.elements.as_ref())
        .flatten()
        .collect();
    assert_eq!(table.coverage().elements, elements.len());
    assert_eq!(
        table.coverage().rows,
        elements.iter().map(|element| element.rows.len()).sum()
    );
    assert_eq!(
        table.coverage().wide_blocks,
        elements.iter().map(|element| element.blocks.len()).sum()
    );
}
