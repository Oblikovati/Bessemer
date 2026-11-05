use super::kernel_frame_tests::{FRAME, context};
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{TrimEdgeRecord, records_with_count};
use super::{ModelField, ModelFieldValue, encode_fields};
use crate::parasolid::{FieldValues, TransmitBody};

fn bend_records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/edge-flanges/cases.bin"),
        b"EFL1",
        684,
    )
    .into_iter()
    .filter(|record| (12..18).contains(&record.0))
    .collect()
}

fn literal_tail(version: u32, counts: [i32; 2], presences: [i32; 2]) -> Vec<u8> {
    let mut bytes = Vec::new();
    append_list(&mut bytes, counts[0]);
    if version > 0x335a {
        append_list(&mut bytes, counts[1]);
    }
    for (presence, gate) in presences.into_iter().zip([0x334c, 0x334f]) {
        if version >= gate {
            append_optional(&mut bytes, presence);
        }
    }
    append_words(&mut bytes, version);
    bytes
}

fn append_optional(bytes: &mut Vec<u8>, presence: i32) {
    bytes.extend(presence.to_le_bytes());
    if presence != 0 {
        bytes.extend_from_slice(&FRAME[1..]);
    }
}

fn append_list(bytes: &mut Vec<u8>, count: i32) {
    bytes.extend(count.to_le_bytes());
    for _ in 0..count.max(0) {
        bytes.extend_from_slice(&FRAME[1..]);
    }
}

fn append_words(bytes: &mut Vec<u8>, version: u32) {
    if version > 0x3734 {
        bytes.extend((-1i32).to_le_bytes());
    }
    for word in [i32::MIN, i32::MAX, -1] {
        bytes.extend(word.to_le_bytes());
    }
}

fn replace_tail(version: u32, bytes: &[u8], replacement: &[u8]) -> Vec<u8> {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let boundary = fields
        .iter()
        .position(|field| field.path.ends_with("body_tail.raw_body_count_ae0"))
        .unwrap();
    let mut prefix = encode_fields(&fields[..boundary], 30000).unwrap();
    let tail_path = fields[boundary]
        .path
        .strip_suffix("raw_body_count_ae0")
        .unwrap();
    let tail_fields = fields[boundary..]
        .iter()
        .take_while(|field| field.path.starts_with(tail_path))
        .count();
    prefix.extend_from_slice(replacement);
    prefix.extend(encode_fields(&fields[boundary + tail_fields..], 30000).unwrap());
    prefix
}

fn modern_graph(counts: [i32; 2], presences: [i32; 2]) -> Vec<u8> {
    let (_, version, bytes, _, _) = bend_records()
        .into_iter()
        .find(|record| record.0 == 12 && record.1 == 19000)
        .unwrap();
    replace_tail(version, bytes, &literal_tail(version, counts, presences))
}

fn kernel_count(fields: &[ModelField]) -> usize {
    fields
        .iter()
        .filter(|field| matches!(field.value, ModelFieldValue::Kernel(_)))
        .count()
}

