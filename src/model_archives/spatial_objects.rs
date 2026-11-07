use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn curvature(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64s(&[
        "curvatures[0]",
        "curvatures[1]",
        "curvatures[2]",
        "curvatures[3]",
        "curvatures[4]",
    ])?;
    reader.u32("time_low")?;
    reader.u32("time_high")?;
    Ok(())
}

pub(super) fn reference_plane(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("unknown_08")?;
    reader.i32("unknown_0c")?;
    reader.u32("unknown_10")?;
    reader.u32("back_face_color")?;
    reader.i32("unknown_18")?;
    reader.u32("unknown_1c")?;
    Ok(())
}
