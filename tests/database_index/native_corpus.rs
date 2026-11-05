use super::*;

fn corpus_packet() -> Vec<u8> {
    let compressed = include_bytes!("../fixtures/database-index/native-files.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(63_716_893)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw.len(), 63_716_892);
    raw
}

fn native_case(cursor: &mut &[u8], raw: &[u8], graph: &HashFileGraph) -> FileCase {
    let offset = u16::from_le_bytes(word(cursor));
    let page = NonZeroU64::new(u64::from_le_bytes(word(cursor))).unwrap();
    let stored_hash = u32::from_le_bytes(word(cursor));
    let key_length = u32::from_le_bytes(word(cursor)) as usize;
    let length = u64::from_le_bytes(word(cursor)) as usize;
    let key = take(cursor, key_length).to_vec();
    let value = take(cursor, length).to_vec();
    let replacement: Vec<_> = value.iter().map(|byte| byte ^ 0xa5).collect();
    let spans = decode_spans(cursor);
    let expected = expected_bytes(raw, &spans, &replacement);
    check_stored_hash(graph, page, offset, stored_hash);
    FileCase {
        page,
        offset,
        key,
        value,
        replacement,
        spans,
        expected,
    }
}

fn check_stored_hash(graph: &HashFileGraph, page: NonZeroU64, offset: u16, hash: u32) {
    let bucket = graph.bucket(page.get()).unwrap();
    let cell = bucket
        .cells()
        .iter()
        .find(|cell| cell.offset() == offset)
        .unwrap();
    assert_eq!(cell.key_hash(), hash);
}

fn expected_bytes(raw: &[u8], spans: &[std::ops::Range<usize>], replacement: &[u8]) -> Vec<u8> {
    let mut expected = raw.to_vec();
    let mut cursor = 0;
    for span in spans {
        expected[span.clone()].copy_from_slice(&replacement[cursor..cursor + span.len()]);
        cursor += span.len();
    }
    assert_eq!(cursor, replacement.len());
    expected
}

fn verify_native_file(cursor: &mut &[u8], ordinal: u32) -> (usize, usize, usize) {
    assert_eq!(u32::from_le_bytes(word(cursor)), ordinal);
    let length = u32::from_le_bytes(word(cursor)) as usize;
    let count = u32::from_le_bytes(word(cursor)) as usize;
    let raw = take(cursor, length);
    let graph = parse(raw);
    let mappings = super::saved_buffers::verify_native_saved_index(raw);
    assert_eq!(graph.coverage().cells, count);
    for _ in 0..count {
        let case = native_case(cursor, raw, &graph);
        verify_case(&graph, raw, &case);
    }
    (graph.coverage().pages, count, mappings)
}

#[test]
fn all_native_file_graphs_match_every_independent_cell_edit_and_fresh_reversal() {
    let packet = corpus_packet();
    let mut cursor = packet.as_slice();
    assert_eq!(take(&mut cursor, 4), b"BFG1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 2486);
    let mut pages = 0;
    let mut cells = 0;
    let mut mappings = 0;
    for ordinal in 0..2486 {
        let checked = verify_native_file(&mut cursor, ordinal);
        pages += checked.0;
        cells += checked.1;
        mappings += checked.2;
    }
    assert_eq!((pages, cells, mappings), (13406, 22748, 3162));
    assert!(cursor.is_empty());
}
