//! Forces reconstruction of independently framed transform-corpus records.
use bessemer::display_lists::{DisplayTransform, TransformEncoding};
use std::error::Error;
mod support;
use support::framing::{require_count, require_finished, take, take_u32};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, destination] = arguments.as_slice() else {
        return Err(format!("{} arguments; expected INPUT OUTPUT", arguments.len()).into());
    };
    let output = rebuild(&std::fs::read(source)?)?;
    std::fs::write(destination, output)?;
    Ok(())
}

fn rebuild(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut remaining = bytes;
    let count = take_u32(&mut remaining)?;
    let mut output = count.to_le_bytes().to_vec();
    require_count(count, remaining.len(), 5)?;
    for _ in 0..count {
        rebuild_record(&mut remaining, &mut output)?;
    }
    require_finished(remaining)?;
    Ok(output)
}

fn rebuild_record(remaining: &mut &[u8], output: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    let kind = take(remaining, 1)?[0];
    let encoding = match kind {
        0 => TransformEncoding::Compact,
        1 => TransformEncoding::Fixed,
        other => return Err(format!("transform encoding {other}; expected 0 or 1").into()),
    };
    let length = take_u32(remaining)? as usize;
    let transform = DisplayTransform::from_bytes(take(remaining, length)?, encoding)?;
    let rebuilt = DisplayTransform::new(transform.contents().clone())?.to_bytes();
    output.push(kind);
    output.extend_from_slice(&(rebuilt.len() as u32).to_le_bytes());
    output.extend_from_slice(&rebuilt);
    Ok(())
}
