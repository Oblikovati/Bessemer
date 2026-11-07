//! Independent solver selection policy preserves physical and header-reader contracts.
use bessemer::{FailureClass, flow::*};

const RAW: &[u8] = include_bytes!("fixtures/flow-solver-selectors/carrier.bin");
const DECISIONS: &str = include_str!("fixtures/flow-solver-selectors/decisions.tsv");

fn source(raw: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(
        raw,
        FlowLimits {
            maximum_wire_bytes: 100_000,
            maximum_records: 2_000,
            maximum_payload_bytes: 100,
        },
    )
    .unwrap()
}

#[test]
fn all_frozen_solver_decisions_match_both_physical_versions() {
    for version in [7_u16, 8] {
        let mut raw = RAW.to_vec();
        raw[..2].copy_from_slice(&version.to_le_bytes());
        let carrier = source(&raw);
        for line in DECISIONS.lines() {
            let (identifier, expected) = line.split_once('\t').unwrap();
            check_decision(&carrier, identifier.parse().unwrap(), expected);
        }
        assert_eq!(carrier.as_bytes(), raw);
    }
}

fn check_decision(carrier: &FlowCarrier, identifier: u32, expected: &str) {
    let selected = carrier.solver_latest(identifier);
    if expected == "reserved" {
        let error = selected.unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
        assert!(error.cause.contains(&format!("{identifier:#x}")));
        assert!(error.cause.contains("expected"));
        return;
    }
    let ordinal: i32 = expected.parse().unwrap();
    if ordinal < 0 {
        assert!(selected.unwrap().is_none());
        return;
    }
    let record = selected.unwrap().unwrap();
    assert_eq!(record.id.ordinal(), ordinal as usize);
    assert_eq!(record.identifier, identifier);
    assert_eq!(record.payload, identifier.to_le_bytes());
}

#[test]
fn controls_stay_physical_and_header_profile_still_accepts_saved_spans() {
    let carrier = source(RAW);
    for identifier in 0xffffff9b..=0xffffff9d {
        assert_eq!(
            carrier
                .records()
                .filter(|r| r.identifier == identifier)
                .count(),
            2
        );
        assert!(carrier.solver_latest(identifier).is_err());
    }
    assert_eq!(
        carrier.latest(0xffffff9c).unwrap().unwrap().payload,
        0xffffff9c_u32.to_le_bytes()
    );
    assert!(carrier.latest(0xffffff9b).is_err());
    assert!(carrier.latest(0xffffff9d).is_err());
    assert_eq!(carrier.as_bytes(), RAW);
}

#[test]
fn empty_carriers_distinguish_absence_from_reserved_errors() {
    let carrier = source(&[8, 0]);
    for identifier in [0, 14, 0xffffff9a, 0xffffff9e, u32::MAX] {
        assert!(carrier.solver_latest(identifier).unwrap().is_none());
    }
    for identifier in 0xffffff9b..=0xffffff9d {
        assert_eq!(
            carrier.solver_latest(identifier).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}

#[test]
fn solver_selection_tracks_new_revisions_without_mutating_old_views() {
    let carrier = source(RAW);
    let before = carrier.solver_latest(14).unwrap().unwrap();
    let edit = carrier.replace_payload(&before.id, b"changed").unwrap();
    let after = edit.result().solver_latest(14).unwrap().unwrap();
    assert_eq!(after.payload, b"changed");
    assert_eq!(after.id.ordinal(), before.id.ordinal());
    assert_ne!(after.id, before.id);
    assert_eq!(before.payload, 14_u32.to_le_bytes());
    assert_eq!(carrier.as_bytes(), RAW);
}
