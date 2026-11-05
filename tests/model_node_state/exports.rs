use super::*;

fn append_bytes(output: &mut Vec<u8>, bytes: &[u8]) {
    output.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    output.extend_from_slice(bytes);
}

fn append_fields(output: &mut Vec<u8>, fields: &[ModelField]) {
    output.extend_from_slice(&(fields.len() as u32).to_le_bytes());
    for field in fields {
        append_bytes(output, field.path.as_bytes());
        append_bytes(output, format!("{:?}", field.value).as_bytes());
    }
}

fn append_view(output: &mut Vec<u8>, node: &ModelNodeStoredState) {
    for word in [
        node.update_state() as u32,
        node.flags_28(),
        node.flags_2c().unwrap(),
        node.update_state_unresolved_bits(),
        node.flags_28_unresolved_bits(),
        node.flags_2c_unresolved_bits().unwrap(),
    ] {
        output.extend_from_slice(&word.to_le_bytes());
    }
    append_bytes(output, node.parameter_class().unwrap_or("").as_bytes());
    for &(flag, _, _, _) in FLAGS {
        output.push(match node.stored_flag(flag) {
            Ok(value) => u8::from(value),
            Err(error) => {
                assert_eq!(error.class, FailureClass::Unsupported);
                2
            }
        });
    }
}

fn append_history(output: &mut Vec<u8>, edit: &PartResolvedFeaturesEdit) {
    output.extend_from_slice(&(edit.unchanged_fields() as u32).to_le_bytes());
    output.extend_from_slice(&(edit.changes().len() as u32).to_le_bytes());
    for change in edit.changes() {
        assert_eq!(change.relation(), ModelFieldRelation::Derived);
        append_bytes(output, change.path().as_bytes());
        output.extend_from_slice(&(change.parents().len() as u32).to_le_bytes());
        for parent in change.parents() {
            output.extend_from_slice(&(parent.index() as u32).to_le_bytes());
        }
        output.extend_from_slice(&(change.result().unwrap().index() as u32).to_le_bytes());
    }
}

fn project_edit(
    output: &mut Vec<u8>,
    archive: &PartResolvedFeaturesArchive,
    case: &NodeCase<'_>,
    ordinal: usize,
    node: &ModelNodeStoredState,
) {
    let flag = if ordinal == 0 {
        ModelNodeStoredFlag::NeedsUpdate
    } else if is_parameter(case) {
        ModelNodeStoredFlag::SharedThickness
    } else {
        ModelNodeStoredFlag::NeedsIdToBeSet
    };
    let (_, relative, mask, _) = FLAGS.iter().find(|option| option.0 == flag).unwrap();
    let enabled = !node.stored_flag(flag).unwrap();
    let edit = archive.edit_node_flag(node, flag, enabled).unwrap();
    let expected = edits::changed_word(case.raw, case.offsets[ordinal] + relative, *mask, enabled);
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    edits::assert_preservation(
        archive,
        &edit,
        node_handle(archive, ordinal).index() + relative / 4,
    );
    append_bytes(output, &expected);
    append_fields(output, edit.archive().fields());
    append_history(output, &edit);
}

fn project_case(archive: &PartResolvedFeaturesArchive, case: &NodeCase<'_>) -> Vec<u8> {
    let mut output = Vec::new();
    append_bytes(&mut output, case.class.as_bytes());
    append_bytes(&mut output, case.raw);
    append_fields(&mut output, archive.fields());
    for ordinal in 0..2 {
        let node = archive.node_state(&node_handle(archive, ordinal)).unwrap();
        assert_projection(&node, case, ordinal);
        append_view(&mut output, &node);
        project_edit(&mut output, archive, case, ordinal, &node);
    }
    output
}

fn parallel_projection(
    archives: &[PartResolvedFeaturesArchive],
    cases: &[NodeCase<'_>],
) -> Vec<u8> {
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|worker| {
                scope.spawn(move || {
                    (worker..cases.len())
                        .step_by(8)
                        .map(|index| (index, project_case(&archives[index], &cases[index])))
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
fn complete_views_fields_and_edit_histories_match_serial_parallel_and_optional_fresh_process_export()
 {
    let (source, manager) = configuration();
    let cases = cases();
    let archives: Vec<_> = cases
        .iter()
        .map(|case| feature_archive(case, &source, &manager))
        .collect();
    let serial: Vec<_> = archives
        .iter()
        .zip(&cases)
        .flat_map(|(archive, case)| project_case(archive, case))
        .collect();
    assert_eq!(parallel_projection(&archives, &cases), serial);
    if let Some(path) = std::env::var_os("BESSEMER_MODEL_NODE_PROJECTION_OUTPUT") {
        std::fs::write(path, &serial).unwrap();
    }
    shared_snapshot_stress(&archives, &cases);
}

fn shared_snapshot_stress(archives: &[PartResolvedFeaturesArchive], cases: &[NodeCase<'_>]) {
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|worker| scope.spawn(move || stress_worker(archives, cases, worker)))
            .collect();
        for job in jobs {
            job.join().unwrap();
        }
    });
}

fn stress_worker(archives: &[PartResolvedFeaturesArchive], cases: &[NodeCase<'_>], worker: usize) {
    for (index, (archive, case)) in archives.iter().zip(cases).enumerate() {
        let option = FLAGS[(index + worker) % FLAGS.len()];
        for ordinal in 0..2 {
            let node = archive.node_state(&node_handle(archive, ordinal)).unwrap();
            if option.3 && !is_parameter(case) {
                assert!(archive.edit_node_flag(&node, option.0, true).is_err());
                continue;
            }
            edits::assert_edit(archive, &node, case, ordinal, option);
        }
    }
}
