use super::*;
use std::{io::Read, ops::Range};

pub(super) struct Case {
    pub name: String,
    pub raw: Vec<u8>,
    pub comment: Vec<u8>,
    pub members: Vec<Member>,
}

pub(super) struct Member {
    pub name: String,
    pub flags: u16,
    pub method: u16,
    pub crc: u32,
    pub payload: Vec<u8>,
    pub extra: Vec<u8>,
    pub comment: Vec<u8>,
    pub ini: Ini,
}

pub(super) enum Ini {
    Unselected,
    Opaque,
    Records(Vec<IniRecord>),
}

pub(super) struct IniRecord {
    pub kind: ElectricalIniKind,
    pub raw: Vec<u8>,
    pub section: Option<Vec<u8>>,
    pub key: Option<Vec<u8>>,
    pub value: Option<Vec<u8>>,
}

fn take<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (head, tail) = cursor.split_at(count);
    *cursor = tail;
    head
}

fn word<const N: usize>(cursor: &mut &[u8]) -> [u8; N] {
    take(cursor, N).try_into().unwrap()
}

fn counted(cursor: &mut &[u8]) -> Vec<u8> {
    let count = u32::from_le_bytes(word(cursor)) as usize;
    take(cursor, count).to_vec()
}

fn optional(cursor: &mut &[u8]) -> Option<Vec<u8>> {
    let count = u32::from_le_bytes(word(cursor));
    (count != u32::MAX).then(|| take(cursor, count as usize).to_vec())
}

pub(super) fn cases() -> Vec<Case> {
    let compressed = include_bytes!("../fixtures/electrical-projects/archive-cases.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(5_600_294)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw.len(), 5_600_293);
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"ECP1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 8);
    let cases = (0..8).map(|_| case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases
}

fn case(cursor: &mut &[u8]) -> Case {
    let name = String::from_utf8(counted(cursor)).unwrap();
    let raw = counted(cursor);
    let comment = counted(cursor);
    let count = u32::from_le_bytes(word(cursor)) as usize;
    assert!(count <= cursor.len() / 25);
    let members = (0..count).map(|_| member(cursor)).collect();
    Case {
        name,
        raw,
        comment,
        members,
    }
}

fn member(cursor: &mut &[u8]) -> Member {
    let name = String::from_utf8(counted(cursor)).unwrap();
    let flags = u16::from_le_bytes(word(cursor));
    let method = u16::from_le_bytes(word(cursor));
    let crc = u32::from_le_bytes(word(cursor));
    let payload = counted(cursor);
    let extra = counted(cursor);
    let comment = counted(cursor);
    let ini = ini(cursor);
    Member {
        name,
        flags,
        method,
        crc,
        payload,
        extra,
        comment,
        ini,
    }
}

fn ini(cursor: &mut &[u8]) -> Ini {
    match take(cursor, 1)[0] {
        0 => Ini::Unselected,
        1 => Ini::Opaque,
        2 => {
            let count = u32::from_le_bytes(word(cursor)) as usize;
            assert!(count <= cursor.len() / 17);
            Ini::Records((0..count).map(|_| record(cursor)).collect())
        }
        tag => panic!("INI tag {tag}; expected 0,1,2"),
    }
}

fn record(cursor: &mut &[u8]) -> IniRecord {
    let kind = match take(cursor, 1)[0] {
        0 => ElectricalIniKind::Other,
        1 => ElectricalIniKind::Section,
        2 => ElectricalIniKind::Field,
        tag => panic!("record tag {tag}; expected 0,1,2"),
    };
    IniRecord {
        kind,
        raw: counted(cursor),
        section: optional(cursor),
        key: optional(cursor),
        value: optional(cursor),
    }
}

pub(super) fn directory_offsets(raw: &[u8], comment: usize, count: usize) -> Vec<usize> {
    directory_records(raw, comment, count)
        .iter()
        .map(|record| u32_at(record, 42) as usize)
        .collect()
}

pub(super) fn directory_records(raw: &[u8], comment: usize, count: usize) -> Vec<&[u8]> {
    let end = raw.len() - 22 - comment;
    assert_eq!(&raw[end..end + 4], b"PK\x05\x06");
    let mut cursor = u32_at(raw, end + 16) as usize;
    let mut records = Vec::new();
    for _ in 0..count {
        assert_eq!(&raw[cursor..cursor + 4], b"PK\x01\x02");
        let size = 46
            + [28, 30, 32]
                .iter()
                .map(|at| usize::from(u16_at(raw, cursor + at)))
                .sum::<usize>();
        records.push(&raw[cursor..cursor + size]);
        cursor += size;
    }
    assert_eq!(cursor, end);
    records
}

pub(super) fn u16_at(raw: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(raw[offset..offset + 2].try_into().unwrap())
}

pub(super) fn u32_at(raw: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap())
}

pub(super) fn encoded_member(raw: &[u8], offset: usize) -> &[u8] {
    assert_eq!(&raw[offset..offset + 4], b"PK\x03\x04");
    let header = 30 + usize::from(u16_at(raw, offset + 26)) + usize::from(u16_at(raw, offset + 28));
    &raw[offset..offset + header + u32_at(raw, offset + 18) as usize]
}

pub(super) fn independent_value_range(records: &[IniRecord], ordinal: usize) -> Range<usize> {
    let record = &records[ordinal];
    let start = records[..ordinal]
        .iter()
        .map(|record| record.raw.len())
        .sum::<usize>();
    let start = start + record.raw.iter().position(|byte| *byte == b'=').unwrap() + 1;
    start..start + record.value.as_ref().unwrap().len()
}
