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

mod spatial_read;
mod spatial_types;
mod spatial_views;
mod spatial_write;
mod view_bitmap;
pub use spatial_types::{SpatialView, ViewBitmap};
pub use spatial_views::SpatialViews;

mod archive;
mod archive_decode;
mod archive_encode;
mod archive_error;
mod archive_objects;
mod archive_scalars;
mod archive_types;
mod archive_view_budget;
mod modern_assembly;
mod modern_body_tess;
mod modern_component;
mod modern_decals;
mod modern_dspbr;
mod modern_lights;
mod modern_material;
mod modern_part_tess;
mod modern_root;
mod modern_ui;
pub use archive::DisplayArchive;
pub use archive_types::{
    DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits, DisplayClassDescriptor,
    DisplayDocumentKind, DisplayField, DisplayFieldValue, DisplayObjectTag,
};

pub use archive_types::{DisplayObjectId, DisplayObjectPointer};

mod modern_concept;
mod modern_pw_scene;
mod modern_reference;
mod modern_scene;
mod modern_walls;
