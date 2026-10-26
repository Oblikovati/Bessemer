use super::ModelFieldValue;
use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/connection-points/cases.bin");
type ConnectionRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ConnectionRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CPF1");
    assert_eq!(cursor.u32().unwrap(), 124);
    let slots: Vec<_> = (0..124).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn connection_route_parameters_and_radial_dimensions_match_independent_fields_bytes_and_slots() {
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn connection_records_reject_every_prefix_trailing_byte_and_old_context() {
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
fn connection_pointer_families_reject_cached_feature_objects() {
    let bytes = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    for label in [
        "point_reference_6f0",
        "payload.line_reference_710",
        "payload.dimension_740",
        "payload.dimension_748",
        "payload.dimension_750",
        "payload.line_reference_718",
        "payload.dimension_788",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(19000, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains("moConnectionPoint_c"),
            "{}",
            error.cause
        );
    }
}

#[test]
fn radial_dimension_uses_a_full_signed_word_after_the_geometric_body() {
    let bytes = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 3 && *version == 19000)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    let suffix = fields.last().unwrap();
    assert_eq!(suffix.path.as_ref(), "root.object.raw_f8");
    assert_eq!(suffix.value, ModelFieldValue::I32(i32::MIN));
    assert_eq!(
        super::encode_fields(std::slice::from_ref(suffix), 30000)
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        super::encode_fields(&fields[1..], 30000).unwrap().len(),
        111
    );
}
