use super::boundaries::owned_body;
use super::edits::{assert_history, assert_preserved, ordinal, replace};
use super::*;

fn replacements() -> Vec<(&'static str, SimulationMaterialValue)> {
    let text = SimulationPersistText::from_units(Some(&[65, 0xd800, 0, 66]), 64).unwrap();
    vec![
        ("word68", SimulationMaterialValue::Signed(42)),
        (
            "property0.scalar",
            SimulationMaterialValue::DoubleBits(0x7ff800000000abcd),
        ),
        (
            "property0.auxiliary_text",
            SimulationMaterialValue::Text(text),
        ),
        (
            "property0.pair_count",
            SimulationMaterialValue::Signed(9999),
        ),
    ]
}

fn check_worker(
    source: &SimulationStudyMaterial,
    options: &[(&str, SimulationMaterialValue)],
    serial: &[SimulationMaterialEdit],
    worker: usize,
) {
    for step in 0..32 {
        let selected = (worker + step) % options.len();
        let (path, value) = &options[selected];
        let edit = replace(source, path, value.clone());
        assert_serial_result(&edit, &serial[selected], source);
        assert_history(&edit, ordinal(source, path), true);
        assert_preserved(&edit, ordinal(source, path));
    }
}

fn assert_serial_result(
    edit: &SimulationMaterialEdit,
    expected: &SimulationMaterialEdit,
    source: &SimulationStudyMaterial,
) {
    assert_eq!(edit.archive().to_bytes(), expected.archive().to_bytes());
    assert_eq!(edit.archive().fields(), expected.archive().fields());
    assert_eq!(edit.source().to_bytes(), source.to_bytes());
    assert_eq!(
        edit.source().coverage().boundary,
        SimulationMaterialBoundary::Prefix
    );
    assert_eq!(edit.parent(), expected.parent());
    assert_eq!(
        edit.coverage().declared_entries,
        expected.coverage().declared_entries
    );
}

fn run_workers(
    source: &SimulationStudyMaterial,
    options: &[(&str, SimulationMaterialValue)],
    serial: &[SimulationMaterialEdit],
) {
    std::thread::scope(|scope| {
        let workers: Vec<std::thread::ScopedJoinHandle<'_, ()>> = (0..8)
            .map(|worker| scope.spawn(move || check_worker(source, options, serial, worker)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn mixed_prefix_origin_edits_match_serial_bytes_fields_coverage_and_all_parents() {
    let original = owned_body(891107, Some(0));
    let mut carrier = original.to_bytes();
    carrier.extend_from_slice(b"untouched following carrier");
    let (source, consumed) =
        SimulationStudyMaterial::from_prefix(&carrier, original.version(), LIMITS).unwrap();
    let options = replacements();
    let serial: Vec<SimulationMaterialEdit> = options
        .iter()
        .map(|(path, value)| replace(&source, path, value.clone()))
        .collect();
    run_workers(&source, &options, &serial);
    assert_eq!(source.to_bytes(), original.to_bytes());
    assert_text_parents(&source, &carrier[..consumed]);
    assert_eq!(&carrier[consumed..], b"untouched following carrier");
}
