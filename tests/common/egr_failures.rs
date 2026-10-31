use super::{Cursor, checks, limits, owned, profile};
use bessemer::simulation::*;
use std::sync::Arc;

fn owned_table(index: usize) -> Vec<u8> {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    cursor.take(8);
    for _ in 0..index {
        cursor.span();
        cursor.span();
        cursor.span();
    }
    cursor.span().to_vec()
}

#[test]
fn invalid_headers_widths_mappings_and_deleted_chains_reject_without_publishing_results() {
    let original = owned_table(3);
    for &(offset, value) in INVALID_PATCHES {
        let mut changed = original.clone();
        changed[offset..offset + 4].copy_from_slice(&i32::to_le_bytes(value));
        let error = SimulationEgrArchive::from_bytes(&changed, profile(), limits()).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains("expected"));
    }
    assert_eq!(checks::table(&original).to_bytes(), original);
}

#[test]
fn large_or_negative_declared_counts_fail_before_directory_allocation_and_parts_copying() {
    let original = owned_table(3);
    for &(offset, value) in INVALID_COUNT_WORDS {
        let mut raw = original.clone();
        raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert!(
            SimulationEgrArchive::from_bytes(
                &raw,
                profile(),
                SimulationEgrLimits {
                    maximum_rows: usize::MAX
                }
            )
            .is_err()
        );
    }
    let source = checks::table(&original);
    let strict = SimulationEgrLimits { maximum_rows: 0 };
    assert!(SimulationEgrArchive::from_parts(source.parts(), profile(), strict).is_err());
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn opaque_header_words_and_descriptor_tags_are_preserved_without_invented_type_constraints() {
    let original = owned_table(3);
    for value in [i32::MIN, -1, 0, i32::MAX] {
        let mut raw = original.clone();
        for offset in [12, 28, 128] {
            raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        let image = checks::table(&raw);
        assert_eq!(image.header_words()[3], value);
        assert_eq!(image.header_words()[7], value);
        assert_eq!(image.fields()[0].tag, value);
        assert_eq!(image.to_bytes(), raw);
    }
}

#[test]
fn checked_holes_are_distinct_from_bad_ranges_and_missing_native_cache_values() {
    let image = checks::table(&owned_table(3));
    assert!(image.live_row_id(2).unwrap().is_none());
    let missing = image.field_range_id(2, 1, 1).unwrap_err();
    assert_eq!(missing.class, bessemer::FailureClass::Missing);
    assert_eq!(
        image.field_range_id(2, 0, 1).unwrap_err().class,
        bessemer::FailureClass::Invalid
    );
    for (first, last) in [(0, 1), (2, 1), (1, 33), (1, u32::MAX)] {
        assert!(image.field_range_id(1, first, last).is_err());
    }
    assert!(image.field_range_id(0, 0, 0).is_err());
    let id = image.field_range_id(1, 1, 1).unwrap();
    assert!(image.edit_field_range(&id, &[0; 3]).is_err());
    assert!(image.edit_field_range(&id, &[0; 5]).is_err());
    assert_eq!(image.field_range(&id).unwrap().bytes.len(), 4);
}

#[test]
fn foreign_revisions_cannot_reuse_physical_live_or_field_range_identities() {
    let first = checks::table(&owned_table(3));
    let second = checks::table(&first.to_bytes());
    let physical = first.physical_id(1).unwrap();
    let live = first.live_row_id(1).unwrap().unwrap();
    let range = first.field_range_id(1, 1, 1).unwrap();
    assert!(second.physical_record(&physical).is_err());
    assert!(second.live_row(&live).is_err());
    assert!(second.conditional_row(&live).is_err());
    assert!(second.field_range(&range).is_err());
    assert!(second.edit_field_range(&range, &[0; 4]).is_err());
    assert_eq!(first.to_bytes(), second.to_bytes());
    assert_ne!(first.revision(), second.revision());
}

#[test]
fn conditional_row_profile_shapes_and_logical_ordinals_reject_every_unassigned_boundary() {
    for widths in [
        vec![],
        vec![4; 31],
        vec![4; 33],
        vec![1; 32],
        vec![usize::MAX; 32],
    ] {
        assert!(SimulationEgrRowProfile::from_loaded_widths(&widths).is_err());
    }
    let row_profile = SimulationEgrRowProfile::from_loaded_widths(&[4; 32]).unwrap();
    for length in (0..128).chain([129, 256]) {
        assert!(SimulationEgrRowImage::from_bytes(&vec![0; length], row_profile).is_err());
    }
}

#[test]
fn table_parts_and_published_views_survive_caller_source_and_returned_byte_mutation() {
    let (image, captured) = retained_views();
    let original = image.to_bytes();
    let mut parts = image.parts();
    Arc::make_mut(&mut parts.field_directory)[0] ^= 1;
    let rebuilt = SimulationEgrArchive::from_parts(parts, profile(), limits()).unwrap();
    assert_ne!(rebuilt.to_bytes(), original);
    let mut returned = image.to_bytes();
    returned.fill(0);
    assert_eq!(image.to_bytes(), original);
    assert_eq!(captured.source.to_bytes(), original);
    assert_eq!(
        image.field_range(&captured.id).unwrap().bytes,
        captured.bytes
    );
}

fn retained_views() -> (SimulationEgrArchive, SimulationEgrFieldRange) {
    let mut raw = owned_table(3);
    let image = checks::table(&raw);
    let id = image.field_range_id(1, 1, 2).unwrap();
    let captured = image.field_range(&id).unwrap();
    raw.fill(0);
    assert_ne!(image.to_bytes(), raw);
    (image, captured)
}

#[test]
fn solver_destination_ownership_and_incomplete_null_state_survive_caller_mutation() {
    for value in [98, 999, 0] {
        let code = SimulationSolverElementCode::from_signed(value);
        let mut destination: Arc<[u8]> = vec![0xa5; 64].into();
        let result = code.project_destination(destination.clone()).unwrap();
        let original = result.destination.to_vec();
        Arc::make_mut(&mut destination).fill(0);
        assert_eq!(result.destination.as_ref(), original);
        if let Some(output) = result.output.as_ref() {
            let mut returned = output.clone();
            Arc::make_mut(&mut returned).fill(255);
            assert_ne!(returned, *output);
        }
        assert_eq!(result.code, code);
    }
}

#[test]
fn all_egr_public_types_are_send_sync_and_parallel_full_edits_and_lines_equal_serial_bytes() {
    assert_send_sync();
    let image = checks::table(&owned_table(3));
    let id = image.field_range_id(1, 1, 2).unwrap();
    let serial = image
        .edit_field_range(&id, &[0x5a; 8])
        .unwrap()
        .archive
        .to_bytes();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (image, id, serial) = (&image, &id, &serial);
            scope.spawn(move || repeat_projection(image, id, serial));
        }
    });
}

