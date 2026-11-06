use super::{api, fixture, packet::*, rebuild, types::*};
#[path = "profile/fixture.rs"]
mod profile_fixture;
#[path = "profile/rebuild.rs"]
mod profile_rebuild;
#[path = "profile/types.rs"]
mod profile_types;
use profile_types::*;
#[test]
fn every_complete_fresh_kind_one_profile_rebuilds_all_nested_bodies_keys_references_and_exact_eof()
{
    let cases = profile_fixture::cases();
    assert_eq!(cases.len(), 1163);
    for case in cases {
        assert_eq!(profile_rebuild::profile(&case), case.raw, "{}", case.name);
    }
}
#[test]
fn existing_typed_metadata_and_first_body_apis_match_complete_profile_boundaries() {
    for case in profile_fixture::cases() {
        check_first_body(case);
    }
}
fn check_first_body(mut case: ProfileCase) {
    let source = api::carrier(&case.raw);
    let id = source.latest(424).unwrap().unwrap().id;
    let prefix = source
        .record_424_metadata_prefix(&id, api::prefix_options())
        .unwrap();
    assert_eq!(
        prefix.coverage().known_prefix_bytes,
        case.backend_start as usize
    );
    let backend = prefix.medium_backend_prefix().unwrap();
    if case.first.is_empty() {
        return;
    }
    let body = backend.first_kind_one_body(api::options()).unwrap();
    let (entry_start, entry_end, expected) = expected_first(&mut case);
    assert_eq!(body.payload_interval(), (entry_start, entry_end));
    api::read::check_body(&body, &expected);
}
#[test]
fn native_complete_profile_keeps_one_object_in_each_table_and_its_signed_outer_attributes() {
    let case = profile_fixture::cases().remove(0);
    assert_eq!(
        (
            case.raw.len(),
            case.backend_start,
            case.backend_end,
            case.footer.end
        ),
        (13191, 112, 13159, 13191)
    );
    assert_eq!(
        (case.first.len(), case.second.len(), case.footer.keys.len()),
        (1, 1, 1)
    );
    check_native_references(&case);
}

fn expected_first(case: &mut ProfileCase) -> (usize, usize, BodyCase) {
    let entry = case.first.remove(0);
    let mut expected = entry.body;
    expected.footer.tail = case.raw[entry.end as usize..].to_vec();
    expected.raw.extend_from_slice(&expected.footer.tail);
    (entry.start as usize + 20, entry.end as usize, expected)
}

fn check_native_references(case: &ProfileCase) {
    assert_eq!(case.second[0].references, [case.first[0].key.clone()]);
    assert_eq!(case.footer.keys, [case.second[0].key.clone()]);
    assert_eq!(case.footer.final_counts, [0, 0]);
    assert!(case.footer.complete);
    assert!(case.footer.fresh);
    assert!(!case.footer.engineering);
    assert!(!case.footer.runtime);
}
