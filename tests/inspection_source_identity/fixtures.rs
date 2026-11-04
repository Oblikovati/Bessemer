pub(super) struct ChecksumCase<'a> {
    pub label: &'a str,
    pub source: &'a [u8],
    pub md5: String,
}

pub(super) struct MetadataCase<'a> {
    pub label: &'a str,
    pub xml: &'a [u8],
    pub size_matches: bool,
    pub md5_matches: bool,
}

struct FixtureCursor<'a> {
    wire: &'a [u8],
    position: usize,
}

impl<'a> FixtureCursor<'a> {
    fn take(&mut self, length: usize) -> &'a [u8] {
        let start = self.position;
        self.position += length;
        self.wire
            .get(start..self.position)
            .expect("expected complete controlled checksum fixture span")
    }

    fn word(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }

    fn finish(&self) {
        assert_eq!(
            self.position,
            self.wire.len(),
            "expected exact checksum fixture EOF"
        );
    }
}

pub(super) fn checksum_cases() -> Vec<ChecksumCase<'static>> {
    let mut reader = FixtureCursor {
        wire: include_bytes!("../fixtures/inspection-source-identity/vectors.bin"),
        position: 0,
    };
    assert_eq!(reader.take(4), b"ISMD");
    let count = reader.word();
    assert_eq!(count, 172);
    let cases = (0..count).map(|_| checksum_case(&mut reader)).collect();
    reader.finish();
    cases
}

fn checksum_case<'a>(reader: &mut FixtureCursor<'a>) -> ChecksumCase<'a> {
    let label_length = reader.word();
    let source_length = reader.word();
    let md5: String = reader
        .take(16)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let label = std::str::from_utf8(reader.take(label_length)).unwrap();
    ChecksumCase {
        label,
        source: reader.take(source_length),
        md5,
    }
}

pub(super) fn metadata_cases() -> Vec<MetadataCase<'static>> {
    let mut reader = FixtureCursor {
        wire: include_bytes!("../fixtures/inspection-source-identity/metadata.bin"),
        position: 0,
    };
    assert_eq!(reader.take(4), b"ISMC");
    let count = reader.word();
    assert_eq!(count, 22);
    let cases = (0..count).map(|_| metadata_case(&mut reader)).collect();
    reader.finish();
    cases
}

fn metadata_case<'a>(reader: &mut FixtureCursor<'a>) -> MetadataCase<'a> {
    let label_length = reader.word();
    let xml_length = reader.word();
    let flags = reader.take(2);
    assert!(flags.iter().all(|flag| *flag <= 1));
    let label = std::str::from_utf8(reader.take(label_length)).unwrap();
    MetadataCase {
        label,
        xml: reader.take(xml_length),
        size_matches: flags[0] == 1,
        md5_matches: flags[1] == 1,
    }
}
