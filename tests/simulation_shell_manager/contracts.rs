use super::*;

fn owned_headers() -> Vec<SimulationShellManagerProperty> {
    let mut cursor = EXPECTED;
    let mut headers = Vec::new();
    while !cursor.is_empty() {
        headers.push(read_original(&mut cursor));
        consume_expected_fields(&mut cursor);
    }
    assert_eq!(headers.len(), 60);
    headers
}

fn decode(
    header: &SimulationShellManagerProperty,
    raw: &[u8],
    limits: SimulationShellManagerLimits,
) -> bessemer::Result<SimulationShellManagerProperty> {
    SimulationShellManagerProperty::from_bytes(raw, header.version(), header.wire_profile(), limits)
}

fn exact_header_limits(header: &SimulationShellManagerProperty) -> SimulationShellManagerLimits {
    SimulationShellManagerLimits {
        maximum_bytes: header.to_bytes().len(),
        maximum_fields: header.fields().len(),
        maximum_entries: 0,
        maximum_text_bytes: 0,
    }
}

fn assert_lowered_header_cap(
    header: &SimulationShellManagerProperty,
    limits: SimulationShellManagerLimits,
) {
    assert!(decode(header, &header.to_bytes(), limits).is_err());
    let rebuilt = SimulationShellManagerProperty::from_fields(
        header.fields().to_vec(),
        header.version(),
        header.wire_profile(),
        limits,
    );
    assert!(rebuilt.is_err());
}

fn check_exact_budgets(header: &SimulationShellManagerProperty) {
    let exact = exact_header_limits(header);
    assert_eq!(
        decode(header, &header.to_bytes(), exact)
            .unwrap()
            .to_bytes(),
        header.to_bytes()
    );
    let mut bytes = exact;
    bytes.maximum_bytes -= 1;
    assert_lowered_header_cap(header, bytes);
    let mut fields = exact;
    fields.maximum_fields -= 1;
    assert_lowered_header_cap(header, fields);
}

#[test]
fn every_owned_header_obeys_exact_and_one_below_positive_budgets_with_zero_text_and_entries() {
    for header in owned_headers() {
        check_exact_budgets(&header);
    }
}

#[test]
fn every_proper_prefix_and_trailing_byte_fails_with_sufficient_byte_budget() {
    for header in owned_headers() {
        let raw = header.to_bytes();
        let limits = SimulationShellManagerLimits {
            maximum_bytes: 32,
            ..LIMITS
        };
        for end in 0..raw.len() {
            assert_eq!(
                decode(&header, &raw[..end], limits).unwrap_err().class,
                bessemer::FailureClass::Invalid
            );
        }
        let trailing = [raw.as_slice(), &[0]].concat();
        assert!(decode(&header, &trailing, limits).is_err());
    }
}

fn assert_noop(header: &SimulationShellManagerProperty, ordinal: usize) {
    let parent = header.field_id(ordinal).unwrap();
    let edit = header
        .edit_field(&parent, header.fields()[ordinal].value.clone())
        .unwrap();
    assert_eq!(edit.result(), parent);
    assert_eq!(edit.archive().to_bytes(), header.to_bytes());
    assert_eq!(
        edit.coverage().completion,
        SimulationShellManagerCompletion::Unchanged
    );
    for (index, row) in edit.history().iter().enumerate() {
        assert_eq!(row.relation, SimulationShellManagerRelation::Copied);
        assert_eq!(
            row.parents.as_ref(),
            &[SimulationShellManagerHistoryParent::Source(
                header.field_id(index).unwrap()
            )]
        );
        assert_eq!(row.result, header.field_id(index).unwrap());
    }
}

#[test]
fn every_owned_field_noop_preserves_revision_identity_and_complete_copied_history() {
    for header in owned_headers() {
        for ordinal in 0..header.fields().len() {
            assert_noop(&header, ordinal);
        }
    }
}

