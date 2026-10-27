use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/draft-surface-identifiers/cases.bin");
type IdentifierRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<IdentifierRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"DSI1");
    assert_eq!(cursor.u32().unwrap(), 54);
    let slots: Vec<_> = (0..54).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_draft_identifier_graphs_match_independent_fields_bytes_and_slots() {
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
fn draft_identifiers_reject_every_prefix_extra_byte_and_old_context() {
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
fn identifier_and_external_reference_roles_reject_the_unrelated_cached_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|record| record.0 % 3 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for label in [
            "source_entity.representation",
            "source_entity.representation.external_feature_reference",
        ] {
            let path = format!("root.object.items[1].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("suObArray"));
        }
    }
}

#[test]
fn identifier_minima_have_one_signed_word_and_no_invented_draft_body_or_map_slot() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|record| record.0 % 3 == 0) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        assert_eq!(slots, 3);
        let class_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(bytes.len() - 6 - class_length, 8);
        assert_eq!(fields.len(), 4);
    }
    for name in [
        "moDraftSurfIdRep",
        "moDraftSurfIdRep_cExtra",
        "moBackDraftOtherIdRep_c",
    ] {
        assert!(super::entity_representations::layout(name).is_none());
    }
}