fn repeat_projection(image: &SimulationEgrArchive, id: &SimulationEgrRangeId, serial: &[u8]) {
    let live = image.live_row_id(1).unwrap().unwrap();
    let ordinal = SimulationEgrLogicalOrdinal::new(1).unwrap();
    let expected = image
        .conditional_row(&live)
        .unwrap()
        .project_egroup(ordinal, i32::MAX)
        .output;
    for _ in 0..32 {
        let result = image.edit_field_range(id, &[0x5a; 8]).unwrap();
        assert_eq!(result.archive.to_bytes(), serial);
        let row = image.conditional_row(&live).unwrap();
        assert_eq!(row.project_egroup(ordinal, i32::MAX).output, expected);
        assert_eq!(row.source_row_id(), Some(&live));
        assert_eq!(row.source_table().unwrap().revision(), image.revision());
    }
}

fn assert_send_sync() {
    assert_frame_types();
    assert_view_types();
    assert_name_types();
}

fn assert_frame_types() {
    fn check<T: Send + Sync>() {}
    check::<SimulationEgrArchive>();
    check::<SimulationEgrProfile>();
    check::<SimulationEgrLimits>();
    check::<SimulationEgrRevision>();
    check::<SimulationEgrLoadedField>();
    check::<SimulationEgrFreeRecord>();
    check::<SimulationEgrParts>();
    check::<SimulationEgrCoverage>();
    check::<SimulationEgrReadCompletion>();
    check::<SimulationEgrCheckMode>();
    check::<SimulationEgrDiagnostic>();
}

