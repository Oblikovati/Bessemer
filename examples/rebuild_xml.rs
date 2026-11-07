//! Forces Unicode/framing reconstruction and emits a namespace/attribute/text oracle trace.
use bessemer::metadata::{MetadataStream, XmlElementRef, XmlLimits, XmlMetadata};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (groups, remainder) = arguments.as_chunks::<4>();
    if groups.is_empty() || !remainder.is_empty() {
        return Err(format!(
            "{} arguments; expected STREAM INPUT OUTPUT TRACE groups",
            arguments.len()
        )
        .into());
    }
    for [stream, source, destination, trace_path] in groups {
        let stream = MetadataStream::parse(stream)?;
        let metadata = XmlMetadata::from_bytes(stream.clone(), &std::fs::read(source)?, limits())?;
        let rebuilt = XmlMetadata::new(stream, metadata.source().clone(), limits())?;
        std::fs::write(destination, rebuilt.to_bytes()?)?;
        std::fs::write(trace_path, trace(&metadata))?;
    }
    Ok(())
}

fn limits() -> XmlLimits {
    XmlLimits {
        maximum_bytes: 64 * 1024 * 1024,
        maximum_nodes: 2_000_000,
        maximum_attributes: 2_000_000,
        maximum_depth: 10_000,
    }
}

fn trace(metadata: &XmlMetadata) -> Vec<u8> {
    let mut output = (metadata.elements().len() as u32).to_le_bytes().to_vec();
    for element in metadata.elements() {
        append_element(&mut output, element);
    }
    output
}

fn append_element(output: &mut Vec<u8>, element: XmlElementRef<'_>) {
    let parent = element
        .parent()
        .map_or(u32::MAX, |parent| parent.ordinal() as u32);
    output.extend_from_slice(&parent.to_le_bytes());
    optional_text(output, element.name().namespace.as_deref());
    text(output, &element.name().local);
    text(output, element.direct_text());
    output.extend_from_slice(&(element.attributes().len() as u32).to_le_bytes());
    for attribute in element.attributes() {
        optional_text(output, attribute.name.namespace.as_deref());
        text(output, &attribute.name.local);
        text(output, &attribute.value);
    }
    output.extend_from_slice(&(element.children().len() as u32).to_le_bytes());
}

fn optional_text(output: &mut Vec<u8>, value: Option<&str>) {
    if let Some(value) = value {
        text(output, value);
        return;
    }
    output.extend_from_slice(&u32::MAX.to_le_bytes());
}

fn text(output: &mut Vec<u8>, value: &str) {
    output.extend_from_slice(&(value.len() as u32).to_le_bytes());
    output.extend_from_slice(value.as_bytes());
}
