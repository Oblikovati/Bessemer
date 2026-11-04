use super::*;

fn assert_all_prefixes(body: &SimulationPostDialog) -> usize {
    let original = body.to_bytes();
    for end in 0..original.len() {
        assert!(
            SimulationPostDialog::from_bytes(&original[..end], body.version(), LIMITS).is_err(),
            "{:?} prefix {end}/{}",
            body.version(),
            original.len()
        );
    }
    assert_eq!(body.to_bytes(), original);
    original.len()
}

#[test]
fn every_proper_parent_prefix_is_incomplete_and_every_appended_remainder_fails() {
    let mut prefixes = 0;
    let mut remainders = 0;
    for body in owned_dialogs() {
        prefixes += assert_all_prefixes(&body);
        for suffix in [&[0][..], &[255; 4][..], &[0x55; 16][..]] {
            let extended = [body.to_bytes().as_slice(), suffix].concat();
            assert!(SimulationPostDialog::from_bytes(&extended, body.version(), LIMITS).is_err());
            remainders += 1;
        }
    }
    assert_eq!(prefixes, 1355987);
    assert_eq!(remainders, 675);
}

#[test]
fn the_modern_parent_profile_does_not_infer_an_older_helper_context() {
    let body = owned_dialogs().remove(0);
    let unsupported = SimulationPersistVersion::new(700334).unwrap();
    assert!(SimulationPostDialog::from_bytes(&body.to_bytes(), unsupported, LIMITS).is_err());
    assert!(
        SimulationPostDialog::from_fields(body.fields().to_vec(), unsupported, LIMITS).is_err()
    );
}
