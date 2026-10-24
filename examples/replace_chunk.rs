//! Replaces a carrier's first chunk; the caller supplies semantically valid content.

use std::error::Error;

use bessemer::block_stream::{BlockStream, ChunkFraming};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, mode, replacement, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE prefixed|guid REPLACEMENT DESTINATION",
            arguments.len()
        )
        .into());
    };
    let framing = match mode.as_str() {
        "prefixed" => ChunkFraming::LengthPrefixed,
        "guid" => ChunkFraming::GuidOnly,
        _ => return Err(format!("framing {mode:?}; expected prefixed or guid").into()),
    };
    let mut carrier = BlockStream::from_bytes(&std::fs::read(source)?, framing, 128 * 1024 * 1024)?;
    carrier.replace_chunk(0, std::fs::read(replacement)?)?;
    std::fs::write(destination, carrier.to_bytes()?)?;
    Ok(())
}
