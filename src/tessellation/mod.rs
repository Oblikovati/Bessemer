//! Complete face-tessellation archive fields and explicit full/lightweight context (clause 12).
mod arrays;
mod block;
mod codec;
mod context;
mod directory;
mod edges;
mod embedded;
mod envelope;
pub(crate) use embedded::{read_embedded, write_embedded};
mod facets;
mod groups;
mod indices;
mod layout;
mod links;
mod output;
mod record;

use crate::{BessemerError, FailureClass};
use envelope::archive_wrapper;

pub use crate::archive_codec::{
    ArchiveString, HeaderBox as FacetBounds, StoredDouble, StoredFloat, TextEncoding,
};
pub use crate::header::ArchiveVersion;
pub use arrays::{ArrayElementSize, DynamicArray, DynamicArrayValues};
pub use block::{FaceTessellationBlockContents, FacetBlockComponent};
pub use context::{FacetArchiveMode, TessellationContext, TessellationLimits};
pub use directory::{
    ComponentTessellationNumbers, PartComponentInfo, TessellationDirectoryContents,
    TessellationNumber,
};
pub use edges::{EdgeVertexPairs, FacetEdgeInfo, FacetEdgePoint, FacetEdgePoints};
pub use facets::{
    FacetConeCull, FacetContainer, FacetEdgeArrays, FacetLateStamps, FacetLegacyTriangles,
    FacetSurface, FacetTextureTessellation,
};
pub use groups::{
    ArrayPair, ByteFlaggedArray, FacetCurvature, FacetDisplayColor, FacetString, Flagged,
};
pub use indices::FacetTriangleIndices;
pub use layout::{FacetStyle, FacetVertexLayout};
pub use links::BlockLinkCoverage;

archive_wrapper! {
    /// Immutable versioned face-tessellation directory with exact no-op preservation.
    TessellationDirectory, TessellationDirectoryContents, "bessemer::tessellation::TessellationDirectory"
}
archive_wrapper! {
    /// Immutable complete block stream with ordered components and archived faces.
    FaceTessellationBlock, FaceTessellationBlockContents, "bessemer::tessellation::FaceTessellationBlock"
}
archive_wrapper! {
    /// Immutable standalone facet container with every documented version-dependent field.
    FacetArchive, FacetContainer, "bessemer::tessellation::FacetArchive"
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "face tessellation",
        entity: "tessellation archive".to_owned(),
        class: FailureClass::Invalid,
        cause: cause.into(),
    }
}
fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError {
        operation: "face tessellation",
        entity: "tessellation archive".to_owned(),
        class: FailureClass::Unsupported,
        cause: cause.into(),
    }
}
