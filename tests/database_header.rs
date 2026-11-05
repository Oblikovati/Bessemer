//! Independent header-buffer boundaries and complete native document replacements (#222).
use bessemer::database_header::{
    DatabaseHeaderBuffer, DatabaseHeaderFields, DatabaseHeaderProfile,
};
use bessemer::{Document, FailureClass};

const PROFILE: DatabaseHeaderProfile = DatabaseHeaderProfile::DefaultWindows512;
const NATIVE: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const STREAMS: [&str; 2] = [
    "Contents/3DExperienceExchange2",
    "Contents/3DExperienceExchange_ConfigHeader",
];

fn literal_buffer(time: u32, sector: u32, page: u32, name: &[u8]) -> Vec<u8> {
    let mut bytes = b"unqlite\xdb\x7c\x27\x12".to_vec();
    for word in [time, sector, page] {
        bytes.extend(word.to_be_bytes());
    }
    bytes.extend((name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(name);
    bytes.extend((bytes.len()..512).map(|index| (index.wrapping_mul(73) ^ 0xa5) as u8));
    bytes
}

fn fields(time: u32, sector: u32, page: u32, name: &[u8]) -> DatabaseHeaderFields {
    DatabaseHeaderFields {
        packed_time: time,
        sector_size: sector,
        page_size: page,
        engine_name: name.to_vec(),
    }
}

#[test]
fn independent_literal_buffers_match_fields_prefix_extents_and_all_tail_bytes() {
    let packet = include_bytes!("fixtures/database-header/cases.bin");
    assert_eq!(&packet[..8], b"DBH1\x10\0\0\0");
    assert_eq!(packet.len(), 8 + 16 * 512);
    for (index, raw) in packet[8..].as_chunks::<512>().0.iter().enumerate() {
        let time = [0, 0x80000001, 0x5d384126, u32::MAX][index % 4];
        let name = [&b"hash"[..], &b"mem\0\xff"[..]][index / 8];
        let expected = packet_fields(index, time, name);
        let buffer = DatabaseHeaderBuffer::from_bytes(raw, PROFILE).unwrap();
        assert_eq!(buffer.fields(), &expected);
        assert_eq!(buffer.profile(), PROFILE);
        assert_eq!(buffer.interpreted_prefix_length(), 25 + name.len());
        assert_eq!(buffer.uninterpreted_tail(), &raw[25 + name.len()..]);
        assert_eq!(buffer.to_bytes(), raw);
        assert_eq!(
            DatabaseHeaderBuffer::new(PROFILE, expected, raw[25 + name.len()..].to_vec()).unwrap(),
            buffer
        );
    }
}

#[test]
fn stored_page_size_accepts_every_integer_without_a_power_of_two_filter() {
    for page in 512..=65536 {
        let raw = literal_buffer(0xffffffff, 512, page, b"hash");
        let buffer = DatabaseHeaderBuffer::from_bytes(&raw, PROFILE).unwrap();
        assert_eq!(buffer.fields().page_size, page);
        assert_eq!(buffer.to_bytes(), raw);
    }
}

fn packet_fields(index: usize, time: u32, name: &[u8]) -> DatabaseHeaderFields {
    fields(
        time,
        32u32 << (index % 12),
        [512, 513, 4096, 65536][index % 4],
        name,
    )
}

#[test]
fn stored_sector_size_follows_its_separate_full_signed_native_domain() {
    for sector in (0..=65537).chain([0x7fffffff, 0x80000000, u32::MAX]) {
        let expected = matches!(
            sector,
            32 | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096 | 8192 | 16384 | 32768 | 65536
        );
        let result =
            DatabaseHeaderBuffer::from_bytes(&literal_buffer(0, sector, 513, b""), PROFILE);
        assert_eq!(result.is_ok(), expected, "sector {sector}");
        if let Err(error) = result {
            assert!(error.cause.contains(&sector.to_string()));
        }
    }
}

#[test]
fn invalid_page_sizes_and_construction_shapes_fail_with_offending_values() {
    for page in (0..512)
        .chain(65537..66000)
        .chain([0x7fffffff, 0x80000000, u32::MAX])
    {
        let result =
            DatabaseHeaderBuffer::new(PROFILE, fields(0, 512, page, b"hash"), vec![0; 483]);
        let error = result.unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains(&page.to_string()));
    }
    for tail in [0, 482, 484, 512] {
        let error =
            DatabaseHeaderBuffer::new(PROFILE, fields(0, 512, 4096, b"hash"), vec![0; tail])
                .unwrap_err();
        assert!(error.cause.contains(&tail.to_string()));
    }
}

#[test]
fn every_name_length_keeps_non_utf8_and_nul_bytes_without_native_clamping() {
    for length in 0..=487 {
        let name: Vec<_> = (0..length)
            .map(|index| [0, 255, 0xc0, b'h'][index % 4])
            .collect();
        let raw = literal_buffer(0, 65536, 65536, &name);
        let buffer = DatabaseHeaderBuffer::from_bytes(&raw, PROFILE).unwrap();
        assert_eq!(buffer.fields().engine_name, name);
        assert_eq!(buffer.to_bytes(), raw);
    }
    for count in [488u16, 512, 65535] {
        let mut raw = literal_buffer(0, 512, 4096, b"hash");
        raw[23..25].copy_from_slice(&count.to_be_bytes());
        let error = DatabaseHeaderBuffer::from_bytes(&raw, PROFILE).unwrap_err();
        assert!(error.cause.contains(&count.to_string()));
    }
}

#[test]
fn oversized_constructed_names_fail_before_buffer_extent_arithmetic() {
    for length in [488, 65535] {
        let invalid = fields(0, 512, 4096, &vec![0xff; length]);
        let error = DatabaseHeaderBuffer::new(PROFILE, invalid, Vec::new()).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains(&length.to_string()));
    }
}

