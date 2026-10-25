use super::{
    AnnotationSectionPlane, AnnotationView, AnnotationViewContents, DisplayVersion,
    DisplayViewLimits,
    output::DisplayOutput,
    view_support::{ViewBudget, require_presence, require_size},
};
use crate::Result;

pub(super) fn encode(
    version: DisplayVersion,
    contents: &Option<AnnotationViewContents>,
    limits: DisplayViewLimits,
) -> Result<Vec<u8>> {
    require_presence(
        contents.is_some(),
        version.value() > 0x1c39,
        "annotation collection",
    )?;
    let mut output = DisplayOutput::new(limits.maximum_bytes);
    let Some(contents) = contents else {
        return Ok(output.finish());
    };
    write_contents(&mut output, contents, version, limits)?;
    Ok(output.finish())
}

fn write_contents(
    output: &mut DisplayOutput,
    contents: &AnnotationViewContents,
    version: DisplayVersion,
    limits: DisplayViewLimits,
) -> Result<()> {
    validate_contents(contents, version, limits)?;
    output.count(contents.views.len(), "annotation views")?;
    let mut budget = ViewBudget::new(limits);
    for view in &contents.views {
        write_view(output, view, version, &mut budget)?;
    }
    if let Some(bitset) = contents.unknown_bitset {
        for word in bitset {
            output.i32(word)?;
        }
    }
    Ok(())
}

fn validate_contents(
    contents: &AnnotationViewContents,
    version: DisplayVersion,
    limits: DisplayViewLimits,
) -> Result<()> {
    require_size(
        contents.views.len(),
        limits.maximum_views,
        "annotation views",
    )?;
    require_presence(
        contents.unknown_bitset.is_some(),
        version.value() > 0x1fa8,
        "annotation bitset",
    )?;
    Ok(())
}

fn write_view(
    output: &mut DisplayOutput,
    view: &AnnotationView,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<()> {
    write_identity(output, view, version)?;
    output.compact(&view.model_to_view)?;
    output.compact(&view.pan_zoom)?;
    output.string(&view.unknown_string)?;
    budget.nested(view.section_planes.len(), "annotation planes")?;
    output.count(view.section_planes.len(), "annotation planes")?;
    for plane in &view.section_planes {
        write_plane(output, plane)?;
    }
    Ok(())
}

fn write_identity(
    output: &mut DisplayOutput,
    view: &AnnotationView,
    version: DisplayVersion,
) -> Result<()> {
    require_presence(
        view.unknown_flag.is_some(),
        version.value() > 0x1c40,
        "annotation flag",
    )?;
    output.string(&view.view_name)?;
    output.i32(view.view_id.value())?;
    output.i32(view.is_shown)?;
    output.i32(view.is_current)?;
    if let Some(flag) = view.unknown_flag {
        output.i32(flag)?;
    }
    output.string(&view.unknown_name2)?;
    Ok(())
}

fn write_plane(output: &mut DisplayOutput, plane: &AnnotationSectionPlane) -> Result<()> {
    output.point(&plane.root_point)?;
    output.point(&plane.normal)?;
    output.double(plane.unknown_double)?;
    output.i32(plane.unknown_integer)
}
