//! Forces typed reconstruction of a directory, block, or standalone facet archive.
use bessemer::tessellation::*;
use std::error::Error;

#[derive(Clone, Copy)]
enum ArchiveKind {
    Directory,
    Block,
    Facet,
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [version, mode, kind, paths @ ..] = arguments.as_slice() else {
        return Err("arguments absent; expected VERSION full|light directory|block|facet INPUT OUTPUT pairs".into());
    };
    let context = context(version, mode)?;
    let kind = archive_kind(kind)?;
    let (pairs, remainder) = paths.as_chunks::<2>();
    if pairs.is_empty() || !remainder.is_empty() {
        return Err(format!("{} paths; expected INPUT OUTPUT pairs", paths.len()).into());
    }
    for [source, destination] in pairs {
        std::fs::write(
            destination,
            rebuild(&std::fs::read(source)?, context, kind)?,
        )?;
    }
    println!("Rebuilt {} tessellation archives", pairs.len());
    Ok(())
}

fn context(version: &str, mode: &str) -> Result<TessellationContext, Box<dyn Error>> {
    let mode = match mode {
        "full" => FacetArchiveMode::Full,
        "light" => FacetArchiveMode::Lightweight,
        other => return Err(format!("archive mode {other:?}; expected full or light").into()),
    };
    Ok(TessellationContext {
        archive_version: ArchiveVersion::new(version.parse()?),
        mode,
    })
}

fn archive_kind(kind: &str) -> Result<ArchiveKind, Box<dyn Error>> {
    match kind {
        "directory" => Ok(ArchiveKind::Directory),
        "block" => Ok(ArchiveKind::Block),
        "facet" => Ok(ArchiveKind::Facet),
        other => Err(format!("archive kind {other:?}; expected directory, block or facet").into()),
    }
}

fn rebuild(
    bytes: &[u8],
    context: TessellationContext,
    kind: ArchiveKind,
) -> bessemer::Result<Vec<u8>> {
    let limits = corpus_limits();
    match kind {
        ArchiveKind::Directory => {
            let source = TessellationDirectory::from_bytes(bytes, context, limits)?;
            TessellationDirectory::new(context, source.contents().clone())?.to_bytes()
        }
        ArchiveKind::Block => {
            let source = FaceTessellationBlock::from_bytes(bytes, context, limits)?;
            FaceTessellationBlock::new(context, source.contents().clone())?.to_bytes()
        }
        ArchiveKind::Facet => {
            let source = FacetArchive::from_bytes(bytes, context, limits)?;
            FacetArchive::new(context, source.contents().clone())?.to_bytes()
        }
    }
}

fn corpus_limits() -> TessellationLimits {
    TessellationLimits {
        maximum_components: 1_000_000,
        maximum_faces: 1_000_000,
        maximum_array_elements: 100_000_000,
    }
}
