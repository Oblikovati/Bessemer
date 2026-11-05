//! Native32 wrapping time semantics and actual persisted header callers (#223).
use bessemer::Document;
use bessemer::database_header::{
    DatabaseHeaderBuffer, DatabaseHeaderProfile, DatabaseTimeComponents,
};

const NATIVE: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const PROFILE: DatabaseHeaderProfile = DatabaseHeaderProfile::DefaultWindows512;
const STREAMS: [&str; 2] = [
    "Contents/3DExperienceExchange2",
    "Contents/3DExperienceExchange_ConfigHeader",
];

fn components(words: [i32; 6]) -> DatabaseTimeComponents {
    DatabaseTimeComponents {
        year: words[0],
        month: words[1],
        day: words[2],
        hour: words[3],
        minute: words[4],
        seconds: words[5],
    }
}

fn record_components(record: &[u8], offset: usize) -> DatabaseTimeComponents {
    components(std::array::from_fn(|index| {
        let start = offset + index * 4;
        i32::from_le_bytes(record[start..start + 4].try_into().unwrap())
    }))
}

#[test]
fn all_independent_big_integer_vectors_match_native_wrapping_and_extraction() {
    let packet = include_bytes!("fixtures/database-time/cases.bin");
    assert_eq!(&packet[..4], b"DBT1");
    let count = u32::from_le_bytes(packet[4..8].try_into().unwrap()) as usize;
    assert_eq!(count, 8422);
    assert_eq!(packet.len(), 8 + count * 52);
    for record in packet[8..].as_chunks::<52>().0 {
        let input = record_components(record, 0);
        let expected = u32::from_le_bytes(record[24..28].try_into().unwrap());
        assert_eq!(input.to_packed_word(), expected, "{input:?}");
        let extracted = DatabaseTimeComponents::from_packed_word(expected);
        assert_eq!(extracted, record_components(record, 28));
        assert_eq!(extracted.to_packed_word(), expected);
    }
}

#[test]
fn zeroed_callback_year_wrap_and_noncalendar_fields_have_literal_results() {
    let cases = [
        ([0; 6], 0x88000000),
        ([2026, 9, 24, 8, 9, 12], 0x5d384126),
        ([2108, 0, 0, 0, 0, 0], 0),
        ([1980, 16, 0, 0, 0, 0], 0x02000000),
        ([1980, 0, 32, 0, 0, 0], 0x00200000),
        ([1980, 0, 1, 32, 0, 0], 0x00010000),
        ([1980, 0, 0, 0, 64, 0], 0x00000800),
        ([1980, 0, 0, 0, 0, 64], 0x00000020),
    ];
    for (input, expected) in cases {
        assert_eq!(components(input).to_packed_word(), expected);
    }
}

#[test]
fn negative_odd_seconds_use_arithmetic_shift_and_positive_odd_seconds_lose_one() {
    for (seconds, expected) in [(-3, 0xfffffffe), (-2, u32::MAX), (-1, u32::MAX)] {
        assert_eq!(
            components([1980, 0, 0, 0, 0, seconds]).to_packed_word(),
            expected
        );
    }
    for seconds in [1, 3, 13, 59, 61] {
        let input = components([2026, 9, 24, 8, 9, seconds]);
        assert_eq!(
            DatabaseTimeComponents::from_packed_word(input.to_packed_word()).seconds,
            seconds - 1
        );
    }
}

#[test]
fn every_bit_and_both_sixteen_bit_halves_replay_the_complete_stored_word() {
    for bit in 0..32 {
        for word in [1u32 << bit, !(1u32 << bit)] {
            assert_eq!(
                DatabaseTimeComponents::from_packed_word(word).to_packed_word(),
                word
            );
        }
    }
    for half in 0..=u16::MAX {
        let low = u32::from(half);
        for word in [low, low << 16, low | 0xffff0000, (low << 16) | 0xffff] {
            assert_eq!(
                DatabaseTimeComponents::from_packed_word(word).to_packed_word(),
                word
            );
        }
    }
}

