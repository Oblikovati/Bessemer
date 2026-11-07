//! Changes the allocation counter in one selected header stream.
use bessemer::header::{ArchiveVersion, DocumentHeader, HeaderObject};
use bessemer::{BessemerError, Document, FailureClass, Result};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, version, stream, next_id, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE VERSION HEADER_STREAM NEXT_ID DESTINATION",
            arguments.len()
        )
        .into());
    };
    let version = ArchiveVersion::new(version.parse()?);
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let bytes = document.read_stream_limited(stream, 64 * 1024 * 1024)?;
    document.replace_stream(stream, updated_next_id(&bytes, version, next_id.parse()?)?)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn updated_next_id(bytes: &[u8], version: ArchiveVersion, next_id: i32) -> Result<Vec<u8>> {
    let header = DocumentHeader::from_bytes(bytes, version, 100_000)?;
    let mut contents = header.contents().clone();
    let node = contents
        .objects
        .get_mut(&contents.root)
        .ok_or_else(|| missing_root(contents.root.value()))?;
    let HeaderObject::Header(record) = &mut node.object else {
        return Err(missing_root(contents.root.value()));
    };
    record.next_id = next_id;
    header.with_contents(contents)?.to_bytes()
}

fn missing_root(identity: u32) -> BessemerError {
    BessemerError {
        operation: "set header next ID",
        entity: format!("object {identity}"),
        class: FailureClass::Invalid,
        cause: "root body missing; expected validated moHeader_c".to_owned(),
    }
}
