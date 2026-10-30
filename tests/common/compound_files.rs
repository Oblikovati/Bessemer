use bessemer::compound::*;
use std::io::Read;

pub const LIMITS: CompoundLimits = CompoundLimits {
    maximum_file_bytes: 100_000_000,
    maximum_stream_bytes: 100_000_000,
    maximum_total_stream_bytes: 100_000_000,
    maximum_directory_entries: 100_000,
    maximum_storage_depth: 1000,
};

pub struct Case<'a> {
    pub bytes: &'a [u8],
    pub oracle: &'a [u8],
    pub canonical: Option<&'a [u8]>,
}

pub struct Cursor<'a>(pub &'a [u8]);

impl<'a> Cursor<'a> {
    pub fn take(&mut self, count: usize) -> &'a [u8] {
        let (head, tail) = self.0.split_at(count);
        self.0 = tail;
        head
    }

    pub fn word(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }

    pub fn span(&mut self) -> &'a [u8] {
        let count = self.word();
        self.take(count)
    }
}

pub fn owned() -> Vec<u8> {
    let source = include_bytes!("../fixtures/compound-files/owned.bin");
    let decoder = flate2::read::ZlibDecoder::new(source.as_slice());
    let mut output = Vec::new();
    decoder.take(100_000_000).read_to_end(&mut output).unwrap();
    output
}

pub fn cases(packet: &[u8], native: bool) -> Vec<Case<'_>> {
    let mut cursor = Cursor(packet);
    assert_eq!(cursor.take(4), if native { b"CFBN" } else { b"CFBO" });
    let cases = (0..cursor.word())
        .map(|_| Case {
            bytes: cursor.span(),
            oracle: cursor.span(),
            canonical: if native { None } else { Some(cursor.span()) },
        })
        .collect();
    assert!(cursor.0.is_empty());
    cases
}

pub fn check_case(case: &Case<'_>) -> CompoundFile {
    let file = CompoundFile::from_bytes(case.bytes, LIMITS).unwrap();
    assert_eq!(file.to_bytes(), case.bytes);
    check_oracle(&file, case.oracle);
    let rebuilt = file.reconstructed_bytes().unwrap();
    if let Some(expected) = case.canonical {
        assert_eq!(rebuilt, expected);
    }
    let reopened = CompoundFile::from_bytes(&rebuilt, LIMITS).unwrap();
    assert_equivalent(&file, &reopened, None);
    file
}

pub fn check_oracle(file: &CompoundFile, oracle: &[u8]) {
    let mut cursor = Cursor(oracle);
    let major = cursor.word();
    assert_eq!(file.coverage().major as usize, major);
    assert_eq!(file.coverage().directory_slots, cursor.word());
    assert_eq!(file.entries().len(), cursor.word());
    for entry in file.entries() {
        check_entry(file, entry, &mut cursor);
    }
    assert!(cursor.0.is_empty());
}

pub fn assert_equivalent(
    original: &CompoundFile,
    result: &CompoundFile,
    replacement: Option<(usize, &[u8])>,
) {
    assert_eq!(original.entries().len(), result.entries().len());
    assert_eq!(original.coverage().major, result.coverage().major);
    for (before, after) in original.entries().iter().zip(result.entries()) {
        assert_eq!(
            (before.id().slot(), before.kind(), before.name_units()),
            (after.id().slot(), after.kind(), after.name_units())
        );
        assert_eq!(
            before.parent().map(|id| id.slot()),
            after.parent().map(|id| id.slot())
        );
        assert_eq!(metadata(before), metadata(after));
        if before.kind() != CompoundEntryKind::Stream {
            continue;
        }
        assert_stream(original, result, before, after, replacement);
    }
}

fn metadata(entry: &CompoundEntry) -> [u8; 128] {
    let mut raw = *entry.directory_record();
    if matches!(
        entry.kind(),
        CompoundEntryKind::Root | CompoundEntryKind::Stream
    ) {
        raw[116..128].fill(0);
    }
    raw
}

pub fn changed_payload(original: &[u8]) -> Vec<u8> {
    let mut replacement = original.to_vec();
    if let Some(first) = replacement.first_mut() {
        *first ^= 1;
    } else {
        replacement.extend_from_slice(b"BessemerCFB");
    }
    replacement
}

fn check_entry(file: &CompoundFile, entry: &CompoundEntry, cursor: &mut Cursor<'_>) {
    assert_eq!(entry.id().slot(), cursor.word());
    assert_eq!(
        entry.parent().map_or(u32::MAX as usize, |id| id.slot()),
        cursor.word()
    );
    assert_eq!(entry.directory_record(), cursor.take(128));
    let expected = cursor.span();
    if entry.kind() == CompoundEntryKind::Stream {
        assert_eq!(file.stream(entry.id()).unwrap(), expected);
    } else {
        assert!(expected.is_empty());
    }
}

fn assert_stream(
    original: &CompoundFile,
    result: &CompoundFile,
    before: &CompoundEntry,
    after: &CompoundEntry,
    replacement: Option<(usize, &[u8])>,
) {
    let expected = replacement
        .filter(|(slot, _)| *slot == before.id().slot())
        .map_or_else(|| original.stream(before.id()).unwrap(), |(_, bytes)| bytes);
    assert_eq!(result.stream(after.id()).unwrap(), expected);
    assert_eq!(after.logical_size(), expected.len());
}

pub fn word_at(raw: &[u8], offset: usize) -> usize {
    u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap()) as usize
}
