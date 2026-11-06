use super::*;

#[test]
fn complete_typed_projection_matches_every_frozen_context_pattern_and_body_interval() {
    for case in fixture::cases() {
        let view = read_case(&case);
        assert_eq!(view.version(), case.version);
        assert_eq!(view.body_bytes(), case.frame.decoded);
        assert_eq!(
            view.options().compatibility_restriction,
            case.frame.restriction
        );
        check_projection(&view, &case.projection);
        check_context(&view, &case.frame.context);
        check_coverage(&view, &case);
    }
}

fn check_projection(view: &FlowFieldValues, projection: &FieldProjection) {
    match projection {
        FieldProjection::Legacy {
            table,
            checked,
            objects,
        } => check_legacy(view, table, *checked, objects),
        FieldProjection::Regions {
            count_checked,
            widths_checked,
            regions,
        } => {
            assert!(*count_checked && *widths_checked);
            check_regions(view, regions);
        }
    }
}

fn check_legacy(view: &FlowFieldValues, table: &[i32], checked: bool, objects: &[FieldObject]) {
    assert_eq!(view.width_table().unwrap().word_count(), table.len());
    assert_eq!(view.coverage().width_table_words, table.len());
    for (index, word) in table.iter().enumerate() {
        assert_eq!(view.width_table_word(index), Some(*word));
    }
    assert_eq!(view.width_table_word(table.len()), None);
    assert_eq!(view.objects().len(), objects.len());
    assert_eq!(view.regions().len(), 0);
    assert_eq!(view.coverage().expected_width_table_checked, Some(checked));
    for expected in objects {
        check_object(view.object(expected.ordinal as usize).unwrap(), expected);
    }
    assert!(view.object(objects.len()).is_none());
    assert!(view.region(0).is_none());
}

fn check_object(actual: FlowFieldObject<'_>, expected: &FieldObject) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(actual.width().value(), expected.width);
    assert_eq!(
        actual.body_interval(),
        (expected.start as usize, expected.end as usize)
    );
    check_wide(actual.wide_words(), &expected.bits, expected.start as usize);
}

fn check_regions(view: &FlowFieldValues, regions: &[FieldRegion]) {
    assert!(view.width_table().is_none());
    assert!(view.width_table_word(0).is_none());
    assert_eq!(view.regions().len(), regions.len());
    assert_eq!(view.objects().len(), 0);
    assert_eq!(view.coverage().expected_width_table_checked, None);
    assert_eq!(view.coverage().width_table_words, 0);
    for expected in regions {
        check_region(view.region(expected.ordinal as usize).unwrap(), expected);
    }
    assert!(view.region(regions.len()).is_none());
    assert!(view.object(0).is_none());
}

fn check_region(actual: FlowFieldRegion<'_>, expected: &FieldRegion) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    let context = (
        actual.row_count().value(),
        actual.width().value(),
        actual.active(),
    );
    assert_eq!(context, (expected.rows, expected.width, expected.active));
    assert_eq!(
        actual.body_interval(),
        (expected.start as usize, expected.end as usize)
    );
    check_wide(
        actual.wide_words(),
        &expected.bits,
        expected.start as usize + 4,
    );
}

fn check_wide(actual: FlowTableWideArray<'_>, expected: &[u64], start: usize) {
    assert_eq!(actual.word_count(), expected.len());
    assert_eq!(
        actual.payload_interval(),
        (start, start + expected.len() * 8)
    );
    for (index, bits) in expected.iter().enumerate() {
        assert_eq!(actual.raw_bits(index), Some(*bits));
    }
    assert_eq!(actual.raw_bits(expected.len()), None);
}

fn check_context(view: &FlowFieldValues, context: &FieldContext) {
    match &view.options().layout {
        FlowFieldLayout::Legacy(actual) => check_legacy_context(actual, context),
        FlowFieldLayout::Regions(actual) => check_region_context(actual, context),
    }
}