#[test]
fn positive_lists_and_signed_optional_bodies_follow_every_owned_version_gate() {
    for (_, version, original, _, slots) in bend_records() {
        let tail = literal_tail(version, [2, 3], [i32::MIN, i32::MAX]);
        let bytes = replace_tail(version, original, &tail);
        let (fields, actual_slots) = decode_graph(version, &bytes, true).unwrap();
        let expected = 2
            + 3 * usize::from(version > 0x335a)
            + usize::from(version >= 0x334c)
            + usize::from(version >= 0x334f);
        assert_eq!(kernel_count(&fields), expected);
        assert_eq!(actual_slots, slots);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn each_raw_branch_requires_its_own_explicit_kernel_context() {
    for (counts, presences) in [
        ([1, 0], [0, 0]),
        ([0, 1], [0, 0]),
        ([0, 0], [-1, 0]),
        ([0, 0], [0, i32::MIN]),
    ] {
        let bytes = modern_graph(counts, presences);
        assert_eq!(
            decode_graph(19000, &bytes, false).unwrap_err().class,
            crate::FailureClass::Missing
        );
        assert_eq!(
            kernel_count(&decode_graph(19000, &bytes, true).unwrap().0),
            1
        );
    }
}

#[test]
fn nonpositive_lists_and_absent_optional_bodies_need_no_kernel_context() {
    for count in [i32::MIN, -32769, -1, 0] {
        let bytes = modern_graph([count, count], [0, 0]);
        let fields = decode_graph(19000, &bytes, false).unwrap().0;
        assert_eq!(kernel_count(&fields), 0);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn raw_bodies_have_no_suppression_byte_and_reject_every_partial_graph() {
    let bytes = modern_graph([2, 1], [-1, 1]);
    for end in 0..bytes.len() {
        assert!(
            decode_graph(19000, &bytes[..end], true).is_err(),
            "prefix {end}"
        );
    }
    assert!(decode_graph(19000, &[bytes, vec![0]].concat(), true).is_err());
    let mut tail = literal_tail(19000, [1, 0], [0, 0]);
    tail.insert(4, 0);
    let base = modern_graph([0, 0], [0, 0]);
    let (_, _, original, _, _) = bend_records()
        .into_iter()
        .find(|record| record.0 == 12 && record.1 == 19000)
        .unwrap();
    assert!(decode_graph(19000, &replace_tail(19000, original, &tail), true).is_err());
    assert!(decode_graph(19000, &base, false).is_ok());
}

#[test]
fn impossible_counts_unknown_kernel_types_and_corrupt_frames_never_publish() {
    let (_, _, original, _, _) = bend_records()
        .into_iter()
        .find(|record| record.0 == 12 && record.1 == 19000)
        .unwrap();
    for offset in [0, 4, 8, 13, 33] {
        let mut tail = literal_tail(19000, [1, 0], [0, 0]);
        tail[offset] ^= 0xff;
        assert!(decode_graph(19000, &replace_tail(19000, original, &tail), true).is_err());
    }
    let tail = [i32::MAX.to_le_bytes().as_slice(), &[0; 40]].concat();
    assert!(decode_graph(19000, &replace_tail(19000, original, &tail), true).is_err());
}

#[test]
fn aggregate_kernel_expansion_is_shared_between_lists_and_optional_bodies() {
    let bytes = modern_graph([1, 1], [-1, 1]);
    let mut kernel = context();
    kernel.maximum_expanded =
        include_bytes!("../fixtures/model-archives/kernel-transmit-one.bin").len();
    let result = super::direct_collection_tests::decode_kernel(
        19000,
        &bytes,
        super::annotation_view_tests::LIMITS,
        Some(kernel),
        |reader, map| reader.object("object", map, super::ModelObjectRole::Any),
    );
    assert!(result.is_err());
}

#[test]
fn typed_kernel_edits_reopen_without_mutating_the_source_graph() {
    let bytes = modern_graph([1, 1], [-1, 1]);
    let (fields, slots) = decode_graph(19000, &bytes, true).unwrap();
    let mut changed = fields.clone();
    for field in &mut changed {
        edit_kernel(field);
    }
    let encoded = encode_fields(&changed, 30000).unwrap();
    assert_ne!(encoded, bytes);
    assert_eq!(
        decode_graph(19000, &encoded, true).unwrap(),
        (changed, slots)
    );
    assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
}

fn edit_kernel(field: &mut ModelField) {
    let ModelFieldValue::Kernel(frame) = &mut field.value else {
        return;
    };
    let mut contents = frame.contents().clone();
    let TransmitBody::Partition(list) = &mut contents.body else {
        panic!("partition")
    };
    list.records[0].fields[0] = FieldValues::Integers(vec![9]);
    **frame = frame.with_contents(contents).unwrap();
}

#[test]
fn published_graphs_own_input_and_write_identically_across_threads() {
    let mut bytes = modern_graph([2, 1], [i32::MIN, i32::MAX]);
    let original = bytes.clone();
    let fields = decode_graph(19000, &bytes, true).unwrap().0;
    bytes.fill(0xa5);
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), original);
        }
    });
}
