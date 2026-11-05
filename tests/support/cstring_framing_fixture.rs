const ORIGINAL_EQUATION: &[u8] = include_bytes!("../fixtures/single-span-splines/owned-2.bin");

/// Build independent count-width frames in a complete equation graph; e.g. `cstring_equation_wire(true, 1)`.
pub(crate) fn cstring_equation_wire(wide: bool, width: usize) -> Vec<u8> {
    let payload = if wide {
        "Ω equation 0"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>()
    } else {
        b"ABC".to_vec()
    };
    let count = if wide { 12_u32 } else { 3_u32 };
    let mut text = literal_count_prefix(wide, width, count);
    text.extend_from_slice(&payload);
    replace_equation_text_frame(text)
}

fn literal_count_prefix(wide: bool, width: usize, count: u32) -> Vec<u8> {
    let mut text = if wide {
        vec![0xff, 0xfe, 0xff]
    } else {
        Vec::new()
    };
    match width {
        0 => text.push(count as u8),
        1 => {
            text.push(0xff);
            text.extend_from_slice(&(count as u16).to_le_bytes());
        }
        2 => {
            text.extend_from_slice(&[0xff, 0xff, 0xff]);
            text.extend_from_slice(&count.to_le_bytes());
        }
        _ => panic!("prefix width {width}; expected0..=2"),
    }
    text
}

fn replace_equation_text_frame(text: Vec<u8>) -> Vec<u8> {
    let mut needle = vec![0xff, 0xfe, 0xff, 12];
    needle.extend("Ω equation 0".encode_utf16().flat_map(u16::to_le_bytes));
    let positions: Vec<_> = ORIGINAL_EQUATION
        .windows(needle.len())
        .enumerate()
        .filter(|(_, window)| *window == needle)
        .map(|(index, _)| index)
        .collect();
    assert_eq!(positions.len(), 1);
    let mut raw = ORIGINAL_EQUATION.to_vec();
    raw.splice(positions[0]..positions[0] + needle.len(), text);
    raw
}
