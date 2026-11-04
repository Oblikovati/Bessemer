use super::super::stored_root_graphs::exports::{append_packet, put_frame, put_tag, put_word};
use super::*;
use crate::model_archives::{
    ModelParameterKind, ModelParameterName, ModelParameterSource, ModelParameterValue,
};
#[path = "fixtures.rs"]
mod owned_fixtures;

pub(super) fn export_parameter_case(
    case: &ResolvedCorpusCase<'_>,
    configuration: &[ModelParameterStoredState],
    features: &[ModelParameterStoredState],
) {
    let Some(path) = std::env::var_os("BESSEMER_PARAMETER_RESULT_PACKET") else {
        return;
    };
    let mut packet = Vec::new();
    put_word(&mut packet, case.version.value());
    put_frame(&mut packet, case.label.as_bytes());
    put_parameters(&mut packet, configuration);
    put_parameters(&mut packet, features);
    append_packet(path, &packet);
}

fn put_parameters(packet: &mut Vec<u8>, parameters: &[ModelParameterStoredState]) {
    put_word(packet, u32::try_from(parameters.len()).unwrap());
    for parameter in parameters {
        put_parameter(packet, parameter);
    }
}

fn put_parameter(packet: &mut Vec<u8>, parameter: &ModelParameterStoredState) {
    let field = parameter
        .source()
        .resolve_field(parameter.definition())
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected physical definition");
    };
    put_tag(packet, tag);
    put_word(packet, kind_word(parameter.kind()));
    put_name(packet, parameter, parameter.node_name());
    put_name(packet, parameter, parameter.parameter_name());
    put_word(packet, u32::from(parameter.alternate_presence().is_some()));
    if let Some(presence) = parameter.alternate_presence() {
        put_word(packet, u32::from(presence));
    }
    put_value(packet, parameter.primary());
    put_word(packet, u32::from(parameter.alternate().is_some()));
    if let Some(alternate) = parameter.alternate() {
        put_value(packet, alternate);
    }
}

fn kind_word(kind: ModelParameterKind) -> u32 {
    match kind {
        ModelParameterKind::Density => 0,
        ModelParameterKind::Length => 1,
        ModelParameterKind::Angle => 2,
        ModelParameterKind::Integer => 3,
        ModelParameterKind::AttributeScalar => 4,
        ModelParameterKind::AttributeString => 5,
    }
}

fn put_name(
    packet: &mut Vec<u8>,
    parameter: &ModelParameterStoredState,
    name: &ModelParameterName,
) {
    put_tag(packet, name.tag());
    put_word(packet, name_origin(parameter, name));
    if let Some(text) = name.text() {
        put_text(packet, text);
    }
}

fn name_origin(parameter: &ModelParameterStoredState, name: &ModelParameterName) -> u32 {
    let Some(field) = name.text_field() else {
        return 0;
    };
    if matches!(
        parameter.source(),
        ModelParameterSource::ResolvedFeatures(_)
    ) && parameter
        .source()
        .configuration_archive()
        .resolve_field(field)
        .is_ok()
    {
        return 1;
    }
    2
}

fn put_text(packet: &mut Vec<u8>, text: &crate::archive_codec::ArchiveString) {
    put_frame(packet, text.text().as_bytes());
    let mut wire = Vec::new();
    text.encode(&mut wire).unwrap();
    put_frame(packet, &wire);
}

fn put_value(packet: &mut Vec<u8>, value: &ModelParameterValue) {
    match value {
        ModelParameterValue::Double(value) => {
            put_word(packet, 0);
            packet.extend(value.bits().to_le_bytes());
        }
        ModelParameterValue::Integer(value) => {
            put_word(packet, 1);
            packet.extend(value.to_le_bytes());
        }
        ModelParameterValue::Text(value) => {
            put_word(packet, 2);
            put_text(packet, value);
        }
    }
}

#[test]
fn owned_parameter_exports_cover_every_class_name_style_and_independent_primary_edit() {
    let configuration = crate::model_archives::context_snapshot_tests::owned_configuration();
    let mut packet = Vec::new();
    for (kind, roots) in owned_fixtures::ORIGINAL.iter().enumerate() {
        for (style, raw) in roots.iter().enumerate() {
            put_owned(&mut packet, &configuration, raw, [kind, style, 0]);
        }
        put_owned(
            &mut packet,
            &configuration,
            owned_fixtures::EDITED[kind],
            [kind, 0, 1],
        );
    }
    if let Some(path) = std::env::var_os("BESSEMER_PARAMETER_OWNED_PACKET") {
        append_packet(path, &packet);
    }
    assert!(!packet.is_empty());
}

fn put_owned(
    packet: &mut Vec<u8>,
    configuration: &crate::model_archives::PartConfigurationArchive,
    raw: &[u8],
    label: [usize; 3],
) {
    for word in label {
        put_word(packet, u32::try_from(word).unwrap());
    }
    let source = PartResolvedFeaturesArchive::from_bytes(
        raw,
        configuration.clone(),
        include_bytes!("../fixtures/sheet-metal/manager-0.bin"),
        LIMITS,
    )
    .unwrap();
    let parameters = source.stored_parameters().unwrap();
    assert_eq!(parameters.len(), 1);
    put_parameters(packet, &parameters);
}
