use super::{
    SimulationComponentVectorInputs, SimulationComponentVectorSource,
    SimulationComponentVectorView, tables,
};

pub(super) fn provider_view(
    inputs: SimulationComponentVectorInputs,
) -> SimulationComponentVectorView {
    let table_rva = selected_table(inputs);
    if let Some(output_bits) = table_rva.and_then(|rva| table_output(rva, inputs.component)) {
        return SimulationComponentVectorView {
            output_bits,
            source: SimulationComponentVectorSource::Table,
            table_rva,
        };
    }
    SimulationComponentVectorView {
        output_bits: fallback_bits(inputs),
        source: SimulationComponentVectorSource::ConstantOrDefault,
        table_rva,
    }
}

fn table_output(rva: u32, component: i32) -> Option<[u32; 4]> {
    let index = usize::try_from(component).ok()?;
    let table = tables::TABLE_INDICES.iter().find(|entry| entry.0 == rva)?.1;
    let pattern = table.get(index)?.checked_sub(b'0')?;
    tables::OUTPUT_PATTERNS.get(usize::from(pattern)).copied()
}

fn selected_table(inputs: SimulationComponentVectorInputs) -> Option<u32> {
    match inputs.analysis {
        0 => analysis_zero(inputs.plot_type, inputs.fourth_word),
        1 if inputs.plot_type == 1 => Some(0x2f0d0),
        2 if inputs.plot_type == 1 => Some(0x2f0ec),
        3 if inputs.plot_type == 5 => Some(0x2f108),
        5 | 6 => analysis_five_six(inputs.plot_type, inputs.fourth_word),
        8 => analysis_eight(inputs.plot_type),
        10 => analysis_ten(inputs.plot_type),
        _ => None,
    }
}

fn analysis_zero(plot: i32, fourth: i32) -> Option<u32> {
    match plot {
        1 => Some(0x2eff4),
        2 | 57 => Some(if fourth == 0 { 0x2f098 } else { 0x2f060 }),
        3 | 22 => Some(0x2f030),
        _ => None,
    }
}

fn analysis_five_six(plot: i32, fourth: i32) -> Option<u32> {
    match plot {
        1 => Some(0x2f328),
        2 => Some(if fourth == 0 { 0x2f2f0 } else { 0x2f2b8 }),
        3 | 22 => Some(0x2f3dc),
        19 => Some(0x2f39c),
        20 => Some(0x2f3bc),
        _ => None,
    }
}

fn analysis_eight(plot: i32) -> Option<u32> {
    match plot {
        1 => Some(0x2f208),
        2 => Some(0x2f1d8),
        3 | 22 => Some(0x2f288),
        19 => Some(0x2f248),
        20 => Some(0x2f268),
        _ => None,
    }
}

fn analysis_ten(plot: i32) -> Option<u32> {
    match plot {
        6 => Some(0x2f134),
        8 => Some(0x2f154),
        9 => Some(0x2f170),
        10 => Some(0x2f18c),
        11 => Some(0x2f1a8),
        _ => None,
    }
}

fn fallback_bits(inputs: SimulationComponentVectorInputs) -> [u32; 4] {
    let first = matches!(inputs.analysis, 5 | 6 | 8) && matches!(inputs.plot_type, 19 | 20)
        || inputs.analysis == 10 && inputs.plot_type == 7 && (0..=4).contains(&inputs.component);
    if first {
        return [0x3f800000, 0, 0, 0];
    }
    [0; 4]
}
