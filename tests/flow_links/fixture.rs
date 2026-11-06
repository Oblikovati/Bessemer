use bessemer::flow::*;
use std::io::{Cursor, Read};

pub struct LinkCase {
    pub seed: FlowCarrier,
    pub candidates: Vec<FlowLinkCandidate>,
    pub requests: Vec<LinkRequest>,
    pub cpt: Option<usize>,
    pub settings: Option<usize>,
    pub stage: FlowRelatedSettingsStage,
}

pub struct LinkRequest {
    pub kind: FlowResultType,
    pub query: FlowGuid,
    pub selected: Option<usize>,
    pub visited: usize,
    pub eligible: usize,
}

pub fn limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 20_000,
        maximum_records: 100,
        maximum_payload_bytes: 10_000,
    }
}

pub fn cases() -> Vec<LinkCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-links/links.bin.gz")[..])
        .take(1_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FLK1");
    let count = u32::from_le_bytes(number(&mut cursor));
    assert_eq!(count, 21);
    let result = (0..count).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> LinkCase {
    let seed = FlowCarrier::from_bytes(&blob(cursor), limits()).unwrap();
    let count = u32::from_le_bytes(number(cursor));
    let candidates = (0..count).map(|_| candidate(cursor)).collect();
    let count = u32::from_le_bytes(number(cursor));
    assert_eq!(count, 60);
    let requests = (0..count).map(|_| request(cursor)).collect();
    let cpt = selected(cursor);
    let settings = selected(cursor);
    let stage = stage(cursor);
    LinkCase {
        seed,
        candidates,
        requests,
        cpt,
        settings,
        stage,
    }
}

fn candidate(cursor: &mut Cursor<&[u8]>) -> FlowLinkCandidate {
    let kind = FlowResultType::try_from(u32::from_le_bytes(number(cursor))).unwrap();
    let path = String::from_utf8(blob(cursor)).unwrap();
    let carrier = FlowCarrier::from_bytes(&blob(cursor), limits()).unwrap();
    FlowLinkCandidate::new(kind, path, carrier)
}

fn request(cursor: &mut Cursor<&[u8]>) -> LinkRequest {
    let kind = FlowResultType::try_from(u32::from_le_bytes(number(cursor))).unwrap();
    let query = FlowGuid::from_bytes(number(cursor));
    let selected = selected(cursor);
    let visited = u32::from_le_bytes(number(cursor)) as usize;
    let eligible = u32::from_le_bytes(number(cursor)) as usize;
    LinkRequest {
        kind,
        query,
        selected,
        visited,
        eligible,
    }
}

fn selected(cursor: &mut Cursor<&[u8]>) -> Option<usize> {
    let value = i32::from_le_bytes(number(cursor));
    if value == -1 {
        return None;
    }
    Some(usize::try_from(value).unwrap())
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let length = u32::from_le_bytes(number(cursor)) as usize;
    let mut bytes = vec![0; length];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn stage(cursor: &mut Cursor<&[u8]>) -> FlowRelatedSettingsStage {
    match number::<1>(cursor)[0] {
        0 => FlowRelatedSettingsStage::NoRelatedCpt,
        2 => FlowRelatedSettingsStage::EmptySettingsKey,
        3 => FlowRelatedSettingsStage::Searched,
        other => panic!("fixture stage {other}; expected 0, 2 or 3"),
    }
}
