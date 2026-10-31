//! Independent complete ParameterSets framing, construction, edits and ownership checks.
use bessemer::simulation::*;
#[path = "simulation_parameter_sets/corpus.rs"]
mod corpus;
#[path = "simulation_parameter_sets/fixture_storage.rs"]
mod fixture_storage;
#[path = "simulation_parameter_sets/projections.rs"]
mod projections;
#[path = "simulation_parameter_sets/support.rs"]
mod support;
use support::*;

#[test]
fn literal_empty_modern_image_has_all_eleven_words_and_completed_empty_records() {
    let source = SimulationParameterSets::from_parts(
        &parts(830010, 0, false),
        version(830010),
        SimulationParameterLimits::default(),
    )
    .unwrap();
    let expected: Vec<u8> = EMPTY_MODERN_WORDS
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    assert_eq!(source.to_bytes(), expected);
    assert_eq!(
        source.coverage().completion,
        SimulationParameterCompletion::CompletedEmptyRecords
    );
    assert_eq!(source.coverage().checked_fields, 11);
    assert_eq!(source.coverage().table_count, 0);
    assert_eq!(source.coverage().record_count, 0);
}

#[test]
fn populated_modern_fields_have_literal_positions_distinct_unsigned_words_and_raw_doubles() {
    let source = modern();
    assert_eq!(source.to_bytes().len(), 96);
    assert_eq!(source.fields().len(), 21);
    assert_eq!(source.fields()[10].stream_offset, 40);
    assert_eq!(source.fields()[15].stream_offset, 60);
    assert_eq!(source.fields()[16].stream_offset, 68);
    assert_eq!(source.fields()[17].stream_offset, 76);
    assert_eq!(source.fields()[20].stream_offset, 92);
    verify_modern_value_kinds(&source);
    verify_complete_field_partition(&source);
}

fn verify_modern_value_kinds(source: &SimulationParameterSets) {
    assert!(matches!(
        source.fields()[12].value,
        SimulationParameterValue::Unsigned(u32::MAX)
    ));
    assert!(matches!(
        source.fields()[15].value,
        SimulationParameterValue::DoubleBits(0x7ff0000000000001)
    ));
    assert!(matches!(
        source.fields()[16].value,
        SimulationParameterValue::DoubleBits(0x8000000000000000)
    ));
}

#[test]
fn all_version_threshold_neighbors_reconstruct_and_keep_legacy_ignored_header_word() {
    for numeric in [
        0,
        700011,
        700012,
        700013,
        700014,
        700018,
        700019,
        810025,
        810026,
        820015,
        820016,
        830009,
        830010,
        i32::MAX,
    ] {
        verify_version_profile(numeric);
    }
}

fn verify_version_profile(numeric: i32) {
    let columns = if numeric < 810026 { 100 } else { 2 };
    let source = SimulationParameterSets::from_parts(
        &parts(numeric, columns, true),
        version(numeric),
        SimulationParameterLimits::default(),
    )
    .unwrap();
    let reopened =
        SimulationParameterSets::from_bytes(&source.to_bytes(), version(numeric), source.limits())
            .unwrap();
    assert_eq!(reopened.to_bytes(), source.to_bytes());
    assert_eq!(reopened.parts().header, source.parts().header);
    assert_eq!(source.coverage().table_count, columns);
    assert_eq!(source.parts().records[0].numbers, numbers(numeric, columns));
    verify_complete_field_partition(&source);
    if numeric < 810026 {
        verify_legacy_header_edit(&source);
    }
}

fn verify_legacy_header_edit(source: &SimulationParameterSets) {
    let edit = source
        .edit_field(
            &source.field_id(0).unwrap(),
            SimulationParameterValue::Signed(-1),
        )
        .unwrap();
    assert_eq!(edit.archive.parts().header.words[0], -1);
    assert_eq!(edit.archive.parts().table.len(), 100);
    verify_edit(&edit);
}

#[test]
fn every_field_noop_changed_pattern_and_structural_rejection_preserve_complete_source() {
    let source = modern();
    for field in source.fields() {
        verify_edit(&source.edit_field(&field.id, field.value.clone()).unwrap());
        let changed = changed_pattern(&field.value);
        let result = source.edit_field(&field.id, changed);
        if matches!(
            field.role,
            SimulationParameterRole::HeaderWord(0) | SimulationParameterRole::RecordCount
        ) {
            assert!(result.is_err());
            continue;
        }
        verify_edit(&result.unwrap());
    }
    assert_eq!(source.to_bytes(), modern().to_bytes());
}

