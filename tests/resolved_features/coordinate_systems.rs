use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/coordinate-systems/cases.bin");
type CoordinateRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<CoordinateRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CSY1");
    assert_eq!(cursor.u32().unwrap(), 304);
    let slots: Vec<_> = (0..304).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn coordinate_objects_match_every_independent_primitive_reconstructed_byte_and_slot() {
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
fn coordinate_objects_reject_every_prefix_trailing_byte_and_old_context() {
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
fn coordinate_axes_follow_equality_numeric_order_and_ignore_duplicate_selectors() {
    for (kind, version, bytes, _, _) in records() {
        if kind != 0 {
            continue;
        }
        let fields = decode(version, bytes).unwrap();
        let selector = |name: &str| match fields
            .iter()
            .find(|field| field.path.as_ref() == name)
            .unwrap()
            .value
        {
            super::ModelFieldValue::U16(value) => value,
            ref value => panic!("selector {value:?}; expected u16"),
        };
        let selected = [
            selector("root.object.payload.selector_71c"),
            selector("root.object.payload.selector_720"),
        ];
        let axes: Vec<_> = fields
            .iter()
            .filter(|field| {
                matches!(
                    field.path.as_ref(),
                    "root.object.payload.axis_6f8"
                        | "root.object.payload.axis_700"
                        | "root.object.payload.axis_708"
                )
            })
            .map(|field| field.path.as_ref())
            .collect();
        let expected: Vec<_> = [
            (1, "root.object.payload.axis_6f8"),
            (2, "root.object.payload.axis_700"),
            (3, "root.object.payload.axis_708"),
        ]
        .into_iter()
        .filter(|(number, _)| selected.contains(number))
        .map(|(_, name)| name)
        .collect();
        assert_eq!(axes, expected);
    }
}

#[test]
fn coordinate_pointer_families_reject_cached_root_objects() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
    {
        let fields = decode(version, bytes).unwrap();
        for label in ["origin_6f0", "axis_6f8", "axis_700", "axis_708"] {
            let path = format!("root.object.payload.{label}");
            if !fields.iter().any(|field| field.path.as_ref() == path) {
                continue;
            }
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let error = decode(version, &changed).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("moCoordSys_c"), "{}", error.cause);
        }
    }
}
