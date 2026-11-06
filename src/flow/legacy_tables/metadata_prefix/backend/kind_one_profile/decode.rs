use super::*;
mod scan;
use scan::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowMediumProfileOptions,
) -> Result<FlowMediumKindOneProfile> {
    validate_options(options)?;
    let prefix = source.record_424_metadata_prefix(id, options.metadata)?;
    let backend = prefix.medium_backend_prefix()?;
    let mut reader = ProfileReader::new(&backend, options)?;
    let first = reader.first_objects()?;
    let second = reader.reference_objects()?;
    let backend_end = reader.position();
    let outer = reader.outer_references()?;
    reader.finish()?;
    Ok(FlowMediumKindOneProfile {
        backend: backend.clone(),
        options,
        first: first.into(),
        second: second.into(),
        outer,
        backend_end,
        references: options.maximum_references - reader.remaining_references,
        words: options.maximum_total_wide_words - reader.remaining_words,
        name_units: options.maximum_total_name_units - reader.remaining_names,
    })
}
fn validate_options(options: FlowMediumProfileOptions) -> Result<()> {
    let bounds = [
        options.maximum_first_objects,
        options.maximum_reference_objects,
        options.maximum_references,
        options.maximum_total_wide_words,
        options.maximum_total_name_units,
        options.body.maximum_body_bytes,
        options.body.maximum_wide_words,
        options.body.maximum_name_units,
    ];
    if bounds.contains(&0) {
        return Err(metadata_invalid(format!(
            "profile options {options:?}; expected all independent bounds positive"
        )));
    }
    Ok(())
}
