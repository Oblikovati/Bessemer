use super::*;

fn scalar_edit(body: &SimulationPostDialog, word: u32) -> SimulationPostDialogEdit {
    body.edit_field(
        &body.field_id(0).unwrap(),
        SimulationPostDialogValue::RawWordBits(word),
    )
    .unwrap()
}

fn assert_serial_result(actual: &SimulationPostDialogEdit, serial: &SimulationPostDialogEdit) {
    assert_eq!(actual.archive().to_bytes(), serial.archive().to_bytes());
    assert_eq!(actual.archive().fields(), serial.archive().fields());
    assert_eq!(
        actual.archive().field_offsets(),
        serial.archive().field_offsets()
    );
    assert_eq!(
        actual.coverage().checked_bytes,
        serial.coverage().checked_bytes
    );
    assert_eq!(
        actual.coverage().declared_entries,
        serial.coverage().declared_entries
    );
    typed_history::assert_serial_outer(actual.history(), serial.history());
    typed_history::assert_serial_nested(actual.nested_history(), serial.nested_history());
}

fn run_worker(bodies: &[SimulationPostDialog], serial: &[Vec<SimulationPostDialogEdit>]) {
    for (body, expected) in bodies.iter().zip(serial) {
        for (word, expected) in [0x12345678, 0xffeeddcc, 0xdeadbeef]
            .into_iter()
            .zip(expected)
        {
            let actual = scalar_edit(body, word);
            assert_serial_result(&actual, expected);
            assert_eq!(body.to_bytes(), expected.source().to_bytes());
        }
    }
}

#[test]
fn eight_workers_replay_all_owned_contexts_against_complete_serial_results() {
    let bodies = owned_dialogs();
    let serial: Vec<Vec<_>> = bodies
        .iter()
        .map(|body| {
            [0x12345678, 0xffeeddcc, 0xdeadbeef]
                .into_iter()
                .map(|word| scalar_edit(body, word))
                .collect()
        })
        .collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (bodies, serial) = (&bodies, &serial);
            scope.spawn(move || run_worker(bodies, serial));
        }
    });
}
