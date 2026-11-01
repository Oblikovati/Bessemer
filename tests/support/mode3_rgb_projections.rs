use super::load_projection::take_projection;
use super::{case_support, read_word, result_support};
use bessemer::simulation::*;
const PARENTS: SimulationPostDialogLimits = SimulationPostDialogLimits {
    maximum_bytes: 10000000,
    maximum_fields: 200000,
    maximum_entries: 10000000,
    maximum_text_bytes: 0x200000,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 1000000,
        maximum_fields: 100000,
        maximum_entries: 1000000,
        maximum_depth: 128,
        maximum_objects: 100000,
    },
};
const STYLES: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn archive_mode3_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationMode3RgbInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let inputs = parent.mode3_geometry_rgb_inputs().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.mode3_geometry_rgb_inputs().unwrap(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let inputs = style.mode3_geometry_rgb_inputs().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.mode3_geometry_rgb_inputs().unwrap(), inputs);
    inputs
}
fn mode3_input_words(inputs: SimulationMode3RgbInputs) -> Vec<u32> {
    let mut words = vec![
        u32::from(inputs.mode_1b8_raw.is_some()),
        inputs.mode_1b8_raw.unwrap_or(0) as u32,
        u32::from(inputs.color_1dc.is_some()),
    ];
    words.extend(inputs.color_1dc.map_or([0; 4], SimulationRgbaColor::bits));
    words.extend([
        u32::from(inputs.plot_type_raw.is_some()),
        inputs.plot_type_raw.unwrap_or(0) as u32,
    ]);
    words
}
fn assert_mode3_projection_scenario(
    cursor: &mut &[u8],
    inputs: SimulationMode3RgbInputs,
) -> Vec<u32> {
    let runtime: [u32; 31] = std::array::from_fn(|_| read_word(cursor));
    let expected: [u32; 54] = std::array::from_fn(|_| read_word(cursor));
    let case = case_support::Mode3FixtureCase {
        inputs,
        group: case_support::mode3_group_from_words(runtime[..19].try_into().unwrap()),
        array: case_support::mode3_array_from_words(runtime[19..].try_into().unwrap()),
    };
    let actual = result_support::mode3_result_words(&case);
    assert_eq!(actual, expected);
    actual
}
fn replay_mode3_projection_case(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let length = read_word(cursor) as usize;
    let raw = take_projection(cursor, length);
    let expected: [u32; 9] = std::array::from_fn(|_| read_word(cursor));
    let inputs = archive_mode3_inputs(family, version, raw);
    assert_eq!(inputs, case_support::mode3_inputs_from_words(expected));
    let mut output = mode3_input_words(inputs);
    assert_eq!(output, expected);
    for _ in 0..4 {
        output.extend(assert_mode3_projection_scenario(cursor, inputs));
    }
    output
}
pub fn replay_mode3_projections(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let output = (0..count)
        .flat_map(|_| replay_mode3_projection_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    output
}
