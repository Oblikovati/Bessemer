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
        shaded(&mut archive, views)?;
    }
    archive.finish()
}

fn view_bodies(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    named_transform(archive)?;
    spatial_transform(archive)?;
    if archive.context.version.value() > 0x3331 {
        archive.f64("orig_font_scale")?;
    }
    annotation_transform(archive)
}

fn named_transform(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let limits = view_limits(archive.limits);
    let version = archive.context.version;
    let mut budget = super::view_support::ViewBudget::new(limits);
    let views = if version.value() > 0x1be7 {
        Some(super::named_read::read_views(
            &mut archive.cursor,
            version,
            &mut budget,
        )?)
    } else {
        None
    };
    archive.field("named_view_transform", Value::NamedViews(views))
}

fn spatial_transform(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let limits = view_limits(archive.limits);
    let version = archive.context.version;
    let views = if version.value() > 0x1c18 {
        Some(super::spatial_read::read_views(
            &mut archive.cursor,
            version,
            limits,
        )?)
    } else {
        None
    };
    archive.field("view_3d_transform", Value::SpatialViews(views))
}

fn annotation_transform(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let limits = view_limits(archive.limits);
    let version = archive.context.version;
    let views = if version.value() > 0x1c39 {
        Some(super::annotation_read::read_contents(
            &mut archive.cursor,
            version,
            limits,
        )?)
    } else {
        None
    };
    archive.field("annotation_view_transform", Value::AnnotationViews(views))
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
    if archive.context.version.value() > 0xbb6 {
        archive.text("camera_name")?;
    }
    archive.i32s(&["is_active_view", "display_mode", "edges_in_shaded_mode"])?;
    archive.object("view_bucket", &["uiViewBucket_c"])?;
    archive.scope("display_items", super::modern_items::list)
}

fn shaded(archive: &mut DisplayDecoder<'_>, views: usize) -> Result<()> {
    old_lights(archive)?;
    archive.object("visual_properties", &["moVisualProperties_c"])?;
    archive.scope("tessellation", tessellation)?;
    display_scene(archive)?;
    if archive.context.kind == DisplayDocumentKind::Assembly
        && archive.context.version.value() > 0x42da
    {
        archive.u32("envelope_system_options.unknown_color")?;
        archive.i32("envelope_system_options.unknown_setting")?;
    }
    archive.scope("markup_ink_views", super::markup_ink::read)?;
    configuration_footer(archive, views)
}

fn configuration_footer(archive: &mut DisplayDecoder<'_>, views: usize) -> Result<()> {
    let version = archive.context.version.value();
    let minimum = if archive.context.kind == DisplayDocumentKind::Assembly {
        0x2403
    } else {
        0x2b5e
    };
    if version <= minimum {
        return Ok(());
    }
    archive.scope("configurations", |archive| {
        super::configurations::read(archive, views)
    })
}

fn require_context(context: &DisplayArchiveContext) -> Result<()> {
    let version = context.version.value();
    require_window_independent(version)?;
    if version < 2388 {
        return Err(unsupported(format!(
            "display context version {}, file version {}, kind {:?}; expected display version >=2388",
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
        DisplayDocumentKind::Assembly => {
            archive.object_array("components", super::modern_assembly::ASSEMBLY_CLASSES)
        }
    }
}

fn require_window_independent(version: u32) -> Result<()> {
    if (0x1fae..0x1fcf).contains(&version) {
        return Err(unsupported(format!(
            "display version {version} ({version:#x}) depends on uncounted open-window transforms; expected a stream-independent layout outside 0x1FAE..=0x1FCE"
        )));
    }
    Ok(())
}

fn root_environment(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.bounds("bbox")?;
    if archive.context.version.value() > 0xfca {
        archive.f64("max_model_size")?;
    }
    archive.object("user_model_env", &["uiUserModelEnv_c"])?;
    archive.object("line_font_mgr", &["uiLineFontMgr_c"])?;
    if archive.context.version.value() > 0xc9c {
        archive.transform("front_view_transform")?;
    }
    view_bodies(archive)
}

pub(super) fn old_lights(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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
    archive.scope("textured_planes", super::textured_planes::read)?;
    if archive.context.version.value() > 0x11ed {
        archive.i32("detail_mask_12")?;
    }
    optional_scene_body(archive)?;
    if archive.context.version.value() <= 0x1fb2 {
        return Ok(());
    }
    archive.i32s(&[
        "units_of_measurement.unknown_flag",
        "units_of_measurement.length_units",
        "units_of_measurement.angle_units",
    ])?;
    Ok(())
}

fn optional_scene_body(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0xf5e {
        return Ok(());
    }
    let scene = archive.i32("scene_type")?;
    if scene == 1 || scene == 2 {
        archive.scope("scene", super::modern_scene::read)?;
    }
    Ok(())
}
