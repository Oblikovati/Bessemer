use super::{DisplayArchiveContext, DisplayFieldValue, archive_decode::DisplayDecoder};
use crate::{
    Result,
    tessellation::{ArchiveVersion, FacetArchiveMode, TessellationContext},
};

pub(super) fn facet_context(context: &DisplayArchiveContext, drawing: bool) -> TessellationContext {
    TessellationContext {
        archive_version: ArchiveVersion::new(context.version.value()),
        mode: if drawing && context.version.value() >= 0x2be5 {
            FacetArchiveMode::Lightweight
        } else {
            FacetArchiveMode::Full
        },
    }
}

pub(super) fn body(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object_array("faces", &["uoTempFaceTessData_c"])?;
    body_metadata(archive)?;
    body_property_map(archive)?;
    super::modern_decals::list(archive, "decals")?;
    if archive.context.version.value() > 0x2017 {
        archive.text("unknown_str")?;
    }
    if archive.context.version.value() > 0x2791 {
        archive.i32("unknown_68")?;
    }
    Ok(())
}

fn body_property_map(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0xb4b {
        return Ok(());
    }
    if archive.context.version.value() <= 0x10ae {
        return single_visual_property(archive, "body_prop_infos[0].value.viz_prop");
    }
    let count = archive.count("body_prop_infos.count", 6)?;
    for index in 0..count {
        archive.scope(&format!("body_prop_infos[{index}]"), |archive| {
            archive.i32("key")?;
            body_property_pointer(archive)
        })?;
    }
    Ok(())
}

fn body_property_pointer(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x10f6 {
        return archive.object("value.viz_prop", &["moVisualProperties_c"]);
    }
    archive.object("value", &["uoBodyPropInfo_c"])
}

pub(super) fn properties(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object("viz_prop", &["moVisualProperties_c"])?;
    archive.i32s(&[
        "unknown_8",
        "display_mode",
        "unknown_10",
        "hlr_display_quality",
    ])
}

pub(super) fn face(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    face_source(archive)?;
    face_visual_properties(archive)?;
    if archive.context.version.value() >= 0xfd8 {
        archive.i32("unknown_34")?;
    }
    if archive.context.version.value() >= 0x117d {
        archive.i32("unknown_68")?;
    }
    face_pairs(archive)?;
    face_identity(archive)
}

fn face_visual_properties(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x10ae {
        return single_visual_property(archive, "face_viz_props[0].value");
    }
    let count = archive.count("face_viz_props.count", 6)?;
    for index in 0..count {
        archive.scope(&format!("face_viz_props[{index}]"), |archive| {
            archive.i32("key")?;
            archive.object("value", &["moVisualProperties_c"])
        })?;
    }
    Ok(())
}

pub(super) fn single_visual_property(archive: &mut DisplayDecoder<'_>, path: &str) -> Result<()> {
    // The native -1 key exists only in memory; retain the actual pointer without a fake key/count.
    archive.entries(1, path)?;
    archive.object(path, &["moVisualProperties_c"])
}

fn face_pairs(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x1203 {
        return Ok(());
    }
    let count = archive.count("unknown_58.count", 8)?;
    for index in 0..count {
        archive.i32s(&[
            &format!("unknown_58[{index}][0]"),
            &format!("unknown_58[{index}][1]"),
        ])?;
    }
    Ok(())
}

fn face_identity(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() >= 0x1401 {
        archive.i32("unknown_38")?;
    }
    if archive.context.version.value() <= 0x2bcf {
        return Ok(());
    }
    if archive.i32("face_string_empty")? == 0 {
        archive.text("face_string")?;
    }
    if archive.context.version.value() > 0x336b {
        archive.i32("face_pers_id")?;
    }
    if archive.context.version.value() >= 0x3743 {
        archive.i32("unknown_78")?;
    }
    Ok(())
}

fn inline_facet(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let context = facet_context(&archive.context, archive.drawing_buckets);
    let available = archive.available_entries();
    let (container, entries) =
        crate::tessellation::read_embedded(&mut archive.cursor, context, available)?;
    archive.entries(entries, "inline facet elements")?;
    archive.field(
        "facet_container",
        DisplayFieldValue::Facet(Box::new(container)),
    )
}

fn face_source(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.restoring_configurations {
        return inline_facet(archive);
    }
    let Some((component_id, index)) = &mut archive.external_component else {
        return inline_facet(archive);
    };
    let source = DisplayFieldValue::ExternalFacet {
        component_id: component_id.clone(),
        index: *index,
    };
    *index = index.checked_add(1).ok_or_else(|| {
        super::archive_error::invalid(format!(
            "external face index {index}; expected less than u32::MAX"
        ))
    })?;
    archive.field("facet_container", source)
}

fn body_metadata(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x506 {
        archive.i32("unknown_20")?;
    }
    if archive.context.version.value() >= 0x82b {
        archive.i32("unknown_24")?;
    }
    if archive.context.version.value() > 0x852 {
        archive.i32("unknown_2c")?;
    }
    Ok(())
}
