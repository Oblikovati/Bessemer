use super::*;

fn append_bytes(output: &mut Vec<u8>, raw: &[u8]) {
    output.extend_from_slice(&(raw.len() as u32).to_le_bytes());
    output.extend_from_slice(raw);
}

fn append_fields(output: &mut Vec<u8>, fields: &[ModelField]) {
    output.extend_from_slice(&(fields.len() as u32).to_le_bytes());
    for field in fields {
        append_bytes(output, field.path.as_bytes());
        append_bytes(output, format!("{:?}", field.value).as_bytes());
    }
}

fn append_edit(
    output: &mut Vec<u8>,
    source: &PartResolvedFeaturesArchive,
    view: &ModelDimensionStoredState,
) {
    let requested = ModelDimensionStoredEdit::FilletRefPointChangeNumber(
        7 - view.fillet_ref_point_change_number(),
    );
    let edit = source.edit_dimension_state(view, requested).unwrap();
    append_bytes(output, &edit.archive().to_bytes().unwrap());
    append_fields(output, edit.archive().fields());
    output.extend_from_slice(&(edit.unchanged_fields() as u32).to_le_bytes());
    assert_eq!(edit.changes().len(), 1);
    let relation = &edit.changes()[0];
    assert_eq!(relation.relation(), ModelFieldRelation::Derived);
    append_bytes(output, relation.path().as_bytes());
    output.extend_from_slice(&(relation.parents()[0].index() as u32).to_le_bytes());
    output.extend_from_slice(&(relation.result().unwrap().index() as u32).to_le_bytes());
}

fn complete_projection(archive: &PartResolvedFeaturesArchive, case: &DimensionCase<'_>) -> Vec<u8> {
    let mut output = Vec::new();
    append_bytes(&mut output, case.class.as_bytes());
    append_bytes(&mut output, case.raw);
    append_fields(&mut output, archive.fields());
    for ordinal in 0..2 {
        let view = archive
            .dimension_state(&dimension_handle(archive, ordinal))
            .unwrap();
        let (mask, identifier) = expected_words(case, ordinal);
        assert_view(&view, mask, identifier);
        output.extend_from_slice(&mask.to_le_bytes());
        output.extend_from_slice(&identifier.to_le_bytes());
        output.extend_from_slice(&view.mask_unresolved_bits().to_le_bytes());
        output.push(u8::from(view.enforce_bend_angle_range()));
        output.push(view.fillet_ref_point_change_number());
        append_edit(&mut output, archive, &view);
    }
    output
}

fn parallel_projection(
    archives: &[PartResolvedFeaturesArchive],
    cases: &[DimensionCase<'_>],
) -> Vec<u8> {
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|worker| {
                scope.spawn(move || {
                    (worker..cases.len())
                        .step_by(8)
                        .map(|index| (index, complete_projection(&archives[index], &cases[index])))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        let mut rows: Vec<_> = jobs
            .into_iter()
            .flat_map(|job| job.join().unwrap())
            .collect();
        rows.sort_by_key(|row| row.0);
        rows.into_iter().flat_map(|row| row.1).collect()
    })
}

#[test]
fn complete_views_fields_edits_and_history_match_serial_and_eight_worker_projections() {
    let (configuration, manager) = base_configuration();
    let cases = cases();
    let archives: Vec<_> = cases
        .iter()
        .map(|case| feature_archive(case, &configuration, &manager))
        .collect();
    let serial: Vec<_> = archives
        .iter()
        .zip(&cases)
        .flat_map(|(archive, case)| complete_projection(archive, case))
        .collect();
    assert_eq!(parallel_projection(&archives, &cases), serial);
    if let Some(path) = std::env::var_os("BESSEMER_DIMENSION_STATE_OUTPUT") {
        std::fs::write(path, &serial).unwrap();
    }
}
