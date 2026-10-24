use super::{
    DisplayVersion, DisplayViewLimits, NamedSectionPlane, NamedSectionSelections,
    NamedSelectionValues, NamedView,
    output::DisplayOutput,
    view_support::{ViewBudget, require_presence, require_size},
};
use crate::Result;

pub(super) fn encode(
    version: DisplayVersion,
    views: &Option<Vec<NamedView>>,
    limits: DisplayViewLimits,
) -> Result<Vec<u8>> {
    require_presence(views.is_some(), version.value() > 0x1be7, "named views")?;
    let mut output = DisplayOutput::new(limits.maximum_bytes);
    let Some(views) = views else {
        return Ok(output.finish());
    };
    require_size(views.len(), limits.maximum_views, "named views")?;
    output.count(views.len(), "named views")?;
    let mut budget = ViewBudget::new(limits);
    for view in views {
        write_view(&mut output, view, version, &mut budget)?;
    }
    Ok(output.finish())
}

fn write_view(
    output: &mut DisplayOutput,
    view: &NamedView,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<()> {
    validate_view_presence(view, version)?;
    output.string(&view.view_name)?;
    output.compact(&view.model_to_view)?;
    output.compact(&view.pan_zoom)?;
    output.string(&view.unknown_string)?;
    if let Some(planes) = &view.section_planes {
        write_planes(output, planes, version, budget)?;
    }
    if let Some(selections) = &view.selections {
        write_selections(output, selections, budget)?;
    }
    if let Some(bitset) = view.unknown_bitset {
        for word in bitset {
            output.i32(word)?;
        }
    }
    Ok(())
}

fn validate_view_presence(view: &NamedView, version: DisplayVersion) -> Result<()> {
    let planes = version.value() > 0x1bf4;
    let bitset = version.value() > 0x200d;
    let selected = version.value() > 0x2003
        && view
            .section_planes
            .as_ref()
            .is_some_and(|planes| !planes.is_empty());
    require_presence(view.section_planes.is_some(), planes, "named planes")?;
    require_presence(view.selections.is_some(), selected, "named selections")?;
    require_presence(view.unknown_bitset.is_some(), bitset, "named bitset")
}

fn write_planes(
    output: &mut DisplayOutput,
    planes: &[NamedSectionPlane],
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<()> {
    budget.nested(planes.len(), "named section planes")?;
    output.count(planes.len(), "named section planes")?;
    for plane in planes {
        write_plane(output, plane, version)?;
    }
    Ok(())
}

fn write_plane(
    output: &mut DisplayOutput,
    plane: &NamedSectionPlane,
    version: DisplayVersion,
) -> Result<()> {
    validate_plane_presence(plane, version)?;
    output.point(&plane.root_point)?;
    output.point(&plane.normal)?;
    output.double(plane.unknown_double)?;
    output.i32(plane.unknown_i0)?;
    if let Some(pair) = plane.unknown_pair {
        for word in pair {
            output.i32(word)?;
        }
    }
    if let Some(word) = plane.unknown_word {
        output.u32(word)?;
    }
    Ok(())
}

fn validate_plane_presence(plane: &NamedSectionPlane, version: DisplayVersion) -> Result<()> {
    let pair = version.value() > 0x2001;
    let word = version.value() > 0x2007;
    require_presence(plane.unknown_pair.is_some(), pair, "named plane pair")?;
    require_presence(plane.unknown_word.is_some(), word, "named plane word")
}

fn write_selections(
    output: &mut DisplayOutput,
    selections: &NamedSectionSelections,
    budget: &mut ViewBudget,
) -> Result<()> {
    require_presence(
        selections.values.is_some(),
        selections.has_selections != 0,
        "named selection payload",
    )?;
    output.i32(selections.has_selections)?;
    if let Some(values) = &selections.values {
        write_selection_values(output, values, budget)?;
    }
    Ok(())
}

fn write_selection_values(
    output: &mut DisplayOutput,
    values: &NamedSelectionValues,
    budget: &mut ViewBudget,
) -> Result<()> {
    budget.nested(values.names.len(), "named selection names")?;
    output.i32(values.unknown_sel0)?;
    output.i32(values.is_part)?;
    output.count(values.names.len(), "named selection names")?;
    for name in &values.names {
        output.string(name)?;
    }
    Ok(())
}
