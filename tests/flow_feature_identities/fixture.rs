use std::io::{Cursor, Read};

pub struct FeatureCase {
    pub name: String,
    pub identifier: u32,
    pub payload: Vec<u8>,
    pub version: u32,
    pub count: usize,
    pub objects: Vec<ExpectedObject>,
    pub tail: Option<Vec<i32>>,
    pub legacy: Vec<(Option<i32>, u32, bool)>,
    pub flags: Option<Vec<u8>>,
}

pub struct ExpectedObject {
    pub words: [u32; 7],
    pub text: Vec<u8>,
    pub logical: Vec<u8>,
    pub class: String,
    pub state: u8,
    pub boolean: bool,
}

pub fn cases() -> Vec<FeatureCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-feature-identities/identities.bin.gz")[..],
    )
    .take(100_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FFI1");
    assert_eq!(u32::from_le_bytes(number(&mut cursor)), 37);
    let selected = (0..37).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    selected
}

fn case(cursor: &mut Cursor<&[u8]>) -> FeatureCase {
    let mut case = FeatureCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        identifier: u32::from_le_bytes(number(cursor)),
        payload: blob(cursor),
        version: u32::from_le_bytes(number(cursor)),
        count: u32::from_le_bytes(number(cursor)) as usize,
        objects: Vec::new(),
        tail: None,
        legacy: Vec::new(),
        flags: None,
    };
    if case.identifier == 144 {
        case.flags = Some(blob(cursor));
        return case;
    }
    assert_eq!(case.identifier, 143);
    case.objects = (0..case.count).map(|_| object(cursor)).collect();
    read_tail(cursor, &mut case);
    case
}

fn object(cursor: &mut Cursor<&[u8]>) -> ExpectedObject {
    let words = std::array::from_fn(|_| u32::from_le_bytes(number(cursor)));
    let text = blob(cursor);
    let logical = blob(cursor);
    let class = String::from_utf8(blob(cursor)).unwrap();
    let state = number::<1>(cursor)[0];
    let boolean = number::<1>(cursor)[0] != 0;
    ExpectedObject {
        words,
        text,
        logical,
        class,
        state,
        boolean,
    }
}

fn read_tail(cursor: &mut Cursor<&[u8]>, case: &mut FeatureCase) {
    let stored = number::<1>(cursor)[0] != 0;
    let count = u32::from_le_bytes(number(cursor));
    if stored {
        case.tail = Some(
            (0..count)
                .map(|_| i32::from_le_bytes(number(cursor)))
                .collect(),
        );
        return;
    }
    assert_eq!(count, 6);
    case.legacy = (0..count).map(|_| context(cursor)).collect();
}

fn context(cursor: &mut Cursor<&[u8]>) -> (Option<i32>, u32, bool) {
    let present = number::<1>(cursor)[0] != 0;
    let value = i32::from_le_bytes(number(cursor));
    let count = u32::from_le_bytes(number(cursor));
    let derived = number::<1>(cursor)[0] != 0;
    (present.then_some(value), count, derived)
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let length = u32::from_le_bytes(number(cursor)) as usize;
    let mut raw = vec![0; length];
    cursor.read_exact(&mut raw).unwrap();
    raw
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut raw = [0; N];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
