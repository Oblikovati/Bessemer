use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/component-coordinate-systems/geometry-roles.bin");
type GeometryRoleRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<GeometryRoleRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CGR1");
    assert_eq!(cursor.u32().unwrap(), 24);
    let slots: Vec<_> = (0..24).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn fresh_and_cached_coordinate_geometry_matches_independent_fields_bytes_and_slots() {
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
fn coordinate_geometry_rejects_prefixes_trailing_bytes_and_old_context() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
        assert!(decode(version, bytes).is_ok());
    }
}

#[test]
fn unrelated_component_geometry_remains_rejected() {
    let stream = include_bytes!("../fixtures/component-geometry/owned-0-0-0-0.bin");
    let fields = decode(19000, &stream[6..stream.len() - 4]).unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.reference_geometry",
        &2u16.to_le_bytes(),
    );
    let error = decode(19000, &changed).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moCompRefAxis_c"), "{}", error.cause);
}
