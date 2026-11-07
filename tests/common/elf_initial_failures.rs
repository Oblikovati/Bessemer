use bessemer::simulation::*;

fn initial() -> SimulationElfInitialArchive {
    SimulationElfInitialArchive::from_parameters(SimulationElfInitialParameters {
        parameter_a: 1,
        parameter_b: 2,
    })
}

#[test]
fn initial_profile_rejects_every_short_prefix_extra_bytes_wrong_words_and_nonzero_continuation() {
    let raw = initial().to_bytes();
    for length in 0..raw.len() {
        assert!(SimulationElfInitialArchive::from_bytes(&raw[..length]).is_err());
    }
    let mut extended = raw.clone();
    extended.push(0);
    assert!(SimulationElfInitialArchive::from_bytes(&extended).is_err());
    for offset in [0, 4, 8, 12, 20, 28, 32, 244, 1048, 1223] {
        let mut changed = raw.clone();
        changed[offset] ^= 1;
        assert!(
            SimulationElfInitialArchive::from_bytes(&changed).is_err(),
            "offset {offset}"
        );
    }
}

#[test]
fn foreign_parameter_ids_and_source_mutation_leave_initializers_isolated() {
    let original = initial();
    let foreign = initial();
    assert!(
        original
            .edit_parameter(&foreign.parameter_id(SimulationElfParameter::A), 7)
            .is_err()
    );
    let mut raw = original.to_bytes();
    let copied = SimulationElfInitialArchive::from_bytes(&raw).unwrap();
    raw.fill(0);
    assert_eq!(copied.to_bytes(), original.to_bytes());
}

#[test]
fn all_initial_image_results_are_send_sync_and_parallel_edits_are_deterministic() {
    fn check<T: Send + Sync>() {}
    check::<SimulationElfInitialParameters>();
    check::<SimulationElfInitialRevision>();
    check::<SimulationElfParameter>();
    check::<SimulationElfParameterId>();
    check::<SimulationElfInitialArchive>();
    check::<SimulationElfCheckMode>();
    check::<SimulationElfInitialCoverage>();
    check::<SimulationElfDiagnostic>();
    check::<SimulationElfInitialFidelity>();
    check::<SimulationElfInitialEdit>();
    check::<SimulationElfInitialStudy>();
    check::<SimulationElfInitialStudyEdit>();
    let original = initial();
    std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edit_revert(&original)))
            .collect();
        for task in tasks {
            task.join().unwrap();
        }
    });
}

fn edit_revert(original: &SimulationElfInitialArchive) {
    let id = original.parameter_id(SimulationElfParameter::B);
    for _ in 0..32 {
        let edit = original.edit_parameter(&id, 0xffffffff).unwrap();
        let reverted = edit.archive.edit_parameter(&edit.result, 2).unwrap();
        assert_eq!(reverted.archive.to_bytes(), original.to_bytes());
        assert_eq!(original.parameter(&id).unwrap(), 2);
    }
}
