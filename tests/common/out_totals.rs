use super::{
    pair_reference::{Cursor, owned},
    reference::LIMITS,
};
use bessemer::simulation::*;

fn image() -> SimulationGenArchive {
    let packet = owned();
    let mut cursor = Cursor(&packet);
    cursor.take(8);
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    SimulationGenArchive::from_bytes(cursor.span(), &master).unwrap()
}

fn totals(body: &str, image: &SimulationGenArchive) -> SimulationOutPairedTotals {
    let mut raw = 1_u32.to_le_bytes().to_vec();
    raw.extend_from_slice(body.as_bytes());
    SimulationOutArchive::from_bytes(&raw, LIMITS)
        .unwrap()
        .paired_totals(image)
}

#[test]
fn exact_totals_never_turn_missing_duplicate_or_non_count_values_into_matches() {
    let image = image();
    let same = totals("Nodes(NUMNP)=2\r\nElements(NUME)=2", &image);
    assert_eq!(same.nodes.status, SimulationOutTotalStatus::Matches);
    assert_eq!(same.elements.status, SimulationOutTotalStatus::Matches);
    let missing = totals("Nodes(NUMNP)=3", &image);
    assert_eq!(missing.nodes.status, SimulationOutTotalStatus::Mismatch);
    assert_eq!(missing.nodes.printed, Some(3));
    assert_eq!(missing.elements.status, SimulationOutTotalStatus::Missing);
    let duplicate = totals(
        "Nodes(NUMNP)=2\r\nAgain(NUMNP)=2\r\nElements(NUME)=.5",
        &image,
    );
    assert_eq!(
        duplicate.nodes.status,
        SimulationOutTotalStatus::Inconclusive
    );
    assert_eq!(duplicate.nodes.candidates.len(), 2);
    assert_eq!(
        duplicate.elements.status,
        SimulationOutTotalStatus::Inconclusive
    );
}

#[test]
fn counts_use_exact_decimal_math_without_rounding_or_exponent_overflow() {
    let image = image();
    for token in ["2.000", "+0.2E1", "0002", "20D-1"] {
        let compared = totals(&format!("Nodes(NUMNP)={token}"), &image);
        assert_eq!(
            compared.nodes.status,
            SimulationOutTotalStatus::Matches,
            "{token}"
        );
    }
    for token in [
        "4294967296",
        "2e99",
        "-2",
        "2e-99",
        "2e-9223372036854775808",
        "20001e-4",
    ] {
        let compared = totals(&format!("Nodes(NUMNP)={token}"), &image);
        assert_eq!(
            compared.nodes.status,
            SimulationOutTotalStatus::Inconclusive,
            "{token}"
        );
        assert_eq!(compared.nodes.printed, None);
    }
    let zero = totals("Nodes(NUMNP)=-0e99", &image);
    assert_eq!(zero.nodes.printed, Some(0));
    assert_eq!(zero.nodes.status, SimulationOutTotalStatus::Mismatch);
}
