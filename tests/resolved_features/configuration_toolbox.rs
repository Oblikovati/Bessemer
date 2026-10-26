use super::ModelFieldValue;
use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/configuration-toolbox/cases.bin");
type ToolboxRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ToolboxRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"TBF1");
    assert_eq!(cursor.u32().unwrap(), 72);
    let slots: Vec<_> = (0..72).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

#[test]
fn toolbox_records_match_independent_primitives_bytes_and_object_slots() {
    let mut reads = 0;
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        reads += oracle.lines().count();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
    assert_eq!(reads, 1836);
}

#[test]
fn toolbox_records_reject_every_prefix_trailing_byte_and_unscoped_version() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn toolbox_items_retain_negative_zero_nan_payload_and_negative_infinity_bits() {
    let mut bits = Vec::new();
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|(kind, _, _, _, _)| *kind == 1)
    {
        let fields = decode(version, bytes).unwrap();
        let word = fields
            .iter()
            .find(|field| field.path.as_ref() == "root.object.raw_20")
            .unwrap();
        let ModelFieldValue::F64(value) = word.value else {
            panic!("expected stored double")
        };
        bits.push(value.bits());
    }
    bits.sort_unstable();
    bits.dedup();
    assert_eq!(
        bits,
        [0x7ff8000000001234, 0x8000000000000000, 0xfff0000000000000]
    );
}

#[test]
fn toolbox_array_count_preflight_rejects_a_count_exceeding_the_remaining_stream() {
    let bytes = records()
        .into_iter()
        .filter(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .nth(6)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.items.count.count_dword",
        &u32::MAX.to_le_bytes(),
    );
    let error = decode(19000, &changed).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("count"), "{}", error.cause);
}
