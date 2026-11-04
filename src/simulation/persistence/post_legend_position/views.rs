use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    post_render::views::render_word,
};
use super::{
    SimulationLegendPositionAxis, SimulationLegendPositionFactor, SimulationLegendPositionFactors,
};
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostStyle {
    /// Read raw position factors with actual wire spans and source IDs; e.g. `style.legend_position_factors()?`.
    pub fn legend_position_factors(&self) -> Result<SimulationLegendPositionFactors> {
        position_from_snapshot(self)
    }
}

impl SimulationPostDialog {
    /// Read embedded legend factors without supplying missing older fields; e.g. `dialog.legend_position_factors()?`.
    pub fn legend_position_factors(&self) -> Result<SimulationLegendPositionFactors> {
        position_from_snapshot(self)
    }
}

pub(super) fn position_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLegendPositionFactors> {
    Ok(SimulationLegendPositionFactors {
        horizontal: position_factor_from_snapshot(
            snapshot,
            SimulationLegendPositionAxis::Horizontal,
        )?,
        vertical: position_factor_from_snapshot(snapshot, SimulationLegendPositionAxis::Vertical)?,
    })
}

fn position_factor_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
    axis: SimulationLegendPositionAxis,
) -> Result<Option<SimulationLegendPositionFactor>> {
    let path = axis.field_path();
    let Some(ordinal) = snapshot
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
    else {
        return Ok(None);
    };
    let raw_bits = render_word(snapshot, path)? as u32;
    let wire_range = position_factor_span(snapshot, ordinal, path, raw_bits)?;
    Ok(Some(SimulationLegendPositionFactor {
        axis,
        source_id: snapshot.field_id(ordinal)?,
        wire_range,
        raw_bits,
    }))
}

fn position_factor_span<T: BinaryLoadSnapshot>(
    snapshot: &T,
    ordinal: usize,
    path: &str,
    raw_bits: u32,
) -> Result<(usize, usize)> {
    let span = snapshot
        .field_offsets()
        .get(ordinal)
        .copied()
        .ok_or_else(|| missing_position_span(path, ordinal))?;
    let observed = snapshot.wire_bytes().get(span.0..span.1);
    if observed != Some(raw_bits.to_le_bytes().as_slice()) {
        return Err(BessemerError::new(
            "legend position factor",
            path,
            FailureClass::Invalid,
            format!(
                "span {span:?} with bytes {observed:?}; expected exactly four source bytes for raw32 {raw_bits:#010x}"
            ),
        ));
    }
    Ok(span)
}

fn missing_position_span(path: &str, ordinal: usize) -> BessemerError {
    BessemerError::new(
        "legend position factor",
        path,
        FailureClass::Missing,
        format!("span None at ordinal {ordinal}; expected one complete four-byte source span"),
    )
}