#[test]
fn arbitrary_time_words_roundtrip_without_calendar_normalization() {
    for value in 0u32..4096 {
        let time = value.wrapping_mul(0x9e3779b9).rotate_left(11);
        let raw = literal_buffer(time, 512, 513, b"hash");
        let buffer = DatabaseHeaderBuffer::from_bytes(&raw, PROFILE).unwrap();
        assert_eq!(buffer.fields().packed_time, time);
        assert_eq!(buffer.to_bytes(), raw);
    }
}

#[test]
fn every_truncated_or_extended_buffer_and_each_signature_mutation_is_rejected() {
    let raw = literal_buffer(0, 512, 4096, b"hash");
    for length in 0..512 {
        assert!(DatabaseHeaderBuffer::from_bytes(&raw[..length], PROFILE).is_err());
    }
    assert!(DatabaseHeaderBuffer::from_bytes(&[raw.clone(), vec![0]].concat(), PROFILE).is_err());
    for index in 0..11 {
        let mut changed = raw.clone();
        changed[index] ^= 0xff;
        assert!(DatabaseHeaderBuffer::from_bytes(&changed, PROFILE).is_err());
    }
}

#[test]
fn native_database_headers_reconstruct_with_explicit_prefix_only_coverage() {
    let document = Document::from_bytes(NATIVE).unwrap();
    for stream in STREAMS {
        let raw = document.read_stream(stream).unwrap();
        let buffer = DatabaseHeaderBuffer::from_bytes(&raw[..512], PROFILE).unwrap();
        assert_eq!(buffer.fields(), &fields(0x5d384126, 512, 4096, b"hash"));
        assert_eq!(buffer.interpreted_prefix_length(), 29);
        assert_eq!(buffer.uninterpreted_tail().len(), 483);
        assert_eq!(buffer.to_bytes(), raw[..512]);
    }
    assert_eq!(document.to_bytes().unwrap(), NATIVE);
}

fn changed_header(original: &[u8]) -> Vec<u8> {
    let buffer = DatabaseHeaderBuffer::from_bytes(&original[..512], PROFILE).unwrap();
    let mut changed = buffer.fields().clone();
    changed.packed_time = 0x80000001;
    let rebuilt =
        DatabaseHeaderBuffer::new(PROFILE, changed, buffer.uninterpreted_tail().to_vec()).unwrap();
    let mut stream = rebuilt.to_bytes();
    stream.extend_from_slice(&original[512..]);
    let mut expected = original.to_vec();
    expected[11..15].copy_from_slice(&0x80000001u32.to_be_bytes());
    assert_eq!(stream, expected);
    assert_eq!(buffer.to_bytes(), original[..512]);
    stream
}

#[test]
fn complete_native_document_replacement_changes_only_the_selected_time_word() {
    let source = Document::from_bytes(NATIVE).unwrap();
    for stream in STREAMS {
        let original = source.read_stream(stream).unwrap();
        let changed = changed_header(&original);
        let mut document = source.clone();
        document.replace_stream(stream, changed.clone()).unwrap();
        let fresh = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(fresh.read_stream(stream).unwrap(), changed);
        assert_unrelated_streams(&source, &fresh, stream);
        let mut reverse = fresh.clone();
        reverse.replace_stream(stream, original.clone()).unwrap();
        let restored = Document::from_bytes(&reverse.to_bytes().unwrap()).unwrap();
        assert_eq!(restored.read_stream(stream).unwrap(), original);
    }
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

fn assert_unrelated_streams(source: &Document, result: &Document, selected: &str) {
    for entry in source.streams().filter(|entry| entry.name != selected) {
        assert_eq!(
            result.read_stream(entry.name).unwrap(),
            source.read_stream(entry.name).unwrap()
        );
    }
}

#[test]
fn owned_buffers_isolate_sources_construction_failures_and_concurrent_writers() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DatabaseHeaderBuffer>();
    assert_send_sync::<DatabaseHeaderFields>();
    assert_send_sync::<DatabaseHeaderProfile>();
    let mut raw = literal_buffer(u32::MAX, 512, 513, b"h\0\xffh");
    let expected = raw.clone();
    let buffer = DatabaseHeaderBuffer::from_bytes(&raw, PROFILE).unwrap();
    raw.fill(0);
    assert!(DatabaseHeaderBuffer::new(PROFILE, buffer.fields().clone(), vec![0]).is_err());
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| buffer.to_bytes())).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(buffer.to_bytes(), expected);
}
