use super::*;

#[test]
#[ignore = "Explicit full two-carrier 24,336-field native edit and logical reversal census"]
fn every_native_group_word_wide_word_and_flag_matches_complete_expected_edits() {
    let mut changed = 0;
    for case in fixture::cases().into_iter().take(2) {
        let record = view(&case);
        for (group, expected) in case.groups.iter().enumerate() {
            for request in checks::requests(expected).into_iter().skip(1).step_by(2) {
                checks::assert_edit(&record, &case, group, request);
                changed += 1;
            }
        }
        assert_eq!(record.source().as_bytes(), case.wire);
    }
    assert_eq!(changed, 24336);
}
