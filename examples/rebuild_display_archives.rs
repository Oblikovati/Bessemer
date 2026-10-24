//! Audits complete context-framed streams, returning explicit failures instead of partial archives.
use bessemer::display_lists::{
    DisplayArchive, DisplayArchiveContext, DisplayArchiveLimits, DisplayDocumentKind,
    DisplayVersion,
};
use std::error::Error;
#[path = "support/archive_scalar_trace.rs"]
mod archive_scalar_trace;
#[path = "support/reference_trace.rs"]
mod reference_trace;
mod support;
use support::framing::{require_count, require_finished, take, take_u32};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (source, destination, trace_path) = input_paths(&arguments)?;
    let (output, trace) = rebuild(&std::fs::read(source)?)?;
    std::fs::write(destination, output)?;
    if let Some(path) = trace_path {
        std::fs::write(path, trace)?;
    }
    Ok(())
}

fn input_paths(arguments: &[String]) -> Result<(&str, &str, Option<&str>), Box<dyn Error>> {
    match arguments {
        [source, destination] => Ok((source, destination, None)),
        [source, destination, trace] => Ok((source, destination, Some(trace))),
        _ => Err(format!(
            "{} arguments; expected INPUT OUTPUT [TRACE]",
            arguments.len()
        )
        .into()),
    }
}

fn rebuild(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let mut remaining = bytes;
    let count = take_u32(&mut remaining)?;
    require_count(count, remaining.len(), 17)?;
    let mut output = count.to_le_bytes().to_vec();
    let mut traces = output.clone();
    for _ in 0..count {
        let context = read_context(&mut remaining)?;
        let length = take_u32(&mut remaining)? as usize;
        let payload = take(&mut remaining, length)?;
        let (status, result, trace) = audit(payload, context);
        output.push(status);
        append_frame(&mut output, &result)?;
        append_frame(&mut traces, &trace)?;
    }
    require_finished(remaining)?;
    Ok((output, traces))
}

fn append_frame(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    output.extend_from_slice(&u32::try_from(bytes.len())?.to_le_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn read_context(remaining: &mut &[u8]) -> Result<DisplayArchiveContext, Box<dyn Error>> {
    let version = DisplayVersion::new(take_u32(remaining)?);
    let file_version = take_u32(remaining)?;
    let kind = match take(remaining, 1)?[0] {
        0 => DisplayDocumentKind::Part,
        1 => DisplayDocumentKind::Assembly,
        other => {
            return Err(format!("document kind {other}; expected part 0 or assembly 1").into());
        }
    };
    let components = component_ids(remaining)?;
    Ok(DisplayArchiveContext {
        version,
        kind,
        file_version,
        face_tessellation_components: std::sync::Arc::new(components),
    })
}

fn component_ids(
    remaining: &mut &[u8],
) -> Result<std::collections::BTreeSet<String>, Box<dyn Error>> {
    let count = take_u32(remaining)?;
    require_count(count, remaining.len(), 4)?;
    let mut components = std::collections::BTreeSet::new();
    for _ in 0..count {
        let length = take_u32(remaining)? as usize;
        components.insert(std::str::from_utf8(take(remaining, length)?)?.to_owned());
    }
    Ok(components)
}

fn audit(payload: &[u8], context: DisplayArchiveContext) -> (u8, Vec<u8>, Vec<u8>) {
    match reconstruct(payload, context) {
        Ok((bytes, trace)) => (0, bytes, trace),
        Err(error) => (1, error.to_string().into_bytes(), Vec::new()),
    }
}

fn reconstruct(
    payload: &[u8],
    context: DisplayArchiveContext,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let limits = DisplayArchiveLimits {
        maximum_bytes: 256 * 1024 * 1024,
        maximum_fields: 2_000_000,
        maximum_objects: 1_000_000,
        maximum_depth: 256,
        maximum_entries: 2_000_000,
    };
    let parsed = DisplayArchive::from_bytes(payload, context.clone(), limits)?;
    let rebuilt = DisplayArchive::new(parsed.contents().clone(), context, limits)?.to_bytes()?;
    if rebuilt != payload {
        return Err("reconstructed stream differed; expected complete byte identity".into());
    }
    Ok((rebuilt, reference_trace::trace(&parsed)))
}
