use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/plane-point-roles/cases.bin");
type PlaneRoleRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<PlaneRoleRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"PPR1");
    assert_eq!(cursor.u32().unwrap(), 144);
    let slots: Vec<_> = (0..144).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn assert_record(record: PlaneRoleRecord) {
    let (_, version, bytes, oracle, slots) = record;
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

#[test]
fn base_point_targets_keep_existing_fresh_and_cached_plane_behavior() {
    for record in records().into_iter().filter(|record| record.0 & 12 == 0) {
        assert_record(record);
    }
}

#[test]
fn fresh_endpoint_and_loft_point_targets_match_independent_complete_plane_fields() {
    for record in records()
        .into_iter()
        .filter(|record| record.0 & 16 == 0 && [4, 8].contains(&(record.0 & 12)))
    {
        assert_record(record);
    }
}

#[test]
fn cached_endpoint_and_loft_point_targets_match_independent_complete_plane_fields() {
    for record in records()
        .into_iter()
        .filter(|record| record.0 & 16 != 0 && [4, 8].contains(&(record.0 & 12)))
    {
        assert_record(record);
    }
}

#[test]
fn unrelated_fresh_and_cached_wrappers_remain_invalid_plane_point_targets() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|record| record.0 & 12 == 12) {
        let error = decode_graph(version, bytes, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains("point_reference_88"),
            "{}",
            error.cause
        );
        assert!(error.cause.contains("moRefWrapper_c"), "{}", error.cause);
    }
}

#[test]
fn all_point_family_plane_records_reject_prefixes_extra_bytes_and_old_contexts() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|record| record.0 & 12 != 12) {
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
