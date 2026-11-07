use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use super::{ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/fold-transforms/cases.bin");
type FoldRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<FoldRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"FLD1");
    assert_eq!(cursor.u32().unwrap(), 105);
    let slots: Vec<_> = (0..105).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_fold_folders_transforms_and_properties_match_independent_fields_bytes_and_slots() {
    for (_, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) = decode_graph(version, bytes, None, LIMITS).unwrap();
        assert_eq!(actual_slots, slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn fold_records_reject_all_prefixes_extra_bytes_and_old_context() {
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
fn fold_transform_and_property_counts_reject_negative_and_unbounded_values() {
    for (_, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for field in fields.iter().filter(|field| {
            field.path.ends_with(".count")
                && (field.path.contains("transforms_") || field.path.contains("properties_158"))
        }) {
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
fn fold_property_roles_reject_cached_folder_and_keep_transforms_and_bytes_distinct() {
    let record = records()
        .into_iter()
        .find(|record| record.0 == 0 && record.1 == 19000 && record.4 == 13)
        .unwrap();
    let fields = decode_graph(record.1, record.2, None, LIMITS).unwrap().0;
    for field in fields
        .iter()
        .filter(|field| field.path.ends_with(".feature") || field.path.ends_with(".property"))
    {
        let changed =
            super::manager_context_tests::replace_field(&fields, &field.path, &2u16.to_le_bytes());
        assert!(
            decode_graph(record.1, &changed, None, LIMITS).is_err(),
            "{}",
            field.path
        );
    }
    assert_eq!(
        fields
            .iter()
            .filter(|field| matches!(field.value, ModelFieldValue::Transform(_)))
            .count(),
        4
    );
    assert_eq!(
        fields
            .iter()
            .filter(|field| field.path.contains(".bytes_108["))
            .count(),
        4
    );
}

#[test]
fn direct_fold_byte_array_requires_exactly_four_entries_without_container_slots() {
    let mut bytes = [0u8; 20];
    bytes[12..14].copy_from_slice(&4u16.to_le_bytes());
    bytes[14..18].copy_from_slice(&[0, 255, 128, 127]);
    for maximum in [4, 3] {
        let result = super::direct_collection_tests::decode_limited(
            19000,
            &bytes,
            super::ModelFieldLimits {
                maximum_entries: maximum,
                ..LIMITS
            },
            |reader, map| {
                super::fold_transforms::read_object(reader, map, "moFoldUpDownSketchXformData_c")
                    .unwrap()
            },
        );
        assert_eq!(result.is_ok(), maximum == 4);
    }
}