fn rewritten_time_stream(raw: &[u8], time: DatabaseTimeComponents) -> Vec<u8> {
    let header = DatabaseHeaderBuffer::from_bytes(&raw[..512], PROFILE).unwrap();
    let mut fields = header.fields().clone();
    fields.packed_time = time.to_packed_word();
    let result =
        DatabaseHeaderBuffer::new(PROFILE, fields, header.uninterpreted_tail().to_vec()).unwrap();
    let mut encoded = result.to_bytes();
    encoded.extend_from_slice(&raw[512..]);
    let mut expected = raw.to_vec();
    expected[11..15].copy_from_slice(&time.to_packed_word().to_be_bytes());
    assert_eq!(encoded, expected);
    assert_eq!(header.to_bytes(), raw[..512]);
    encoded
}

#[test]
fn native_header_time_words_extract_and_reconstruct_with_the_default_month_origin() {
    let source = Document::from_bytes(NATIVE).unwrap();
    for stream in STREAMS {
        let raw = source.read_stream(stream).unwrap();
        let header = DatabaseHeaderBuffer::from_bytes(&raw[..512], PROFILE).unwrap();
        let extracted = DatabaseTimeComponents::from_packed_word(header.fields().packed_time);
        assert_eq!(extracted, components([2026, 9, 24, 8, 9, 12]));
        assert_eq!(rewritten_time_stream(&raw, extracted), raw);
    }
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

fn check_containing_document(stream: &str, input: DatabaseTimeComponents, expected_word: u32) {
    let source = Document::from_bytes(NATIVE).unwrap();
    let original = source.read_stream(stream).unwrap();
    assert_eq!(input.to_packed_word(), expected_word);
    let replacement = rewritten_time_stream(&original, input);
    let mut document = source.clone();
    document
        .replace_stream(stream, replacement.clone())
        .unwrap();
    let fresh = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(fresh.read_stream(stream).unwrap(), replacement);
    for entry in source.streams().filter(|entry| entry.name != stream) {
        assert_eq!(
            fresh.read_stream(entry.name).unwrap(),
            source.read_stream(entry.name).unwrap()
        );
    }
    reverse_containing_document(&fresh, &original, stream);
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

fn reverse_containing_document(fresh: &Document, original: &[u8], stream: &str) {
    let mut reverse = fresh.clone();
    reverse.replace_stream(stream, original.to_vec()).unwrap();
    let reopened = Document::from_bytes(&reverse.to_bytes().unwrap()).unwrap();
    assert_eq!(reopened.read_stream(stream).unwrap(), original);
}

#[test]
fn zeroed_odd_and_wrapped_components_reach_complete_containing_document_writes() {
    for stream in STREAMS {
        for (input, expected) in [
            ([0; 6], 0x88000000),
            ([2026, 9, 24, 8, 9, 13], 0x5d384126),
            ([1980, 0, 0, 0, 0, -3], 0xfffffffe),
        ] {
            check_containing_document(stream, components(input), expected);
        }
    }
}

#[test]
fn lost_odd_second_produces_an_exact_complete_carrier_noop() {
    let source = Document::from_bytes(NATIVE).unwrap();
    for stream in STREAMS {
        let raw = source.read_stream(stream).unwrap();
        let replacement = rewritten_time_stream(&raw, components([2026, 9, 24, 8, 9, 13]));
        assert_eq!(replacement, raw);
        let mut document = source.clone();
        document.replace_stream(stream, replacement).unwrap();
        assert_eq!(document.to_bytes().unwrap(), NATIVE);
    }
}

#[test]
fn immutable_time_components_are_send_sync_and_replay_identically_on_eight_workers() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DatabaseTimeComponents>();
    let input = components([i32::MIN, i32::MAX, -1, 32, 64, -3]);
    let expected = input.to_packed_word();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| input.to_packed_word()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(input.to_packed_word(), expected);
}
