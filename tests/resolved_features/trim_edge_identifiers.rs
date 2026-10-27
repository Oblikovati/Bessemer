use super::lightweight_section_tests::next_case;
use super::solid_reference_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

pub(super) type TrimEdgeRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<TrimEdgeRecord> {
    records_from(
        include_bytes!("../fixtures/trim-edge-identifiers/cases.bin"),
        b"TEI1",
    )
}

pub(super) fn records_from(packet: &'static [u8], magic: &[u8]) -> Vec<TrimEdgeRecord> {
    let mut cursor = ArchiveCursor::new(packet);
    assert_eq!(cursor.take(4).unwrap(), magic);
    assert_eq!(cursor.u32().unwrap(), 60);
    let slots: Vec<_> = (0..60).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_trim_edge_identifiers_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

pub(super) fn assert_complete_records(records: Vec<TrimEdgeRecord>) {
    for (_, version, bytes, oracle, slots) in records {
        let (fields, actual_slots) = decode_graph(version, bytes, false).unwrap();
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
fn trim_edge_identifiers_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

pub(super) fn assert_rejected_boundaries(records: Vec<TrimEdgeRecord>) {
    for (kind, version, bytes, _, _) in records {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], false).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), false).is_err());
        assert!(decode_graph(0x27cb, bytes, false).is_err());
    }
}

#[test]
fn empty_trim_edge_bodies_have_ten_bytes_and_no_direct_entity_object_slot() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 % 6 == 0) {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(bytes.len() - 6 - name_length, 10);
        assert_eq!(slots, 3);
        assert_eq!(decode_graph(version, bytes, false).unwrap().1, 3);
    }
}

#[test]
fn trim_edge_source_and_external_roles_reject_unrelated_cached_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 6 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in ["source_entity.representation", "external_feature_reference"] {
            let path = format!("root.object.items[0].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("{label}: expected rejected suObArray role");
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn tagged_entity_id_owners_use_the_finite_trim_edge_representation_family() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 6 == 5) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].representation",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("entity-ID representation: expected rejected suObArray role");
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

#[test]
fn only_both_native_bound_trim_edge_names_select_the_existing_source_layout() {
    use super::entity_representations::{RepresentationLayout, layout};
    for name in ["moLeftTrimEdgeIdRep_c", "moRightTrimEdgeIdRep_c"] {
        assert_eq!(layout(name), Some(RepresentationLayout::Source(&["48"])));
        assert_eq!(layout(&format!("{name}_unknown")), None);
    }
}
