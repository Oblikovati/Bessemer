use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/component-coordinate-systems/cases.bin");
type ComponentCoordinateRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ComponentCoordinateRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CCR1");
    assert_eq!(cursor.u32().unwrap(), 160);
    let slots: Vec<_> = (0..160).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn component_coordinate_objects_match_every_independent_primitive_reconstructed_byte_and_slot() {
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
fn component_coordinate_objects_reject_every_prefix_trailing_byte_and_old_context() {
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
fn fixed_component_transform_preserves_high_bits_and_rejects_only_documented_special_kinds() {
    let bytes = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    assert_eq!(
        fields
            .iter()
            .filter(|field| field.path.starts_with("root.object.transform_100["))
            .count(),
        30
    );
    for flags in 0..=255u32 {
        let word = 0xa5a50000 | flags;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.transform_100[28]",
            &word.to_le_bytes(),
        );
        let result = decode(19000, &changed);
        if flags & 0xf0 == 0 {
            let parsed = result.unwrap();
            assert_eq!(super::encode_fields(&parsed, 30000).unwrap(), changed);
            continue;
        }
        let error = result.unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(
            error.cause.contains(&format!("{word:#010x}")),
            "{}",
            error.cause
        );
        assert!(error.cause.contains("word & 0xF0 == 0"), "{}", error.cause);
    }
}

#[test]
fn component_coordinate_geometry_rejects_a_cached_component_as_its_own_geometry() {
    let bytes = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.reference_geometry",
        &2u16.to_le_bytes(),
    );
    let error = decode(19000, &changed).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moCompCoordSys_c"), "{}", error.cause);
}
