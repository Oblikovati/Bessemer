use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/component-reference-curves/cases.bin");
type ComponentCurveRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ComponentCurveRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CRC1");
    assert_eq!(cursor.u32().unwrap(), 198);
    let slots: Vec<_> = (0..198).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_component_reference_curve_graphs_match_independent_fields_bytes_and_slots() {
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
fn component_reference_curves_reject_every_prefix_extra_byte_and_old_context() {
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
fn component_curve_wrapper_role_rejects_the_unrelated_cached_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 3 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].component_reference",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, None, LIMITS) else {
            panic!("cached suObArray: expected a rejected component-reference role");
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("suObArray"));
    }
}

#[test]
fn component_curve_has_only_the_shared_component_feature_body_and_retained_id_time_pair() {
    for (kind, version, bytes, _, slots) in records()
        .into_iter()
        .filter(|row| row.0 == 0 && row.1 >= 0x3ef6)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let name_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(slots, 3, "{kind}");
        assert_eq!(bytes.len() - 6 - name_length, 90);
        assert!(fields[fields.len() - 2].path.ends_with("feature_id"));
        assert!(fields.last().unwrap().path.ends_with("feature_time"));
    }
}
