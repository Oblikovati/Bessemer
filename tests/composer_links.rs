//! Full local equality oracles, lossless lexical edits and immutable identity contracts.
#[path = "common/composer_links.rs"]
mod reference;
use bessemer::composer::links::*;

fn cases() -> Vec<reference::LinkCase<'static>> {
    reference::link_cases(include_bytes!("fixtures/composer-links/links.bin"))
}

#[test]
fn every_owned_complete_index_matches_every_reference_projection_field() {
    let cases = cases();
    assert_eq!(cases.len(), 11);
    for case in cases {
        reference::check_case(&case);
    }
}

#[test]
fn lexical_reference_edit_preserves_bom_comments_spacing_quotes_and_other_attributes() {
    let cases = cases();
    let source = reference::check_case(&cases[4]);
    let id = &source.references()[0].id;
    let edited = source.edit_reference(id, "ABC").unwrap();
    assert_eq!(edited.index.to_bytes(), cases[5].xml);
    assert_eq!(reference::oracle(&edited.index), cases[5].oracle);
    assert_eq!(source.to_bytes(), cases[4].xml);
    assert!(edited.index.reference(id).is_err());
    assert_eq!(edited.history.parent, *id);
    assert_eq!(edited.history.result, edited.index.references()[0].id);
    assert_eq!(
        edited.fidelity,
        ComposerLinkEditFidelity::ExactAttributeText
    );
}

#[test]
fn repeated_and_duplicate_keys_retain_all_candidates_and_group_order() {
    let index = reference::check_case(&cases()[1]);
    assert_eq!(index.references()[0].status, ComposerLinkStatus::Ambiguous);
    assert_eq!(entry_indices(&index.references()[0].candidates), [0, 1]);
    assert_eq!(entry_indices(index.unkeyed_geodes()), [2, 3]);
    assert_eq!(
        index.references()[0].candidates[0].scene_revision(),
        index.scene_revision()
    );
    let index = reference::check_case(&cases()[2]);
    assert_eq!(entry_indices(index.unreferenced_geodes()), [0, 2, 1]);
}

fn entry_indices(geodes: &[ComposerGeodeId]) -> Vec<usize> {
    geodes.iter().map(|id| id.entry_index()).collect()
}

#[test]
fn exact_reference_edits_escape_special_characters_without_normalizing_their_values() {
    let index = reference::check_case(&cases()[4]);
    let value = "a<&\"'\n\t\rΩ";
    let edited = index
        .edit_reference(&index.references()[0].id, value)
        .unwrap();
    assert_eq!(edited.index.references()[0].identifier.as_ref(), value);
    assert_eq!(
        edited.index.references()[0].status,
        ComposerLinkStatus::Unmatched
    );
    assert!(edited.index.to_bytes().starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(
        index
            .edit_reference(&index.references()[0].id, "bad\0value")
            .is_err()
    );
}

#[test]
fn noops_keep_identity_and_foreign_references_cannot_edit_another_index() {
    let index = reference::check_case(&cases()[0]);
    let id = &index.references()[0].id;
    let noop = index
        .edit_reference(id, &index.references()[0].identifier)
        .unwrap();
    assert_eq!(
        noop.completion,
        bessemer::display_lists::DisplayEditCompletion::Unchanged
    );
    assert_eq!(noop.index.references()[0].id, *id);
    assert_eq!(noop.index.to_bytes(), index.to_bytes());
    let other = reference::check_case(&cases()[0]);
    assert!(index.reference(&other.references()[0].id).is_err());
    assert!(
        index
            .edit_reference(&other.references()[0].id, "abc")
            .is_err()
    );
}

#[test]
fn malformed_unsupported_and_externally_declared_xml_is_rejected() {
    let scene = reference::scene(&cases()[0]);
    for bytes in [
        b"".as_slice(),
        b"<x/>",
        b"<DocumentSMG Version='8'/>",
        b"<DocumentSMG Version='7.9'><x></DocumentSMG>",
        b"<!DOCTYPE DocumentSMG><DocumentSMG Version='7.9'/>",
        b"<DocumentSMG Version='7.9'><!-- <!ENTITY bad> --></DocumentSMG>",
        b"<DocumentSMG Version='7.9'>&unknown;</DocumentSMG>",
        b"\xff\xfe<\0x\0/\0>\0",
    ] {
        assert!(ComposerXmlLinks::from_bytes(bytes, &scene, reference::LIMITS).is_err());
    }
}

#[test]
fn all_public_link_index_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<ComposerXmlLinks>();
    assert_traits::<ComposerLinkLimits>();
    assert_traits::<ComposerGeodeId>();
    assert_traits::<ComposerGuidGroup>();
    assert_traits::<ComposerXmlReferenceId>();
    assert_traits::<ComposerXmlReference>();
    assert_traits::<ComposerLinkStatus>();
    assert_traits::<ComposerLinkCheckMode>();
    assert_traits::<ComposerLinkCoverage>();
    assert_traits::<ComposerLinkDiagnostic>();
    assert_traits::<ComposerLinkEditFidelity>();
    assert_traits::<ComposerLinkHistory>();
    assert_traits::<ComposerLinkEdit>();
}

