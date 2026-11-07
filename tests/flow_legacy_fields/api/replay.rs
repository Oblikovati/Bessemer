use super::*;
use std::io::{Read, Write};
type Transcript = flate2::write::ZlibEncoder<Vec<u8>>;
const START: &[u8] = b"BESSEMER_LEGACY_FIELDS_START\0";
const END: &[u8] = b"\0BESSEMER_LEGACY_FIELDS_END";
#[test]
fn two_fresh_processes_match_source_verified_reads_queries_and_representative_edits() {
    let first = process_transcript();
    assert_eq!(first, process_transcript());
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(first.as_slice())
        .take(64_000_001)
        .read_to_end(&mut decoded)
        .unwrap();
    assert!(!decoded.is_empty());
    assert!(decoded.len() <= 64_000_000);
}
fn process_transcript() -> Vec<u8> {
    crate::process::transcript("api::replay::legacy_fields_replay_worker", START, END)
}
#[test]
#[ignore = "The parent invokes this independent Source-verifying worker twice"]
fn legacy_fields_replay_worker() {
    let mut transcript = Transcript::new(Vec::new(), flate2::Compression::default());
    append_words(&mut transcript);
    append_groups(&mut transcript);
    append_queries(&mut transcript);
    selection_api_preserves_all_400_frozen_preference_latest_empty_and_failure_cases();
    let compressed = transcript.finish().unwrap();
    let mut output = std::io::stdout().lock();
    output.write_all(START).unwrap();
    output.write_all(&compressed).unwrap();
    output.write_all(END).unwrap();
}
fn append(transcript: &mut Transcript, bytes: &[u8]) {
    transcript
        .write_all(&(bytes.len() as u64).to_le_bytes())
        .unwrap();
    transcript.write_all(bytes).unwrap();
}
fn append_words(transcript: &mut Transcript) {
    for row in fixture::words() {
        let fields = read_words(&row);
        check_word_values(&fields, &row);
        append(transcript, fields.payload_bytes());
        if fields.stored_count() != 0 {
            let value = !row.patterns[0];
            let edit = fields.replace_pattern(0, value).unwrap();
            let mut expected = row.raw.clone();
            let (start, end) = fields.pattern_interval(0).unwrap();
            expected[start..end].copy_from_slice(&value.to_le_bytes());
            assert_eq!(edit.result().payload_bytes(), expected);
            append(transcript, edit.result().source().as_bytes());
        }
    }
}
fn append_groups(transcript: &mut Transcript) {
    for row in fixture::groups() {
        let fields = contracts::open_group(&row.stored, group_options()).unwrap();
        check_group_fields(&fields, &row);
        check_group_scopes(&fields, &row);
        append(transcript, fields.payload_bytes());
        append_group_edit(transcript, &fields);
    }
}
fn append_group_edit(transcript: &mut Transcript, fields: &FlowLegacyGroup155) {
    let Some(field) = fields
        .fields()
        .find(|field| field.kind() == FlowConditionalFieldKind::MemberA0Pattern)
    else {
        return;
    };
    let original = field.value(0).unwrap();
    let FlowConditionalValue::WordBits(bits) = original else {
        panic!("expected raw header word")
    };
    let value = FlowConditionalValue::WordBits(!bits);
    let edit = fields.replace_field(&field.id(), 0, value).unwrap();
    let mut expected = fields.payload_bytes().to_vec();
    let offset = field.element_offset(0).unwrap();
    expected[offset..offset + 4].copy_from_slice(&(!bits).to_le_bytes());
    assert_eq!(edit.result().payload_bytes(), expected);
    append(transcript, edit.result().source().as_bytes());
}
fn append_queries(transcript: &mut Transcript) {
    let mut raw = vec![0; 2560];
    for row in fixture::queries() {
        raw[56..64].copy_from_slice(&row.slot7.to_le_bytes());
        raw[80..88].copy_from_slice(&row.slot10.to_le_bytes());
        let source = carrier::carrier(105, &raw);
        let id = source.latest(105).unwrap().unwrap().id;
        let fields = source.record_105_fields(&id, word_options()).unwrap();
        let query = fields.queries_105().unwrap();
        check_query(query, &row.query);
        let status = query.slot7.precheck_status().unwrap_or(0);
        append(
            transcript,
            &[
                u8::from(query.slot10_nonzero),
                u8::from(query.slot7.would_call_virtual_78()),
            ],
        );
        append(transcript, &status.to_le_bytes());
    }
}
#[test]
fn shared_legacy_snapshots_match_serial_edits_after_input_destruction_across_eight_workers() {
    let mut row = fixture::groups()
        .into_iter()
        .find(|row| row.stored.len() > 100 && row.stored.len() < 256)
        .unwrap();
    let fields = contracts::open_group(&row.stored, group_options()).unwrap();
    let original = row.stored.clone();
    row.stored.fill(0);
    drop(row);
    let serial = mixed_group_requests(&fields);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| mixed_group_requests(&fields)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    assert_eq!(fields.payload_bytes(), original);
    check_word_isolation();
}
fn mixed_group_requests(fields: &FlowLegacyGroup155) -> Vec<Vec<u8>> {
    let field = fields
        .fields()
        .find(|field| field.kind() == FlowConditionalFieldKind::MemberA0Pattern)
        .unwrap();
    (0..32)
        .map(|value| {
            let edit = fields
                .replace_field(&field.id(), 0, FlowConditionalValue::WordBits(value))
                .unwrap();
            assert!(
                fields
                    .replace_field(&field.id(), 0, FlowConditionalValue::Byte(0))
                    .is_err()
            );
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}
fn check_word_isolation() {
    for identifier in [105, 113] {
        let mut row = fixture::words()
            .into_iter()
            .find(|row| row.identifier == identifier && !row.patterns.is_empty())
            .unwrap();
        let fields = read_words(&row);
        let original = row.raw.clone();
        row.raw.fill(0);
        drop(row);
        let edit = fields
            .replace_pattern(0, !fields.pattern(0).unwrap())
            .unwrap();
        assert!(fields.replace_pattern(usize::MAX, 0).is_err());
        assert_eq!(fields.payload_bytes(), original);
        assert_eq!(edit.source().payload_bytes(), original);
    }
}
