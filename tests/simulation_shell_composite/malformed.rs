use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-shell-composite/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-shell-composite/budgets.bin");

fn check_boundary(
    body: &SimulationShellComposite,
    limits: SimulationShellCompositeLimits,
    valid: bool,
) {
    let parsed = SimulationShellComposite::from_bytes(
        &body.to_bytes(),
        body.version(),
        body.profile(),
        limits,
    );
    let constructed = SimulationShellComposite::from_fields(
        body.fields().to_vec(),
        body.version(),
        body.profile(),
        limits,
    );
    assert_eq!(parsed.is_ok(), valid, "{limits:?}");
    assert_eq!(constructed.is_ok(), valid, "{limits:?}");
    if let Ok(parsed) = parsed {
        assert_eq!(parsed.to_bytes(), body.to_bytes());
    }
    if let Ok(constructed) = constructed {
        assert_eq!(constructed.to_bytes(), body.to_bytes());
    }
}

#[test]
fn all_owned_contexts_obey_independent_exact_and_one_below_seven_budget_axes() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let original = read_original(&mut cursor);
        let mut exact = load_limits::read_reference_limits(&mut cursor, LIMITS);
        exact.maximum_bytes = original.to_bytes().len();
        check_boundary(&original, exact, true);
        for reduced in load_limits::reduced_limits(exact) {
            check_boundary(&original, reduced, false);
        }
        count += 1;
    }
    assert_eq!(count, 270);
}

#[test]
fn all_targeted_mutations_match_actual_reference_outcomes_with_positive_controls() {
    let mut cursor = MALFORMED;
    let mut counts = (0, 0);
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        let (yes, no) = load_limits::check_mutations(&mut cursor, &body.to_bytes(), |raw| {
            SimulationShellComposite::from_bytes(raw, body.version(), body.profile(), LIMITS)
                .is_ok()
        });
        counts.0 += yes;
        counts.1 += no;
    }
    assert_eq!(counts, (2400, 2070));
}