#[test]
fn complete_empty_index_is_distinct_from_malformed_or_incomplete_input() {
    let index = reference::check_case(&cases()[10]);
    assert!(index.references().is_empty() && index.guid_groups().is_empty());
    assert_eq!(index.coverage().geodes, 0);
    assert_eq!(
        index.coverage().mode,
        ComposerLinkCheckMode::ExactLocalEquality
    );
    assert_eq!(
        index.diagnostics(),
        [
            ComposerLinkDiagnostic::RuntimeResolutionNotChecked,
            ComposerLinkDiagnostic::GraphSideEffectsNotChecked,
            ComposerLinkDiagnostic::ProjectSemanticsNotChecked,
            ComposerLinkDiagnostic::NativeApplicationNotChecked
        ]
    );
}

#[test]
fn every_xml_and_index_budget_rejects_before_publishing_a_partial_index() {
    let cases = cases();
    let scene = reference::scene(&cases[0]);
    let mut limits = reference::LIMITS;
    limits.xml.maximum_bytes = cases[0].xml.len() - 1;
    assert!(ComposerXmlLinks::from_bytes(cases[0].xml, &scene, limits).is_err());
    check_limit(&scene, |limits| limits.xml.maximum_nodes = 1);
    check_limit(&scene, |limits| limits.xml.maximum_attributes = 0);
    check_limit(&scene, |limits| limits.xml.maximum_depth = 1);
    check_limit(&scene, |limits| limits.maximum_geodes = 0);
    check_limit(&scene, |limits| limits.maximum_references = 0);
    check_limit(&scene, |limits| limits.maximum_path_units = 0);
}

fn check_limit(
    scene: &bessemer::composer::geometry::ComposerGeometryArchive,
    change: impl FnOnce(&mut ComposerLinkLimits),
) {
    let mut limits = reference::LIMITS;
    change(&mut limits);
    assert!(ComposerXmlLinks::from_bytes(cases()[0].xml, scene, limits).is_err());
}

#[test]
fn owned_xml_and_geometry_snapshots_survive_input_mutation_drops_and_parallel_edits() {
    let cases = cases();
    let scene = reference::scene(&cases[4]);
    let mut xml = cases[4].xml.to_vec();
    let source = ComposerXmlLinks::from_bytes(&xml, &scene, reference::LIMITS).unwrap();
    xml.fill(0);
    drop(scene);
    assert_eq!(source.to_bytes(), cases[4].xml);
    let expected = edited_bytes(&source);
    std::thread::scope(|scope| {
        let handles = (0..6)
            .map(|_| scope.spawn(|| edited_bytes(&source)))
            .collect::<Vec<_>>();
        for handle in handles {
            assert_eq!(handle.join().unwrap(), expected);
        }
    });
    assert_eq!(source.to_bytes(), cases[4].xml);
}

fn edited_bytes(source: &ComposerXmlLinks) -> Vec<u8> {
    source
        .edit_reference(&source.references()[0].id, "ABC")
        .unwrap()
        .index
        .to_bytes()
}
