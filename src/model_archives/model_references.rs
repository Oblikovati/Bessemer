use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

// #58: these concrete readers already preserve the documented inherited wrapper body.
pub(super) const WRAPPER_CLASSES: &[&str] = &[
    "moRefWrapper_c",
    "moGeneralSurfRef_w",
    "moGeneralCurveRef_w",
    "moSingleFaceRef_w",
    "moCircleRef_w",
    "moCylinderRef_w",
    "moSphereRef_w",
    "moPointRef_w",
    "moLineRef_w",
    "moLoopRef_w",
    "moSketchExtRef_w",
    "moTempAxisRef_w",
    "moCoordSysRef_w",
];

// The ordinary wrapper's explicit component family follows the scoped 11.16 reader.
// Unknown classes are not admitted through name prefixes or structural guesses.
const COMPONENTS: &[&str] = &[
    "moCompBody_c",
    "moCompSolidBody_c",
    "moCompSurfaceBody_c",
    "moCompRef_c",
    "moCompEntGroup_c",
    "moCompMultiEdge_c",
    "moCompEdge_c",
    "moCompFace_c",
    "moCompLoop_c",
    "moCompGlobalVar_c",
    "moFeatureDimHandle_c",
    "moSkDimHandleRadial_c",
    "moSkDimHandleOffset_c",
    "moSkDimHandleValG2_c",
    "moSkDimHandleValG3_c",
    "moCompSketchEntHandle_c",
    "moCompSilhVertex_c",
    "moCompRefPlane_c",
    "moCompRefSurface_c",
    "moCompRefAxis_c",
    "moCompRefPoint_c",
    "moCompSilhEdge_c",
    "moCompVertex_c",
    "moCompFeature_c",
    "moCompBodyFeature_c",
    "moCompProfile_c",
    "moCompReferenceCurve_c",
    "moCompCoordSys_c",
    "moCompHolder_c",
];

pub(super) fn wrapper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object(
        "component_reference",
        map,
        ModelObjectRole::Declared(COMPONENTS),
    )?;
    reader.u16("unknown_60")?;
    reader.object("component", map, ModelObjectRole::Any)?;
    reader.i32s(&["unknown_64", "unknown_08"])?;
    optional_transforms(reader)?;
    reader.i32s(&["unknown_0c", "unknown_20", "unknown_6c", "unknown_70"])?;
    if reader.version() > 0x2faf {
        reader.i32("unknown_74")?;
    }
    reader.i32s(&["unknown_4c", "unknown_68", "unknown_34", "unknown_30"])?;
    final_words(reader)
}

fn optional_transforms(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for offset in ["10", "18"] {
        if reader.u16(&format!("has_transform_{offset}"))? != 0 {
            reader.transform(&format!("transform_{offset}"))?;
        }
    }
    Ok(())
}

fn final_words(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() >= 0x2b73 {
        reader.i32("unknown_90")?;
    }
    if reader.version() >= 0x3b02 {
        reader.i32("unknown_38")?;
    }
    Ok(())
}

pub(super) fn dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.f64("unknown_a8")?;
    wrapper(reader, map)
}

pub(super) fn global_variable(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    wrapper(reader, map)?;
    reader.text("unknown_a8")?;
    Ok(())
}

pub(super) fn component_transform(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    wrapper(reader, map)?;
    reader.transform("transform_a8")?;
    reader.transform("transform_130")?;
    Ok(())
}
