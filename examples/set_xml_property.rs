//! Edits one selected property's value leaf without dropping extra values or dictionary aliases.
use bessemer::{
    Document,
    metadata::{MetadataStream, XmlElementId, XmlLimits, XmlMetadata},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, stream, section, property, value, text, destination] = arguments.as_slice() else {
        return Err(format!("{} arguments; expected SOURCE STREAM SECTION_INDEX PROPERTY_INDEX VALUE_INDEX TEXT DESTINATION",arguments.len()).into());
    };
    let stream = MetadataStream::parse(stream)?;
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let bytes = document.read_stream_limited(stream.name(), 64 * 1024 * 1024)?;
    let metadata = XmlMetadata::from_bytes(stream.clone(), &bytes, limits())?;
    let identity = value_identity(
        &metadata,
        section.parse()?,
        property.parse()?,
        value.parse()?,
    )?;
    let edited = metadata.with_leaf_text(&identity, text, limits())?;
    document.replace_stream(stream.name(), edited.to_bytes()?)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn value_identity(
    metadata: &XmlMetadata,
    section: usize,
    property: usize,
    value: usize,
) -> Result<XmlElementId, Box<dyn Error>> {
    let section_view = metadata
        .property_sections()
        .nth(section)
        .ok_or_else(|| format!("property section index {section}; expected an existing section"))?;
    let property_view = section_view.properties().nth(property).ok_or_else(|| {
        format!("property index {property}; expected an existing property in section {section}")
    })?;
    let value_view=property_view.values().nth(value).ok_or_else(||format!("value index {value}; expected an existing value in property {property}, section {section}"))?;
    Ok(value_view.element().id())
}

fn limits() -> XmlLimits {
    XmlLimits {
        maximum_bytes: 64 * 1024 * 1024,
        maximum_nodes: 2_000_000,
        maximum_attributes: 2_000_000,
        maximum_depth: 10_000,
    }
}
