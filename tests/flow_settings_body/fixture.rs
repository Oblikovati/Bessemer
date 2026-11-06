use FlowSettingsBodyMember::*;
use bessemer::flow::*;
use std::io::{Cursor, Read};

const MEMBERS: [(u8, FlowSettingsBodyMember); 18] = [
    (0x34, Member34),
    (0x38, Member38),
    (0x4c, Member4c),
    (0x4d, Member4d),
    (0x50, Member50),
    (0x2d, Member2d),
    (0x44, Member44),
    (0x0f, Member0f),
    (0x10, Member10),
    (0x11, Member11),
    (0x88, Member88),
    (0x3c, Member3c),
    (0x41, Member41),
    (0x40, Member40),
    (0x1e, Member1e),
    (0x89, Member89),
    (0x8a, Member8a),
    (0x48, Member48),
];

pub struct BodyCase {
    pub version: u32,
    pub payload: Vec<u8>,
    pub fields: Vec<ExpectedMember>,
    pub contexts: Vec<(Option<i32>, FlowSettingsLegacyMember)>,
}

pub struct ExpectedMember {
    pub member: FlowSettingsBodyMember,
    pub offset: usize,
    pub value: FlowSettingsBodyValue,
}

pub fn cases() -> Vec<BodyCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-settings-body/body.bin.gz")[..])
        .take(20_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FSB1");
    assert_eq!(u32::from_le_bytes(number(&mut cursor)), 12);
    let cases = (0..12).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    cases
}

fn case(cursor: &mut Cursor<&[u8]>) -> BodyCase {
    let version = u32::from_le_bytes(number(cursor));
    let payload = blob(cursor);
    let count = u32::from_le_bytes(number(cursor));
    let fields = (0..count).map(|_| field(cursor)).collect();
    let count = u32::from_le_bytes(number(cursor));
    assert_eq!(count, 6);
    let contexts = (0..count).map(|_| context(cursor)).collect();
    BodyCase {
        version,
        payload,
        fields,
        contexts,
    }
}

fn field(cursor: &mut Cursor<&[u8]>) -> ExpectedMember {
    let label = number::<1>(cursor)[0];
    let member = MEMBERS.iter().find(|entry| entry.0 == label).unwrap().1;
    let offset = u32::from_le_bytes(number(cursor)) as usize;
    let shape = number::<1>(cursor)[0];
    let raw = blob(cursor);
    let value = match shape {
        0 => {
            assert_eq!(raw.len(), 1);
            FlowSettingsBodyValue::Byte(raw[0])
        }
        1 => FlowSettingsBodyValue::Word(u32::from_le_bytes(raw.try_into().unwrap())),
        2 => FlowSettingsBodyValue::Raw4(raw.try_into().unwrap()),
        3 => FlowSettingsBodyValue::Raw8(raw.try_into().unwrap()),
        other => panic!("fixture shape {other}; expected 0..=3"),
    };
    ExpectedMember {
        member,
        offset,
        value,
    }
}

fn context(cursor: &mut Cursor<&[u8]>) -> (Option<i32>, FlowSettingsLegacyMember) {
    let present = number::<1>(cursor)[0] != 0;
    let context = i32::from_le_bytes(number(cursor));
    let status = number::<1>(cursor)[0];
    let value = number::<1>(cursor)[0];
    let projected = match status {
        0 => FlowSettingsLegacyMember::RequiresRuntimeIntegerSlot88,
        1 => FlowSettingsLegacyMember::DerivedRuntime { context, value },
        2 => FlowSettingsLegacyMember::Stored(value),
        other => panic!("fixture context status {other}; expected 0..=2"),
    };
    (present.then_some(context), projected)
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
