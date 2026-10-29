//! Complete installed packed-array reconstruction and edit corpus.
#[path = "common/composer_arrays.rs"]
mod reference;

#[test]
#[ignore = "requires freeze_arrays.py --export against the installed corpus"]
fn all_installed_array_blocks_reconstruct_construct_and_edit_exactly() {
    let packet = std::fs::read("target/composer-arrays-native.bin").unwrap();
    let cases = reference::read_cases(&packet);
    assert_eq!(cases.len(), 8196);
    assert_eq!(
        cases.iter().filter(|case| case.dimensions != 0).count(),
        4367
    );
    assert_eq!(
        cases.iter().map(|case| case.wire.len()).sum::<usize>(),
        44629425
    );
    let mut edited = Vec::new();
    for case in &cases {
        edited.extend(reference::check_case(case));
    }
    std::fs::write("target/composer-arrays-native-edited.bin", edited).unwrap();
}
