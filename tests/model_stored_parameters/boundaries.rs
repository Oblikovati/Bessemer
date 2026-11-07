use super::*;

fn retained_configuration() -> PartConfigurationArchive {
    let (_, source) = fixture(fixtures::ORIGINAL[0][0]);
    state(&source).source().configuration_archive().clone()
}

fn boundary_read(
    raw: &[u8],
    configuration: &PartConfigurationArchive,
    limits: ModelArchiveLimits,
) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_bytes(
        raw,
        configuration.clone(),
        include_bytes!("../fixtures/sheet-metal/manager-0.bin"),
        limits,
    )
}

#[test]
fn every_owned_parameter_prefix_and_appended_octet_fails_before_projection() {
    let configuration = retained_configuration();
    for roots in fixtures::ORIGINAL {
        for raw in roots {
            assert_incomplete_parameter_bytes(raw, &configuration);
        }
    }
    for raw in fixtures::INHERITED {
        assert_incomplete_parameter_bytes(raw, &configuration);
    }
}

fn assert_incomplete_parameter_bytes(raw: &[u8], configuration: &PartConfigurationArchive) {
    let source = boundary_read(raw, configuration, LIMITS).unwrap();
    let parameter = state(&source);
    for end in 0..raw.len() {
        assert!(
            boundary_read(&raw[..end], configuration, LIMITS).is_err(),
            "accepted proper prefix {end}/{}",
            raw.len()
        );
    }
    for byte in 0..=255 {
        let mut appended = raw.to_vec();
        appended.push(byte);
        assert!(boundary_read(&appended, configuration, LIMITS).is_err());
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(state(&source).primary(), parameter.primary());
}

#[test]
fn exact_byte_and_field_caps_pass_and_one_below_rejects_complete_owned_parameters() {
    let configuration = retained_configuration();
    for roots in fixtures::ORIGINAL {
        for raw in roots {
            assert_parameter_caps(raw, &configuration);
        }
    }
    for raw in fixtures::INHERITED {
        assert_parameter_caps(raw, &configuration);
    }
}

fn assert_parameter_caps(raw: &[u8], configuration: &PartConfigurationArchive) {
    let expanded = parameter_budget_graph(raw);
    let raw = expanded.as_slice();
    let source = boundary_read(raw, configuration, LIMITS).unwrap();
    let exact = parameter_exact_limits(raw, &source);
    let admitted = boundary_read(raw, configuration, exact).unwrap();
    assert_eq!(state(&admitted).primary(), state(&source).primary());
    let bytes = ModelArchiveLimits {
        maximum_bytes: raw.len() - 1,
        ..exact
    };
    let fields = ModelArchiveLimits {
        maximum_fields: source.fields().len() - 1,
        ..exact
    };
    assert!(boundary_read(raw, configuration, bytes).is_err());
    assert!(boundary_read(raw, configuration, fields).is_err());
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn parameter_exact_limits(raw: &[u8], source: &PartResolvedFeaturesArchive) -> ModelArchiveLimits {
    ModelArchiveLimits {
        maximum_bytes: raw.len(),
        maximum_fields: source.fields().len(),
        ..LIMITS
    }
}

#[test]
fn rejected_text_growth_keeps_complete_input_and_snapshot_intact() {
    let configuration = retained_configuration();
    let expanded = parameter_budget_graph(fixtures::ORIGINAL[5][0]);
    let raw = expanded.as_slice();
    let limits = ModelArchiveLimits {
        maximum_bytes: raw.len(),
        ..LIMITS
    };
    let source = boundary_read(raw, &configuration, limits).unwrap();
    let parameter = state(&source);
    let text = ArchiveString::new("longer Ω value", TextEncoding::Utf16).unwrap();
    let result = source.edit_stored_parameter(
        &parameter,
        ModelParameterStoredEdit::Primary(ModelParameterValue::Text(text)),
    );
    assert!(result.is_err());
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(state(&source).primary(), parameter.primary());
    source.resolve_field(parameter.definition()).unwrap();
}

fn parameter_budget_graph(raw: &[u8]) -> Vec<u8> {
    let mut expanded = raw.to_vec();
    expanded[4..6].copy_from_slice(&4097u16.to_le_bytes());
    expanded.resize(raw.len() + 8192, 0);
    expanded
}

#[test]
fn explicit_null_roots_require_the_exact_root_entry_budget() {
    let configuration = retained_configuration();
    for roots in fixtures::ORIGINAL {
        for raw in roots {
            assert_parameter_root_cap(raw, &configuration);
        }
    }
    for raw in fixtures::INHERITED {
        assert_parameter_root_cap(raw, &configuration);
    }
}

fn assert_parameter_root_cap(raw: &[u8], configuration: &PartConfigurationArchive) {
    let expanded = parameter_budget_graph(raw);
    let limits = ModelArchiveLimits {
        maximum_entries: 4097,
        ..LIMITS
    };
    let source = boundary_read(&expanded, configuration, limits).unwrap();
    assert_eq!(source.to_bytes().unwrap(), expanded);
    assert_eq!(source.stored_parameters().unwrap().len(), 1);
    let insufficient = ModelArchiveLimits {
        maximum_entries: 4096,
        ..limits
    };
    assert!(boundary_read(&expanded, configuration, insufficient).is_err());
    assert_eq!(source.to_bytes().unwrap(), expanded);
}
