use super::packet::{PacketCursor, number};

pub fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
