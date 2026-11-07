use super::*;

#[test]
#[ignore = "Explicit complete 11,904-cell native serialized edit/reversal census"]
fn every_native_signed_cell_edit_and_logical_reverse_matches_complete_expected_bytes() {
    let case = fixture::cases().remove(0);
    let arrays = fixture::view(&case);
    let mut edits = 0;
    for group in [FlowArrayGroup::First, FlowArrayGroup::Second] {
        let values = group_values(&arrays, group).unwrap();
        for (index, &before) in values.iter().enumerate() {
            assert_cell_edit(&arrays, group, index, before.wrapping_add(0x1020304));
            edits += 1;
        }
    }
    assert_eq!(edits, 11904);
    assert_eq!(arrays.source().as_bytes(), case.raw);
}
