use std::io::{Cursor, Read};

pub struct GroupCase {
    pub name: String,
    pub wire: Vec<u8>,
    pub ordinal: usize,
    pub compressed: bool,
    pub decoded: Vec<u8>,
    pub version: i32,
    pub groups: Vec<ExpectedGroup>,
}

pub struct ExpectedGroup {
    pub start: usize,
    pub end: usize,
    pub auxiliary: Option<i32>,
    pub flags_present: bool,
    pub members: Vec<ExpectedMember>,
}

pub struct ExpectedMember {
    pub word: u32,
    pub wide: u64,
    pub flag: Option<u8>,
}

pub fn cases() -> Vec<GroupCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-groups/groups.bin.gz")[..])
        .take(4_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FGP1");
    let count = u32::from_le_bytes(number(&mut cursor));
    let selected = (0..count).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    selected
}

fn case(cursor: &mut Cursor<&[u8]>) -> GroupCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let wire = blob(cursor);
    let ordinal = u32::from_le_bytes(number(cursor)) as usize;
    let compressed = boolean(cursor);
    let decoded = blob(cursor);
    let version = i32::from_le_bytes(number(cursor));
    let count = u32::from_le_bytes(number(cursor));
    let groups = (0..count).map(|_| group(cursor)).collect();
    GroupCase {
        name,
        wire,
        ordinal,
        compressed,
        decoded,
        version,
        groups,
    }
}

fn group(cursor: &mut Cursor<&[u8]>) -> ExpectedGroup {
    let start = usize::try_from(u64::from_le_bytes(number(cursor))).unwrap();
    let end = usize::try_from(u64::from_le_bytes(number(cursor))).unwrap();
    let auxiliary = boolean(cursor).then(|| i32::from_le_bytes(number(cursor)));
    let flags_present = boolean(cursor);
    let count = u32::from_le_bytes(number(cursor));
    let members = (0..count).map(|_| member(cursor, flags_present)).collect();
    ExpectedGroup {
        start,
        end,
        auxiliary,
        flags_present,
        members,
    }
}

fn member(cursor: &mut Cursor<&[u8]>, flags_present: bool) -> ExpectedMember {
    let word = u32::from_le_bytes(number(cursor));
    let wide = u64::from_le_bytes(number(cursor));
    let flag = flags_present.then(|| number::<1>(cursor)[0]);
    ExpectedMember { word, wide, flag }
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut raw = [0; N];
    cursor.read_exact(&mut raw).unwrap();
    raw
}

fn boolean(cursor: &mut Cursor<&[u8]>) -> bool {
    match number::<1>(cursor)[0] {
        0 => false,
        1 => true,
        other => panic!("invalid fixture presence {other}"),
    }
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let length = usize::try_from(u64::from_le_bytes(number(cursor))).unwrap();
    let mut raw = vec![0; length];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
