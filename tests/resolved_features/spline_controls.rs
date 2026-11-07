use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/spline-controls/cases.bin");
type ControlRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ControlRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"SPC1");
    assert_eq!(cursor.u32().unwrap(), 72);
    let slots: Vec<_> = (0..72).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn all_three_complete_control_classes_and_real_sketch_owners_match_fields_bytes_and_slots() {
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
fn spline_controls_reject_every_incomplete_prefix_extra_byte_and_old_context() {
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
fn declared_sketch_and_user_object_roles_still_reject_unrelated_cached_classes() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|record| record.0 % 4 == 0 || record.0 % 4 == 3)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let path = if kind % 4 == 0 {
            "root.object.sketch_08"
        } else {
            "root.object.visual.user_objects[0]"
        };
        let changed =
            super::manager_context_tests::replace_field(&fields, path, &2u16.to_le_bytes());
        let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("object role"), "{}", error.cause);
    }
}

#[test]
fn direct_control_handles_have_no_tag_or_map_slot_and_preserve_independent_indices() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|record| record.0 % 4 == 0)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        assert_eq!(slots, 3);
        assert_eq!(
            fields
                .iter()
                .filter(|field| matches!(field.value, super::ModelFieldValue::ObjectTag(_)))
                .count(),
            2
        );
        assert_eq!(
            fields
                .iter()
                .filter(|field| field.path.ends_with(".entity_index"))
                .count(),
            if kind / 4 == 1 { 1 } else { 2 }
        );
    }
}
