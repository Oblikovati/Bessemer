use bessemer::display_lists::DisplayDocumentKind;
#[path = "../../examples/support/framing.rs"]
mod framing;

#[derive(Clone, Copy)]
pub(crate) struct FileContextCase {
    pub version: u32,
    pub file_version: u32,
    pub kind: DisplayDocumentKind,
    pub scenario: u8,
    pub entries: usize,
    pub objects: usize,
    pub bytes: &'static [u8],
    pub expected: &'static [u8],
}

fn file_context_blob(remaining: &mut &'static [u8]) -> &'static [u8] {
    let length = framing::take_u32(remaining).unwrap() as usize;
    framing::take(remaining, length).unwrap()
}

fn read_case(remaining: &mut &'static [u8]) -> FileContextCase {
    let version = framing::take_u32(remaining).unwrap();
    let file_version = framing::take_u32(remaining).unwrap();
    let kind = match framing::take(remaining, 1).unwrap()[0] {
        0 => DisplayDocumentKind::Part,
        1 => DisplayDocumentKind::Assembly,
        other => panic!("kind {other}; expected 0 or 1"),
    };
    let scenario = framing::take(remaining, 1).unwrap()[0];
    let entries = framing::take_u32(remaining).unwrap() as usize;
    let objects = framing::take_u32(remaining).unwrap() as usize;
    FileContextCase {
        version,
        file_version,
        kind,
        scenario,
        entries,
        objects,
        bytes: file_context_blob(remaining),
        expected: file_context_blob(remaining),
    }
}

pub(crate) fn decode(bytes: &'static [u8]) -> Vec<FileContextCase> {
    let mut remaining = bytes;
    let count = framing::take_u32(&mut remaining).unwrap();
    framing::require_count(count, remaining.len(), 26).unwrap();
    let cases = (0..count).map(|_| read_case(&mut remaining)).collect();
    framing::require_finished(remaining).unwrap();
    cases
}
