//! Edits the first history stamp's comment in an existing SolidWorks document.

use std::error::Error;

use bessemer::history::{ArchiveString, TextEncoding, VersionHistory};
use bessemer::{BessemerError, Document, FailureClass, Result};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, stream, comment, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE HISTORY_STREAM COMMENT DESTINATION",
            arguments.len()
        )
        .into());
    };
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let history = document.read_stream_limited(stream, 64 * 1024 * 1024)?;
    document.replace_stream(stream, updated_comment(&history, comment)?)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn updated_comment(bytes: &[u8], comment: &str) -> Result<Vec<u8>> {
    let source = VersionHistory::from_bytes(bytes)?;
    let mut contents = source.contents().clone();
    let first = contents.states.first_mut().ok_or_else(|| BessemerError {
        operation: "set history comment",
        entity: "History states".into(),
        class: FailureClass::Invalid,
        cause: "state count 0; expected at least one state".into(),
    })?;
    first.stamp.comment = ArchiveString::new(comment, TextEncoding::Utf16)?;
    source.with_contents(contents)?.to_bytes()
}