#[test]
fn all_prefixes_surplus_bytes_negative_huge_counts_and_explicit_budgets_fail() {
    let source = modern();
    let raw = source.to_bytes();
    verify_prefixes_and_surplus(&source);
    for offset in [0, 28] {
        verify_bad_count(&raw, offset);
    }
    let limits = SimulationParameterLimits {
        maximum_records: 0,
        ..source.limits()
    };
    assert!(SimulationParameterSets::from_bytes(&raw, source.version(), limits).is_err());
    let limits = SimulationParameterLimits {
        maximum_table_items: 1,
        ..source.limits()
    };
    assert!(SimulationParameterSets::from_bytes(&raw, source.version(), limits).is_err());
}

fn verify_prefixes_and_surplus(source: &SimulationParameterSets) {
    let raw = source.to_bytes();
    for end in 0..raw.len() {
        assert!(
            SimulationParameterSets::from_bytes(&raw[..end], source.version(), source.limits())
                .is_err()
        );
    }
    let mut extra = raw.clone();
    extra.push(0);
    assert!(
        SimulationParameterSets::from_bytes(&extra, source.version(), source.limits()).is_err()
    );
}

fn verify_bad_count(raw: &[u8], offset: usize) {
    for value in [-1i32, i32::MAX] {
        let mut invalid = raw.to_vec();
        invalid[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        let limits = SimulationParameterLimits {
            maximum_records: i32::MAX as usize,
            maximum_table_items: i32::MAX as usize,
            ..Default::default()
        };
        let error =
            SimulationParameterSets::from_bytes(&invalid, version(830010), limits).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains("expected"));
    }
}

#[test]
fn wrong_construction_counts_optional_shapes_and_text_budgets_are_rejected() {
    let source = modern();
    let mut invalid = source.parts().clone();
    invalid.table.pop();
    assert!(
        SimulationParameterSets::from_parts(&invalid, source.version(), source.limits()).is_err()
    );
    let mut invalid = source.parts().clone();
    invalid.records[0].numbers.values_bits.pop();
    assert!(
        SimulationParameterSets::from_parts(&invalid, source.version(), source.limits()).is_err()
    );
    let mut invalid = source.parts().clone();
    invalid.header.gate700013 = None;
    assert!(
        SimulationParameterSets::from_parts(&invalid, source.version(), source.limits()).is_err()
    );
    let mut invalid = parts(0, 100, true);
    invalid.records[0].numbers.conditional_word = Some(0);
    assert!(SimulationParameterSets::from_parts(&invalid, version(0), source.limits()).is_err());
}

#[test]
fn foreign_ids_wrong_types_and_oversized_text_do_not_change_published_source() {
    let source = modern();
    let foreign = modern().field_id(0).unwrap();
    assert!(source.field(&foreign).is_err());
    expect_invalid_edit(&source, &foreign, SimulationParameterValue::Signed(0));
    assert!(source.field_id(usize::MAX).is_err());
    expect_invalid_edit(
        &source,
        &source.field_id(12).unwrap(),
        SimulationParameterValue::Signed(-1),
    );
    verify_text_budget_error(&source);
    assert_eq!(source.to_bytes(), modern().to_bytes());
}

fn verify_text_budget_error(source: &SimulationParameterSets) {
    let limits = SimulationParameterLimits {
        maximum_text_bytes: 2,
        ..source.limits()
    };
    let bounded =
        SimulationParameterSets::from_bytes(&source.to_bytes(), source.version(), limits).unwrap();
    let oversized = SimulationPersistText::from_payload(Some(&[0x81, 0, 0]), 8).unwrap();
    expect_invalid_edit(
        &bounded,
        &bounded.field_id(10).unwrap(),
        SimulationParameterValue::Text(oversized),
    );
}

#[test]
fn caller_parts_drop_and_returned_arrays_fields_and_bytes_remain_isolated() {
    let mut original = parts(830010, 2, true);
    let source = construct(&original, 830010, Default::default());
    original.table.fill(0);
    original.records[0].numbers.values_bits.fill(0);
    drop(original);
    let mut copied = source.parts().clone();
    copied.table.fill(0);
    copied.records[0].numbers.values_bits.fill(0);
    let mut field = source.fields()[0].clone();
    field.value = SimulationParameterValue::Signed(-1);
    let mut exported = source.to_bytes();
    exported.fill(0x81);
    assert_eq!(source.to_bytes(), modern().to_bytes());
    assert_eq!(
        source.field(&field.id).unwrap().value.to_bytes(),
        2i32.to_le_bytes()
    );
}

#[test]
fn arrays_past_native_retention_preserve_every_value_and_last_field_edit() {
    for count in [10000usize, 10001] {
        verify_large_array(count);
    }
}

