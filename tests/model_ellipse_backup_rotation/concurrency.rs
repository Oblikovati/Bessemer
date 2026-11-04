use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use bessemer::BessemerError;

#[derive(Debug, PartialEq, Eq)]
struct EllipseOutcome {
    changed: ModelVisualOperation,
    unchanged: ModelVisualOperation,
    foreign: BessemerError,
    stale: BessemerError,
}

fn ellipse_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    case: &EllipseCase,
    owner: usize,
) -> EllipseOutcome {
    let states = source.ellipse_backup_rotation_states().unwrap();
    let state = &states[owner];
    let (changed, unchanged) = ellipse_successful_outcomes(source, case, owner, state);
    let (rejected, stale) = ellipse_failed_outcomes(foreign, &changed, state);
    EllipseOutcome {
        changed: model_visual_operation(source, &changed),
        unchanged: model_visual_operation(source, &unchanged),
        foreign: rejected,
        stale,
    }
}

fn ellipse_successful_outcomes(
    source: &PartResolvedFeaturesArchive,
    case: &EllipseCase,
    owner: usize,
    state: &ModelEllipseBackupRotationState,
) -> (PartResolvedFeaturesEdit, PartResolvedFeaturesEdit) {
    let original = state.rotation_direction_snapshot();
    let changed = source
        .edit_ellipse_backup_rotation(state, !original)
        .unwrap();
    let unchanged = source
        .edit_ellipse_backup_rotation(state, original)
        .unwrap();
    assert_eq!(
        changed.archive().to_bytes().unwrap(),
        ellipse_expected_bytes(case, owner, !original)
    );
    contracts::verify_ellipse_result(source, state, &changed, !original);
    contracts::verify_ellipse_result(source, state, &unchanged, original);
    (changed, unchanged)
}

fn ellipse_failed_outcomes(
    foreign: &PartResolvedFeaturesArchive,
    changed: &PartResolvedFeaturesEdit,
    state: &ModelEllipseBackupRotationState,
) -> (BessemerError, BessemerError) {
    let parent = state.owner_field();
    let stale_archive = changed.archive();
    let rejected = foreign.edit_ellipse_backup_rotation(state, 0).unwrap_err();
    let stale = stale_archive
        .edit_ellipse_backup_rotation(state, 0)
        .unwrap_err();
    assert_eq!(rejected, foreign.resolve_field(parent).unwrap_err());
    assert_eq!(stale, stale_archive.resolve_field(parent).unwrap_err());
    (rejected, stale)
}

fn ellipse_serial_outcomes(
    cases: &[EllipseCase],
    sources: &[PartResolvedFeaturesArchive],
    foreign: &[PartResolvedFeaturesArchive],
) -> Vec<EllipseOutcome> {
    let mut outcomes = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        for owner in 0..2 {
            outcomes.push(ellipse_outcome(
                &sources[index],
                &foreign[index],
                case,
                owner,
            ));
        }
        assert_eq!(sources[index].to_bytes().unwrap(), case.raw);
        assert_eq!(foreign[index].to_bytes().unwrap(), case.raw);
    }
    outcomes
}

#[test]
fn eight_workers_share_every_current_snapshot_and_match_complete_serial_operations() {
    let cases: Vec<_> = ellipse_cases()
        .into_iter()
        .filter(|case| case.version == 19000)
        .collect();
    let sources: Vec<_> = cases.iter().map(current_ellipse_archive).collect();
    let foreign: Vec<_> = cases.iter().map(current_ellipse_archive).collect();
    let expected = ellipse_serial_outcomes(&cases, &sources, &foreign);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| ellipse_serial_outcomes(&cases, &sources, &foreign)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(expected.len(), 72);
}
