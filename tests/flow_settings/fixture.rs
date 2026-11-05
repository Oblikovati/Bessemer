use bessemer::flow::*;
use std::io::Read;

pub struct SelectionCase {
    pub incoming: FlowGuid,
    pub cached: FlowGuid,
    pub precedence: u8,
    pub action: FlowSettingsAction,
    pub load: bool,
    pub after: FlowGuid,
    pub warning: bool,
}

pub fn cases() -> Vec<SelectionCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-settings/selection.bin.gz")[..])
        .take(5_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(&raw[..4], b"FSS1");
    let count = u32::from_le_bytes(raw[4..8].try_into().unwrap()) as usize;
    assert_eq!(raw.len(), 8 + 52 * count);
    raw[8..].as_chunks::<52>().0.iter().map(case).collect()
}

fn case(raw: &[u8; 52]) -> SelectionCase {
    let action = match raw[33] {
        0 => FlowSettingsAction::RetainFldSettings,
        1 => FlowSettingsAction::SkipCachedIdentity,
        2 => FlowSettingsAction::InvokeLoaderSequence,
        other => panic!("invalid fixture action {other}"),
    };
    SelectionCase {
        incoming: FlowGuid::from_bytes(raw[..16].try_into().unwrap()),
        cached: FlowGuid::from_bytes(raw[16..32].try_into().unwrap()),
        precedence: raw[32],
        action,
        load: boolean(raw[34]),
        after: FlowGuid::from_bytes(raw[35..51].try_into().unwrap()),
        warning: boolean(raw[51]),
    }
}

fn boolean(value: u8) -> bool {
    match value {
        0 => false,
        1 => true,
        other => panic!("invalid fixture boolean {other}"),
    }
}
