//! Replaces the documented Inspection Job Number without rewriting cached drawing identities.
use bessemer::{
    Document,
    metadata::{MetadataStream, XmlElementRef, XmlLimits, XmlMetadata},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, job_number, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE JOB_NUMBER DESTINATION",
            arguments.len()
        )
        .into());
    };
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    replace_job_number(&mut document, job_number)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn replace_job_number(document: &mut Document, job_number: &str) -> Result<(), Box<dyn Error>> {
    let stream = MetadataStream::parse("ThirdPty/InspectionXpert")?;
    let bytes = document.read_stream_limited(stream.name(), limits().maximum_bytes)?;
    let metadata = XmlMetadata::from_bytes(stream.clone(), &bytes, limits())?;
    let root = metadata
        .root()
        .ok_or("Inspection project has no root; expected ExtInspectionProject")?;
    let settings = unique_child(root, "Settings")?;
    let job = unique_child(settings, "JobNumber")?;
    let edited = metadata.with_leaf_text(&job.id(), job_number, limits())?;
    document.replace_stream(stream.name(), edited.to_bytes()?)?;
    Ok(())
}

fn unique_child<'a>(
    parent: XmlElementRef<'a>,
    local: &str,
) -> Result<XmlElementRef<'a>, Box<dyn Error>> {
    let mut matches = parent.children().filter(|element| {
        element.name().namespace.is_none() && element.name().local.as_ref() == local
    });
    let first = matches.next();
    if first.is_none() || matches.next().is_some() {
        return Err(format!(
            "Inspection {:?}/{local} has zero or multiple matches; expected one unqualified child",
            parent.name()
        )
        .into());
    }
    first.ok_or_else(|| format!("Inspection {local} absent; expected one unqualified child").into())
}

fn limits() -> XmlLimits {
    XmlLimits {
        maximum_bytes: 64 * 1024 * 1024,
        maximum_nodes: 2_000_000,
        maximum_attributes: 2_000_000,
        maximum_depth: 10_000,
    }
}
