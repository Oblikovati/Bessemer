//! Rebuilds extracted standalone histories from typed fields, without copying original bytes.

use std::error::Error;

use bessemer::history::VersionHistory;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (pairs, remainder) = arguments.as_chunks::<2>();
    if pairs.is_empty() || !remainder.is_empty() {
        return Err(format!(
            "{} arguments; expected one or more INPUT OUTPUT pairs",
            arguments.len()
        )
        .into());
    }
    for [source, destination] in pairs {
        let bytes = std::fs::read(source).map_err(|error| {
            format!("input {source:?}: {error}; expected readable history bytes")
        })?;
        let parsed = VersionHistory::from_bytes(&bytes)?;
        let rebuilt = VersionHistory::new(parsed.root_schema(), parsed.contents().clone())?;
        std::fs::write(destination, rebuilt.to_bytes()?).map_err(|error| {
            format!("output {destination:?}: {error}; expected writable history path")
        })?;
    }
    println!("Rebuilt {} histories", pairs.len());
    Ok(())
}
