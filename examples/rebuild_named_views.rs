//! Forces reconstruction of version-framed named-view corpus records.
use bessemer::display_lists::{DisplayVersion, DisplayViewLimits, NamedViews};
use std::error::Error;
#[path = "support/named_trace.rs"]
mod named_trace;
mod support;
use support::framing::{require_count, require_finished, take, take_u32};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, destination, trace] = arguments.as_slice() else {
        return Err(format!("{} arguments; expected INPUT OUTPUT TRACE", arguments.len()).into());
    };
    let (output, traces) = rebuild(&std::fs::read(source)?)?;
    std::fs::write(destination, output)?;
    std::fs::write(trace, traces)?;
    Ok(())
}

fn rebuild(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let mut remaining = bytes;
    let count = take_u32(&mut remaining)?;
    require_count(count, remaining.len(), 9)?;
    let mut output = count.to_le_bytes().to_vec();
    let mut traces = count.to_le_bytes().to_vec();
    for _ in 0..count {
        rebuild_record(&mut remaining, &mut output, &mut traces)?;
    }
    require_finished(remaining)?;
    Ok((output, traces))
}

fn rebuild_record(
    remaining: &mut &[u8],
    output: &mut Vec<u8>,
    traces: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let version = take_u32(remaining)?;
    let scenario = take(remaining, 1)?[0];
    let length = take_u32(remaining)? as usize;
    let archive = NamedViews::from_bytes(
        take(remaining, length)?,
        DisplayVersion::new(version),
        limits(),
    )?;
    let trace = named_trace::trace(&archive);
    traces.extend_from_slice(&(trace.len() as u32).to_le_bytes());
    traces.extend_from_slice(&trace);
    let rebuilt =
        NamedViews::new(archive.contents().clone(), archive.version(), limits())?.to_bytes()?;
    output.extend_from_slice(&version.to_le_bytes());
    output.push(scenario);
    output.extend_from_slice(&(rebuilt.len() as u32).to_le_bytes());
    output.extend_from_slice(&rebuilt);
    Ok(())
}

fn limits() -> DisplayViewLimits {
    DisplayViewLimits {
        maximum_bytes: 64 * 1024 * 1024,
        maximum_views: 1_000_000,
        maximum_nested_entries: 2_000_000,
    }
}

#[path = "support/view_trace.rs"]
mod view_trace;
