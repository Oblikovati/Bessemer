use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/sketch-geometry-backups/cases.bin");
type BackupRecord = (u8, u32, &'static [u8], &'static str, usize);
const PARABOLA_POINT_LABELS: [&str; 6] = [
    "segment_base.points[0]",
    "segment_base.points[1]",
    "point_20",
    "point_28",
    "point_30",
    "point_38",
];

fn records() -> Vec<BackupRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"GBC1");
    assert_eq!(cursor.u32().unwrap(), 174);
    let slots: Vec<_> = (0..174).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn all_four_geometry_backup_and_handle_classes_match_independent_fields_bytes_and_slots() {
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
fn complete_backup_graphs_reject_every_prefix_extra_byte_and_old_context() {
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
fn all_six_parabola_point_roles_reject_cached_nonpoint_backups() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|record| record.0 == 0) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for label in PARABOLA_POINT_LABELS {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &format!("root.object.{label}"),
                &2u16.to_le_bytes(),
            );
            let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("moParabolaBackedUpData_c"));
        }
    }
}

#[test]
fn declared_backup_and_handle_owners_reject_their_cached_unrelated_parent() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|record| record.0 % 3 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let label = if kind / 3 < 2 {
            "backup"
        } else {
            "entity_handle"
        };
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("object role"));
    }
}

#[test]
fn native_bound_backup_minima_have_no_extra_base_tag_owner_word_or_map_slot() {
    for (kind, version, bytes, _, slots) in records()
        .into_iter()
        .filter(|record| record.0 == 0 || record.0 == 3)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        assert_eq!(slots, 3);
        let class_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(
            bytes.len() - 6 - class_length,
            if kind == 0 { 36 } else { 56 }
        );
        let tags = fields
            .iter()
            .filter(|field| matches!(field.value, super::ModelFieldValue::ObjectTag(_)))
            .count();
        assert_eq!(tags, if kind == 0 { 7 } else { 1 });
    }
}
