//! Source-positive empty wires; the pending reader fix must preserve this baseline.
use bessemer::Document;

const EMPTY: &[u8] = include_bytes!("fixtures/container-empty/empty.bin");
const NATIVE: &[u8] = include_bytes!("fixtures/container-empty/native.bin");
const FILES: [&[u8]; 3] = [
    include_bytes!("fixtures/blank-part.SLDPRT"),
    include_bytes!("fixtures/blank-assembly.SLDASM"),
    include_bytes!("fixtures/blank-drawing.SLDDRW"),
];
struct EmptyReference<'a> {
    key: u32,
    local: [u8; 4],
    central: [u8; 4],
    end: [u8; 4],
    raw: &'a [u8],
}
fn fixed<const N: usize>(raw: &[u8], start: usize) -> [u8; N] {
    raw[start..start + N].try_into().unwrap()
}
fn empty_reference(row: &[u8; 46]) -> EmptyReference<'_> {
    EmptyReference {
        key: u32::from_be_bytes(fixed(row, 0)),
        local: fixed(row, 4),
        central: fixed(row, 8),
        end: fixed(row, 12),
        raw: &row[16..46],
    }
}
fn empty_cases() -> Vec<EmptyReference<'static>> {
    assert_eq!(&EMPTY[..4], b"CES1");
    assert_eq!(u32::from_le_bytes(fixed(EMPTY, 4)), 1000);
    assert_eq!(EMPTY.len(), 8 + 1000 * 46);
    EMPTY[8..]
        .as_chunks::<46>()
        .0
        .iter()
        .map(empty_reference)
        .collect()
}
#[test]
fn all_1000_source_accepted_empty_wires_have_complete_canonical_header_and_end_shapes() {
    let rows = empty_cases();
    let mut previous = None;
    for row in rows {
        assert!(previous.is_none_or(|key| key < row.key));
        previous = Some(row.key);
        assert_eq!(row.raw.len(), 30);
        assert_eq!(u32::from_be_bytes(fixed(row.raw, 0)), row.key);
        assert_eq!(u32::from_be_bytes(fixed(row.raw, 4)), 4);
        assert_eq!(fixed::<4>(row.raw, 8), row.end);
        assert_eq!(&row.raw[12..], &[0; 18]);
    }
}
#[test]
fn registered_signature_rows_match_all_three_native_fixture_headers() {
    assert_eq!(&NATIVE[..4], b"CEN1");
    assert_eq!(u32::from_le_bytes(fixed(NATIVE, 4)), 3);
    assert_eq!(NATIVE.len(), 8 + 3 * 41);
    let registry = empty_cases();
    for row in NATIVE[8..].as_chunks::<41>().0 {
        let raw = FILES[usize::from(row[0])];
        let key = u32::from_be_bytes(fixed(row, 1));
        let signatures = registry.iter().find(|entry| entry.key == key).unwrap();
        check_native_signatures(raw, row, signatures);
        let document = Document::from_bytes(raw).unwrap();
        assert_eq!(document.key(), key);
        assert_eq!(document.to_bytes().unwrap(), raw);
    }
}
fn check_native_signatures(raw: &[u8], row: &[u8], signatures: &EmptyReference<'_>) {
    for (index, signature) in [signatures.local, signatures.central, signatures.end]
        .iter()
        .enumerate()
    {
        let offset = u64::from_le_bytes(fixed(row, 17 + index * 8)) as usize;
        assert_eq!(fixed::<4>(raw, offset), *signature);
        assert_eq!(fixed::<4>(row, 5 + index * 4), *signature);
    }
}
