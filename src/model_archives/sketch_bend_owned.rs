use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Caller-declared kind-7 owned layout; this does not certify native runtime objects.
/// Example: `context.with_sketch_bend_owned_profile(SketchBendOwnedProfile::CanonicalRuledGrid)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SketchBendOwnedProfile {
    /// Normal canonical nonnull ruled-face/grid construction with ordinary arrays (flag 0x10 clear).
    CanonicalRuledGrid,
}

/// Retain the declared canonical delegate and its separate final word; e.g. `read_face(reader, map)?`.
pub(super) fn read_face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("grid_58", |reader| read_grid(reader, map))?;
    reader.i32("raw_138")?;
    Ok(())
}

fn read_grid(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_b0")?;
    double_vector(reader, "vector_b8")?;
    double_vector(reader, "vector_d0")?;
    for offset in ["118", "138"] {
        model_lists::pointers(
            reader,
            map,
            &format!("array_{offset}"),
            ModelObjectRole::Any,
        )?;
    }
    reader.f64s(&["raw_e8", "raw_f0"])
}
