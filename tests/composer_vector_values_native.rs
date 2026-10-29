//! Every installed vector compared with complete profile-pattern expectations.
#[path = "common/composer_vector_values.rs"]
mod reference;

#[test]
#[ignore = "requires freeze_values.py --export against the installed corpus"]
fn complete_installed_vector_values_match_all_declared_profile_patterns() {
    let raw = std::fs::read("target/composer-vectors-native.bin").unwrap();
    let cases = reference::value_cases(&raw);
    assert_eq!(cases.len(), 4367);
    assert_eq!(
        cases.iter().map(|case| case.wire.len()).sum::<usize>(),
        35263999
    );
    assert_eq!(
        cases.iter().map(|case| case.expected.len()).sum::<usize>(),
        66250232
    );
    let mut reconstructed = Vec::new();
    for case in cases {
        reconstructed.extend(reference::verify_case(&case));
    }
    std::fs::write("target/composer-vectors-reconstructed.bin", reconstructed).unwrap();
}
