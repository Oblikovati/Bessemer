use super::*;
use std::io::Write;
fn stored(raw: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
pub(super) fn sample() -> ValueCase {
    cases()
        .into_iter()
        .find(|row| row.name == "level-6")
        .unwrap()
}
#[test]
fn exact_and_one_below_independent_object_shared_bl_row_bl_value_and_decoded_limits() {
    for row in cases() {
        check_limits(&row);
    }
    let row = sample();
    assert!(
        open(
            &row.frame.stored,
            &context(&row),
            FlowImsValueOptions {
                maximum_decoded_bytes: 0,
                ..options()
            }
        )
        .is_err()
    );
}
fn check_limits(row: &ValueCase) {
    let context = context(row);
    let [shared, rows, bl] = totals(row);
    let limits = FlowImsValueOptions {
        maximum_decoded_bytes: row.frame.decoded.len(),
        maximum_objects: row.frame.objects.len(),
        maximum_shared_values: shared,
        maximum_bl_rows: rows,
        maximum_bl_values: bl,
    };
    assert!(open(&row.frame.stored, &context, limits).is_ok());
    for field in 0..5 {
        check_lower_limit(row, &context, limits, field);
    }
}
fn check_lower_limit(
    row: &ValueCase,
    context: &FlowImsValueContext,
    mut limits: FlowImsValueOptions,
    field: usize,
) {
    let limit = match field {
        0 => &mut limits.maximum_decoded_bytes,
        1 => &mut limits.maximum_objects,
        2 => &mut limits.maximum_shared_values,
        3 => &mut limits.maximum_bl_rows,
        _ => &mut limits.maximum_bl_values,
    };
    if *limit == 0 {
        return;
    }
    *limit -= 1;
    assert!(open(&row.frame.stored, context, limits).is_err());
}
#[test]
fn duplicate_identifier_objects_accumulate_shared_bl_row_and_bl_value_budgets() {
    let rows = cases();
    let definitions = rows.iter().find(|row| row.name == "objects-3").unwrap();
    let body = rows.iter().find(|row| row.name == "level-6").unwrap();
    let context = context(definitions);
    let mut decoded = vec![2];
    for _ in 0..3 {
        decoded.extend_from_slice(&body.frame.decoded[1..]);
    }
    let limits = repeated_limits(decoded.len());
    let payload = stored(&decoded);
    let view = open(&payload, &context, limits).unwrap();
    let proof = view.coverage();
    assert_eq!(
        [proof.shared_values, proof.bl_rows, proof.bl_values],
        [12, 3, 54]
    );
    check_repeated_objects(&view, body.frame.decoded.len() - 1);
    check_repeated_limits(&payload, &context, limits);
    assert_eq!(view.decoded_bytes(), decoded);
}
fn repeated_limits(bytes: usize) -> FlowImsValueOptions {
    FlowImsValueOptions {
        maximum_decoded_bytes: bytes,
        maximum_objects: 3,
        maximum_shared_values: 12,
        maximum_bl_rows: 3,
        maximum_bl_values: 54,
    }
}
fn check_repeated_objects(view: &FlowImsValues, body_bytes: usize) {
    assert_eq!(view.stored_count(), 3);
    for (ordinal, identifier) in [7, -1, 7].into_iter().enumerate() {
        let object = view.object(ordinal).unwrap();
        assert_eq!(object.identifier(), identifier);
        assert_eq!(
            object.body_interval(),
            (1 + ordinal * body_bytes, 1 + (ordinal + 1) * body_bytes)
        );
        let bl = object.bl().unwrap();
        assert_eq!(bl.row_count(), 1);
        assert_eq!(bl.row(0).unwrap().raw_bits(17), Some(17));
    }
}
fn check_repeated_limits(
    payload: &[u8],
    context: &FlowImsValueContext,
    limits: FlowImsValueOptions,
) {
    for kind in 0..3 {
        let mut lower = limits;
        match kind {
            0 => lower.maximum_shared_values -= 1,
            1 => lower.maximum_bl_rows -= 1,
            _ => lower.maximum_bl_values -= 1,
        }
        assert!(open(payload, context, lower).is_err());
    }
}
#[test]
fn zero_bounds_keep_empty_contexts_and_present_empty_storage_distinct() {
    let rows = cases();
    let row = rows.iter().find(|row| row.name == "objects-0").unwrap();
    let limits = FlowImsValueOptions {
        maximum_decoded_bytes: 1,
        maximum_objects: 0,
        maximum_shared_values: 0,
        maximum_bl_rows: 0,
        maximum_bl_values: 0,
    };
    let view = open(&row.frame.stored, &context(row), limits).unwrap();
    assert_eq!(view.decoded_bytes(), [2]);
    assert_eq!(view.stored_count(), 0);
    assert_eq!(view.coverage().shared_arrays, 0);
}
#[test]
fn zero_word_and_row_bounds_keep_present_empty_storage_distinct() {
    let rows = cases();
    for name in ["length-0-0", "bl-rows-0"] {
        let row = rows.iter().find(|row| row.name == name).unwrap();
        check_empty_storage(row);
    }
}
fn check_empty_storage(row: &ValueCase) {
    let limits = FlowImsValueOptions {
        maximum_decoded_bytes: row.frame.decoded.len(),
        maximum_objects: 1,
        maximum_shared_values: 0,
        maximum_bl_rows: 0,
        maximum_bl_values: 0,
    };
    let view = open(&row.frame.stored, &context(row), limits).unwrap();
    assert_eq!(
        view.coverage().shared_arrays,
        view.coverage().empty_shared_arrays
    );
    assert_eq!(view.coverage().bl_bodies, view.coverage().empty_bl_bodies);
}

#[test]
fn context_identity_uses_exact_definition_revision_and_retains_both_interpretations() {
    let row = sample();
    let first = context(&row);
    assert_eq!(first, first.clone());
    assert_ne!(first, context(&row));
    let before = open(&row.frame.stored, &first, options()).unwrap();
    let edit = first.definitions().replace_identifier(0, -7).unwrap();
    let second =
        FlowImsValueContext::new(edit.result(), FlowImsFreshObjects::FreshDeclared).unwrap();
    let after = open(&row.frame.stored, &second, options()).unwrap();
    assert_ne!(first, second);
    assert_eq!(before.object(0).unwrap().identifier(), 0);
    assert_eq!(after.object(0).unwrap().identifier(), -7);
    assert_eq!(
        before.context().definitions().decoded_bytes(),
        row.context.decoded
    );
    assert_eq!(before.decoded_bytes(), after.decoded_bytes());
}
#[test]
fn nonzero_definition_allocation_and_unsupported_ims_versions_fail_context_creation() {
    let row = sample();
    let definitions = check_context(&row);
    for flag in 1..=255_u8 {
        let edit = definitions.replace_allocation_byte(0, flag).unwrap();
        assert!(
            FlowImsValueContext::new(edit.result(), FlowImsFreshObjects::FreshDeclared).is_err()
        );
    }
    for version in [0, 1, 3] {
        let definitions = other_definition(version, &row.context.decoded[9..25]);
        assert!(
            FlowImsValueContext::new(&definitions, FlowImsFreshObjects::FreshDeclared).is_err()
        );
    }
    assert_eq!(definitions.decoded_bytes(), row.context.decoded);
}
fn other_definition(version: i32, guid: &[u8]) -> FlowResultDefinitions {
    let mut raw = vec![1];
    raw.extend_from_slice(&1_i32.to_le_bytes());
    raw.extend_from_slice(&0_i32.to_le_bytes());
    raw.extend_from_slice(guid);
    raw.extend_from_slice(&version.to_le_bytes());
    append_definition_storage(&mut raw, version);
    let source = state_carrier::carrier(500, &stored(&raw));
    let id = source.records().nth(1).unwrap().id;
    let options = FlowResultDefinitionOptions {
        maximum_decoded_bytes: 1000,
        maximum_objects: 1,
        maximum_values: 0,
    };
    source.record_500_ims_definitions(&id, options).unwrap()
}
fn append_definition_storage(raw: &mut Vec<u8>, version: i32) {
    let array = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    if version >= 1 {
        raw.extend_from_slice(&array);
    }
    if version >= 2 {
        raw.extend_from_slice(&array);
        raw.push(0);
    }
    if version == 3 {
        raw.extend_from_slice(&array);
        raw.extend_from_slice(&array);
    }
    raw.extend_from_slice(&[0; 16]);
}

#[test]
fn proper_prefixes_decoded_suffixes_and_all_unknown_outer_versions_are_rejected() {
    let row = sample();
    let context = context(&row);
    for length in 0..row.frame.decoded.len() {
        assert!(open(&stored(&row.frame.decoded[..length]), &context, options()).is_err());
    }
    let mut raw = row.frame.decoded.clone();
    raw.push(0);
    assert!(open(&stored(&raw), &context, options()).is_err());
    for version in 0..=255_u8 {
        if version == 2 {
            continue;
        }
        assert!(open(&stored(&[version]), &context, options()).is_err());
    }
}
#[test]
fn value_bl_versions_shared_markers_and_unsigned_native_counts_are_checked() {
    let row = sample();
    let object = &row.frame.objects[0];
    for version in [i32::MIN, -1, 0, 2, 4, i32::MAX] {
        reject_patch(&row, object.start as usize, &version.to_le_bytes());
    }
    let bl = object.bl.as_ref().unwrap();
    for version in [i32::MIN, -1, 1, i32::MAX] {
        reject_patch(&row, bl.start as usize, &version.to_le_bytes());
    }
    for array in case_arrays(&row) {
        check_array_bounds(&row, array);
    }
    for count in [i32::MAX as u64, 1 << 31, u64::MAX] {
        reject_patch(&row, bl.start as usize + 4, &count.to_le_bytes());
    }
}
fn check_array_bounds(row: &ValueCase, array: &ValueArray) {
    for marker in [i32::MIN, -1, 0, 2, i32::MAX] {
        reject_patch(row, array.start as usize, &marker.to_le_bytes());
    }
    for count in [i32::MAX as u64, 1 << 31, u64::MAX] {
        reject_patch(row, array.start as usize + 4, &count.to_le_bytes());
    }
}
fn reject_patch(row: &ValueCase, offset: usize, replacement: &[u8]) {
    let mut raw = row.frame.decoded.clone();
    raw[offset..offset + replacement.len()].copy_from_slice(replacement);
    let limits = FlowImsValueOptions {
        maximum_objects: usize::MAX,
        maximum_shared_values: usize::MAX,
        maximum_bl_rows: usize::MAX,
        maximum_bl_values: usize::MAX,
        ..options()
    };
    assert!(open(&stored(&raw), &context(row), limits).is_err());
}
#[test]
fn truncated_corrupt_concatenated_and_extended_zlib_streams_fail() {
    let row = sample();
    let context = context(&row);
    for length in 0..row.frame.stored.len() {
        assert!(open(&row.frame.stored[..length], &context, options()).is_err());
    }
    let mut corrupt = row.frame.stored.clone();
    *corrupt.last_mut().unwrap() ^= 1;
    assert!(open(&corrupt, &context, options()).is_err());
    for suffix in [b"x".as_slice(), row.frame.stored.as_slice()] {
        let mut raw = row.frame.stored.clone();
        raw.extend_from_slice(suffix);
        assert!(open(&raw, &context, options()).is_err());
    }
}
#[test]
fn mismatched_context_counts_wrong_foreign_and_stale_occurrences_fail_without_input_mutation() {
    let rows = cases();
    let row = rows.iter().find(|r| r.name == "level-6").unwrap();
    let view = check_case(row);
    for count in [0, 2, 257] {
        let other = rows
            .iter()
            .find(|r| r.name == format!("objects-{count}"))
            .unwrap();
        assert!(open(&row.frame.stored, &context(other), options()).is_err());
    }
    let foreign = open(&row.frame.stored, view.context(), options()).unwrap();
    reject_id(view.source(), foreign.record(), view.context());
    reject_id(
        view.source(),
        &view.source().records().nth(2).unwrap().id,
        view.context(),
    );
    let changed = view.replace_flag(0, FlowImsValueFlag::Bl, 254).unwrap();
    reject_id(changed.result().source(), view.record(), view.context());
    assert_eq!(view.decoded_bytes(), row.frame.decoded);
}
fn reject_id(source: &FlowCarrier, id: &FlowRecordId, context: &FlowImsValueContext) {
    assert!(
        source
            .record_501_ims_values(id, context, options())
            .is_err()
    );
}
#[test]
fn checked_objects_arrays_rows_columns_absence_and_presence_transitions_preserve_sources() {
    let row = sample();
    let view = check_case(&row);
    for ordinal in [view.stored_count(), usize::MAX] {
        assert!(view.object(ordinal).is_err());
        assert!(view.replace_flag(ordinal, FLAGS[0], 1).is_err());
        assert!(
            view.replace_shared_pattern(ordinal, ARRAYS[0], 0, 0)
                .is_err()
        );
        assert!(view.replace_bl_pattern(ordinal, 0, 0, 0).is_err());
    }
    check_element_indices(&view);
    let absent = cases().into_iter().find(|r| r.name == "flag-0-0").unwrap();
    let view = check_case(&absent);
    assert!(view.replace_shared_pattern(0, ARRAYS[0], 0, 0).is_err());
    assert!(view.replace_bl_pattern(0, 0, 0, 0).is_err());
    assert!(view.replace_flag(0, FLAGS[0], 1).is_err());
    assert_eq!(view.decoded_bytes(), absent.frame.decoded);
}
fn check_element_indices(view: &FlowImsValues) {
    let object = view.object(0).unwrap();
    for kind in ARRAYS {
        let count = object.array(kind).unwrap().word_count();
        for index in [count, usize::MAX] {
            assert!(view.replace_shared_pattern(0, kind, index, 0).is_err());
        }
    }
    let bl = object.bl().unwrap();
    for row in [bl.row_count(), usize::MAX] {
        assert!(view.replace_bl_pattern(0, row, 0, 0).is_err());
    }
    for column in [18, usize::MAX] {
        assert!(view.replace_bl_pattern(0, 0, column, 0).is_err());
    }
    for flag in FLAGS {
        assert!(view.replace_flag(0, flag, 0).is_err());
    }
}
#[test]
fn all_new_and_reused_public_types_are_send_and_sync() {
    fn traits<T: Send + Sync>() {}
    traits::<FlowImsFreshObjects>();
    traits::<FlowImsValueContext>();
    traits::<FlowImsValueOptions>();
    traits::<FlowImsValueArray>();
    traits::<FlowImsValueFlag>();
    traits::<FlowImsValues>();
    traits::<FlowImsValue<'static>>();
    traits::<FlowImsBlRows<'static>>();
    traits::<FlowImsValueCoverage>();
    traits::<FlowImsValueDiagnostic>();
    traits::<FlowTableWideArray<'static>>();
    traits::<FlowSimpleTableEdit<FlowImsValues>>();
    traits::<FlowResultDefinitions>();
}
