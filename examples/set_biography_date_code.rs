//! Changes the first biography record's stored product build date code.
use bessemer::biography::BiographyArchive;
use bessemer::{BessemerError, Document, FailureClass, Result};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, stream, date_code, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE BIOGRAPHY_STREAM DATE_CODE DESTINATION",
            arguments.len()
        )
        .into());
    };
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let bytes = document.read_stream_limited(stream, 64 * 1024 * 1024)?;
    document.replace_stream(stream, updated_date_code(&bytes, date_code.parse()?)?)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn updated_date_code(bytes: &[u8], date_code: i32) -> Result<Vec<u8>> {
    let archive = BiographyArchive::from_bytes(bytes)?;
    let mut records = archive.records().to_vec();
    let first = records.first_mut().ok_or_else(|| BessemerError {
        operation: "set biography date code",
        entity: "biography record 0".to_owned(),
        class: FailureClass::Missing,
        cause: "record count 0; expected at least one biography record".to_owned(),
    })?;
    first.date_code = date_code;
    archive.with_records(records)?.to_bytes()
}