fn assert_view_types() {
    fn check<T: Send + Sync>() {}
    check::<SimulationEgrPhysicalId>();
    check::<SimulationEgrRowId>();
    check::<SimulationEgrRangeId>();
    check::<SimulationEgrFieldRange>();
    check::<SimulationEgrFieldEdit>();
    check::<SimulationEgrRecordHistory>();
    check::<SimulationEgrRecordRelation>();
    check::<SimulationEgrRowProfile>();
    check::<SimulationEgrRowImage>();
    check::<SimulationEgrRowView>();
    check::<SimulationEgrLogicalOrdinal>();
    check::<SimulationEgroupCompletion>();
    check::<SimulationEgroupProjection>();
    check::<SimulationEgrDescriptorProjection>();
}

fn assert_name_types() {
    fn check<T: Send + Sync>() {}
    check::<SimulationSolverElementCode>();
    check::<SimulationSolverNameState>();
    check::<SimulationSolverNameCompletion>();
    check::<SimulationSolverNameCheckMode>();
    check::<SimulationSolverNameProjection>();
}

#[test]
fn malformed_supplied_regions_reject_without_normalizing_or_truncating_parts() {
    let image = checks::table(&owned_table(3));
    let original = image.to_bytes();
    for region in 0..3 {
        let mut parts = image.parts();
        let target = match region {
            0 => &mut parts.field_directory,
            1 => &mut parts.row_directory,
            _ => &mut parts.allocated_records,
        };
        let mut bytes = target.to_vec();
        bytes.push(0);
        *target = bytes.into();
        assert!(SimulationEgrArchive::from_parts(parts, profile(), limits()).is_err());
    }
    assert_eq!(image.to_bytes(), original);
}

#[test]
fn completed_field_edit_retains_both_parents_after_source_and_replacement_are_dropped() {
    let edit = dropped_input_edit();
    assert_eq!(edit.replacement.as_ref(), &[0x5a; 8]);
    assert_eq!(edit.result.bytes.as_ref(), &[0x5a; 8]);
    let parent = edit.source.field_range(&edit.parent.id).unwrap();
    assert_eq!(parent.bytes, edit.parent.bytes);
    assert_eq!(
        edit.archive.field_range(&edit.result.id).unwrap().bytes,
        edit.result.bytes
    );
    assert_ne!(edit.source.revision(), edit.archive.revision());
}

fn dropped_input_edit() -> SimulationEgrFieldEdit {
    let image = checks::table(&owned_table(3));
    let id = image.field_range_id(1, 1, 2).unwrap();
    let mut replacement = vec![0x5a; 8];
    let result = image.edit_field_range(&id, &replacement).unwrap();
    replacement.fill(0);
    assert_eq!(result.replacement.as_ref(), &[0x5a; 8]);
    result
}

const INVALID_PATCHES: &[(usize, i32)] = &[
    (0, 127),
    (4, -1),
    (8, 33),
    (16, -1),
    (20, -1),
    (24, -1),
    (32, 5),
    (132, 0),
    (384, 1),
    (640, 2),
    (640, 1),
    (640, 4),
    (640, -1),
];

const INVALID_COUNT_WORDS: &[(usize, i32)] = &[
    (4, i32::MAX),
    (4, i32::MIN),
    (8, i32::MAX),
    (16, i32::MAX),
    (20, i32::MAX),
];

#[test]
fn logical_ordinals_reject_zero_and_unsigned_only_patterns() {
    for ordinal in [0, 0x80000000, u32::MAX] {
        assert!(SimulationEgrLogicalOrdinal::new(ordinal).is_err());
    }
    assert_eq!(
        SimulationEgrLogicalOrdinal::new(i32::MAX as u32)
            .unwrap()
            .one_based(),
        i32::MAX as u32
    );
}
