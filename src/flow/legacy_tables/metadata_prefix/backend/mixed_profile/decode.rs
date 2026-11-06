use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowMediumMixedOptions,
) -> Result<FlowMediumMixedProfile> {
    validate_options(options)?;
    let prefix = source.record_424_metadata_prefix(id, options.profile.metadata)?;
    let backend = prefix.medium_backend_prefix()?;
    let mut reader = scan::MixedReader::new(&backend, options)?;
    let first = reader.first_objects()?;
    let second = reader.reference_objects()?;
    let backend_end = reader.position();
    let outer = reader.outer_references()?;
    reader.finish()?;
    Ok(reader.complete(first, second, outer, backend_end))
}
pub(super) fn body(
    backend: &FlowMediumBackendPrefix,
    start: usize,
    kind: FlowMediumKind,
    options: FlowMediumMixedOptions,
) -> Result<FlowMediumBody> {
    match kind {
        FlowMediumKind::One => FlowMediumKindOne::read_at(backend, start, options.profile.body)
            .map(FlowMediumBody::KindOne),
        FlowMediumKind::Two => FlowMediumKindTwo::read_at(
            backend,
            start,
            FlowMediumKindTwoOptions {
                body: options.profile.body,
                maximum_word_patterns: options.maximum_body_word_patterns,
            },
        )
        .map(FlowMediumBody::KindTwo),
    }
}

fn validate_options(options: FlowMediumMixedOptions) -> Result<()> {
    validate_profile_options(options.profile)?;
    if options.maximum_body_word_patterns == 0 || options.maximum_total_word_patterns == 0 {
        return Err(metadata_invalid(format!(
            "mixed options {options:?}; expected independent positive word-pattern bounds"
        )));
    }
    Ok(())
}
