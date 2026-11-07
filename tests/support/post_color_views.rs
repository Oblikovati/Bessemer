use super::*;

const CHANNELS: [SimulationColorChannel; 4] = [
    SimulationColorChannel::Red,
    SimulationColorChannel::Green,
    SimulationColorChannel::Blue,
    SimulationColorChannel::Alpha,
];

fn assert_color(cursor: &mut &[u8], actual: Option<&SimulationPostColorView>) {
    let length = read_word(cursor) as usize;
    let path = std::str::from_utf8(take_projection(cursor, length)).unwrap();
    let span = (read_word(cursor) as usize, read_word(cursor) as usize);
    let raw = take_projection(cursor, 16);
    let mask = read_word(cursor);
    let values = std::array::from_fn::<_, 4, _>(|_| read_word(cursor));
    assert!(mask < 16);
    let Some(actual) = actual else { return };
    assert_eq!(actual.path.as_ref(), path);
    assert_eq!(actual.wire_range, span);
    assert_eq!(actual.color.to_bytes(), raw);
    for (index, channel) in CHANNELS.into_iter().enumerate() {
        let expected = (mask & (1 << index) != 0).then_some(values[index]);
        assert_eq!(
            actual.color.finite_channel(channel).map(f32::to_bits),
            expected
        );
    }
}

pub(super) fn assert_packet(
    mut packet: &[u8],
    family: u32,
    mut actual: impl Iterator<Item = (i32, Vec<SimulationPostColorView>)>,
) -> usize {
    assert_eq!(read_word(&mut packet), 1);
    let mut matched = 0;
    for _ in 0..read_word(&mut packet) {
        let kind = read_word(&mut packet);
        let version = read_word(&mut packet) as i32;
        let count = read_word(&mut packet) as usize;
        let selected = (kind == family).then(|| actual.next().unwrap());
        if let Some((current, colors)) = &selected {
            assert_eq!((*current, colors.len()), (version, count));
        }
        for index in 0..count {
            assert_color(
                &mut packet,
                selected.as_ref().map(|(_, colors)| &colors[index]),
            );
        }
        matched += usize::from(selected.is_some());
    }
    assert!(packet.is_empty() && actual.next().is_none());
    matched
}

pub(super) fn assert_threaded<T: Sync>(
    bodies: &[T],
    query: fn(&T) -> bessemer::Result<Vec<SimulationPostColorView>>,
) {
    let serial: Vec<_> = bodies.iter().map(|body| query(body).unwrap()).collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_replayed(bodies, &serial, query));
        }
    });
}

fn assert_replayed<T>(
    bodies: &[T],
    serial: &[Vec<SimulationPostColorView>],
    query: fn(&T) -> bessemer::Result<Vec<SimulationPostColorView>>,
) {
    for (body, expected) in bodies.iter().zip(serial) {
        assert_eq!(&query(body).unwrap(), expected);
    }
}

fn project_color(output: &mut Vec<u8>, view: &SimulationPostColorView) {
    project_word(output, view.path.len());
    output.extend_from_slice(view.path.as_bytes());
    project_word(output, view.wire_range.0);
    project_word(output, view.wire_range.1);
    output.extend(view.color.to_bytes());
    let mut mask = 0;
    let mut values = [0; 4];
    for (index, channel) in CHANNELS.into_iter().enumerate() {
        if let Some(value) = view.color.finite_channel(channel) {
            mask |= 1 << index;
            values[index] = value.to_bits();
        }
    }
    project_word(output, mask);
    for bits in values {
        output.extend(bits.to_le_bytes());
    }
}

pub(super) fn project_collections(
    family: usize,
    contexts: impl Iterator<Item = (i32, Vec<SimulationPostColorView>)>,
) -> Vec<u8> {
    let mut output = vec![1, 0, 0, 0, 0, 0, 0, 0];
    let mut count = 0_u32;
    for (version, colors) in contexts {
        project_word(&mut output, family);
        project_word(&mut output, version as usize);
        project_word(&mut output, colors.len());
        for color in colors {
            project_color(&mut output, &color);
        }
        count += 1;
    }
    output[4..8].copy_from_slice(&count.to_le_bytes());
    output
}