fn check_legacy_context(actual: &FlowLegacyFieldLayout, context: &FieldContext) {
    let FieldContext::Legacy {
        table_count,
        object_widths,
        expected,
    } = context
    else {
        panic!("expected legacy context");
    };
    assert_eq!(actual.width_table_count().value(), *table_count);
    assert_eq!(field_counts(actual.object_widths()), *object_widths);
    let actual = actual.expected_width_table().map(expected_counts);
    assert_eq!(actual, *expected);
}

fn field_counts(counts: &[FlowFieldCount]) -> Vec<u32> {
    counts.iter().map(|count| count.value()).collect()
}
fn expected_counts(entries: &[Option<FlowFieldCount>]) -> Vec<Option<u32>> {
    entries
        .iter()
        .map(|entry| entry.map(FlowFieldCount::value))
        .collect()
}

fn check_region_context(actual: &FlowRegionFieldLayout, context: &FieldContext) {
    let FieldContext::Regions {
        rows,
        widths,
        active,
    } = context
    else {
        panic!("expected region context");
    };
    let expected = expected_regions(rows, widths, active);
    let actual = actual_regions(actual);
    assert_eq!(actual, expected);
}

fn check_coverage(view: &FlowFieldValues, case: &FieldCase) {
    let actual = view.coverage();
    assert_eq!(
        coverage_bytes(actual),
        case.byte_counts.map(|value| value as usize)
    );
    assert_eq!(
        (actual.objects, actual.regions, actual.wide_words),
        projection_counts(&case.projection)
    );
    check_region_checks(actual, &case.projection);
    assert_eq!(actual.achieved_error, 0);
    assert_eq!(view.record().ordinal(), 1);
    assert_eq!(view.source().records().len(), 3);
    assert_eq!(view.diagnostics().len(), 8);
    check_exclusions(view, case.flags);
    check_mode(actual.mode, case.frame.compressed);
}

fn coverage_bytes(actual: FlowFieldValueCoverage) -> [usize; 4] {
    [
        actual.stored_version_bytes,
        actual.stored_body_bytes,
        actual.decoded_body_bytes,
        actual.decoded_payload_bytes,
    ]
}
fn projection_counts(projection: &FieldProjection) -> (usize, usize, usize) {
    match projection {
        FieldProjection::Legacy { objects, .. } => (
            objects.len(),
            0,
            objects.iter().map(|item| item.bits.len()).sum(),
        ),
        FieldProjection::Regions { regions, .. } => (
            0,
            regions.len(),
            regions.iter().map(|item| item.bits.len()).sum(),
        ),
    }
}
fn check_mode(actual: FlowFieldValueCheckMode, compressed: bool) {
    assert_eq!(
        actual,
        if compressed {
            FlowFieldValueCheckMode::ExactZlibVersionedBody
        } else {
            FlowFieldValueCheckMode::ExactRawVersionedBody
        }
    );
}
fn check_exclusions(view: &FlowFieldValues, expected: [bool; 4]) {
    use FlowFieldValueDiagnostic as Exclusion;
    let diagnostics = view.diagnostics();
    let actual = [
        !diagnostics.contains(&Exclusion::NativeContextNotExecuted),
        diagnostics.contains(&Exclusion::LayoutContextAssumed),
        !diagnostics.contains(&Exclusion::NativeStorageNotReproduced),
        !diagnostics.contains(&Exclusion::EngineeringColumnsNotResolved),
    ];
    assert_eq!(actual, expected);
}

fn expected_regions(rows: &[u32], widths: &[u32], active: &[bool]) -> Vec<(u32, u32, bool)> {
    rows.iter()
        .zip(widths)
        .zip(active)
        .map(|((rows, width), active)| (*rows, *width, *active))
        .collect()
}

fn actual_regions(actual: &FlowRegionFieldLayout) -> Vec<(u32, u32, bool)> {
    actual
        .regions()
        .iter()
        .map(|region| {
            (
                region.row_count.value(),
                region.width.value(),
                region.active,
            )
        })
        .collect()
}

fn check_region_checks(actual: FlowFieldValueCoverage, projection: &FieldProjection) {
    assert_eq!(
        actual.expected_region_count_checked,
        matches!(projection, FieldProjection::Regions { .. })
    );
    assert_eq!(
        actual.expected_region_count_checked,
        actual.expected_region_widths_checked
    );
}
