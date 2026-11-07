use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelObjectRole};
use crate::archive_codec::{ArchiveClass, ArchiveCursor, ObjectReader};

const PACKET: &[u8] = include_bytes!("../fixtures/circular-patterns/cases.bin");
type CircularRecord = (u32, &'static [u8], &'static str, usize);

fn records() -> Vec<CircularRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CPF1");
    assert_eq!(cursor.u32().unwrap(), 22);
    let slots: Vec<_> = (0..22).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (_, version, bytes, oracle) = next_case(&mut cursor);
            (version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

fn decode_graph(version: u32, bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut slots = 0;
    let fields = decode_collection(version, bytes, |reader, map| {
        reader.object("object", map, ModelObjectRole::Any)?;
        slots = std::mem::replace(map, ObjectReader::new())
            .into_snapshot()
            .slot_count();
        Ok(())
    })?;
    Ok((fields, slots))
}

fn payload_bytes(primary: u16, alternate: u16, counts: &[i32]) -> Vec<u8> {
    let mut bytes: Vec<_> = [1, primary, 1, 2, 4, 3]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect();
    bytes.push(255);
    bytes.extend(alternate.to_le_bytes());
    bytes.push(254);
    bytes.extend(2u16.to_le_bytes());
    bytes.extend([253, 252, 251, 250, 249]);
    bytes.extend(counts.iter().flat_map(|word| word.to_le_bytes()));
    bytes.extend([0; 48]);
    bytes
}

fn decode_payload(bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        for name in [
            "moDisplayDistanceDim_c",
            "moDisplayAngularDim_c",
            "moLineRef_w",
            "suObList",
        ] {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
        }
        super::circular_patterns::payload(reader, map)
    })
}

#[test]
fn complete_circular_patterns_match_independent_fields_bytes_and_object_slots() {
    for (version, bytes, oracle, expected_slots) in records() {
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
fn circular_patterns_reject_all_truncated_prefixes_trailing_bytes_and_old_contexts() {
    for (version, bytes, _, _) in records() {
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
fn circular_dimensions_require_one_alternative_before_reading_collections() {
    for (primary, alternate) in [(2, 0), (0, 2), (2, 2)] {
        let bytes = payload_bytes(primary, alternate, &[0, 0]);
        let fields = decode_payload(&bytes).unwrap();
        assert_eq!(super::encode_fields(&fields, 1000).unwrap(), bytes);
    }
    let error = decode_payload(&payload_bytes(0, 0, &[-1])).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("1138=null and 1198=null"));
}

#[test]
fn circular_integer_counts_reject_negative_and_unbounded_maps_and_vectors() {
    for counts in [
        &[-1][..],
        &[i32::MAX],
        &[1, -1, -1],
        &[0, -1],
        &[0, i32::MAX],
    ] {
        let error = decode_payload(&payload_bytes(2, 2, counts)).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("count"), "{}", error.cause);
    }
}

#[test]
fn circular_pointer_roles_reject_wrong_dimension_and_line_families() {
    for (primary, alternate) in [(4, 2), (2, 1)] {
        assert!(decode_payload(&payload_bytes(primary, alternate, &[0, 0])).is_err());
    }
    for offset in [10, 16] {
        let mut bytes = payload_bytes(2, 2, &[0, 0]);
        bytes[offset..offset + 2].copy_from_slice(&1u16.to_le_bytes());
        let error = decode_payload(&bytes).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moDisplayDistanceDim_c"));
    }
}
