use super::*;

#[test]
fn dimension_types_are_send_sync_and_retain_inputs_after_caller_buffer_mutation() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ModelDimensionStoredState>();
    send_sync::<ModelDimensionStoredEdit>();
    let (configuration, manager) = base_configuration();
    let case = cases().remove(0);
    let mut bytes = case.raw.to_vec();
    let source =
        PartResolvedFeaturesArchive::from_bytes(&bytes, configuration, &manager, LIMITS).unwrap();
    let view = source
        .dimension_state(&dimension_handle(&source, 0))
        .unwrap();
    bytes.fill(0xa5);
    assert_view(&view, case.mask, case.identifier);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

#[test]
fn foreign_stale_wrong_selection_and_every_invalid_counter_fail_without_mutation() {
    let (configuration, manager) = base_configuration();
    let case = cases().remove(0);
    let source = feature_archive(&case, &configuration, &manager);
    let foreign = feature_archive(&case, &configuration, &manager);
    let handle = dimension_handle(&source, 0);
    let view = source.dimension_state(&handle).unwrap();
    assert_foreign(&foreign, &view, &handle);
    assert!(
        source
            .dimension_state(&source.field_handle(0).unwrap())
            .is_err()
    );
    assert!(source.field_handle(usize::MAX).is_err());
    assert_counter_failures(&source, &view);
    assert_stale(&source, &view, &handle);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn assert_counter_failures(source: &PartResolvedFeaturesArchive, view: &ModelDimensionStoredState) {
    for value in 8..=255 {
        let error = source
            .edit_dimension_state(
                view,
                ModelDimensionStoredEdit::FilletRefPointChangeNumber(value),
            )
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains(&value.to_string()));
        assert!(error.cause.contains("0..=7"));
    }
}

fn assert_foreign(
    source: &PartResolvedFeaturesArchive,
    view: &ModelDimensionStoredState,
    handle: &ModelFieldHandle,
) {
    assert!(source.dimension_state(handle).is_err());
    assert!(
        source
            .edit_dimension_state(view, ModelDimensionStoredEdit::RedundantId(0))
            .is_err()
    );
}

fn assert_stale(
    source: &PartResolvedFeaturesArchive,
    view: &ModelDimensionStoredState,
    handle: &ModelFieldHandle,
) {
    let changed = source
        .edit_dimension_state(view, ModelDimensionStoredEdit::EnforceBendAngleRange(true))
        .unwrap();
    assert!(changed.archive().dimension_state(handle).is_err());
    assert!(
        changed
            .archive()
            .edit_dimension_state(view, ModelDimensionStoredEdit::RedundantId(1))
            .is_err()
    );
}

fn shared_worker(
    archives: &[PartResolvedFeaturesArchive],
    cases: &[DimensionCase<'_>],
    worker: usize,
) -> [usize; 2] {
    let request = shared_request(worker);
    for (source, case) in archives.iter().zip(cases) {
        let ordinal = worker % 2;
        edits::assert_request(source, case, ordinal, request);
        let view = source
            .dimension_state(&dimension_handle(source, ordinal))
            .unwrap();
        let failure = ModelDimensionStoredEdit::FilletRefPointChangeNumber(8 + worker as u8);
        assert_eq!(
            source
                .edit_dimension_state(&view, failure)
                .unwrap_err()
                .class,
            FailureClass::Invalid
        );
        assert_eq!(source.to_bytes().unwrap(), case.raw);
    }
    [cases.len(), cases.len()]
}

fn shared_request(worker: usize) -> ModelDimensionStoredEdit {
    if worker.is_multiple_of(3) {
        return ModelDimensionStoredEdit::RedundantId(worker as i32);
    }
    ModelDimensionStoredEdit::FilletRefPointChangeNumber(worker as u8)
}

#[test]
fn shared_immutable_snapshots_isolate_eight_workers_successful_edits_and_expected_failures() {
    let (configuration, manager) = base_configuration();
    let cases = cases();
    let archives: Vec<_> = cases
        .iter()
        .map(|case| feature_archive(case, &configuration, &manager))
        .collect();
    let outcomes = std::thread::scope(|scope| {
        let shared = (&archives, &cases);
        let jobs: Vec<_> = (0..8)
            .map(|worker| scope.spawn(move || shared_worker(shared.0, shared.1, worker)))
            .collect();
        jobs.into_iter()
            .map(|job| job.join().unwrap())
            .fold([0; 2], |sum, row| [sum[0] + row[0], sum[1] + row[1]])
    });
    assert_eq!(outcomes, [43520, 43520]);
}
