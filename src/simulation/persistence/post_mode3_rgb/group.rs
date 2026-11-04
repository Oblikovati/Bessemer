use super::super::SimulationRgbaColor;
use super::{mode3_required, types::*};
use crate::Result;

pub(super) type Mode3GroupSelection = (
    Option<[SimulationRgbaColor; 2]>,
    [u32; 3],
    SimulationMode3RgbOrigin,
);

pub(super) fn mode3_group_rejection(
    group: SimulationMode3RgbGroup,
) -> Result<Option<SimulationMode3RgbStage>> {
    let status = mode3_required(
        group.status_raw,
        "group status",
        "Some(raw signed32 status)",
    )?;
    if status != 0 {
        return Ok(Some(SimulationMode3RgbStage::GroupHidden));
    }
    let items = mode3_required(
        group.item_test_raw,
        "item test",
        "Some(raw signed32 external result)",
    )?;
    if items == 0 {
        return Ok(Some(SimulationMode3RgbStage::GroupEmpty));
    }
    Ok(None)
}
pub(super) fn mode3_selected_color(
    mode: i32,
    candidate: [u32; 3],
    group: SimulationMode3RgbGroup,
) -> Result<Mode3GroupSelection> {
    if mode != 3 {
        return mode3_replaced_color(group, None);
    }
    let face = mode3_required(group.face_raw, "face", "Some(raw signed32 face code)")?;
    if face == 0x405 {
        return mode3_replaced_color(group, Some(face));
    }
    Ok((None, candidate, SimulationMode3RgbOrigin::Stored1dc))
}
fn mode3_replaced_color(
    group: SimulationMode3RgbGroup,
    known_face: Option<i32>,
) -> Result<Mode3GroupSelection> {
    let [first, other] = mode3_group_parents(group)?;
    let face = mode3_required(
        known_face.or(group.face_raw),
        "face",
        "Some(raw signed32 face code)",
    )?;
    let (selected, origin) = if face == 0x404 {
        (first, SimulationMode3RgbOrigin::GroupFirst)
    } else {
        (other, SimulationMode3RgbOrigin::GroupOther)
    };
    Ok((Some([first, other]), mode3_rgb_words(selected), origin))
}
pub(super) fn mode3_rgb_words(color: SimulationRgbaColor) -> [u32; 3] {
    let words = color.bits();
    [words[0], words[1], words[2]]
}
pub(super) fn mode3_submission_stage(
    inputs: SimulationMode3RgbInputs,
    group: SimulationMode3RgbGroup,
) -> Result<SimulationMode3RgbStage> {
    if group.pass == SimulationMode3RgbPass::Reflected {
        return Ok(SimulationMode3RgbStage::ArrayRequested);
    }
    let plot = mode3_required(
        inputs.plot_type_raw,
        "parent170",
        "Some(raw signed32 plot code)",
    )?;
    if plot != 54 {
        return Ok(SimulationMode3RgbStage::ArrayRequested);
    }
    let selector = mode3_required(
        group.selector_raw,
        "group selector",
        "Some(raw signed32 selector)",
    )?;
    if matches!(selector, 1 | 2) {
        return Ok(SimulationMode3RgbStage::PrimaryFiltered);
    }
    Ok(SimulationMode3RgbStage::ArrayRequested)
}

fn mode3_group_parents(group: SimulationMode3RgbGroup) -> Result<[SimulationRgbaColor; 2]> {
    let first = mode3_required(
        group.first_color,
        "first group color",
        "Some(complete first RGBA)",
    )?;
    let other = mode3_required(
        group.other_color,
        "other group color",
        "Some(complete other RGBA)",
    )?;
    Ok([first, other])
}
