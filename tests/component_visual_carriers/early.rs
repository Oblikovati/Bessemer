use super::*;

#[test]
fn early_component_visuals_preserve_real_custom_width_presence_and_all_stored_bits() {
    let cases = complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/earliest-visual-properties.bin"
    ));
    assert_eq!(cases.len(), 220);
    assert_eq!(
        cases.iter().filter(|case| case.version < 0xf41).count(),
        132
    );
    assert_eq!(cases.iter().filter(|case| case.version == 3904).count(), 4);
    assert_eq!(cases.iter().filter(|case| case.version == 3905).count(), 4);
    assert_eq!(
        cases
            .into_iter()
            .map(verify_component_baseline)
            .sum::<usize>(),
        220
    );
}
