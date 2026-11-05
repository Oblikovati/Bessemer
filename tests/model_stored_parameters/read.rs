use super::*;

#[test]
fn all_six_classes_preserve_both_names_literal_values_and_complete_read_contract() {
    for (kind, roots) in fixtures::ORIGINAL.iter().enumerate() {
        for (style, raw) in roots.iter().enumerate() {
            let (_, source) = fixture(raw);
            let parameter = state(&source);
            assert_eq!(parameter.kind(), KINDS[kind]);
            assert_names(&parameter, style);
            assert_values(&parameter, kind, style);
            assert_read_contract(&source, &parameter);
        }
    }
}

fn assert_names(parameter: &ModelParameterStoredState, style: usize) {
    let node = parameter.node_name();
    let named = parameter.parameter_name();
    let expected_node = if style == 0 { None } else { Some("node Ω") };
    let expected_parameter = match style {
        1 => Some("node Ω"),
        3 => None,
        _ => Some("parameter λ"),
    };
    assert_eq!(node.text().map(|text| text.text()), expected_node);
    assert_eq!(named.text().map(|text| text.text()), expected_parameter);
    assert_ne!(node.pointer(), named.pointer());
    assert_eq!(node.text_field() == named.text_field(), style == 1);
    assert_eq!(
        matches!(node.tag().record, ModelObjectRecord::Null),
        style == 0
    );
    assert_eq!(
        matches!(named.tag().record, ModelObjectRecord::Null),
        style == 3
    );
}

fn assert_values(parameter: &ModelParameterStoredState, kind: usize, style: usize) {
    assert_eq!(parameter.primary(), &expected_primary(kind, style));
    if kind == 5 {
        assert_eq!(parameter.alternate_presence(), None);
        assert_eq!(parameter.alternate(), None);
        return;
    }
    assert_eq!(
        parameter.alternate_presence(),
        Some([0, 1, 255, 128][style])
    );
    let alternate = expected_alternate(kind, style);
    assert_eq!(parameter.alternate(), alternate.as_ref());
}

fn expected_primary(kind: usize, style: usize) -> ModelParameterValue {
    match kind {
        3 => ModelParameterValue::Integer([i32::MIN, -1, 0, i32::MAX][style]),
        5 => {
            let encoding = if style == 0 || style == 2 {
                TextEncoding::Latin1
            } else {
                TextEncoding::Utf16
            };
            let text = ["", "A\0Ω", "ABC", "😀\0z"][style];
            ModelParameterValue::Text(ArchiveString::new(text, encoding).unwrap())
        }
        _ => ModelParameterValue::Double(StoredDouble::from_bits(PRIMARY_BITS[style])),
    }
}

fn expected_alternate(kind: usize, style: usize) -> Option<ModelParameterValue> {
    if style == 0 {
        return None;
    }
    if kind == 3 {
        return Some(ModelParameterValue::Integer(
            [i32::MAX, i32::MIN, 42, -42][style],
        ));
    }
    Some(ModelParameterValue::Double(StoredDouble::from_bits(
        ALTERNATE_BITS[style],
    )))
}

fn assert_read_contract(
    source: &PartResolvedFeaturesArchive,
    parameter: &ModelParameterStoredState,
) {
    assert_eq!(parameter.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(parameter.coverage(), source.coverage());
    assert_eq!(parameter.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(parameter.achieved_stored_value_error(), 0);
    assert_eq!(parameter.diagnostics().len(), 3);
    let selected = source.stored_parameter(parameter.definition()).unwrap();
    assert_eq!(selected.primary(), parameter.primary());
    assert_source_and_history(source, parameter);
}

fn assert_source_and_history(
    source: &PartResolvedFeaturesArchive,
    parameter: &ModelParameterStoredState,
) {
    for relation in parameter.history() {
        assert_eq!(
            relation.parents.as_slice(),
            std::slice::from_ref(&relation.result)
        );
        parameter.source().resolve_field(&relation.result).unwrap();
    }
    match parameter.source() {
        ModelParameterSource::ResolvedFeatures(snapshot) => {
            assert_eq!(snapshot.fields(), source.fields())
        }
        other => panic!("expected feature source, got {other:?}"),
    }
}
