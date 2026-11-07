use super::*;

pub(crate) fn verify_native_saved_index(raw: &[u8]) -> usize {
    let source = parse_saved(raw);
    assert_eq!(source.to_bytes(), raw);
    verify_saved_mutation(&source, &native_header_change(raw));
    for ordinal in 0..source.mappings().len() {
        verify_native_mapping(&source, raw, ordinal);
    }
    source.mappings().len()
}

fn native_header_change(raw: &[u8]) -> SavedMutation {
    let words = noncanonical_header_words();
    let expected = changed_header_bytes(raw, 4096, words);
    SavedMutation {
        request: SavedRequest::Header(words),
        span: 4104..4128,
        expected,
    }
}

fn verify_native_mapping(source: &HashFileIndexBuffer, raw: &[u8], ordinal: usize) {
    let start = 4096 + 44 + ordinal * 16;
    let mut expected = raw.to_vec();
    write_word(&mut expected, start, u64::MAX);
    write_word(&mut expected, start + 8, 0);
    verify_saved_mutation(
        source,
        &SavedMutation {
            request: SavedRequest::Mapping(ordinal, OUT_OF_FILE_PAIR),
            span: start..start + 16,
            expected,
        },
    );
}
