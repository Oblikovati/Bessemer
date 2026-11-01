use super::read_word;
use bessemer::simulation::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode3FixtureCase {
    pub inputs: SimulationMode3RgbInputs,
    pub group: SimulationMode3RgbGroup,
    pub array: SimulationMode3RgbArrayContext,
}
fn known_word(tag: u32, raw: u32) -> Option<i32> {
    assert!(tag <= 1);
    (tag == 1).then_some(raw as i32)
}
fn color_words(words: [u32; 5]) -> Option<SimulationRgbaColor> {
    assert!(words[0] <= 1);
    (words[0] == 1).then(|| SimulationRgbaColor::from_bits(words[1..].try_into().unwrap()))
}
pub fn mode3_inputs_from_words(words: [u32; 9]) -> SimulationMode3RgbInputs {
    SimulationMode3RgbInputs {
        mode_1b8_raw: known_word(words[0], words[1]),
        color_1dc: color_words(words[2..7].try_into().unwrap()),
        plot_type_raw: known_word(words[7], words[8]),
    }
}
pub fn mode3_group_from_words(words: [u32; 19]) -> SimulationMode3RgbGroup {
    assert!(words[0] <= 1);
    SimulationMode3RgbGroup {
        pass: if words[0] == 0 {
            SimulationMode3RgbPass::Primary
        } else {
            SimulationMode3RgbPass::Reflected
        },
        status_raw: known_word(words[1], words[2]),
        item_test_raw: known_word(words[3], words[4]),
        face_raw: known_word(words[5], words[6]),
        first_color: color_words(words[7..12].try_into().unwrap()),
        other_color: color_words(words[12..17].try_into().unwrap()),
        selector_raw: known_word(words[17], words[18]),
    }
}
fn seeded_triples(words: [u32; 4]) -> Option<SimulationMode3RgbTriples> {
    assert!(words[0] <= 1);
    if words[0] == 0 {
        return None;
    }
    let triples: Vec<[u32; 3]> = (0..words[1])
        .map(|index| {
            std::array::from_fn(|channel| {
                words[2]
                    ^ (index + 1).wrapping_mul(0x9e3779b9)
                    ^ (channel as u32).wrapping_mul(0x85ebca6b)
            })
        })
        .collect();
    Some(SimulationMode3RgbTriples::from_slice(&triples, words[3] as usize).unwrap())
}
pub fn mode3_array_from_words(words: [u32; 12]) -> SimulationMode3RgbArrayContext {
    SimulationMode3RgbArrayContext {
        capacity_raw: known_word(words[0], words[1]),
        existing: seeded_triples(words[2..6].try_into().unwrap()),
        growth_result_raw: known_word(words[6], words[7]),
        after_growth: seeded_triples(words[8..12].try_into().unwrap()),
    }
}
pub fn read_mode3_case(cursor: &mut &[u8]) -> Mode3FixtureCase {
    let words: [u32; 40] = std::array::from_fn(|_| read_word(cursor));
    Mode3FixtureCase {
        inputs: mode3_inputs_from_words(words[..9].try_into().unwrap()),
        group: mode3_group_from_words(words[9..28].try_into().unwrap()),
        array: mode3_array_from_words(words[28..].try_into().unwrap()),
    }
}
