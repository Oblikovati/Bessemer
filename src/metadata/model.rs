use super::{
    MetadataStream, XmlLimits, XmlSource, encoding, invalid,
    types::{XmlMetrics, XmlModel},
    unsupported,
};
use crate::Result;

pub(super) fn parse(
    stream: &MetadataStream,
    source: &XmlSource,
    limits: XmlLimits,
) -> Result<XmlModel> {
    encoding::require_format(stream, source)?;
    let Some(text) = &source.document else {
        return Ok(XmlModel::default());
    };
    if text.starts_with('\u{feff}') {
        return Err(invalid(
            "XML source starts with U+FEFF; expected BOM recorded in XmlFormat",
        ));
    }
    encoding::require_size(text.len(), limits.maximum_bytes, "decoded UTF-8 XML bytes")?;
    let physical_bytes = encoding::encoded_length(source)?;
    encoding::require_size(physical_bytes, limits.maximum_bytes, "physical XML bytes")?;
    let mut model = super::reader::parse_document(text, limits)?;
    require_root(stream, &model)?;
    model.metrics.source_bytes = text.len();
    model.metrics.physical_bytes = physical_bytes;
    require_limits(model.metrics, limits)?;
    Ok(model)
}

fn require_root(stream: &MetadataStream, model: &XmlModel) -> Result<()> {
    let expected = stream.family().expected_root();
    let actual = model.elements.first().map(|element| &element.name);
    if actual == Some(&expected) {
        return Ok(());
    }
    Err(unsupported(format!(
        "XML root {actual:?} for {:?}; expected {expected:?}",
        stream.family()
    )))
}

pub(super) fn require_limits(metrics: XmlMetrics, limits: XmlLimits) -> Result<()> {
    require_byte_limits(metrics, limits.maximum_bytes)?;
    encoding::require_size(
        metrics.nodes as usize,
        limits.maximum_nodes as usize,
        "XML lexical events",
    )?;
    encoding::require_size(
        metrics.attributes,
        limits.maximum_attributes,
        "XML attributes",
    )?;
    encoding::require_size(metrics.depth, limits.maximum_depth, "XML element depth")
}

fn require_byte_limits(metrics: XmlMetrics, maximum: usize) -> Result<()> {
    encoding::require_size(metrics.physical_bytes, maximum, "physical XML bytes")?;
    encoding::require_size(metrics.source_bytes, maximum, "decoded UTF-8 XML bytes")
}
