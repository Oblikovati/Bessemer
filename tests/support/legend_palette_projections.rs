use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn named_base_slots(colors: &[SimulationPostColorView]) -> usize {
    const PATHS: [&str; 9] = [
        "legend.color_18",
        "legend.color_28",
        "legend.color_38",
        "legend.color_48",
        "legend.color_58",
        "legend.color_68",
        "legend.color_78",
        "legend.color_88",
        "legend.color_98",
    ];
    colors
        .iter()
        .filter(|color| PATHS.contains(&color.path.as_ref()))
        .count()
}

fn parent_palette_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> (SimulationLegendPaletteInputs, usize) {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = parent.legend_palette_inputs().unwrap();
    let slots = named_base_slots(&parent.colors().unwrap());
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_palette_inputs().unwrap(), inputs);
    (inputs, slots)
}

fn style_palette_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> (SimulationLegendPaletteInputs, usize) {
    let style = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = style.legend_palette_inputs().unwrap();
    let slots = named_base_slots(&style.colors().unwrap());
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.legend_palette_inputs().unwrap(), inputs);
    (inputs, slots)
}

fn archive_palette_inputs(
    family: u32,
    version: u32,
    raw: &[u8],
) -> (SimulationLegendPaletteInputs, usize) {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return parent_palette_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_palette_inputs(raw, version)
}

fn replay_palette_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let (inputs, slots) = archive_palette_inputs(family, version, &raw);
    assert_eq!(read_word(cursor) as usize, slots);
    assert_eq!(read_word(cursor), 7);
    let mut output = Vec::new();
    for _ in 0..7 {
        let mut probe = *cursor;
        assert_eq!(case_support::read_palette_case(&mut probe).inputs, inputs);
        output.extend(case_support::assert_palette_case(cursor));
    }
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    output
}

pub fn replay_palette_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut output = Vec::new();
    for _ in 0..expected_count {
        output.extend(replay_palette_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}

pub fn replay_palette_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_palette_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_palette_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
