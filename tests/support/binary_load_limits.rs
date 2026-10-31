use super::load_projection::read_word;
use bessemer::simulation::SimulationThermalLimits;

pub(super) use super::load_mutations::check_mutations;

pub(super) fn read_reference_limits(
    cursor: &mut &[u8],
    base: SimulationThermalLimits,
) -> SimulationThermalLimits {
    let mut limits = base;
    limits.maximum_entries = read_word(cursor) as usize;
    limits.maximum_text_bytes = read_word(cursor) as usize;
    limits.embedded.maximum_bytes = read_word(cursor) as usize;
    limits.embedded.maximum_objects = read_word(cursor) as usize;
    limits.maximum_fields = read_word(cursor) as usize;
    limits.embedded.maximum_fields = read_word(cursor) as usize;
    limits
}

pub(super) fn reduced_limits(exact: SimulationThermalLimits) -> Vec<SimulationThermalLimits> {
    (0..7)
        .filter_map(|index| lower_limit(exact, index))
        .collect()
}

fn lower_limit(
    mut limits: SimulationThermalLimits,
    index: usize,
) -> Option<SimulationThermalLimits> {
    let value = match index {
        0 => &mut limits.maximum_bytes,
        1 => &mut limits.maximum_fields,
        2 => &mut limits.maximum_entries,
        3 => &mut limits.maximum_text_bytes,
        4 => &mut limits.embedded.maximum_bytes,
        5 => &mut limits.embedded.maximum_objects,
        _ => &mut limits.embedded.maximum_fields,
    };
    if *value == 0 {
        return None;
    }
    *value -= 1;
    Some(limits)
}
