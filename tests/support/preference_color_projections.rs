use super::load_projection::take_projection;
use super::{case_support, read_word};
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

fn archive_preference_colors(
    family: u32,
    version: u32,
    raw: &[u8],
) -> Vec<SimulationPostColorView> {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let colors = parent.colors().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.colors().unwrap(), colors);
        return colors;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let colors = style.colors().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.colors().unwrap(), colors);
    colors
}
fn replay_preference_route(cursor: &mut &[u8], colors: &[SimulationPostColorView]) -> Vec<u32> {
    let route = read_word(cursor);
    let present = read_word(cursor);
    let original: [u32; 4] = std::array::from_fn(|_| read_word(cursor));
    assert!(present <= 1);
    assert_preference_route_presence(route, present, original, colors);
    let mut output = vec![route, present];
    output.extend(original);
    if present == 0 {
        return output;
    }
    for _ in 0..4 {
        output.extend(assert_preference_projection_scenario(
            cursor, route, original,
        ));
    }
    output
}
fn assert_preference_projection_scenario(
    cursor: &mut &[u8],
    route: u32,
    original: [u32; 4],
) -> Vec<u32> {
    let case = case_support::read_preference_case(cursor);
    let expected: [u32; 18] = std::array::from_fn(|_| read_word(cursor));
    assert_eq!(case.words[0], route);
    assert_eq!(case.original.bits(), original);
    let actual = case_support::preference_result_words(&case);
    assert_eq!(actual, expected);
    actual
}
fn replay_preference_projection_case(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let length = read_word(cursor) as usize;
    let raw = take_projection(cursor, length);
    let colors = archive_preference_colors(family, version, raw);
    (0..8)
        .flat_map(|_| replay_preference_route(cursor, &colors))
        .collect()
}
pub fn replay_preference_projections(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let output = (0..count)
        .flat_map(|_| replay_preference_projection_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    output
}

fn assert_preference_route_presence(
    route: u32,
    present: u32,
    original: [u32; 4],
    colors: &[SimulationPostColorView],
) {
    let descriptor = SimulationStylePreferenceColorRoute::from_style_member(route).unwrap();
    let actual = colors
        .iter()
        .find(|color| color.path.as_ref() == descriptor.field_path())
        .map(|color| color.color);
    assert_eq!(
        actual.map(SimulationRgbaColor::bits),
        (present == 1).then_some(original)
    );
}
