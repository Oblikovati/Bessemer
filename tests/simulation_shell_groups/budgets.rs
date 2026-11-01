use super::*;

const EXPECTED_BUDGETS: &[u8] =
    include_bytes!("../fixtures/simulation-shell-manager/group-budgets.bin");
type GroupCapSelector = fn(&mut SimulationShellGroupLimits) -> &mut usize;

fn cap_selectors() -> [GroupCapSelector; 4] {
    [
        |limits| &mut limits.maximum_bytes,
        |limits| &mut limits.maximum_fields,
        |limits| &mut limits.maximum_entries,
        |limits| &mut limits.maximum_text_bytes,
    ]
}

fn read_exact_limits(cursor: &mut &[u8]) -> SimulationShellGroupLimits {
    SimulationShellGroupLimits {
        maximum_bytes: read_word(cursor) as usize,
        maximum_fields: read_word(cursor) as usize,
        maximum_entries: read_word(cursor) as usize,
        maximum_text_bytes: read_word(cursor) as usize,
    }
}

fn check_lowered(
    group: &SimulationShellGroup,
    exact: SimulationShellGroupLimits,
    select: GroupCapSelector,
) {
    let mut limits = exact;
    let value = select(&mut limits);
    if *value == 0 {
        return;
    }
    *value -= 1;
    assert_limited_group_rejected(group, limits);
}

fn assert_limited_group_rejected(group: &SimulationShellGroup, limits: SimulationShellGroupLimits) {
    let decoded = SimulationShellGroup::from_bytes(
        &group.to_bytes(),
        group.version(),
        group.wire_profile(),
        limits,
    );
    let constructed = SimulationShellGroup::from_fields(
        group.fields().to_vec(),
        group.version(),
        group.wire_profile(),
        limits,
    );
    assert!(decoded.is_err() && constructed.is_err());
}

fn check_budget_packet(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        let group = read_group(&mut cursor);
        let exact = read_exact_limits(&mut cursor);
        let accepted = SimulationShellGroup::from_bytes(
            &group.to_bytes(),
            group.version(),
            group.wire_profile(),
            exact,
        )
        .unwrap();
        assert_exact_requirements(&accepted, exact);
        for select in cap_selectors() {
            check_lowered(&group, exact, select);
        }
        count += 1;
    }
    count
}

#[test]
fn every_owned_group_obeys_independent_exact_and_one_below_all_four_aggregate_budgets() {
    assert_eq!(check_budget_packet(EXPECTED_BUDGETS), 72);
}

#[test]
#[ignore = "requires complete independently generated native group resource requirements"]
fn all_native_empty_groups_match_exact_zero_text_and_entry_requirements() {
    let path = std::env::var_os("BESSEMER_SHELL_GROUP_NATIVE_BUDGET_INPUT")
        .expect("set native group resource requirements");
    assert_eq!(check_budget_packet(&std::fs::read(path).unwrap()), 79);
}

fn assert_exact_requirements(group: &SimulationShellGroup, exact: SimulationShellGroupLimits) {
    let constructed = SimulationShellGroup::from_fields(
        group.fields().to_vec(),
        group.version(),
        group.wire_profile(),
        exact,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), group.to_bytes());
    assert_eq!(group.coverage().record_count, exact.maximum_entries);
    assert_eq!(group.coverage().text_bytes, exact.maximum_text_bytes);
    assert_eq!(group.fields().len(), exact.maximum_fields);
    assert_eq!(group.to_bytes().len(), exact.maximum_bytes);
    assert_eq!(group.coverage().limits, exact);
}
