use super::*;

fn assert_shared<T: Send + Sync>() {}

#[test]
fn all_public_composite_types_are_send_and_sync() {
    assert_shared::<SimulationShellComposite>();
    assert_shared::<SimulationShellCompositeEdit>();
    assert_shared::<SimulationShellCompositeProfile>();
    assert_shared::<SimulationShellCompositeValue>();
    assert_shared::<SimulationShellCompositeField>();
    assert_shared::<SimulationShellCompositeFieldId>();
    assert_shared::<SimulationShellCompositeHistory>();
    assert_shared::<SimulationShellCompositeHistoryParent>();
    assert_shared::<SimulationShellCompositeNestedHistory>();
    assert_shared::<SimulationShellCompositeNestedParent>();
    assert_shared::<SimulationShellCompositeRelation>();
    assert_shared::<SimulationShellCompositeLimits>();
    assert_shared::<SimulationShellCompositeCoverage>();
    assert_shared::<SimulationShellCompositeCompletion>();
    assert_shared::<SimulationShellCompositeCheckMode>();
    assert_shared::<SimulationShellCompositeDiagnostic>();
}

fn edit_option(body: &SimulationShellComposite, option: i32) -> SimulationShellCompositeEdit {
    let index = usize::from(body.coverage().declared_entries > 0);
    let SimulationShellCompositeValue::Signed(word) = body.fields()[index].value else {
        panic!("expected signed scalar")
    };
    let mask = if body.fields().len() == 1 {
        0
    } else {
        option * 32
    };
    body.edit_field(
        &body.field_id(index).unwrap(),
        SimulationShellCompositeValue::Signed(word ^ mask),
    )
    .unwrap()
}

fn compare_outer(actual: &SimulationShellCompositeEdit, expected: &SimulationShellCompositeEdit) {
    assert_eq!(actual.archive().to_bytes(), expected.archive().to_bytes());
    assert_eq!(actual.archive().fields(), expected.archive().fields());
    assert_eq!(
        actual.archive().field_offsets(),
        expected.archive().field_offsets()
    );
    assert_eq!(actual.parent(), expected.parent());
    assert_eq!(actual.history().len(), expected.history().len());
    for (row, serial) in actual.history().iter().zip(expected.history()) {
        assert_eq!(row.relation, serial.relation);
        assert_eq!(row.parents, serial.parents);
        assert_eq!(row.result.ordinal(), serial.result.ordinal());
        assert_eq!(
            row.result,
            actual.archive().field_id(row.result.ordinal()).unwrap()
        );
    }
}

fn compare_nested(actual: &SimulationShellCompositeEdit, expected: &SimulationShellCompositeEdit) {
    assert_eq!(
        actual.nested_history().len(),
        expected.nested_history().len()
    );
    for (row, serial) in actual
        .nested_history()
        .iter()
        .zip(expected.nested_history())
    {
        assert_eq!(row.path, serial.path);
        assert_eq!(row.relation, serial.relation);
        assert_eq!(row.outer_parent, serial.outer_parent);
        assert_eq!(row.outer_result.ordinal(), serial.outer_result.ordinal());
        assert_eq!(row.parents, serial.parents);
        assert_eq!(
            row.result.as_ref().map(|id| id.index()),
            serial.result.as_ref().map(|id| id.index())
        );
        assert_nested_owner(actual, row);
    }
}

fn assert_nested_owner(
    edit: &SimulationShellCompositeEdit,
    row: &SimulationShellCompositeNestedHistory,
) {
    if let Some(id) = &row.result {
        let SimulationShellCompositeValue::Embedded(archive) =
            &edit.archive().fields()[row.outer_result.ordinal()].value
        else {
            panic!("expected embedded result")
        };
        assert_eq!(*id, archive.field_handle(id.index()).unwrap());
    }
}

fn run_worker(
    bodies: &[SimulationShellComposite],
    serial: &[Vec<SimulationShellCompositeEdit>],
    worker: usize,
) {
    for step in 0..36 {
        let selected = (worker * 36 + step) % bodies.len();
        for (option, expected) in serial[selected].iter().enumerate() {
            let edit = edit_option(&bodies[selected], option as i32);
            compare_outer(&edit, expected);
            compare_nested(&edit, expected);
            assert_eq!(edit.source().to_bytes(), bodies[selected].to_bytes());
        }
    }
}

#[test]
fn eight_workers_cover_every_owned_context_with_three_options_and_all_ordered_parents() {
    let bodies = contracts::owned_composites();
    let originals: Vec<Vec<u8>> = bodies
        .iter()
        .map(SimulationShellComposite::to_bytes)
        .collect();
    let serial: Vec<Vec<SimulationShellCompositeEdit>> = bodies
        .iter()
        .map(|body| (0..3).map(|option| edit_option(body, option)).collect())
        .collect();
    run_workers(&bodies, &serial);
    for (body, original) in bodies.iter().zip(originals) {
        assert_eq!(body.to_bytes(), original);
    }
}

fn run_workers(bodies: &[SimulationShellComposite], serial: &[Vec<SimulationShellCompositeEdit>]) {
    std::thread::scope(|scope| {
        let handles: Vec<std::thread::ScopedJoinHandle<'_, ()>> = (0..8)
            .map(|worker| scope.spawn(move || run_worker(bodies, serial, worker)))
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
    });
}
