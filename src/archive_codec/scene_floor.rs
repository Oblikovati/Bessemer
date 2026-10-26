use super::context_reader::ArchiveGraphRead;
use crate::Result;

pub(crate) fn read(archive: &mut impl ArchiveGraphRead) -> Result<()> {
    archive.i32("floor_dir")?;
    archive.f64("floor_offset")?;
    archive.i32("floor_offset_dir")?;
    archive.f64s(&["floor_rotation", "env_rotation"])?;
    archive.i32("transparent_shadow_floor")?;
    archive.f32("floor_reflectivity")?;
    archive.i32s(&[
        "resize_automatic",
        "ambient_occlusion",
        "background_fit_to_camera_fov",
    ])?;
    if archive.version() > 0x1211 {
        archive.i32("flags")?;
    }
    historical_floor(archive)?;
    field_of_view(archive)
}

fn historical_floor(archive: &mut impl ArchiveGraphRead) -> Result<()> {
    let version = archive.version();
    if version > 0x1827 {
        archive.i32("flatten_floor")?;
        archive.f64("environment_size")?;
    }
    if version > 0x1841 {
        archive.f64("flattened_floor_height")?;
    }
    if version > 0x185e {
        archive.i32("accurate_environment_lighting")?;
    }
    if version > 0x186c {
        archive.graph_transform("flatten_floor_xform")?;
    }
    Ok(())
}

fn field_of_view(archive: &mut impl ArchiveGraphRead) -> Result<()> {
    if archive.version() > 0x42ec {
        archive.f64("hdr_exposure")?;
        archive.i32("dspbr_tone_map")?;
    }
    if archive.version() > 0x46ba {
        archive.f64("field_of_view")?;
    }
    Ok(())
}