fn verify_large_array(count: usize) {
    let limits = SimulationParameterLimits {
        maximum_table_items: count,
        ..Default::default()
    };
    let source = construct(&parts(830010, count, true), 830010, limits);
    verify_array_values(&source, count);
    verify_final_array_edit(&source, count);
    if count > 10000 {
        assert!(
            SimulationParameterSets::from_bytes(
                &source.to_bytes(),
                source.version(),
                Default::default()
            )
            .is_err()
        );
    }
}

fn verify_array_values(source: &SimulationParameterSets, count: usize) {
    assert_eq!(source.parts().table.len(), count);
    assert_eq!(
        source.parts().records[0].numbers.values_bits,
        numbers(830010, count).values_bits
    );
}

#[test]
fn original_input_bytes_are_owned_before_caller_mutation_and_drop() {
    let mut original = modern().to_bytes();
    let source =
        SimulationParameterSets::from_bytes(&original, version(830010), Default::default())
            .unwrap();
    original.fill(0x81);
    drop(original);
    assert_eq!(source.to_bytes(), modern().to_bytes());
    assert_eq!(
        source.parts().records[0].text.source_parent(),
        source.to_bytes()
    );
    let SimulationParameterValue::Text(text) = &source.fields()[10].value else {
        panic!("expected the literal first record text");
    };
    assert!(std::ptr::eq(
        text.source_parent().as_ptr(),
        source.parts().records[0].text.source_parent().as_ptr()
    ));
}

#[test]
fn text_noop_keeps_original_ids_and_entire_independent_replacement_parent() {
    let source = modern();
    let parent = [0x91, 0x92, 0, 0, 0, 0, 0x93];
    let text = SimulationPersistText::read_from(&parent, 2, 8).unwrap();
    let edit = source
        .edit_field(
            &source.field_id(10).unwrap(),
            SimulationParameterValue::Text(text),
        )
        .unwrap();
    verify_edit(&edit);
    assert_eq!(
        edit.coverage.completion,
        SimulationParameterCompletion::Unchanged
    );
    let SimulationParameterValue::Text(replacement) = edit.replacement else {
        panic!("expected the retained raw text replacement");
    };
    assert_eq!(replacement.source_parent(), parent);
    assert_eq!(replacement.source_offsets(), (2, 6, 6));
}

fn verify_final_array_edit(source: &SimulationParameterSets, count: usize) {
    let field = find_role(
        source,
        SimulationParameterRole::RecordValue {
            record: 0,
            column: count - 1,
        },
    );
    let edit = source
        .edit_field(&field.id, SimulationParameterValue::DoubleBits(1))
        .unwrap();
    verify_edit(&edit);
}

#[test]
fn all_public_parameter_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationParameterLimits>();
    assert_send_sync::<SimulationParameterHeader>();
    assert_send_sync::<SimulationParameterRecordNumbers>();
    assert_send_sync::<SimulationParameterRecordParts>();
    assert_send_sync::<SimulationParameterParts>();
    assert_send_sync::<SimulationParameterRevision>();
    assert_send_sync::<SimulationParameterFieldId>();
    assert_send_sync::<SimulationParameterRole>();
    assert_send_sync::<SimulationParameterValue>();
    assert_send_sync::<SimulationParameterField>();
    assert_send_sync::<SimulationParameterCompletion>();
    assert_send_sync::<SimulationParameterCheckMode>();
    assert_send_sync::<SimulationParameterDiagnostic>();
    assert_send_sync::<SimulationParameterCoverage>();
    assert_send_sync::<SimulationParameterRelation>();
    assert_send_sync::<SimulationParameterHistory>();
    assert_send_sync::<SimulationParameterEdit>();
    assert_send_sync::<SimulationParameterSets>();
}

#[test]
fn concurrent_complete_parameter_edits_and_all_history_match_serial_results() {
    let source = modern();
    let serial = source
        .edit_field(
            &source.field_id(16).unwrap(),
            SimulationParameterValue::DoubleBits(1),
        )
        .unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, serial) = (&source, &serial);
            scope.spawn(move || repeat_edit(source, serial));
        }
    });
}

fn repeat_edit(source: &SimulationParameterSets, serial: &SimulationParameterEdit) {
    for _ in 0..32 {
        let edit = source
            .edit_field(
                &source.field_id(16).unwrap(),
                SimulationParameterValue::DoubleBits(1),
            )
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edit.coverage, serial.coverage);
        verify_edit(&edit);
        assert_eq!(source.to_bytes(), modern().to_bytes());
    }
}
