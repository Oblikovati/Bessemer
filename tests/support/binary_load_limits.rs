use super::load_projection::read_word;
use bessemer::simulation::SimulationThermalLimits;

fn splice_mutant(cursor: &mut &[u8], original: &[u8]) -> Vec<u8> {
    let start = read_word(cursor) as usize;
    let removed = read_word(cursor) as usize;
    let size = read_word(cursor) as usize;
    let (replacement, remaining) = cursor.split_at(size);
    *cursor = remaining;
    let mut image = original[..start].to_vec();
    image.extend_from_slice(replacement);
    image.extend_from_slice(&original[start + removed..]);
    image
}

pub(super) fn check_mutations(
    cursor: &mut &[u8],
    original: &[u8],
    accepts: impl Fn(&[u8]) -> bool,
) -> (usize, usize) {
    let (mut accepted, mut rejected) = (0, 0);
    for _ in 0..read_word(cursor) {
        let expected = read_word(cursor) != 0;
        let image = splice_mutant(cursor, original);
        assert_eq!(
            accepts(&image),
            expected,
            "expected complete actual reference acceptance"
        );
        accepted += usize::from(expected);
        rejected += usize::from(!expected);
    }
    (accepted, rejected)
}

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
