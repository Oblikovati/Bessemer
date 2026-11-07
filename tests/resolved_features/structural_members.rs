use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const PACKET: &[u8] = include_bytes!("../fixtures/structural-members/cases.bin");
type MemberRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<MemberRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"STM1");
    assert_eq!(cursor.u32().unwrap(), 90);
    let slots: Vec<_> = (0..90).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn all_member_profile_contour_index_and_orientation_classes_match_independent_fields_bytes_and_slots()
 {
    let mut observations = 0;
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
        observations += oracle.lines().count();
    }
    assert_eq!(observations, 25171);
}

#[test]
fn structural_records_reject_every_prefix_extra_byte_and_old_context() {
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
fn signed_member_and_contour_counts_reject_negative_and_impossible_extents() {
    for (_, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for field in fields.iter().filter(|field| {
            field.path.ends_with(".count")
                && (field.path.contains("curves_")
                    || field.path.contains("points_830")
                    || field.path.contains("contours.count"))
        }) {
            for count in [-1i32, i32::MAX] {
                let changed = super::manager_context_tests::replace_field(
                    &fields,
                    &field.path,
                    &count.to_le_bytes(),
                );
                assert!(
                    decode_graph(version, &changed, None, LIMITS).is_err(),
                    "{}",
                    field.path
                );
            }
        }
    }
}

#[test]
fn declared_member_references_reject_the_cached_member_root() {
    let record = records()
        .into_iter()
        .find(|record| record.0 == 0 && record.1 == 19000 && record.4 == 37)
        .unwrap();
    let fields = decode_graph(record.1, record.2, None, LIMITS).unwrap().0;
    for label in [
        "profile_820",
        "plane_828",
        "curves_890[0]",
        "points_830[0].point",
        "point_950",
        "properties_990",
        "contours_9c0",
        "advanced_properties_9a8",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode_graph(record.1, &changed, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains("moWeldMemberFeat_c"),
            "{}",
            error.cause
        );
    }
}

#[test]
fn direct_leaf_minima_do_not_prepend_runtime_inheritance_arrays_or_allocate_slots() {
    for (kind, size) in [(2, 20), (6, 52), (7, 36), (8, 68)] {
        let bytes = vec![0u8; size];
        let fields =
            super::direct_collection_tests::decode_collection(19000, &bytes, |reader, map| {
                let result = match kind {
                    2 => super::structural_contours::contours(reader, map),
                    6 => super::structural_contours::orientation(reader, map),
                    7 => super::structural_contours::single(reader, map),
                    8 => super::structural_contours::segment(reader, map),
                    _ => unreachable!(),
                };
                assert_eq!(
                    std::mem::replace(map, ObjectReader::new())
                        .into_snapshot()
                        .slot_count(),
                    1
                );
                result
            })
            .unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}
