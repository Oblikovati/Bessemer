use super::{
    DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits, DisplayDocumentKind,
    DisplayFieldValue as Value, DisplayViewLimits,
    archive_decode::DisplayDecoder,
    archive_error::{invalid, unsupported},
};
use crate::Result;

pub(super) fn read(
    bytes: &[u8],
    context: &DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<DisplayArchiveContents> {
    require_context(context)?;
    let mut archive = DisplayDecoder::new(bytes, context.clone(), limits)?;
    let format = archive.i32("dl_format")?;
    if format != 1 {
        return Err(invalid(format!("display-list format {format}; expected 1")));
    }
    let views = archive.count("view_count", 78)?;
    root_environment(&mut archive)?;
    for index in 0..views {
        archive.scope(&format!("views[{index}]"), view)?;
    }
    if archive.i32("has_shaded_data")? != 0 {
        shaded(&mut archive)?;
    }
    archive.finish()
}

fn view_bodies(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let limits = view_limits(archive.limits);
    let version = archive.context.version;
    let mut budget = super::view_support::ViewBudget::new(limits);
    let named = super::named_read::read_views(&mut archive.cursor, version, &mut budget)?;
    archive.field("named_view_transform", Value::NamedViews(Some(named)))?;
    let spatial = super::spatial_read::read_views(&mut archive.cursor, version, limits)?;
    archive.field("view_3d_transform", Value::SpatialViews(Some(spatial)))?;
    archive.f64("orig_font_scale")?;
    let annotations = super::annotation_read::read_contents(&mut archive.cursor, version, limits)?;
    archive.field(
        "annotation_view_transform",
        Value::AnnotationViews(Some(annotations)),
    )
}

pub(super) fn view_limits(limits: DisplayArchiveLimits) -> DisplayViewLimits {
    DisplayViewLimits {
        maximum_bytes: limits.maximum_bytes,
        maximum_views: limits.maximum_entries,
        maximum_nested_entries: limits.maximum_entries,
    }
}

fn view(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("view_index")?;
    archive.transform("model_view_xform")?;
    archive.transform("pan_zoom_xform")?;
    archive.text("camera_name")?;
    archive.i32s(&["is_active_view", "display_mode", "edges_in_shaded_mode"])?;
    archive.object("view_bucket", &["uiViewBucket_c"])?;
    archive.scope("display_items", super::modern_items::list)
}

fn shaded(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    old_lights(archive)?;
    archive.object("visual_properties", &["moVisualProperties_c"])?;
    archive.scope("tessellation", tessellation)?;
    display_scene(archive)?;
    if archive.context.kind == DisplayDocumentKind::Assembly {
        archive.u32("envelope_system_options.unknown_color")?;
        archive.i32("envelope_system_options.unknown_setting")?;
    }
    ink_dimensions(archive)?;
    let count = archive.i32("configurations.config_count")?;
    if count > 0 {
        return Err(unsupported(format!(
            "display configuration count {count}; populated configuration tails are pending implementation"
        )));
    }
    Ok(())
}

fn require_context(context: &DisplayArchiveContext) -> Result<()> {
    if !(18000..=19000).contains(&context.version.value()) || context.file_version != 19000 {
        return Err(unsupported(format!(
            "display context version {}, file version {}, kind {:?}; expected display version 18000..=19000 and file version 19000",
            context.version.value(),
            context.file_version,
            context.kind
        )));
    }
    Ok(())
}

fn tessellation(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    match archive.context.kind {
        DisplayDocumentKind::Part => super::modern_part_tess::read(archive),
        DisplayDocumentKind::Assembly => archive.object_array(
            "components",
            &["uoTempAssemblySHDData_c", "uoTempSubAssemblySHDData_c"],
        ),
    }
}

fn root_environment(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.bounds("bbox")?;
    archive.f64("max_model_size")?;
    archive.object("user_model_env", &["uiUserModelEnv_c"])?;
    archive.object("line_font_mgr", &["uiLineFontMgr_c"])?;
    archive.transform("front_view_transform")?;
    view_bodies(archive)
}

fn old_lights(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let present = archive.i32("old_lights.has_lights")?;
    if present != 0 {
        archive.object_array(
            "old_lights.items",
            &[
                "moAmbientLight_c",
                "moLightFeature_c",
                "moDirectionLight_c",
                "moSpotLight_c",
                "moPointLight_c",
            ],
        )?;
    }
    Ok(())
}

fn display_scene(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let planes = archive.count("textured_planes.count", 36)?;
    if planes != 0 {
        return Err(unsupported(format!(
            "textured plane count {planes}; populated textured planes are pending implementation"
        )));
    }
    archive.i32("detail_mask_12")?;
    let scene = archive.i32("scene_type")?;
    if scene == 1 || scene == 2 {
        archive.scope("scene", super::modern_scene::read)?;
    }
    archive.i32s(&[
        "units_of_measurement.unknown_flag",
        "units_of_measurement.length_units",
        "units_of_measurement.angle_units",
    ])?;
    Ok(())
}

fn ink_dimensions(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let ink = archive.count("markup_ink_views.view_count", 6)?;
    // #30: the configuration dimension has no row payload when the ink-view count is zero.
    let configs = archive.i32("markup_ink_views.configuration_count")?;
    if configs < 0 {
        return Err(invalid(format!(
            "ink configuration count {configs}; expected nonnegative dimension"
        )));
    }
    archive.entries(configs as usize, "ink configuration dimension")?;
    if ink != 0 {
        return Err(unsupported(format!(
            "ink view count {ink}, config count {configs}; ink bodies are pending implementation"
        )));
    }
    Ok(())
}
