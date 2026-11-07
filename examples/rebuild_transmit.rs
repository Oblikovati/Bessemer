//! Reconstructs transmit files from typed fields using an explicit base-schema file.
use bessemer::parasolid::{BaseSchema, TransmitFile, TransmitFormat, TransmitLimits};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let bare = arguments
        .first()
        .is_some_and(|argument| argument == "--bare");
    let paths = if bare { &arguments[1..] } else { &arguments };
    let (schema_path, pairs) = paths
        .split_first()
        .ok_or("arguments absent; expected [--bare] BASE_SCHEMA INPUT OUTPUT pairs")?;
    let (pairs, remainder) = pairs.as_chunks::<2>();
    if pairs.is_empty() || !remainder.is_empty() {
        return Err(format!(
            "{} paths; expected BASE_SCHEMA INPUT OUTPUT pairs",
            paths.len()
        )
        .into());
    }
    let base = BaseSchema::from_bytes(&std::fs::read(schema_path)?)?;
    for [source, destination] in pairs {
        rebuild_pair(&base, source, destination, bare)?;
    }
    println!("Rebuilt {} transmit files", pairs.len());
    Ok(())
}

fn rebuild_pair(
    base: &BaseSchema,
    source: &str,
    destination: &str,
    bare: bool,
) -> Result<(), Box<dyn Error>> {
    let limits = TransmitLimits {
        maximum_records: 10_000_000,
        maximum_field_elements: 100_000_000,
    };
    let parsed = TransmitFile::from_bytes(&std::fs::read(source)?, base, limits)?;
    let mut contents = parsed.contents().clone();
    if bare {
        contents.format = TransmitFormat::Bare;
    }
    let rebuilt = TransmitFile::new(base, contents)?;
    std::fs::write(destination, rebuilt.to_bytes()?)?;
    Ok(())
}
