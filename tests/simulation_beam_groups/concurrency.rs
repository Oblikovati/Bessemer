use super::edits::{assert_history, assert_isolated, owned_body, replace, replacements};
use super::*;

struct EditRequest {
    source: SimulationBeamGroupBody,
    path: &'static str,
    value: SimulationBeamGroupValue,
}

impl EditRequest {
    fn apply(&self) -> SimulationBeamGroupEdit {
        replace(&self.source, self.path, self.value.clone())
    }
}

fn requests() -> Vec<EditRequest> {
    let group = owned_body(0);
    let mut requests: Vec<EditRequest> = replacements()
        .into_iter()
        .map(|(path, value)| EditRequest {
            source: group.clone(),
            path,
            value,
        })
        .collect();
    requests.extend(scalar_requests());
    requests
}

fn scalar_requests() -> Vec<EditRequest> {
    let text = SimulationPersistText::from_units(Some(&[65, 0xd800]), 32).unwrap();
    vec![
        EditRequest {
            source: owned_body(1),
            path: "joint.wordf0",
            value: SimulationBeamGroupValue::Signed(123),
        },
        EditRequest {
            source: owned_body(2),
            path: "joint_list.group0.pairs[0]",
            value: SimulationBeamGroupValue::Signed(123),
        },
        EditRequest {
            source: owned_body(3),
            path: "folder.text28",
            value: SimulationBeamGroupValue::Text(text),
        },
    ]
}

fn assert_nested(actual: &SimulationBeamGroupEdit, serial: &SimulationBeamGroupEdit) {
    assert_eq!(actual.nested_history().len(), serial.nested_history().len());
    for (row, expected) in actual.nested_history().iter().zip(serial.nested_history()) {
        assert_eq!(row.path, expected.path);
        assert_eq!(row.relation, expected.relation);
        assert_eq!(row.outer_parent, expected.outer_parent);
        assert_eq!(row.outer_result.ordinal(), expected.outer_result.ordinal());
        assert_eq!(row.parents, expected.parents);
        assert_eq!(
            row.result.as_ref().map(|id| id.index()),
            expected.result.as_ref().map(|id| id.index())
        );
        if let Some(id) = &row.result {
            let SimulationBeamGroupValue::Embedded(archive) =
                &actual.archive().fields()[row.outer_result.ordinal()].value
            else {
                panic!("expected complete embedded result");
            };
            assert_eq!(*id, archive.field_handle(id.index()).unwrap());
        }
    }
}

fn assert_coverage(actual: SimulationBeamGroupCoverage, serial: SimulationBeamGroupCoverage) {
    assert_eq!(actual.kind, serial.kind);
    assert_eq!(actual.version, serial.version);
    assert_eq!(actual.completion, serial.completion);
    assert_eq!(actual.check_mode, serial.check_mode);
    assert_eq!(actual.checked_bytes, serial.checked_bytes);
    assert_eq!(actual.checked_fields, serial.checked_fields);
    assert_eq!(actual.embedded_archives, serial.embedded_archives);
    assert_eq!(actual.embedded_fields, serial.embedded_fields);
    assert_eq!(actual.diagnostics, serial.diagnostics);
    assert_eq!(
        actual.requested_pattern_error_bits,
        serial.requested_pattern_error_bits
    );
}

fn assert_serial(actual: &SimulationBeamGroupEdit, serial: &SimulationBeamGroupEdit) {
    assert_eq!(actual.archive().to_bytes(), serial.archive().to_bytes());
    assert_eq!(actual.archive().fields(), serial.archive().fields());
    assert_eq!(actual.source().to_bytes(), serial.source().to_bytes());
    assert_eq!(actual.parent(), serial.parent());
    assert_eq!(
        actual.result(),
        actual
            .archive()
            .field_id(actual.result().ordinal())
            .unwrap()
    );
    assert_coverage(actual.coverage(), serial.coverage());
    assert_nested(actual, serial);
    assert_history(actual);
    assert_isolated(actual);
}

fn run_worker(requests: &[EditRequest], serial: &[SimulationBeamGroupEdit], worker: usize) {
    for step in 0..32 {
        let selected = (worker + step) % requests.len();
        assert_serial(&requests[selected].apply(), &serial[selected]);
    }
}

fn run_workers(requests: &[EditRequest], serial: &[SimulationBeamGroupEdit]) {
    std::thread::scope(|scope| {
        let workers: Vec<std::thread::ScopedJoinHandle<'_, ()>> = (0..8)
            .map(|worker| scope.spawn(move || run_worker(requests, serial, worker)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn all_four_body_kinds_and_five_value_kinds_match_serial_under_mixed_concurrent_edits() {
    let requests = requests();
    let originals: Vec<Vec<u8>> = requests
        .iter()
        .map(|request| request.source.to_bytes())
        .collect();
    let serial: Vec<SimulationBeamGroupEdit> = requests.iter().map(EditRequest::apply).collect();
    run_workers(&requests, &serial);
    for (request, original) in requests.iter().zip(originals) {
        assert_eq!(request.source.to_bytes(), original);
    }
}
