use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/face-blend-items/cases.bin");
type FaceBlendRecord = (u8, u32, &'static [u8], &'static str, usize);

const ITEM_LABELS: &[&str] = &["face_08", "edge_10"];
const PAIR_LABELS: &[&str] = &[
    "first_08",
    "second_10",
    "first_08.face_08",
    "first_08.edge_10",
];

fn records() -> Vec<FaceBlendRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"FBI1");
    assert_eq!(cursor.u32().unwrap(), 120);
    let slots: Vec<_> = (0..120).map(|_| cursor.u32().unwrap() as usize).collect();
    let result = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    result
}

#[test]
fn complete_face_blend_graphs_match_independent_fields_bytes_and_slots() {
    for (_, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) = decode_graph(version, bytes, None, LIMITS).unwrap();
        assert_eq!(actual_slots, slots);
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
fn face_blends_reject_every_prefix_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], None, LIMITS).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), None, LIMITS).is_err());
        assert!(decode_graph(0x27cb, bytes, None, LIMITS).is_err());
    }
}

#[test]
fn face_blend_item_roles_reject_cached_array_for_each_declared_pointer() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 3 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let labels: &[&str] = if kind < 3 { ITEM_LABELS } else { PAIR_LABELS };
        for label in labels {
            let path = format!("root.object.items[0].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("suObArray"));
        }
    }
}

#[test]
fn face_blend_minima_have_no_invented_base_payload_or_point_allocation_tag() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 % 3 == 0) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let class_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        let body_bytes = bytes.len() - 6 - class_length;
        let point = fields
            .iter()
            .any(|field| field.path.ends_with("point_18[0]"));
        assert_eq!(slots, 3);
        assert_eq!(body_bytes, expected_body_bytes(kind, point));
    }
}

fn expected_body_bytes(kind: u8, point: bool) -> usize {
    match (kind >= 3, point) {
        (true, _) => 4,
        (false, true) => 29,
        (false, false) => 5,
    }
}
