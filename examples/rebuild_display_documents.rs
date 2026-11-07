//! Force complete display-list reconstruction using context resolved from owned document containers.
use bessemer::Document;
use bessemer::display_lists::{
    DisplayArchive, DisplayArchiveContext, DisplayArchiveLimits, DisplayDocumentKind,
};
use bessemer::tessellation::{
    ArchiveVersion, FacetArchiveMode, TessellationContext, TessellationLimits,
};
use std::error::Error;
mod support;
use support::framing::{require_count, require_finished, take, take_u32};

type RebuildResult<T> = Result<T, Box<dyn Error>>;

fn main() -> RebuildResult<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, destination, trace] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected INPUT OUTPUT CONTEXT_TRACE",
            arguments.len()
        )
        .into());
    };
    let (output, contexts) = rebuild(&std::fs::read(source)?)?;
    std::fs::write(destination, output)?;
    std::fs::write(trace, contexts)?;
    Ok(())
}

fn rebuild(mut remaining: &[u8]) -> RebuildResult<(Vec<u8>, Vec<u8>)> {
    let count = take_u32(&mut remaining)?;
    require_count(count, remaining.len(), 13)?;
    let mut output = count.to_le_bytes().to_vec();
    let mut contexts = count.to_le_bytes().to_vec();
    for _ in 0..count {
        let (status, payload, context) = read_record(&mut remaining)?;
        output.push(status);
        append_blob(&mut output, &payload)?;
        append_blob(&mut contexts, &context)?;
    }
    require_finished(remaining)?;
    Ok((output, contexts))
}

fn read_record(remaining: &mut &[u8]) -> RebuildResult<(u8, Vec<u8>, Vec<u8>)> {
    let kind = document_kind(take(remaining, 1)?[0])?;
    let file_version = take_u32(remaining)?;
    let directory_version = take_u32(remaining)?;
    let length = take_u32(remaining)? as usize;
    Ok(audit(
        take(remaining, length)?,
        kind,
        file_version,
        directory_version,
    ))
}

fn document_kind(value: u8) -> RebuildResult<DisplayDocumentKind> {
    match value {
        0 => Ok(DisplayDocumentKind::Part),
        1 => Ok(DisplayDocumentKind::Assembly),
        _ => Err(format!("document kind {value}; expected 0 for part or 1 for assembly").into()),
    }
}

fn audit(
    payload: &[u8],
    kind: DisplayDocumentKind,
    file_version: u32,
    directory_version: u32,
) -> (u8, Vec<u8>, Vec<u8>) {
    match reconstruct(payload, kind, file_version, directory_version) {
        Ok((document, context)) => (0, document, context),
        Err(error) => (1, error.to_string().into_bytes(), Vec::new()),
    }
}

fn reconstruct(
    payload: &[u8],
    kind: DisplayDocumentKind,
    file_version: u32,
    directory_version: u32,
) -> RebuildResult<(Vec<u8>, Vec<u8>)> {
    let mut document = Document::from_bytes(payload)?;
    let context = resolve_context(&document, kind, file_version, directory_version)?;
    replace_display_stream(&mut document, &context)?;
    Ok((document.to_bytes()?, context_trace(&context)?))
}

fn resolve_context(
    document: &Document,
    kind: DisplayDocumentKind,
    file_version: u32,
    directory_version: u32,
) -> RebuildResult<DisplayArchiveContext> {
    let tessellation = TessellationContext {
        archive_version: ArchiveVersion::new(directory_version),
        mode: FacetArchiveMode::Full,
    };
    Ok(DisplayArchiveContext::from_document(
        document,
        kind,
        file_version,
        tessellation,
        tessellation_limits(),
        32 * 1024 * 1024,
    )?)
}

fn replace_display_stream(
    document: &mut Document,
    context: &DisplayArchiveContext,
) -> RebuildResult<()> {
    let original =
        document.read_stream_limited("Contents/DisplayLists", archive_limits().maximum_bytes)?;
    document.replace_stream(
        "Contents/DisplayLists",
        rebuild_stream(&original, context.clone())?,
    )?;
    Ok(())
}

fn tessellation_limits() -> TessellationLimits {
    TessellationLimits {
        maximum_components: 1_000_000,
        maximum_faces: 1_000_000,
        maximum_array_elements: 2_000_000,
    }
}

fn rebuild_stream(bytes: &[u8], context: DisplayArchiveContext) -> RebuildResult<Vec<u8>> {
    let parsed = DisplayArchive::from_bytes(bytes, context.clone(), archive_limits())?;
    let rebuilt =
        DisplayArchive::new(parsed.contents().clone(), context, archive_limits())?.to_bytes()?;
    if rebuilt != bytes {
        return Err("reconstructed display stream differed; expected exact forced bytes".into());
    }
    Ok(rebuilt)
}

fn context_trace(context: &DisplayArchiveContext) -> RebuildResult<Vec<u8>> {
    let mut output = context.version.value().to_le_bytes().to_vec();
    output.extend_from_slice(&context.file_version.to_le_bytes());
    output.push(u8::from(context.kind == DisplayDocumentKind::Assembly));
    let count = u32::try_from(context.face_tessellation_components.len())?;
    output.extend_from_slice(&count.to_le_bytes());
    for identity in context.face_tessellation_components.iter() {
        append_blob(&mut output, identity.as_bytes())?;
    }
    Ok(output)
}

fn append_blob(output: &mut Vec<u8>, bytes: &[u8]) -> RebuildResult<()> {
    let length = u32::try_from(bytes.len())?;
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(bytes);
    Ok(())
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
