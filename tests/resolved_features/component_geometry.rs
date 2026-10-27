use super::ModelFieldValue;
use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/component-geometry/cases.bin");
type ComponentRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ComponentRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CGF1");
    assert_eq!(cursor.u32().unwrap(), 152);
    let slots: Vec<_> = (0..152).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn component_axes_points_and_wrappers_match_independent_primitives_bytes_and_slots() {
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
fn component_geometry_rejects_every_prefix_trailing_byte_and_unsupported_context() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn component_geometry_keeps_signed_feature_ids_unsigned_times_and_no_plane_suffix() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode(version, bytes).unwrap();
        let prefix = if kind < 2 {
            "root.object"
        } else {
            "root.object.component_reference"
        };
        for (name, expected) in [
            ("feature_id", ModelFieldValue::I32(i32::MIN)),
            ("feature_time", ModelFieldValue::U32(0xfedcba98)),
        ] {
            let field = fields
                .iter()
                .find(|field| field.path.as_ref() == format!("{prefix}.{name}"))
                .unwrap();
            assert_eq!(field.value, expected);
        }
        assert!(
            !fields
                .iter()
                .any(|field| field.path.as_ref() == format!("{prefix}.word_100"))
        );
    }
}

#[test]
fn component_geometry_rejects_a_cached_component_in_the_geometry_role() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|(kind, version, _, _, _)| *kind < 2 && *version == 19000)
    {
        let fields = decode(version, bytes).unwrap();
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.reference_geometry",
            &2u16.to_le_bytes(),
        );
        let error = decode(version, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moCompRef"), "{}", error.cause);
        assert!(error.cause.contains("moRefGeom_c"), "{}", error.cause);
    }
}
