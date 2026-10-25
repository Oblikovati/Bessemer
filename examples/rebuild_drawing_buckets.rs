//! Forces complete drawing VBLists reconstruction for independent corpus verification.
use bessemer::display_lists::{DisplayArchiveLimits, DrawingBucketArchive};
use std::error::Error;
#[path = "support/archive_scalar_trace.rs"]
mod archive_scalar_trace;
#[path = "support/drawing_bucket_trace.rs"]
mod drawing_bucket_trace;
#[path = "support/framing.rs"]
mod framing;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let [mode, input, output] = args.as_slice()
        && mode == "--document"
    {
        std::fs::write(output, rebuild_document(&std::fs::read(input)?)?)?;
        return Ok(());
    }
    let [input, output, trace] = args.as_slice() else {
        return Err(format!("arguments {args:?}; expected INPUT OUTPUT TRACE").into());
    };
    let (bytes, fields) = rebuild(&std::fs::read(input)?)?;
    std::fs::write(output, bytes)?;
    std::fs::write(trace, fields)?;
    Ok(())
}

fn rebuild_document(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut document = bessemer::Document::from_bytes(bytes)?;
    let archive = DrawingBucketArchive::from_document(&document, limits())?;
    let forced = DrawingBucketArchive::new(
        archive.contents().clone(),
        archive.model_version(),
        limits(),
    )?;
    document.replace_stream("Contents/VBLists", forced.to_bytes()?)?;
    let output = document.to_bytes()?;
    let reopened = bessemer::Document::from_bytes(&output)?;
    let reparsed = DrawingBucketArchive::from_document(&reopened, limits())?;
    if reparsed.contents() != forced.contents() {
        return Err("reopened drawing fields differ; expected the forced contents".into());
    }
    Ok(output)
}

fn limits() -> DisplayArchiveLimits {
    DisplayArchiveLimits {
        maximum_bytes: 64 * 1024 * 1024,
        maximum_fields: 1_000_000,
        maximum_objects: 100_000,
        maximum_depth: 128,
        maximum_entries: 1_000_000,
    }
}

fn rebuild(input: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let mut rest = input;
    let count = framing::take_u32(&mut rest)?;
    framing::require_count(count, rest.len(), 4)?;
    let mut output = count.to_le_bytes().to_vec();
    let mut traces = output.clone();
    for _ in 0..count {
        let size = framing::take_u32(&mut rest)? as usize;
        let (bytes, fields) = force_record(framing::take(&mut rest, size)?)?;
        append(&mut output, &bytes)?;
        append(&mut traces, &fields)?;
    }
    framing::require_finished(rest)?;
    Ok((output, traces))
}

fn force_record(mut record: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let version = framing::take_u32(&mut record)?;
    let version = bessemer::header::ArchiveVersion::new(version);
    let archive = DrawingBucketArchive::from_bytes(record, version, limits())?;
    let forced = DrawingBucketArchive::new(archive.contents().clone(), version, limits())?;
    let mut output = version.value().to_le_bytes().to_vec();
    output.extend(forced.to_bytes()?);
    Ok((output, drawing_bucket_trace::trace(&forced)))
}

fn append(output: &mut Vec<u8>, value: &[u8]) -> Result<(), Box<dyn Error>> {
    let count = u32::try_from(value.len())?;
    output.extend(count.to_le_bytes());
    output.extend(value);
    Ok(())
}
