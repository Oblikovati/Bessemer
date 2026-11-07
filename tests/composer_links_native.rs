//! Complete installed local XML/geode equality index, independent of runtime resolution.
#[path = "common/composer_links.rs"]
mod reference;
use bessemer::composer::links::ComposerLinkStatus;

#[test]
#[ignore = "requires freeze_links.py --export against the installed corpus"]
fn all_installed_project_indexes_match_every_complete_reference_projection() {
    let raw = std::fs::read("target/composer-links-native.bin").unwrap();
    let cases = reference::link_cases(&raw);
    assert_eq!(cases.len(), 38);
    let mut geodes = 0;
    let mut unique = 0;
    let mut unmatched = 0;
    let mut edits = b"LXED".to_vec();
    edits.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for case in cases {
        let index = reference::check_case(&case);
        geodes += index.coverage().geodes;
        unique += status_count(&index, ComposerLinkStatus::Unique);
        unmatched += status_count(&index, ComposerLinkStatus::Unmatched);
        assert!(index.unkeyed_geodes().is_empty() && index.unreferenced_geodes().is_empty());
        append_native_edit(&mut edits, &case, &index);
    }
    assert_eq!((geodes, unique, unmatched), (1787, 4416, 29));
    std::fs::write("target/composer-links-native-edits.bin", edits).unwrap();
}

fn status_count(
    index: &bessemer::composer::links::ComposerXmlLinks,
    status: ComposerLinkStatus,
) -> usize {
    index
        .references()
        .iter()
        .filter(|reference| reference.status == status)
        .count()
}

fn append_native_edit(
    raw: &mut Vec<u8>,
    case: &reference::LinkCase<'_>,
    source: &bessemer::composer::links::ComposerXmlLinks,
) {
    let edit = source
        .edit_reference(&source.references()[0].id, "Bessemer unmatched reference Ω")
        .unwrap();
    let edited = edit.index.to_bytes();
    let oracle = reference::oracle(&edit.index);
    for size in [case.scene.len(), case.xml.len(), edited.len(), oracle.len()] {
        raw.extend_from_slice(&u32::try_from(size).unwrap().to_le_bytes());
    }
    raw.extend_from_slice(case.scene);
    raw.extend_from_slice(case.xml);
    raw.extend_from_slice(&edited);
    raw.extend_from_slice(&oracle);
    assert_eq!(source.to_bytes(), case.xml);
}
