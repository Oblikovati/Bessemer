use super::reader::ModelArchiveContext;
use super::{
    AssemblyRootKind, ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, encode_fields,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 5000,
    maximum_fields: 2000,
    maximum_entries: 2000,
    maximum_depth: 32,
    maximum_objects: 32,
};

struct ObjectCase {
    bytes: &'static [u8],
    oracle: &'static str,
    slots: usize,
}

fn cases() -> Vec<ObjectCase> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/assembly-configurations/assembly-objects.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"ASOB");
    let count = cursor.u32().unwrap();
    assert_eq!(count, 20);
    let result = (0..count).map(|_| next_case(&mut cursor)).collect();
    cursor.finish().unwrap();
    result
}

fn next_case(cursor: &mut ArchiveCursor<'static>) -> ObjectCase {
    let length = cursor.u32().unwrap() as usize;
    let oracle_length = cursor.u32().unwrap() as usize;
    let slots = cursor.u32().unwrap() as usize;
    ObjectCase {
        bytes: cursor.take(length).unwrap(),
        oracle: std::str::from_utf8(cursor.take(oracle_length).unwrap()).unwrap(),
        slots,
    }
}

fn decode(bytes: &[u8], limits: ModelFieldLimits) -> Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        limits,
        ModelArchiveContext::OrdinaryAssemblyExternalXml(AssemblyRootKind::Assembly),
    )?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn assembly_objects_match_real_reference_primitives_all_transform_modes_and_every_faceted_variant()
{
    for case in cases() {
        let (fields, slots) = decode(case.bytes, LIMITS).unwrap();
        assert_eq!(slots, case.slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            case.bytes,
            case.oracle,
            case.oracle.lines().count(),
        );
        let encoded = encode_fields(&fields, LIMITS.maximum_bytes).unwrap();
        assert_eq!(encoded, case.bytes);
        assert_eq!(decode(&encoded, LIMITS).unwrap().0, fields);
    }
}

#[test]
fn every_assembly_object_prefix_and_appended_byte_rejects_without_partial_success() {
    for case in cases() {
        for end in 0..case.bytes.len() {
            assert!(decode(&case.bytes[..end], LIMITS).is_err(), "prefix {end}");
        }
        let mut extra = case.bytes.to_vec();
        extra.push(0);
        assert!(decode(&extra, LIMITS).is_err());
    }
}

#[test]
fn camera_point_counts_and_every_operation_budget_reject_before_publishing() {
    for case in cases() {
        let (fields, _) = decode(case.bytes, LIMITS).unwrap();
        let Some(index) = fields
            .iter()
            .position(|field| field.path.ends_with(".faceted_reference.points.count"))
        else {
            continue;
        };
        for count in [i32::MIN, i32::MAX] {
            let mut changed = fields.clone();
            changed[index].value = super::ModelFieldValue::I32(count);
            assert!(
                decode(
                    &encode_fields(&changed, LIMITS.maximum_bytes).unwrap(),
                    LIMITS
                )
                .is_err()
            );
        }
        for limits in [
            ModelFieldLimits {
                maximum_bytes: case.bytes.len() - 1,
                ..LIMITS
            },
            ModelFieldLimits {
                maximum_fields: 1,
                ..LIMITS
            },
            ModelFieldLimits {
                maximum_entries: 1,
                ..LIMITS
            },
            ModelFieldLimits {
                maximum_depth: 1,
                ..LIMITS
            },
            ModelFieldLimits {
                maximum_objects: 1,
                ..LIMITS
            },
        ] {
            assert!(decode(case.bytes, limits).is_err());
        }
    }
}

#[test]
fn unselected_and_old_object_contexts_remain_failed_after_the_first_rejection() {
    for case in cases() {
        for (context, version) in [
            (ModelArchiveContext::Unspecified, 19000),
            (
                ModelArchiveContext::OrdinaryAssemblyExternalXml(AssemblyRootKind::Assembly),
                0x27cb,
            ),
        ] {
            let mut reader = ModelDecoder::new_in_context(
                case.bytes,
                ArchiveVersion::new(version),
                LIMITS,
                context,
            )
            .unwrap();
            let mut map = ObjectReader::new();
            let failure = reader
                .object("object", &mut map, ModelObjectRole::Any)
                .unwrap_err();
            assert_eq!(failure.class, crate::FailureClass::Unsupported);
            assert_eq!(reader.finish().unwrap_err(), failure);
        }
    }
}
