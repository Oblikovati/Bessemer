use std::io::{Cursor, Read};

pub struct Rgb555Case<'a> {
    pub original: &'a [u8],
    pub edited: &'a [u8],
    pub words: Vec<u16>,
    pub edited_words: Vec<u16>,
}

fn rgb555_blob<'a>(cursor: &mut Cursor<&'a [u8]>) -> &'a [u8] {
    let mut size = [0; 4];
    cursor.read_exact(&mut size).unwrap();
    let start = cursor.position() as usize;
    let end = start + u32::from_le_bytes(size) as usize;
    cursor.set_position(end as u64);
    &cursor.get_ref()[start..end]
}

fn expected_rgb555_words(bytes: &[u8]) -> Vec<u16> {
    let (words, remainder) = bytes.as_chunks::<2>();
    assert!(remainder.is_empty());
    words.iter().map(|word| u16::from_le_bytes(*word)).collect()
}

fn rgb555_case<'a>(cursor: &mut Cursor<&'a [u8]>) -> Rgb555Case<'a> {
    Rgb555Case {
        original: rgb555_blob(cursor),
        edited: rgb555_blob(cursor),
        words: expected_rgb555_words(rgb555_blob(cursor)),
        edited_words: expected_rgb555_words(rgb555_blob(cursor)),
    }
}

pub fn rgb555_cases() -> Vec<Rgb555Case<'static>> {
    let packet = include_bytes!("../fixtures/preview-rgb555/cases.bin");
    let mut cursor = Cursor::new(packet.as_slice());
    let mut header = [0; 8];
    cursor.read_exact(&mut header).unwrap();
    assert_eq!(&header[..4], b"B165");
    assert_eq!(u32::from_le_bytes(header[4..].try_into().unwrap()), 136);
    let cases = (0..136).map(|_| rgb555_case(&mut cursor)).collect();
    assert_eq!(cursor.position() as usize, packet.len());
    cases
}
