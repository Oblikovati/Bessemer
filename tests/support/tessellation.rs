// Each integration crate exercises a different subset of these owned fixtures.
#![allow(dead_code)]
use bessemer::tessellation::*;

pub const MODERN: &[u8] = include_bytes!("../fixtures/tessellation/facet-4a38-1.bin");
pub const BLOCK: &[u8] = include_bytes!("../fixtures/tessellation/block.bin");
pub const DIRECTORY: &[u8] = include_bytes!("../fixtures/tessellation/directory-3b23.bin");
pub const LIMITS: TessellationLimits = TessellationLimits {
    maximum_components: 100,
    maximum_faces: 100,
    maximum_array_elements: 1000,
};

pub fn context(version: u32, mode: FacetArchiveMode) -> TessellationContext {
    TessellationContext {
        archive_version: ArchiveVersion::new(version),
        mode,
    }
}

pub fn modern_context() -> TessellationContext {
    context(19000, FacetArchiveMode::Full)
}

pub fn parse(bytes: &[u8]) -> FacetArchive {
    FacetArchive::from_bytes(bytes, modern_context(), LIMITS).unwrap()
}

pub fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/tessellation")
            .join(name),
    )
    .unwrap()
}

pub fn directory() -> TessellationDirectory {
    TessellationDirectory::from_bytes(DIRECTORY, modern_context(), LIMITS).unwrap()
}

pub fn block() -> FaceTessellationBlock {
    FaceTessellationBlock::from_bytes(BLOCK, modern_context(), LIMITS).unwrap()
}
