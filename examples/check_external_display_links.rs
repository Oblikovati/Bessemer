//! Reports complete external facet links or a typed failure without publishing partial geometry.
use bessemer::display_lists::{
    DisplayArchive, DisplayArchiveContext, DisplayArchiveLimits, DisplayDocumentKind,
    DisplayVersion, ExternalFaceLink, ExternalFaceLinks,
};
use bessemer::tessellation::{
    ArchiveVersion, FaceTessellationBlock, FacetArchive, FacetArchiveMode, TessellationContext,
    TessellationDirectory, TessellationLimits,
};
use bessemer::{BessemerError, Document, FailureClass};
use std::{collections::BTreeMap, error::Error, path::PathBuf, sync::Arc};

struct LinkOptions {
    source: PathBuf,
    destination: PathBuf,
    kind: DisplayDocumentKind,
    file_version: u32,
    context: TessellationContext,
}

struct LinkInputs {
    display: DisplayArchive,
    directory: TessellationDirectory,
    blocks: BTreeMap<String, FaceTessellationBlock>,
    context: TessellationContext,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = options(std::env::args().skip(1).collect())?;
    let document = Document::from_bytes(&std::fs::read(&options.source)?)?;
    let report = match load_inputs(&document, &options).and_then(|inputs| report(&inputs)) {
        Ok(report) => report,
        Err(error) => error_report(&error)?,
    };
    std::fs::write(&options.destination, report)?;
    Ok(())
}

fn options(arguments: Vec<String>) -> Result<LinkOptions, Box<dyn Error>> {
    let [source, kind, file_version, directory_version, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE part|assembly FILE_VERSION DIRECTORY_VERSION REPORT",
            arguments.len()
        )
        .into());
    };
    let context = TessellationContext {
        archive_version: ArchiveVersion::new(parse_version(directory_version)?),
        mode: FacetArchiveMode::Full,
    };
    Ok(LinkOptions {
        source: source.into(),
        destination: destination.into(),
        kind: parse_kind(kind)?,
        file_version: parse_version(file_version)?,
        context,
    })
}

fn parse_kind(value: &str) -> Result<DisplayDocumentKind, Box<dyn Error>> {
    match value {
        "part" => Ok(DisplayDocumentKind::Part),
        "assembly" => Ok(DisplayDocumentKind::Assembly),
        other => Err(format!("document kind {other:?}; expected part or assembly").into()),
    }
}

fn parse_version(value: &str) -> Result<u32, Box<dyn Error>> {
    value.parse().map_err(|error| {
        format!("version {value:?}: {error}; expected an unsigned 32-bit decimal").into()
    })
}

fn load_inputs(document: &Document, options: &LinkOptions) -> bessemer::Result<LinkInputs> {
    let bytes = document.read_stream_limited("FaceTessellations/Directory", 32 * 1024 * 1024)?;
    let directory =
        TessellationDirectory::from_bytes(&bytes, options.context, tessellation_limits())?;
    let context = display_context(document, &directory, options)?;
    let bytes =
        document.read_stream_limited("Contents/DisplayLists", archive_limits().maximum_bytes)?;
    let display = DisplayArchive::from_bytes(&bytes, context, archive_limits())?;
    let blocks = load_blocks(document, options.context)?;
    Ok(LinkInputs {
        display,
        directory,
        blocks,
        context: options.context,
    })
}

fn display_context(
    document: &Document,
    directory: &TessellationDirectory,
    options: &LinkOptions,
) -> bessemer::Result<DisplayArchiveContext> {
    Ok(DisplayArchiveContext {
        version: DisplayVersion::from_stream_names(document.streams().map(|stream| stream.name))?,
        kind: options.kind,
        file_version: options.file_version,
        face_tessellation_components: Arc::new(directory.component_ids()),
    })
}

fn load_blocks(
    document: &Document,
    context: TessellationContext,
) -> bessemer::Result<BTreeMap<String, FaceTessellationBlock>> {
    let mut blocks = BTreeMap::new();
    for stream in document.streams() {
        let Some(name) = stream.name.strip_prefix("FaceTessellations/") else {
            continue;
        };
        if name == "Directory" {
            continue;
        }
        let bytes = document.read_stream_limited(stream.name, archive_limits().maximum_bytes)?;
        blocks.insert(
            name.to_owned(),
            FaceTessellationBlock::from_bytes(&bytes, context, tessellation_limits())?,
        );
    }
    Ok(blocks)
}

fn report(inputs: &LinkInputs) -> bessemer::Result<Vec<u8>> {
    let linked = inputs
        .display
        .check_external_face_links(&inputs.directory, &inputs.blocks)?;
    let mut output = vec![0];
    coverage_report(&mut output, &linked);
    let count =
        u32::try_from(linked.links().len()).map_err(|_| report_size(linked.links().len()))?;
    output.extend_from_slice(&count.to_le_bytes());
    for link in linked.links() {
        append_link(&mut output, link, inputs.context)?;
    }
    Ok(output)
}

fn append_link(
    output: &mut Vec<u8>,
    link: &ExternalFaceLink<'_>,
    context: TessellationContext,
) -> bessemer::Result<()> {
    append_blob(output, link.display_path.as_bytes())?;
    append_blob(output, link.component_id.as_bytes())?;
    append_blob(output, link.block_name.as_bytes())?;
    output.extend_from_slice(&link.index_in_block.to_le_bytes());
    output.extend_from_slice(&link.face_index.to_le_bytes());
    append_blob(
        output,
        &FacetArchive::new(context, link.facet.clone())?.to_bytes()?,
    )
}

fn coverage_report(output: &mut Vec<u8>, linked: &ExternalFaceLinks<'_>) {
    let counts = linked.coverage();
    for count in [
        counts.checked_blocks,
        counts.checked_directory_references,
        counts.directory_entries_without_block_names,
        counts.available_block_components,
        counts.unreferenced_block_components,
        counts.checked_face_references,
        counts.unique_referenced_faces,
        counts.available_block_faces,
        counts.unreferenced_block_faces,
    ] {
        output.extend_from_slice(&(count as u64).to_le_bytes());
    }
}

fn error_report(error: &BessemerError) -> bessemer::Result<Vec<u8>> {
    let status = match error.class {
        FailureClass::Missing => 1,
        FailureClass::Invalid => 2,
        FailureClass::Unsupported => 3,
        FailureClass::Integrity => 4,
    };
    let mut output = vec![status];
    for field in [error.operation, &error.entity, &error.cause] {
        append_blob(&mut output, field.as_bytes())?;
    }
    Ok(output)
}

fn append_blob(output: &mut Vec<u8>, bytes: &[u8]) -> bessemer::Result<()> {
    let length = u32::try_from(bytes.len()).map_err(|_| report_size(bytes.len()))?;
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn report_size(length: usize) -> BessemerError {
    BessemerError {
        operation: "report external links",
        entity: "length".into(),
        class: FailureClass::Unsupported,
        cause: format!("report field length {length}; expected at most u32::MAX bytes"),
    }
}

fn archive_limits() -> DisplayArchiveLimits {
    DisplayArchiveLimits {
        maximum_bytes: 256 * 1024 * 1024,
        maximum_fields: 2_000_000,
        maximum_objects: 1_000_000,
        maximum_depth: 256,
        maximum_entries: 2_000_000,
    }
}

fn tessellation_limits() -> TessellationLimits {
    TessellationLimits {
        maximum_components: 1_000_000,
        maximum_faces: 1_000_000,
        maximum_array_elements: 100_000_000,
    }
}
