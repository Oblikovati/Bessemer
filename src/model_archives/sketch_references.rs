use super::{
    ModelDecoder, ModelObjectRole, component_references, model_references, sketch_backups,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) const HANDLES: &[&str] = &[
    "sgEntHandle",
    "sgLineHandle",
    "sgArcHandle",
    "sgPointHandle",
    "sgSplineHandle",
    "sgEllipseHandle",
    "sgTextHandle",
    "sg3DPlaneHandle",
    "sg3DSphereHandle",
    // 11.341 / C-383 binds both primary slots directly to sgEntHandle::Serialize.
    "sg3DParametricSurfaceHandle",
    "sgParabolaHandle",
    // 11.342 / C-384 separately binds these primary slots to the same serializer.
    "sgConicHandle",
    "sg3DCylinderHandle",
];

pub(super) fn handle(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.escaped_index("entity_index", 0x777f)?;
    reader.i32s(&["reference_id", "dimension_on_cm"])
}

pub(super) fn component_handle(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    component_references::base(reader, map)?;
    reader.i32("feature_id")?;
    reader.u32("feature_time")?;
    reader.object("entity_handle", map, ModelObjectRole::Declared(HANDLES))?;
    reader.u32("unknown_100")?;
    Ok(())
}

pub(super) fn external_entity(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    // #46: the declared wrapper family includes the earlier sketch/axis variants
    // and the fresh/cached single-face variant exposed by native case 351.
    reader.object(
        "reference",
        map,
        ModelObjectRole::Declared(model_references::WRAPPER_CLASSES),
    )?;
    for (name, class) in [
        ("view", "moDrView"),
        ("intersection", "moSkIntersectInfo_c"),
        ("isocurve", "moSkIsoCurveInfo_c"),
    ] {
        reader.object(name, map, ModelObjectRole::Declared(&[class]))?;
    }
    reader.text("reference_axis_name")
}

pub(super) fn external_reference(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    model_references::wrapper(reader, map)?;
    let mode = reader.i16("mode")?;
    if !(-2..=0).contains(&mode) {
        return reader.checked(Err(super::invalid(format!(
            "sketch external-reference mode {mode}; expected -2..=0"
        ))));
    }
    reader.object(
        "backup",
        map,
        ModelObjectRole::Declared(sketch_backups::CLASSES),
    )
}

pub(super) fn isocurve_information(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.f64("ratio_08")?;
    for name in ["parameter_direction_10", "reference_type_18", "index_1c"] {
        reader.u16(name)?;
    }
    // #54 / C-386: the unaligned ID precedes +14; DB labels add no ordinary wire strings.
    reader.i32("dimension_id_20")?;
    reader.u16("skip_holes_14")?;
    Ok(())
}

pub(super) fn temporary_axis(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.f64s(&["unknown_a8", "unknown_b0"])?;
    double_vector(reader, "point_c0")?;
    double_vector(reader, "vector_d8")?;
    reader.u8("unknown_f0")?;
    if reader.version() > 0x371d {
        double_vector(reader, "point_f8")?;
        double_vector(reader, "point_110")?;
        reader.i32("unknown_128")?;
    }
    Ok(())
}

pub(super) fn silhouette_vertex(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    super::boundary_references::silhouette_prefix(reader, map)?;
    reader.f64s(&["unknown_1b0", "unknown_1b8"])?;
    reader.u32("flags_1c8")?;
    Ok(())
}

pub(super) fn intersection_info(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.u16("x_type")?;
    reader.i32s(&["edge_count", "edge_index", "x_count", "x_index"])
}
