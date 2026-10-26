mod cursor;
mod dspbr;
mod geometry;
pub(crate) mod light_payload;
pub(crate) mod line_style;
pub(crate) mod material;
pub(crate) mod material_properties;
mod material_reader;
mod number;
mod object_framing;
mod object_roles;
mod objects;
mod preserved;
mod text;
pub(crate) mod transform_codec;
pub(crate) mod transform_types;
pub(crate) mod vectors;
pub(crate) mod visibility;

pub(crate) use cursor::ArchiveCursor;
pub(crate) use material_reader::MaterialRead;
pub(crate) use object_framing::{ObjectClassView, write_object_framing};
pub(crate) use object_roles::{ArchiveObjectRole, require_object_role};
#[cfg(test)]
pub(crate) use objects::FramedObjectRecord;
pub(crate) use objects::{
    ArchiveClass, ArchiveMapIndex, ObjectFraming, ObjectReader, ObjectRecord, ObjectWriter,
};
pub(crate) use preserved::PreservedContents;
pub use text::{ArchiveString, TextEncoding};

use crate::{BessemerError, FailureClass};

pub(crate) fn malformed(cause: impl Into<String>) -> BessemerError {
    BessemerError::new("archive codec", "archive", FailureClass::Invalid, cause)
}

pub use number::{StoredDouble, StoredFloat};
pub use transform_types::{
    CompactTransform, FixedTransform, PerspectiveBase, PerspectivePayload, TransformContents,
    TransformEncoding,
};

pub use geometry::HeaderBox;
pub(crate) use geometry::{read_box, read_point, write_box};
