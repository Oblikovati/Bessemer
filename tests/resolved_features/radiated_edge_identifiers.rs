use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/radiated-edge-identifiers/cases.bin");
type RadiatedRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<RadiatedRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"REI1");
    assert_eq!(cursor.u32().unwrap(), 18);
    let slots: Vec<_> = (0..18).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_radiated_identifiers_and_declared_role_owners_match_independent_fields_bytes_and_slots()
{
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
fn radiated_identifiers_reject_every_prefix_trailing_byte_and_old_context() {
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
fn representation_role_rejects_unrelated_cached_array_without_admitting_name_prefixes() {
    let record = records()
        .into_iter()
        .rev()
        .find(|record| record.0 == 2)
        .unwrap();
    let fields = decode_graph(record.1, record.2, None, LIMITS).unwrap().0;
    let path = fields
        .iter()
        .find(|field| field.path.ends_with("source_entity.representation"))
        .unwrap()
        .path
        .clone();
    let changed = super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
    let error = decode_graph(record.1, &changed, None, LIMITS).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("suObArray"));
    for name in [
        "moRadiateEdgeIdRep",
        "moRadiateEdgeIdRep_c_extra",
        "moRadiateOtherIdRep_c",
    ] {
        assert!(super::entity_representations::layout(name).is_none());
    }
}
