//! Logical index oracles preserve recognition/fallback distinctions and explicit incomplete-document qualifiers.
#[path = "flow_index_replay/api.rs"]
mod api;
#[path = "flow_index_replay/fixture.rs"]
mod fixture;
#[path = "flow_index_replay/oracles.rs"]
mod oracles;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/process.rs"]
mod process;
#[path = "flow_index_replay/runtime.rs"]
mod runtime;
use bessemer::flow::*;
use fixture::*;
use std::collections::BTreeMap;
const INDEX: u32 = 0xffffff9b;
const SAVED: u32 = 0xffffff9c;
const DELETED: u32 = 0xffffff9d;

#[test]
fn all_native_and_owned_physical_spans_and_completed_maps_match_independent_replay_equations() {
    let rows = cases();
    assert_eq!(rows.len(), 1070);
    for row in rows {
        oracles::carrier(&row);
        for (forced, outcome) in [false, true].into_iter().zip(&row.outcomes) {
            check_outcome(&row, forced, outcome);
        }
    }
}
fn check_outcome(row: &ReplayCase, forced: bool, outcome: &ReplayOutcome) {
    let projection = match outcome {
        ReplayOutcome::Completed(projection) => projection,
        ReplayOutcome::Rejected {
            forced: stored,
            cause,
        } => {
            assert_eq!(*stored, forced);
            assert!(!forced && !cause.is_empty(), "{}", row.name);
            return;
        }
    };
    check_completed(row, forced, projection);
}
fn check_completed(row: &ReplayCase, forced: bool, projection: &ReplayProjection) {
    assert_eq!(projection.forced, forced);
    assert_eq!(projection.version, row.version);
    assert_eq!(projection.physical_count as usize, row.physical.len());
    assert_eq!(projection.qualifications, [false; 3]);
    if !projection.terminal_adopted {
        assert!(projection.terminal.is_none());
        let (active, saved) = oracles::physical_maps(row);
        oracles::assert_maps(projection, &active, &saved);
        return;
    }
    assert!(!forced);
    let index = oracles::terminal_projection(row, projection.terminal.as_ref().unwrap());
    let (active, saved) = oracles::terminal_maps(row, &index);
    oracles::assert_maps(projection, &active, &saved);
}
#[test]
fn native_carriers_use_scan_only_and_keep_all_66_active_records_without_saved_entries() {
    let rows = cases();
    let native: Vec<_> = rows.iter().filter(|row| row.family == "native").collect();
    assert_eq!(native.len(), 3);
    assert_eq!(
        native.iter().map(|row| row.raw.len()).sum::<usize>(),
        2936303
    );
    let mut active = 0;
    for row in native {
        for outcome in &row.outcomes {
            let ReplayOutcome::Completed(projection) = outcome else {
                panic!("native scan must complete");
            };
            assert!(!projection.terminal_adopted && projection.terminal.is_none());
            assert!(projection.saved.is_empty());
            active += projection.active.len();
        }
    }
    assert_eq!(active, 132);
}
#[test]
fn recognized_bad_controls_fail_preference_while_forced_scan_keeps_their_opaque_physical_records() {
    for row in cases() {
        if matched_failure(&row) {
            check_rejected_preference(&row);
        }
        if row.name.starts_with("fallback-") {
            check_scanned_pair(&row);
        }
    }
}
fn matched_failure(row: &ReplayCase) -> bool {
    row.name.starts_with("matched-")
        || row.name.starts_with("saved-mismatch")
        || row.name == "saved-missing"
        || row.name == "saved-malformed-selected"
}
fn check_rejected_preference(row: &ReplayCase) {
    let failed = matches!(
        row.outcomes[0],
        ReplayOutcome::Rejected { forced: false, .. }
    );
    assert!(failed, "{}", row.name);
    assert!(
        matches!(row.outcomes[1], ReplayOutcome::Completed(_)),
        "{}",
        row.name
    );
}
fn check_scanned_pair(row: &ReplayCase) {
    let preferred = completed(&row.outcomes[0]);
    let forced = completed(&row.outcomes[1]);
    assert!(!preferred.terminal_adopted && !forced.terminal_adopted);
    assert_eq!(preferred.active, forced.active);
    assert_eq!(preferred.saved, forced.saved);
}

#[test]
fn duplicates_deleted_spans_terminal_self_override_and_saved_reference_order_are_preserved() {
    let rows = cases();
    let row = rows
        .iter()
        .find(|row| row.name == "terminal-8-duplicates-self-reserved")
        .unwrap();
    check_terminal_self(row);
    check_saved_selections(&rows);
    let row = rows
        .iter()
        .find(|row| row.name == "scan-8-deleted-3")
        .unwrap();
    let projection = completed(&row.outcomes[0]);
    assert!(projection.active.is_empty());
    assert_eq!(projection.saved, [(2, 13), (15, 13), (28, 13)]);
}
fn check_terminal_self(row: &ReplayCase) {
    let preferred = completed(&row.outcomes[0]);
    let active: BTreeMap<_, _> = preferred
        .active
        .iter()
        .map(|span| (span.identifier, *span))
        .collect();
    assert_eq!(active[&999].start, 901);
    assert_eq!(&active[&INDEX], row.physical.last().unwrap());
    assert!(active.contains_key(&DELETED));
}
fn check_saved_selections(rows: &[ReplayCase]) {
    for (name, expected) in [
        ("saved-select-earlier", 1),
        ("saved-select-later", 2),
        ("saved-directive-last-earlier", 1),
        ("saved-directive-last-later", 2),
    ] {
        let row = rows.iter().find(|row| row.name == name).unwrap();
        assert_eq!(completed(&row.outcomes[0]).saved, [(7, expected)]);
    }
    let row = rows
        .iter()
        .find(|row| row.name == "saved-duplicate-last-wins")
        .unwrap();
    assert_eq!(completed(&row.outcomes[0]).saved, [(0, 1), (100, 4)]);
}
fn completed(outcome: &ReplayOutcome) -> &ReplayProjection {
    let ReplayOutcome::Completed(projection) = outcome else {
        panic!("expected completed oracle projection");
    };
    projection
}
