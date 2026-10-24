//! Reconstructs standalone biography archives from typed fields.
use bessemer::biography::BiographyArchive;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let abbreviate = arguments
        .first()
        .is_some_and(|argument| argument == "--abbreviate");
    let paths = if abbreviate {
        &arguments[1..]
    } else {
        &arguments
    };
    let (pairs, remainder) = paths.as_chunks::<2>();
    if pairs.is_empty() || !remainder.is_empty() {
        return Err(format!(
            "{} arguments; expected [--abbreviate] INPUT OUTPUT pairs",
            arguments.len()
        )
        .into());
    }
    for [source, destination] in pairs {
        rebuild_pair(source, destination, abbreviate)?;
    }
    println!("Rebuilt {} biographies", pairs.len());
    Ok(())
}

fn rebuild_pair(source: &str, destination: &str, abbreviate: bool) -> Result<(), Box<dyn Error>> {
    let parsed = BiographyArchive::from_bytes(&std::fs::read(source)?)?;
    let records = if abbreviate {
        parsed.abbreviated_records()?
    } else {
        parsed.records().to_vec()
    };
    let rebuilt = BiographyArchive::new(parsed.root_schema(), records)?;
    std::fs::write(destination, rebuilt.to_bytes()?)?;
    Ok(())
}
