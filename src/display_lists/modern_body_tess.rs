use super::{DisplayArchiveContext, DisplayFieldValue, archive_decode::DisplayDecoder};
use crate::{
    Result,
    tessellation::{ArchiveVersion, FacetArchiveMode, TessellationContext},
};

pub(super) fn facet_context(context: &DisplayArchiveContext) -> TessellationContext {
    TessellationContext {
        archive_version: ArchiveVersion::new(context.version.value()),
        mode: FacetArchiveMode::Full,
    }
}

pub(super) fn body(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object_array("faces", &["uoTempFaceTessData_c"])?;
    archive.i32s(&["unknown_20", "unknown_24", "unknown_2c"])?;
    let count = archive.count("body_prop_infos.count", 6)?;
    for index in 0..count {
        archive.scope(&format!("body_prop_infos[{index}]"), |archive| {
            archive.i32("key")?;
            archive.object("value", &["uoBodyPropInfo_c"])
        })?;
    }
    super::modern_decals::empty_list(archive, "decals")?;
    if archive.context.version.value() > 0x2017 {
        archive.text("unknown_str")?;
    }
    if archive.context.version.value() > 0x2791 {
        archive.i32("unknown_68")?;
    }
    Ok(())
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
    let count = archive.count("face_viz_props.count", 6)?;
    for index in 0..count {
        archive.scope(&format!("face_viz_props[{index}]"), |archive| {
            archive.i32("key")?;
            archive.object("value", &["moVisualProperties_c"])
        })?;
    }
    archive.i32s(&["unknown_34", "unknown_68"])?;
    let count = archive.count("unknown_58.count", 8)?;
    for index in 0..count {
        archive.i32s(&[
            &format!("unknown_58[{index}][0]"),
            &format!("unknown_58[{index}][1]"),
        ])?;
    }
    face_identity(archive)
}

fn face_identity(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unknown_38")?;
    if archive.context.version.value() <= 0x2bcf {
        return Ok(());
    }
    if archive.i32("face_string_empty")? == 0 {
        archive.text("face_string")?;
    }
    archive.i32s(&["face_pers_id", "unknown_78"])
}

fn inline_facet(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let context = facet_context(&archive.context);
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
