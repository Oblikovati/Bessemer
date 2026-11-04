use super::*;

#[test]
fn inherited_name_aliases_keep_configuration_identity_text_and_history() {
    let (_, original) = fixture(fixtures::ORIGINAL[0][0]);
    let snapshot = state(&original);
    let configuration = snapshot.source().configuration_archive();
    let (identity, text) = inherited_name(configuration);
    let raw = inherited_parameter_bytes(identity);
    let source = inherited_archive(&raw, configuration);
    let parameter = state(&source);
    assert_eq!(parameter.node_name().text(), Some(&text));
    assert_eq!(parameter.parameter_name().text(), Some(&text));
    assert_eq!(
        parameter.node_name().text_field(),
        parameter.parameter_name().text_field()
    );
    assert_inherited_history(&parameter);
    let edit = apply_parameter(&source, &parameter, primary_request(0));
    assert_eq!(edit.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(edit.archive().to_bytes().unwrap(), raw);
}

fn inherited_archive(
    raw: &[u8],
    configuration: &PartConfigurationArchive,
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_bytes(
        raw,
        configuration.clone(),
        include_bytes!("../fixtures/sheet-metal/manager-0.bin"),
        LIMITS,
    )
    .unwrap()
}

fn inherited_name(source: &PartConfigurationArchive) -> (ModelObjectId, ArchiveString) {
    for (index, field) in source.fields().iter().enumerate() {
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            continue;
        };
        let ModelObjectRecord::New {
            index: identity,
            class,
        } = &tag.record
        else {
            continue;
        };
        if class.name != "moNodeName_c" {
            continue;
        }
        let ModelFieldValue::Text(text) = &source.fields()[index + 1].value else {
            panic!("expected name text");
        };
        return (*identity, text.clone());
    }
    panic!("expected complete configuration with an inherited name definition");
}

fn inherited_parameter_bytes(identity: ModelObjectId) -> Vec<u8> {
    let name = b"moDensityParameter_c";
    let mut raw = (-123i32).to_le_bytes().to_vec();
    for word in [1u16, 65535, 7, name.len() as u16] {
        raw.extend(word.to_le_bytes());
    }
    raw.extend(name);
    raw.extend(u16::try_from(identity.value()).unwrap().to_le_bytes());
    for word in [0xfffffffeu32, 0xffffffff, 0xfffffffd, 0x80000000] {
        raw.extend(word.to_le_bytes());
    }
    raw.extend(b"\x04help");
    raw.extend(u16::try_from(identity.value()).unwrap().to_le_bytes());
    raw.extend(0x3ff4000000000000u64.to_le_bytes());
    raw.push(0);
    raw
}

fn assert_inherited_history(parameter: &ModelParameterStoredState) {
    let definition = parameter.node_name().text_field().unwrap();
    let field = parameter.source().resolve_field(definition).unwrap();
    assert!(matches!(field.value, ModelFieldValue::Text(_)));
    assert!(
        parameter
            .source()
            .configuration_archive()
            .resolve_field(definition)
            .is_ok()
    );
    assert_eq!(
        parameter
            .history()
            .iter()
            .filter(|copy| &copy.result == definition)
            .count(),
        1
    );
}
