use super::{
    DisplayVersion, DisplayViewLimits, SpatialView,
    output::DisplayOutput,
    view_bitmap,
    view_plane::{self, PlaneGates},
    view_support::{ViewBudget, require_presence, require_size},
};
use crate::Result;

pub(super) fn encode(
    version: DisplayVersion,
    views: &Option<Vec<SpatialView>>,
    limits: DisplayViewLimits,
) -> Result<Vec<u8>> {
    require_presence(views.is_some(), version.value() > 0x1c18, "3D views")?;
    let mut output = DisplayOutput::new(limits.maximum_bytes);
    let Some(views) = views else {
        return Ok(output.finish());
    };
    require_size(views.len(), limits.maximum_views, "3D views")?;
    output.count(views.len(), "3D views")?;
    let mut budget = ViewBudget::new(limits);
    for view in views {
        write_view(&mut output, view, version, &mut budget)?;
    }
    Ok(output.finish())
}

fn write_view(
    output: &mut DisplayOutput,
    view: &SpatialView,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<()> {
    validate_header(view, version)?;
    validate_tail(view, version)?;
    write_header(output, view)?;
    view_plane::write_list(
        output,
        &view.section_planes,
        version,
        budget,
        PlaneGates {
            pair_after: 0x2012,
            word_after: 0x2012,
            pair_label: "3D plane pair",
            word_label: "3D plane word",
        },
        "3D planes",
    )?;
    write_tail(output, view)
}

fn validate_header(view: &SpatialView, version: DisplayVersion) -> Result<()> {
    for (present, threshold, label) in [
        (view.unknown_active.is_some(), 0x46c9, "3D active flag"),
        (view.preview_bitmap.is_some(), 0x1c1e, "3D bitmap"),
        (view.unknown_ids_string.is_some(), 0x1c42, "3D IDs string"),
    ] {
        require_presence(present, version.value() > threshold, label)?;
    }
    Ok(())
}

fn validate_tail(view: &SpatialView, version: DisplayVersion) -> Result<()> {
    let after = version.value() > 0x2012 && !view.section_planes.is_empty();
    require_presence(
        view.unknown_after_planes.is_some(),
        after,
        "3D after-plane field",
    )?;
    validate_bitset_pair(view, version)?;
    require_presence(
        view.unknown_text_string.is_some(),
        version.value() > 0x3331,
        "3D text string",
    )
}

fn validate_bitset_pair(view: &SpatialView, version: DisplayVersion) -> Result<()> {
    let expected = version.value() > 0x1fb5;
    require_presence(view.unknown_468.is_some(), expected, "3D unknown_468")?;
    require_presence(view.unknown_bitset.is_some(), expected, "3D bitset")
}

fn write_header(output: &mut DisplayOutput, view: &SpatialView) -> Result<()> {
    output.string(&view.view_name)?;
    if let Some(value) = view.unknown_active {
        output.i32(value)?;
    }
    output.compact(&view.model_to_view)?;
    output.compact(&view.pan_zoom)?;
    output.string(&view.unknown_string)?;
    if let Some(bitmap) = &view.preview_bitmap {
        view_bitmap::write(output, bitmap)?;
    }
    if let Some(text) = &view.unknown_ids_string {
        output.string(text)?;
    }
    Ok(())
}

fn write_tail(output: &mut DisplayOutput, view: &SpatialView) -> Result<()> {
    if let Some(value) = view.unknown_after_planes {
        output.i32(value)?;
    }
    if let Some(value) = view.unknown_468 {
        output.i32(value)?;
    }
    if let Some(bitset) = view.unknown_bitset {
        for word in bitset {
            output.i32(word)?;
        }
    }
    if let Some(text) = &view.unknown_text_string {
        output.string(text)?;
    }
    Ok(())
}
