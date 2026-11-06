use std::io::{Cursor, Read};
pub type PacketCursor = Cursor<Vec<u8>>;

pub fn cursor(bytes: &[u8], magic: [u8; 4]) -> PacketCursor {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(bytes)
        .take(16_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert!(raw.len() <= 16_000_000);
    let mut cursor = Cursor::new(raw);
    assert_eq!(number::<4>(&mut cursor), magic);
    cursor
}

pub fn finish(cursor: &PacketCursor) {
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
}

pub fn counted<T>(cursor: &mut PacketCursor, read: fn(&mut PacketCursor) -> T) -> Vec<T> {
    (0..word(cursor)).map(|_| read(cursor)).collect()
}

pub fn optional<T>(cursor: &mut PacketCursor, read: fn(&mut PacketCursor) -> T) -> Option<T> {
    boolean(cursor).then(|| read(cursor))
}

pub(super) fn number<const N: usize>(cursor: &mut PacketCursor) -> [u8; N] {
    let mut raw = [0; N];
    cursor.read_exact(&mut raw).unwrap();
    raw
}

pub fn byte(cursor: &mut PacketCursor) -> u8 {
    number::<1>(cursor)[0]
}
pub fn boolean(cursor: &mut PacketCursor) -> bool {
    let value = byte(cursor);
    assert!(value <= 1);
    value != 0
}
pub fn word(cursor: &mut PacketCursor) -> u32 {
    u32::from_le_bytes(number(cursor))
}
pub fn signed(cursor: &mut PacketCursor) -> i32 {
    i32::from_le_bytes(number(cursor))
}

pub fn blob(cursor: &mut PacketCursor) -> Vec<u8> {
    let mut raw = vec![0; word(cursor) as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
