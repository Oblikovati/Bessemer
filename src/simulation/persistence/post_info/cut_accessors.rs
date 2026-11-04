use super::cut_view::{direction_token, primitive_kind};
use super::{
    SimulationPostInfo, SimulationPostInfoCutView, SimulationPostInfoSlotGroup,
    SimulationPostInfoValue,
};
use crate::{Result, simulation::invalid};

impl SimulationPostInfo {
    /// Read a physical slot regardless of active counts; e.g. `info.cut_primitive(SimulationPostInfoSlotGroup::Section, 0)?`.
    pub fn cut_primitive(
        &self,
        group: SimulationPostInfoSlotGroup,
        index: usize,
    ) -> Result<SimulationPostInfoCutView> {
        let start = cut_slot_start(group, index)?;
        let primitive_code = signed_cut_word(self, start)?;
        let direction_code = signed_cut_word(self, start + 1)?;
        let shared_scalar_bits = cut_scalar_bits(self, start + 8)?;
        Ok(SimulationPostInfoCutView {
            group,
            index,
            primitive_code,
            direction_code,
            primitive_kind: primitive_kind(primitive_code),
            direction_token: direction_token(direction_code),
            normal_bits: cut_vector_bits(self, start + 2)?,
            center_bits: cut_vector_bits(self, start + 5)?,
            shared_scalar_bits,
            negated_d_bits: shared_scalar_bits ^ 0x80000000,
        })
    }
}

fn cut_slot_start(group: SimulationPostInfoSlotGroup, index: usize) -> Result<usize> {
    if index >= 6 {
        return Err(invalid(
            "PostInfo cut slot",
            format!("{group:?} index {index}; expected 0..6"),
        ));
    }
    let group_start = match group {
        SimulationPostInfoSlotGroup::Section => 0,
        SimulationPostInfoSlotGroup::Iso => 6,
    };
    Ok(7 + (group_start + index) * 9)
}

fn cut_vector_bits(info: &SimulationPostInfo, start: usize) -> Result<[u32; 3]> {
    Ok([
        cut_scalar_bits(info, start)?,
        cut_scalar_bits(info, start + 1)?,
        cut_scalar_bits(info, start + 2)?,
    ])
}

fn signed_cut_word(info: &SimulationPostInfo, ordinal: usize) -> Result<i32> {
    match info.fields().get(ordinal).map(|field| &field.value) {
        Some(SimulationPostInfoValue::Signed(value)) => Ok(*value),
        value => Err(invalid(
            "PostInfo word view",
            format!("field {ordinal}: {value:?}; expected signed32 word"),
        )),
    }
}

fn cut_scalar_bits(info: &SimulationPostInfo, ordinal: usize) -> Result<u32> {
    match info.fields().get(ordinal).map(|field| &field.value) {
        Some(SimulationPostInfoValue::Float32Bits(bits)) => Ok(*bits),
        value => Err(invalid(
            "PostInfo scalar view",
            format!("field {ordinal}: {value:?}; expected float32 pattern"),
        )),
    }
}
