//! SolidWorks display-list archive codecs, under development for clause 13.
mod transform;
mod transform_codec;
mod transform_types;

pub use crate::archive_codec::StoredDouble;
pub use transform::DisplayTransform;
pub use transform_types::{
    CompactTransform, FixedTransform, PerspectiveBase, PerspectivePayload, TransformContents,
    TransformEncoding,
};

mod named_types;
mod version;
pub use crate::archive_codec::{ArchiveString, TextEncoding};
pub use named_types::{
    DisplayViewLimits, NamedSectionPlane, NamedSectionSelections, NamedSelectionValues, NamedView,
};
pub use version::DisplayVersion;

mod named_read;
mod output;
mod view_support;

mod named_views;
mod named_write;
pub use named_views::NamedViews;

mod view_envelope;

mod annotation_read;
mod annotation_types;
mod annotation_views;
mod annotation_write;
pub use annotation_types::{
    AnnotationSectionPlane, AnnotationView, AnnotationViewContents, AnnotationViewId,
};
pub use annotation_views::AnnotationViews;

mod view_plane;
pub use view_plane::ViewSectionPlane;
