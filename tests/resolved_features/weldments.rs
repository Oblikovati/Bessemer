use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/weldments/cases.bin");
type WeldmentRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<WeldmentRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"WLD1");
    assert_eq!(cursor.u32().unwrap(), 42);
    let slots: Vec<_> = (0..42).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn weldments_direct_properties_and_tagged_aliases_match_independent_fields_bytes_and_slots() {
    let mut observations = 0;
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
        observations += oracle.lines().count();
    }
    assert_eq!(observations, 2367);
}

#[test]
fn weldments_and_tagged_properties_reject_every_prefix_extra_byte_and_old_context() {
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
fn direct_and_tagged_custom_property_counts_reject_negative_and_unbounded_values() {
    for (_, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with("properties.count"))
        {
            for count in [-1i32, i32::MAX] {
                let changed = super::manager_context_tests::replace_field(
                    &fields,
                    &field.path,
                    &count.to_le_bytes(),
                );
                assert!(
                    decode_graph(version, &changed, None, LIMITS).is_err(),
                    "{}",
                    field.path
                );
            }
        }
    }
}

#[test]
fn three_minimal_direct_properties_charge_three_entries_and_no_object_slots() {
    let mut bytes = vec![0u8; 76];
    bytes[..4].copy_from_slice(&3i32.to_le_bytes());
    for maximum in [3, 2] {
        let result = super::direct_collection_tests::decode_limited(
            19000,
            &bytes,
            super::ModelFieldLimits {
                maximum_entries: maximum,
                ..LIMITS
            },
            |reader, map| {
                super::custom_properties::list(reader, "properties")?;
                assert_eq!(
                    std::mem::replace(map, crate::archive_codec::ObjectReader::new())
                        .into_snapshot()
                        .slot_count(),
                    1
                );
                Ok(())
            },
        );
        assert_eq!(result.is_ok(), maximum == 3);
    }
}
