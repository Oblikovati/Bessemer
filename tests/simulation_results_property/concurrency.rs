use super::*;

fn result_packet(edit: &SimulationResultsEdit) -> Vec<u8> {
    let body = edit.archive();
    let mut packet = body.to_bytes();
    project_word(&mut packet, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(&mut packet, field, *span);
    }
    packet.extend(format!("{:?}", edit.coverage()).as_bytes());
    packet
}

fn run_body(body: &SimulationResultsProperty) -> (Vec<u8>, usize) {
    let mut packet = Vec::new();
    let mut operations = 0;
    for (index, replacement) in typed_edits::choices(body) {
        let edit = typed_edits::replace(body, index, replacement);
        let selected = result_packet(&edit);
        project_word(&mut packet, selected.len());
        packet.extend(selected);
        operations += 1;
    }
    packet.extend(constructed_noop(body));
    (packet, operations + 2)
}

fn constructed_noop(body: &SimulationResultsProperty) -> Vec<u8> {
    let rebuilt =
        SimulationResultsProperty::from_fields(body.fields().to_vec(), body.context(), LIMITS)
            .unwrap();
    assert_eq!(rebuilt.fields(), body.fields());
    let id = body.field_id(0).unwrap();
    let noop = body
        .edit_field(&id, body.field(&id).unwrap().value.clone())
        .unwrap();
    assert_eq!(noop.result(), id);
    assert_eq!(
        noop.coverage().completion,
        SimulationResultsCompletion::Unchanged
    );
    result_packet(&noop)
}

fn worker_pass(
    bodies: &[SimulationResultsProperty],
    serial: &[(Vec<u8>, usize)],
    worker: usize,
) -> usize {
    let mut operations = 0;
    for step in 0..bodies.len() {
        let index = (step + worker * 97) % bodies.len();
        let actual = run_body(&bodies[index]);
        assert_eq!(actual, serial[index], "worker {worker}, context {index}");
        operations += actual.1;
    }
    operations
}

#[test]
fn mixed_concurrent_reads_construction_noops_and_typed_edits_match_complete_serial_results() {
    let mut bodies = Vec::new();
    contracts::each_owned(|_, _, body| bodies.push(body));
    let serial: Vec<_> = bodies.iter().map(run_body).collect();
    let expected: usize = serial.iter().map(|(_, count)| count).sum();
    let total = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| {
                let (bodies, serial) = (&bodies, &serial);
                scope.spawn(move || worker_pass(bodies, serial, worker))
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .sum::<usize>()
    });
    assert_eq!(total, expected * 8);
    assert_eq!(total, 34840);
    println!("complete mixed concurrent operations: {total}");
}
