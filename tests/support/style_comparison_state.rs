use super::load_projection::read_word;
use bessemer::simulation::*;
pub const CHANNELS: [SimulationColorChannel; 4] = [
    SimulationColorChannel::Red,
    SimulationColorChannel::Green,
    SimulationColorChannel::Blue,
    SimulationColorChannel::Alpha,
];
const LEGEND_WORDS: [u32; 16] = [
    8, 12, 16, 20, 0xa8, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xd4, 0xe8, 0xfc, 0xac, 0xb0,
];
const LEGEND_COLORS: [u32; 12] = [
    0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78, 0x88, 0x98, 0xd8, 0xec, 0x100,
];

fn read_mask(cursor: &mut &[u8]) -> u64 {
    u64::from(read_word(cursor)) | (u64::from(read_word(cursor)) << 32)
}

fn read_optional(cursor: &mut &[u8], mask: u64, index: usize) -> Option<u32> {
    let word = read_word(cursor);
    let present = mask & (1 << index) != 0;
    if !present {
        assert_eq!(word, 0);
    }
    present.then_some(word)
}

fn legend_input_path(index: usize) -> String {
    if index < 16 {
        return format!("legend.word_{:x}", LEGEND_WORDS[index]);
    }
    format!(
        "legend.color_{:x}.component_{}",
        LEGEND_COLORS[(index - 16) / 4],
        (index - 16) % 4
    )
}

fn read_legend_state(cursor: &mut &[u8]) -> SimulationLegendComparisonState {
    let mask = read_mask(cursor);
    let mut fields = Vec::new();
    for index in 0..64 {
        let Some(bits) = read_optional(cursor, mask, index) else {
            continue;
        };
        fields.push(SimulationPostStyleField {
            path: legend_input_path(index).into(),
            value: SimulationPostStyleValue::RawWordBits(bits),
        });
    }
    SimulationLegendComparisonState::from_fields(&fields).unwrap()
}

pub fn read_style_state(cursor: &mut &[u8]) -> SimulationStyleComparisonState {
    let mask = read_mask(cursor);
    assert_eq!(mask >> 39, 0);
    let values: [Option<u32>; 39] = std::array::from_fn(|index| read_optional(cursor, mask, index));
    let legend = read_legend_state(cursor);
    let mut state = SimulationStyleComparisonState::from_legend_state(legend);
    for (index, member) in SimulationStyleComparisonWord::ALL.into_iter().enumerate() {
        state.set_word_bits(member, values[index]);
    }
    for (color_index, color) in SimulationStyleComparisonColor::ALL.into_iter().enumerate() {
        for (component, channel) in CHANNELS.into_iter().enumerate() {
            state.set_component_bits(color, channel, values[11 + color_index * 4 + component]);
        }
    }
    state
}

pub fn style_input_path(index: usize) -> String {
    if index < 11 {
        return SimulationStyleComparisonWord::ALL[index]
            .field_path()
            .into();
    }
    format!(
        "{}.component_{}",
        SimulationStyleComparisonColor::ALL[(index - 11) / 4].field_path(),
        (index - 11) % 4
    )
}
