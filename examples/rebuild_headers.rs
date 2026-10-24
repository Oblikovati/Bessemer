//! Rebuilds extracted headers from typed fields with an explicit archive version.

use bessemer::header::{ArchiveVersion, DocumentHeader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (triples, remainder) = arguments.as_chunks::<3>();
    if triples.is_empty() || !remainder.is_empty() {
        return Err(format!(
            "{} arguments; expected VERSION INPUT OUTPUT triples",
            arguments.len()
        )
        .into());
    }
    for [version, source, destination] in triples {
        let version = ArchiveVersion::new(version.parse()?);
        let parsed = DocumentHeader::from_bytes(&std::fs::read(source)?, version, 1_000_000)?;
        let rebuilt = DocumentHeader::new(version, parsed.contents().clone())?;
        std::fs::write(destination, rebuilt.to_bytes()?)?;
    }
    println!("Rebuilt {} headers", triples.len());
    Ok(())
}
