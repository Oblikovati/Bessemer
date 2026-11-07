use super::super::{
    SimulationPostDialog, SimulationPostStyle, SimulationRgbaColor,
    load_snapshot::BinaryLoadSnapshot, post_render::views::render_word, rgba::collect_wire_colors,
};
use super::{SimulationPlot54Colors, SimulationPlot54Inputs};
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostDialog {
    /// Retain actual optional colors and the parent code without defaults; e.g. `dialog.plot54_color_inputs()?`.
    pub fn plot54_color_inputs(&self) -> Result<SimulationPlot54Inputs> {
        Ok(SimulationPlot54Inputs {
            plot_type_raw_code: Some(render_word(self, "fourteen_words0c_28_30_3c_518_51c[1]")?),
            colors: plot54_snapshot_colors(self)?,
        })
    }
}

impl SimulationPostStyle {
    /// Keep the standalone parent code unknown and color-pair absence explicit; e.g. `style.plot54_color_inputs()?`.
    pub fn plot54_color_inputs(&self) -> Result<SimulationPlot54Inputs> {
        Ok(SimulationPlot54Inputs {
            plot_type_raw_code: None,
            colors: plot54_snapshot_colors(self)?,
        })
    }
}

fn plot54_snapshot_colors<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<Option<SimulationPlot54Colors>> {
    let colors = collect_wire_colors(snapshot.fields(), snapshot.field_offsets())?;
    let ac = colors
        .iter()
        .find(|color| color.path.as_ref() == "style.color_2ac")
        .map(|color| color.color);
    let bc = colors
        .iter()
        .find(|color| color.path.as_ref() == "style.color_2bc")
        .map(|color| color.color);
    plot54_complete_pair(ac, bc)
}

fn plot54_complete_pair(
    ac: Option<SimulationRgbaColor>,
    bc: Option<SimulationRgbaColor>,
) -> Result<Option<SimulationPlot54Colors>> {
    match (ac, bc) {
        (None, None) => Ok(None),
        (Some(color_2ac), Some(forwarded_2bc)) => Ok(Some(SimulationPlot54Colors {
            color_2ac,
            forwarded_2bc,
        })),
        observed => Err(BessemerError::new(
            "plot54 file inputs",
            "2AC/2BC colors",
            FailureClass::Invalid,
            format!("{observed:?}; expected two complete stored RGBA groups or two absent groups"),
        )),
    }
}

#[cfg(test)]
mod pair_tests {
    #[test]
    fn incomplete_stored_pair_is_invalid_instead_of_a_missing_default() {
        let color = super::SimulationRgbaColor::from_bits([0; 4]);
        for (ac, bc) in [(Some(color), None), (None, Some(color))] {
            let error = super::plot54_complete_pair(ac, bc).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(
                error
                    .cause
                    .contains("expected two complete stored RGBA groups")
            );
        }
    }
}