#[test]
fn field_ownership_rejects_foreign_stale_and_out_of_bounds_handles() {
    let header = &owned_headers()[0];
    let independent = decode(header, &header.to_bytes(), LIMITS).unwrap();
    let original = header.field_id(0).unwrap();
    assert!(independent.field(&original).is_err());
    assert!(header.field_id(usize::MAX).is_err());
    let edit = header
        .edit_field(&original, SimulationShellManagerValue::Signed(17))
        .unwrap();
    assert!(edit.archive().field(&original).is_err());
    assert!(header.field(&edit.result()).is_err());
    assert!(header.field(&original).is_ok());
    assert!(edit.archive().field(&edit.result()).is_ok());
}

fn invalid_header_field_sets(
    header: &SimulationShellManagerProperty,
) -> [Vec<SimulationShellManagerField>; 4] {
    let mut path = header.fields().to_vec();
    path[0].path = "unbound-header-word".into();
    let mut kind = header.fields().to_vec();
    kind[0].value = SimulationShellManagerValue::Unsigned(0);
    let mut absent = header.fields().to_vec();
    absent.pop();
    let mut extra = header.fields().to_vec();
    extra.push(header.fields()[0].clone());
    [path, kind, absent, extra]
}

#[test]
fn construction_rejects_wrong_paths_kinds_and_missing_or_extra_fields() {
    for header in owned_headers() {
        for fields in invalid_header_field_sets(&header) {
            assert!(
                SimulationShellManagerProperty::from_fields(
                    fields,
                    header.version(),
                    header.wire_profile(),
                    LIMITS
                )
                .is_err()
            );
        }
    }
}

#[test]
fn mutable_inputs_successful_edits_and_failed_requests_remain_isolated() {
    for original in owned_headers() {
        let mut input = original.to_bytes();
        let source = decode(&original, &input, LIMITS).unwrap();
        let parent = source.field_id(0).unwrap();
        let edit = source
            .edit_field(&parent, SimulationShellManagerValue::Signed(23))
            .unwrap();
        assert!(
            source
                .edit_field(&parent, SimulationShellManagerValue::Unsigned(23))
                .is_err()
        );
        input.fill(0x5a);
        assert_eq!(source.to_bytes(), original.to_bytes());
        assert_eq!(edit.source().to_bytes(), original.to_bytes());
        assert_eq!(&edit.archive().to_bytes()[..4], &23i32.to_le_bytes());
        assert_eq!(&edit.archive().to_bytes()[4..], &original.to_bytes()[4..]);
    }
}

fn edit_signature(header: &SimulationShellManagerProperty, replacement: i32) -> Vec<u8> {
    let edit = header
        .edit_field(
            &header.field_id(0).unwrap(),
            SimulationShellManagerValue::Signed(replacement),
        )
        .unwrap();
    let result = edit.archive();
    let mut output = result.to_bytes();
    for (field, span) in result.fields().iter().zip(result.field_offsets()) {
        project_field(&mut output, field, *span);
    }
    let source_ids: Vec<_> = (0..header.fields().len())
        .map(|index| header.field_id(index).unwrap())
        .collect();
    let result_ids: Vec<_> = (0..result.fields().len())
        .map(|index| result.field_id(index).unwrap())
        .collect();
    load_projection::project_outer_history(&mut output, edit.history(), &source_ids, &result_ids);
    output
}

fn assert_parallel_options(
    headers: &[SimulationShellManagerProperty],
    expected: &[[Vec<u8>; 3]],
    worker: usize,
) {
    let replacements = [-2147483648, 0, 2147483647];
    for iteration in 0..32 {
        let source = (worker + iteration) % headers.len();
        let option = (worker * 7 + iteration) % replacements.len();
        assert_eq!(
            edit_signature(&headers[source], replacements[option]),
            expected[source][option]
        );
    }
}

#[test]
fn mixed_parallel_edits_match_serial_bytes_fields_and_all_ordered_history_parents() {
    let headers = std::sync::Arc::new(owned_headers());
    let replacements = [-2147483648, 0, 2147483647];
    let expected: Vec<_> = headers
        .iter()
        .map(|header| replacements.map(|word| edit_signature(header, word)))
        .collect();
    std::thread::scope(|scope| {
        for worker in 0..8 {
            let headers = headers.clone();
            let expected = &expected;
            scope.spawn(move || assert_parallel_options(&headers, expected, worker));
        }
    });
}
