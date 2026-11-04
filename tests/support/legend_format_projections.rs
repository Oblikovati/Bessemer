use super::load_projection::take_projection;
use super::{case_support, read_word};
use bessemer::simulation::*;
#[path = "post_legend_limits.rs"]
mod limits;
use limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};

fn archive_legend_format(family: u32, version: u32, raw: &[u8]) -> SimulationLegendFormatInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
        let inputs = parent.legend_format_inputs().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.legend_format_inputs().unwrap(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = style.legend_format_inputs().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.legend_format_inputs().unwrap(), inputs);
    inputs
}

fn replay_legend_parent(cursor: &mut &[u8]) -> Vec<u8> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let original = raw.to_vec();
    let inputs = archive_legend_format(family, version, raw);
    assert_eq!(read_word(cursor), 7);
    let mut output = Vec::new();
    for _ in 0..7 {
        let mut probe = *cursor;
        assert_eq!(case_support::read_legend_case(&mut probe).inputs, inputs);
        output.extend(case_support::assert_legend_case(cursor));
    }
    assert_eq!(raw, original);
    output
}

pub fn replay_legend_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u8> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut output = Vec::new();
    for _ in 0..expected_count {
        output.extend(replay_legend_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}

pub fn replay_legend_projection_parallel(packet: &[u8], expected_count: u32) -> Vec<u8> {
    let serial = replay_legend_projection(packet, expected_count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_legend_projection(packet, expected_count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
