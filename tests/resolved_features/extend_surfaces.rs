use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/extend-surfaces/cases.bin"),
        b"EXS1",
        126,
    )
}

#[test]
fn complete_extended_surfaces_data_and_keepers_match_independent_bytes_fields_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn extension_records_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn extension_end_conditions_select_only_their_documented_pointer() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 1) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let expected = match kind % 6 {
            0 | 5 => Some("root.object.distance_28"),
            1 => Some("root.object.reference_20"),
            2 => Some("root.object.reference_18"),
            _ => None,
        };
        let selected = fields
            .iter()
            .filter(|f| {
                matches!(
                    f.path.as_ref(),
                    "root.object.distance_28"
                        | "root.object.reference_20"
                        | "root.object.reference_18"
                )
            })
            .map(|f| f.path.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(selected, expected.into_iter().collect::<Vec<_>>());
    }
}

#[test]
fn arbitrary_negative_extension_modes_do_not_invent_an_extra_pointer() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 9 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.end_condition_0c",
        &i32::MIN.to_le_bytes(),
    );
    let parsed = decode_graph(version, &changed, true).unwrap().0;
    assert_eq!(super::encode_fields(&parsed, 30000).unwrap(), changed);
}

#[test]
fn extension_pointer_roles_reject_an_unrelated_cached_data_root() {
    for (kind, path) in [(6, "distance_28"), (7, "reference_20"), (8, "reference_18")] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{path}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}
