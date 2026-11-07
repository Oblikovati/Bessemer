use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use super::{ModelDecoder, ModelField, ModelFieldValue};
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const PACKET: &[u8] = include_bytes!("../fixtures/drafts/cases.bin");
type DraftRecord = (u8, u32, &'static [u8], &'static str, usize);
const DECLARED_REFERENCES: [&str; 6] = [
    "dimension_800",
    "direction_868",
    "word_array_828",
    "surface_body_850",
    "plane_858",
    "dimension_808",
];

fn records() -> Vec<DraftRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"DRF1");
    assert_eq!(cursor.u32().unwrap(), 90);
    let slots: Vec<_> = (0..90).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn assert_bad_field(version: u32, fields: &[ModelField], path: &str, replacement: &[u8]) {
    let changed = super::manager_context_tests::replace_field(fields, path, replacement);
    assert!(
        decode_graph(version, &changed, None, LIMITS).is_err(),
        "{path}"
    );
}

fn read_minimum(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    draft: bool,
) -> crate::Result<()> {
    let result = if draft {
        super::draft_records::read(reader, map)
    } else {
        super::draft_keepers::read(reader, map)
    };
    let snapshot = std::mem::replace(map, ObjectReader::new()).into_snapshot();
    assert_eq!(snapshot.slot_count(), 1);
    result
}

fn assert_minimum(version: u32, size: usize, draft: bool) {
    let bytes = vec![0; size];
    let fields =
        super::direct_collection_tests::decode_collection(version, &bytes, |reader, map| {
            read_minimum(reader, map, draft)
        })
        .unwrap();
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
}

fn assert_empty_large_stride(version: u32, fields: &[ModelField]) {
    let path = "root.object.payload.dynamic_doubles_878.element_size";
    let changed =
        super::manager_context_tests::replace_field(fields, path, &0xfffffff8u32.to_le_bytes());
    let parsed = decode_graph(version, &changed, None, LIMITS).unwrap().0;
    assert_eq!(super::encode_fields(&parsed, 30000).unwrap(), changed);
}

#[test]
fn complete_drafts_and_both_keeper_classes_match_independent_fields_bytes_and_map_slots() {
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
    assert_eq!(observations, 18383);
}

#[test]
fn draft_records_reject_every_prefix_extra_byte_and_old_context() {
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
fn signed_draft_group_face_point_and_entity_id_counts_reject_negative_or_impossible_extents() {
    for (_, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let counts = fields.iter().filter(|field| {
            field.path.ends_with(".count") && matches!(field.value, ModelFieldValue::I32(_))
        });
        for field in counts {
            for count in [-1i32, i32::MAX] {
                assert_bad_field(version, &fields, &field.path, &count.to_le_bytes());
            }
        }
    }
}

#[test]
fn draft_dynamic_arrays_reject_nonword_strides() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|record| record.0 < 6) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let sizes = fields
            .iter()
            .filter(|field| field.path.ends_with("dynamic_doubles_878.element_size"));
        for field in sizes {
            for size in [1u32, 6, 7, 9] {
                assert_bad_field(version, &fields, &field.path, &size.to_le_bytes());
            }
        }
    }
}

#[test]
fn draft_dynamic_arrays_reject_impossible_counts_but_keep_empty_large_strides() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|record| record.0 < 6) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with("dynamic_doubles_878.count"))
        {
            assert_bad_field(version, &fields, &field.path, &u32::MAX.to_le_bytes());
        }
        if kind == 3 {
            assert_empty_large_stride(version, &fields);
        }
    }
}

#[test]
fn declared_draft_references_reject_the_cached_draft_root() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|record| record.0 == 1 && record.1 >= 0x3747)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for label in DECLARED_REFERENCES {
            let path = format!("root.object.payload.{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("moDraft_c"));
        }
    }
}

#[test]
fn direct_draft_and_keeper_minima_match_independent_native_sizes_without_new_slots() {
    for (version, draft_size, keeper_size) in [
        (0x27cc, 311, 6),
        (0x3746, 311, 6),
        (0x3747, 325, 10),
        (19000, 325, 10),
    ] {
        assert_minimum(version, draft_size, true);
        assert_minimum(version, keeper_size, false);
    }
}
