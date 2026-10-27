use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/shared-sketch-handles/cases.bin");
type SharedHandleRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<SharedHandleRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"SHH1");
    assert_eq!(cursor.u32().unwrap(), 144);
    let slots: Vec<_> = (0..144).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn shared_handle_aliases_component_owners_and_indices_match_independent_fields_bytes_and_slots() {
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
fn shared_handle_records_reject_all_prefixes_extra_bytes_and_old_contexts() {
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
fn declared_handle_role_rejects_unrelated_cached_component_and_similar_names() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|record| record.0 % 3 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.entity_handle",
            &2u16.to_le_bytes(),
        );
        let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moCompSketchEntHandle_c"));
    }
    for name in [
        "sgParabolaHandleExtra",
        "sg3DParametricSurfaceHandle_c",
        "sgUnknownHandle",
    ] {
        assert!(!super::sketch_references::HANDLES.contains(&name));
    }
}

#[test]
fn tagged_shared_handles_append_no_derived_suffix_or_extra_map_slot() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|record| record.0 % 3 == 0) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        assert_eq!(slots, 3);
        let name_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert!([10, 14].contains(&(bytes.len() - 6 - name_length)));
        assert!([4, 5].contains(&fields.len()));
        assert_eq!(
            fields
                .iter()
                .filter(|field| matches!(field.value, super::ModelFieldValue::ObjectTag(_)))
                .count(),
            1
        );
    }
}
