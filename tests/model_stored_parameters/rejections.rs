use super::*;

#[test]
fn every_layout_rejects_foreign_and_stale_snapshots_for_every_edit_selector() {
    for (kind, roots) in fixtures::ORIGINAL.iter().enumerate() {
        for raw in roots {
            assert_revision_rejections(kind, raw);
        }
    }
}

fn assert_revision_rejections(kind: usize, raw: &[u8]) {
    let (_, source) = fixture(raw);
    let parameter = state(&source);
    let (_, foreign) = fixture(raw);
    assert_all_selectors_rejected(&foreign, &parameter, kind);
    let changed = apply_parameter(&source, &parameter, primary_request(kind));
    assert_eq!(changed.completion(), ModelEditCompletion::Changed);
    assert_all_selectors_rejected(changed.archive(), &parameter, kind);
    assert_eq!(source.to_bytes().unwrap(), raw);
    source.resolve_field(parameter.definition()).unwrap();
}

fn assert_all_selectors_rejected(
    source: &PartResolvedFeaturesArchive,
    parameter: &ModelParameterStoredState,
    kind: usize,
) {
    let original = source.to_bytes().unwrap();
    let requests = [
        primary_request(kind),
        ModelParameterStoredEdit::Alternate(edited_value(kind)),
        ModelParameterStoredEdit::AlternatePresence(parameter.alternate_presence().unwrap_or(0)),
    ];
    for request in requests {
        let error = source
            .edit_stored_parameter(parameter, request)
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert_eq!(source.to_bytes().unwrap(), original);
    }
}

#[test]
fn every_layout_rejects_both_incompatible_value_types_without_mutation() {
    for roots in fixtures::ORIGINAL {
        for raw in roots {
            assert_wrong_parameter_types(raw);
        }
    }
}

fn assert_wrong_parameter_types(raw: &[u8]) {
    let (_, source) = fixture(raw);
    let parameter = state(&source);
    for candidate in [edited_value(0), edited_value(3), edited_value(5)] {
        if std::mem::discriminant(&candidate) == std::mem::discriminant(parameter.primary()) {
            continue;
        }
        assert_wrong_value_selectors(&source, &parameter, candidate);
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(state(&source).primary(), parameter.primary());
    source.resolve_field(parameter.definition()).unwrap();
}

fn assert_wrong_value_selectors(
    source: &PartResolvedFeaturesArchive,
    parameter: &ModelParameterStoredState,
    candidate: ModelParameterValue,
) {
    let requests = [
        ModelParameterStoredEdit::Primary(candidate.clone()),
        ModelParameterStoredEdit::Alternate(candidate),
    ];
    for request in requests {
        assert!(source.edit_stored_parameter(parameter, request).is_err());
    }
}
