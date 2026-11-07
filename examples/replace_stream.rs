//! Low-level stream replacement; the caller supplies semantically valid content.

use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, stream, replacement, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE STREAM REPLACEMENT DESTINATION",
            arguments.len()
        )
        .into());
    };
    let mut document = bessemer::Document::from_bytes(&std::fs::read(PathBuf::from(source))?)?;
    document.replace_stream(stream, std::fs::read(PathBuf::from(replacement))?)?;
    std::fs::write(PathBuf::from(destination), document.to_bytes()?)?;
    Ok(())
}
