use super::*;
use bessemer::FailureClass;
use std::io::Write;
fn stored(raw: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
fn sample() -> DefinitionCase {
    cases()
        .into_iter()
        .find(|row| row.name == "pattern-0-00000000")
        .unwrap()
}
#[test]
fn exact_decoded_object_and_total_value_bounds_accept_and_one_below_rejects() {
    for row in cases() {
        check_limits(&row);
    }
    let row = sample();
    for maximum in [0, 1, 20] {
        assert!(
            open(
                &row.stored,
                FlowResultDefinitionOptions {
                    maximum_decoded_bytes: maximum,
                    ..options()
                }
            )
            .is_err()
        );
    }
}
fn check_limits(row: &DefinitionCase) {
    let mut limits = FlowResultDefinitionOptions {
        maximum_decoded_bytes: row.decoded.len(),
        maximum_objects: row.groups[0].objects.len(),
        maximum_values: raw_count(row),
    };
    assert!(open(&row.stored, limits).is_ok());
    limits.maximum_decoded_bytes -= 1;
    assert!(open(&row.stored, limits).is_err());
    limits.maximum_decoded_bytes += 1;
    if limits.maximum_objects > 0 {
        limits.maximum_objects -= 1;
        assert!(open(&row.stored, limits).is_err());
        limits.maximum_objects += 1;
    }
    if limits.maximum_values > 0 {
        limits.maximum_values -= 1;
        assert!(open(&row.stored, limits).is_err());
    }
}
#[test]
fn zero_object_and_value_bounds_distinguish_empty_collections_and_present_empty_arrays() {
    let rows = cases();
    let row = rows.iter().find(|row| row.name == "objects-0").unwrap();
    let limits = FlowResultDefinitionOptions {
        maximum_decoded_bytes: 21,
        maximum_objects: 0,
        maximum_values: 0,
    };
    let empty = open(&row.stored, limits).unwrap();
    assert_eq!(
        (empty.stored_count(), empty.coverage().stored_arrays),
        (0, 0)
    );
    for count in [1, 2, 3, 16, 64, 257] {
        let row = rows
            .iter()
            .find(|row| row.name == format!("objects-{count}"))
            .unwrap();
        check_present_empty(row, count);
    }
}
fn check_present_empty(row: &DefinitionCase, count: usize) {
    let mut limits = options();
    limits.maximum_values = 0;
    let view = open(&row.stored, limits).unwrap();
    assert_eq!(view.coverage().stored_arrays, view.coverage().empty_arrays);
    assert_eq!(view.stored_count(), count);
}

#[test]
fn proper_prefixes_appended_decoded_bytes_and_invalid_outer_versions_are_rejected() {
    let row = sample();
    for length in 0..row.decoded.len() {
        assert!(open(&stored(&row.decoded[..length]), options()).is_err());
    }
    let mut extended = row.decoded.clone();
    extended.push(0);
    assert!(open(&stored(&extended), options()).is_err());
    for version in 0..=255_u8 {
        if version == 1 {
            continue;
        }
        let error = open(&stored(&[version]), options()).unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
    }
}
#[test]
fn every_guid_bit_invalid_ims_versions_and_secondary_counts_are_rejected() {
    let row = sample();
    for bit in 0..128 {
        let mut changed = row.decoded.clone();
        changed[9 + bit / 8] ^= 1 << (bit % 8);
        assert!(open(&stored(&changed), options()).is_err());
    }
    for version in [i32::MIN, -1, 4, i32::MAX] {
        reject_patch(&row, 25, &version.to_le_bytes());
    }
    for count in [-1_i32, 1, i32::MAX] {
        for group in &row.groups[1..] {
            reject_patch(&row, group.start as usize, &count.to_le_bytes());
        }
    }
    for count in [-1_i32, i32::MAX] {
        reject_patch(&row, 1, &count.to_le_bytes());
    }
}
#[test]
fn shared_array_markers_counts_and_unsigned_native_transfer_limits_are_strict() {
    let row = sample();
    for array in row.groups[0].objects[0].body.arrays.iter().flatten() {
        for marker in [i32::MIN, -1, 0, 2, i32::MAX] {
            reject_patch(&row, array.start as usize, &marker.to_le_bytes());
        }
        for count in [i32::MAX as u64, 1 << 31, u64::MAX] {
            reject_patch(&row, array.start as usize + 4, &count.to_le_bytes());
        }
    }
}
fn reject_patch(row: &DefinitionCase, offset: usize, replacement: &[u8]) {
    let mut raw = row.decoded.clone();
    raw[offset..offset + replacement.len()].copy_from_slice(replacement);
    let limits = FlowResultDefinitionOptions {
        maximum_objects: usize::MAX,
        maximum_values: usize::MAX,
        ..options()
    };
    assert!(open(&stored(&raw), limits).is_err());
}
#[test]
fn truncated_corrupt_concatenated_and_extended_compressed_streams_fail() {
    let row = sample();
    for length in 0..row.stored.len() {
        assert!(open(&row.stored[..length], options()).is_err());
    }
    let mut corrupt = row.stored.clone();
    *corrupt.last_mut().unwrap() ^= 1;
    assert!(open(&corrupt, options()).is_err());
    for suffix in [b"x".as_slice(), row.stored.as_slice()] {
        let mut raw = row.stored.clone();
        raw.extend_from_slice(suffix);
        assert!(open(&raw, options()).is_err());
    }
}
#[test]
fn invalid_indices_absent_fields_foreign_wrong_and_stale_occurrences_preserve_inputs() {
    let row = sample();
    let view = check_case(&row);
    check_bad_indices(&view);
    check_foreign_occurrences(&view);
    let legacy = cases()
        .into_iter()
        .find(|row| row.name == "identifier-0-00000000")
        .unwrap();
    let view = check_case(&legacy);
    assert!(view.replace_allocation_byte(0, 1).is_err());
    for kind in KINDS {
        assert!(view.replace_pattern(0, kind, 0, 1).is_err());
    }
    assert_eq!(view.decoded_bytes(), legacy.decoded);
}
fn check_bad_indices(view: &FlowResultDefinitions) {
    for ordinal in [view.stored_count(), usize::MAX] {
        assert!(view.definition(ordinal).is_err());
        assert!(view.replace_identifier(ordinal, 0).is_err());
        assert!(view.replace_allocation_byte(ordinal, 0).is_err());
        assert!(view.replace_pattern(ordinal, KINDS[0], 0, 0).is_err());
    }
    for ordinal in [5, usize::MAX] {
        assert!(view.collection_interval(ordinal).is_err());
    }
    for kind in KINDS {
        let count = view
            .definition(0)
            .unwrap()
            .array(kind)
            .unwrap()
            .word_count();
        for index in [count, usize::MAX] {
            assert!(view.replace_pattern(0, kind, index, 0).is_err());
        }
    }
}
fn check_foreign_occurrences(view: &FlowResultDefinitions) {
    let before = view.source().as_bytes().to_vec();
    let foreign = snapshot(&before);
    reject_id(view.source(), &foreign.records().nth(1).unwrap().id);
    reject_id(view.source(), &view.source().records().nth(2).unwrap().id);
    let changed = view.replace_identifier(0, -7).unwrap();
    reject_id(changed.result().source(), view.record());
    assert_eq!(view.source().as_bytes(), before);
}
fn reject_id(source: &FlowCarrier, id: &FlowRecordId) {
    let error = source
        .record_500_ims_definitions(id, options())
        .unwrap_err();
    assert!(!error.cause.is_empty());
}
#[test]
fn all_new_and_reused_public_types_are_send_and_sync() {
    fn traits<T: Send + Sync>() {}
    traits::<FlowResultDefinitionOptions>();
    traits::<FlowImsDefinitionVersion>();
    traits::<FlowImsArrayKind>();
    traits::<FlowResultDefinitions>();
    traits::<FlowImsDefinition<'static>>();
    traits::<FlowResultDefinitionCoverage>();
    traits::<FlowBlockWordArray<'static>>();
    traits::<FlowSimpleTableEdit<FlowResultDefinitions>>();
    traits::<FlowSimpleTableDiagnostic>();
}
