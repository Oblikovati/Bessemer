use super::*;

fn native_member(offset: u32) -> SimulationLegacyPlotWideMember {
    match offset {
        0x30 => SimulationLegacyPlotWideMember::Buffer30,
        0x13c => SimulationLegacyPlotWideMember::Buffer13C,
        0x23c => SimulationLegacyPlotWideMember::Buffer23C,
        0x33c => SimulationLegacyPlotWideMember::Buffer33C,
        0x454 => SimulationLegacyPlotWideMember::Buffer454,
        other => panic!("{other:x}; expected one of the five fixed-wide member offsets"),
    }
}

fn check_original_wide_view(plot: &SimulationLegacyPlot, cursor: &mut &[u8], output: &mut Vec<u8>) {
    let member = native_member(read_word(cursor));
    let offsets = (read_word(cursor) as usize, read_word(cursor) as usize);
    let limit = read_word(cursor) as usize;
    let prefix = read_word(cursor) as usize;
    let terminated = read_word(cursor) != 0;
    let expected = take_projection(cursor, limit * 2);
    let view = plot.fixed_wide_view(member).unwrap().unwrap();
    assert_eq!(view.member(), member);
    assert_eq!(view.source_offsets(), offsets);
    assert_eq!(member.code_unit_limit(), limit);
    assert_eq!(view.prefix_code_unit_count(), prefix);
    assert_eq!(view.terminator_present(), terminated);
    assert_eq!(view.raw_bytes(), expected);
    assert_eq!(view.prefix_bytes(), &expected[..prefix * 2]);
    assert_eq!(view.remainder_bytes(), &expected[prefix * 2..]);
    project_word(output, view.prefix_code_unit_count());
    project_word(output, usize::from(view.terminator_present()));
    output.extend_from_slice(view.raw_bytes());
}

fn check_original_wide_case(cursor: &mut &[u8], output: &mut Vec<u8>) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let dispatch = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let count = read_word(cursor);
    assert_eq!(count, 5);
    let plot =
        SimulationLegacyPlot::from_bytes(take_projection(cursor, size), version, LIMITS).unwrap();
    assert_eq!(plot.named_stream_dispatch_enabled(), dispatch);
    for _ in 0..count {
        check_original_wide_view(&plot, cursor, output);
    }
    dispatch
}

pub(super) fn project_original_wide_views(output: &mut Vec<u8>) {
    let packet = std::fs::read("target/simulation-legacy-plot-native-wide.bin").unwrap();
    let mut cursor = packet.as_slice();
    let (mut count, mut compatible) = (0, 0);
    while !cursor.is_empty() {
        compatible += usize::from(check_original_wide_case(&mut cursor, output));
        count += 1;
    }
    assert_eq!((count, compatible), (48, 40));
}
