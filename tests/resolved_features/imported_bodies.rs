use super::annotation_view_tests::LIMITS;
use super::direct_collection_tests::decode_kernel;
use super::kernel_frame_tests::context;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};

const PACKET: &[u8] = include_bytes!("../fixtures/imported-bodies/cases.bin");
type ImportedRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ImportedRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"IBF1");
    assert_eq!(cursor.u32().unwrap(), 48);
    let slots: Vec<_> = (0..48).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn decode_graph(
    version: u32,
    bytes: &[u8],
    kernel: bool,
) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut slots = 0;
    let fields = decode_kernel(
        version,
        bytes,
        LIMITS,
        kernel.then(context),
        |reader, map| {
            reader.object("object", map, ModelObjectRole::Any)?;
            slots = std::mem::replace(map, ObjectReader::new())
                .into_snapshot()
                .slot_count();
            Ok(())
        },
    )?;
    Ok((fields, slots))
}

#[test]
fn complete_imported_and_base_bodies_match_independent_primitives_bytes_and_slots() {
    let mut primitives = 0;
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes, true).unwrap();
        assert_eq!(slots, expected_slots);
        let count = oracle.lines().count();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, count);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        primitives += count;
    }
    assert_eq!(primitives, 7622);
}

#[test]
fn imported_bodies_reject_every_truncated_prefix_trailing_byte_and_old_context() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], true).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), true).is_err());
        assert!(decode_graph(0x27cb, bytes, true).is_err());
    }
}

#[test]
fn imported_inline_partitions_require_explicit_kernel_context_and_remain_typed() {
    for (_, version, bytes, _, _) in records() {
        let (fields, _) = decode_graph(version, bytes, true).unwrap();
        let frame = fields.iter().find_map(|field| match &field.value {
            ModelFieldValue::Kernel(frame) => Some(frame),
            _ => None,
        });
        if let Some(frame) = frame {
            assert_eq!(frame.record_count(), 1);
            assert_eq!(
                frame.to_bytes().unwrap(),
                super::kernel_frame_tests::FRAME[1..]
            );
            assert_eq!(
                decode_graph(version, bytes, false).unwrap_err().class,
                crate::FailureClass::Missing
            );
        } else {
            assert!(decode_graph(version, bytes, false).is_ok());
        }
    }
}

#[test]
fn only_base_bodies_above_the_gate_have_fixed_signed_integer_vectors() {
    for (kind, version, bytes, _, _) in records() {
        let (fields, _) = decode_graph(version, bytes, true).unwrap();
        let count = fields
            .iter()
            .find(|field| field.path.ends_with("raw_words_850.count"));
        assert_eq!(count.is_some(), kind == 1 && version > 0x2bcc);
        if kind == 1
            && version == 19000
            && count.is_some_and(|field| matches!(field.value, ModelFieldValue::I32(3)))
        {
            let values: Vec<_> = fields
                .iter()
                .filter(|field| field.path.contains("raw_words_850["))
                .map(|field| field.value.clone())
                .collect();
            assert_eq!(
                values,
                [
                    ModelFieldValue::I32(i32::MIN),
                    ModelFieldValue::I32(0),
                    ModelFieldValue::I32(i32::MAX)
                ]
            );
        }
    }
}

#[test]
fn base_body_counts_reject_negative_and_unbounded_values() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 1 && *version == 19000)
        .unwrap();
    let (fields, _) = decode_graph(version, bytes, true).unwrap();
    for count in [-1i32, i32::MAX] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.base_body.raw_words_850.count",
            &count.to_le_bytes(),
        );
        let error = decode_graph(version, &changed, true).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("count"), "{}", error.cause);
    }
}
