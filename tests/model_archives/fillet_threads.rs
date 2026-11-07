use super::annotation_view_tests::LIMITS;
use super::direct_collection_tests::decode_kernel;
use super::kernel_frames::CompressedKernelContext;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::archive_codec::{ArchiveClass, ArchiveCursor};
use crate::parasolid::{FieldValues, TransmitBody};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/fillet-thread-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/fillet-thread-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/fillet-thread-edited.bin");
const LOADED: &[&str] = &["moCThreadInstanceId_c", "moCThreadCacheData_c"];

fn decode(
    mode: u8,
    version: u32,
    bytes: &[u8],
    context: Option<CompressedKernelContext>,
) -> crate::Result<Vec<ModelField>> {
    if ![0, 128].contains(&mode) {
        return Err(super::invalid(format!(
            "fillet/thread mode {mode}; expected 0 or 128"
        )));
    }
    decode_kernel(version, bytes, LIMITS, context, |reader, map| {
        if mode == 128 {
            for name in LOADED {
                map.append_loaded_object(Some(ArchiveClass {
                    name: (*name).into(),
                    schema: 1,
                }))?;
            }
        }
        reader.object("object", map, ModelObjectRole::Any)
    })
}

#[test]
fn complete_fillet_and_thread_records_match_independent_values_and_exact_bytes() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, Some(context.clone())).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (990, 74824));
}

#[test]
fn fillet_thread_prefixes_and_trailing_bytes_never_publish_partial_results() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(mode, version, &bytes[..length], Some(context.clone())).is_err(),
                "mode {mode}, prefix {length}"
            );
        }
        assert!(
            decode(
                mode,
                version,
                &[bytes, &[0]].concat(),
                Some(context.clone())
            )
            .is_err()
        );
    }
}

#[test]
fn fillet_edit_matches_independent_bytes_and_preserves_the_original() {
    let context = super::kernel_frame_tests::context();
    let original = decode(0, 19000, ORIGINAL, Some(context.clone())).unwrap();
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.tail.word_b90")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(0, 19000, EDITED, Some(context)).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn fillet_thread_results_survive_source_overwrite() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source, Some(context.clone())).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn fillet_parallel_writes_preserve_cached_bodies_and_duplicate_maps() {
    let fields = decode(
        0,
        19000,
        ORIGINAL,
        Some(super::kernel_frame_tests::context()),
    )
    .unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), ORIGINAL);
        }
    });
}

#[test]
fn cached_fillet_bodies_are_typed_and_need_context() {
    let fields = decode(
        0,
        19000,
        ORIGINAL,
        Some(super::kernel_frame_tests::context()),
    )
    .unwrap();
    let frames: Vec<_> = fields
        .iter()
        .filter_map(|field| match &field.value {
            ModelFieldValue::Kernel(frame) => Some(frame),
            _ => None,
        })
        .collect();
    assert_eq!(frames.len(), 2);
    for (frame, id) in frames.into_iter().zip([-7, -11]) {
        let TransmitBody::Partition(list) = &frame.contents().body else {
            panic!("expected partition");
        };
        assert_eq!(list.records.len(), 1);
        assert_eq!(
            list.records[0].fields,
            vec![FieldValues::Integers(vec![id])]
        );
    }
    assert_eq!(
        decode(0, 19000, ORIGINAL, None).unwrap_err().class,
        crate::FailureClass::Missing
    );
}

#[test]
fn scoped_thread_instance_and_cache_targets_are_required() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        if mode != 128 {
            continue;
        }
        assert!(decode(0, version, bytes, Some(context.clone())).is_err());
        checked += 1;
    }
    assert_eq!(checked, 374);
}

#[test]
fn cached_body_and_map_counts_reject_negative_and_impossible_lengths() {
    let context = super::kernel_frame_tests::context();
    let fields = decode(0, 19000, ORIGINAL, Some(context.clone())).unwrap();
    let mut checked = 0;
    for (index, field) in fields.iter().enumerate() {
        if !field.path.ends_with(".count") || !matches!(field.value, ModelFieldValue::I32(_)) {
            continue;
        }
        for value in [i32::MIN, i32::MAX] {
            let mut changed = fields.clone();
            changed[index].value = ModelFieldValue::I32(value);
            assert!(
                decode(
                    0,
                    19000,
                    &encode_fields(&changed, 30000).unwrap(),
                    Some(context.clone())
                )
                .is_err(),
                "{}",
                field.path
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 17);
}

#[test]
fn cached_fillet_arrays_and_kernel_records_share_one_budget() {
    let context = super::kernel_frame_tests::context();
    for maximum_entries in [75, 76] {
        let limits = super::ModelFieldLimits {
            maximum_entries,
            ..LIMITS
        };
        let result = decode_kernel(
            19000,
            ORIGINAL,
            limits,
            Some(context.clone()),
            |reader, map| reader.object("object", map, ModelObjectRole::Any),
        );
        assert_eq!(result.is_ok(), maximum_entries == 76, "{result:?}");
    }
}

#[test]
fn thread_point_presence_requires_exactly_one() {
    let context = super::kernel_frame_tests::context();
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, Some(context.clone())).unwrap();
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with("has_point_120"))
        {
            let prefix = field.path.strip_suffix("has_point_120").unwrap();
            let present = fields
                .iter()
                .any(|field| field.path.starts_with(&format!("{prefix}point_120[")));
            assert_eq!(present, field.value == ModelFieldValue::U8(1));
            checked += 1;
        }
    }
    assert_eq!(checked, 572);
}

#[test]
fn thread_parallel_arrays_and_caches_charge_each_entry() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes, None).ok();
        let Some(fields) = fields else { continue };
        if !fields.iter().any(|field| {
            field.path.as_ref() == "root.object.instance_count"
                && field.value == ModelFieldValue::I32(2)
        }) {
            continue;
        }
        for maximum_entries in [7, 8] {
            let limits = super::ModelFieldLimits {
                maximum_entries,
                ..LIMITS
            };
            let result = decode_kernel(version, bytes, limits, None, |reader, map| {
                for name in LOADED {
                    map.append_loaded_object(Some(ArchiveClass {
                        name: (*name).into(),
                        schema: 1,
                    }))?;
                }
                reader.object("object", map, ModelObjectRole::Any)
            });
            assert_eq!(result.is_ok(), maximum_entries == 8, "{result:?}");
        }
        checked += 1;
    }
    assert_eq!(checked, 176);
}

#[test]
fn thread_instance_and_cache_counts_reject_impossible_lengths() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let Ok(fields) = decode(mode, version, bytes, None) else {
            continue;
        };
        for (index, field) in fields.iter().enumerate().filter(|(_, field)| {
            ["root.object.instance_count", "root.object.caches.count"]
                .contains(&field.path.as_ref())
        }) {
            for value in [i32::MIN, i32::MAX] {
                let mut changed = fields.clone();
                changed[index].value = ModelFieldValue::I32(value);
                let changed = encode_fields(&changed, 30000).unwrap();
                assert!(
                    decode(mode, version, &changed, None).is_err(),
                    "{}",
                    field.path
                );
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 1056);
}
