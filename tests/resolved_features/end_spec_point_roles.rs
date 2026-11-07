use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/end-spec-point-roles/cases.bin");
type EndPointRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<EndPointRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"ESP1");
    assert_eq!(cursor.u32().unwrap(), 96);
    let slots: Vec<_> = (0..96).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn assert_complete(cached: bool) {
    for (_, version, bytes, oracle, slots) in records()
        .into_iter()
        .filter(|record| record.0 % 8 / 2 < 3 && (record.0 % 2 != 0) == cached)
    {
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
fn fresh_end_and_revolution_point_roles_accept_the_complete_verified_point_family() {
    assert_complete(false);
}

#[test]
fn cached_end_and_revolution_point_roles_accept_the_complete_verified_point_family() {
    assert_complete(true);
}

#[test]
fn end_point_roles_reject_unrelated_wrappers_for_fresh_and_cached_tags() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|record| record.0 % 8 / 2 == 3) {
        let error = decode_graph(version, bytes, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moRefWrapper_c"));
    }
}

#[test]
fn valid_end_point_graphs_reject_every_truncation_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|record| record.0 % 8 / 2 < 3)
    {
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
