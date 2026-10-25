use bessemer::display_lists::DisplayDocumentKind;
#[path = "../../examples/support/framing.rs"]
pub(crate) mod framing;

#[derive(Clone, Copy)]
pub(crate) struct OldCase {
    pub version: u32,
    pub kind: DisplayDocumentKind,
    pub scenario: u8,
    pub bytes: &'static [u8],
    pub expected: &'static [u8],
    pub scalars: &'static str,
}

fn blob(remaining: &mut &'static [u8]) -> &'static [u8] {
    let length = framing::take_u32(remaining).unwrap() as usize;
    framing::take(remaining, length).unwrap()
}

fn read_case(remaining: &mut &'static [u8]) -> OldCase {
    let version = framing::take_u32(remaining).unwrap();
    let kind = if framing::take(remaining, 1).unwrap()[0] == 0 {
        DisplayDocumentKind::Part
    } else {
        DisplayDocumentKind::Assembly
    };
    let scenario = framing::take(remaining, 1).unwrap()[0];
    let bytes = blob(remaining);
    let expected = blob(remaining);
    let scalars = std::str::from_utf8(blob(remaining)).unwrap();
    OldCase {
        version,
        kind,
        scenario,
        bytes,
        expected,
        scalars,
    }
}

pub(crate) fn decode(bytes: &'static [u8]) -> Vec<OldCase> {
    let mut remaining = bytes;
    let count = framing::take_u32(&mut remaining).unwrap();
    framing::require_count(count, remaining.len(), 18).unwrap();
    let cases = (0..count).map(|_| read_case(&mut remaining)).collect();
    framing::require_finished(remaining).unwrap();
    cases
}
