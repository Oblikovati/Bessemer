use super::annotation_view_tests::LIMITS;
use super::configuration_manager::SavedHelperState;
use super::kernel_frame_tests::context;
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/flat-patterns/cases.bin");
type FlatRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<FlatRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"FLT1");
    assert_eq!(cursor.u32().unwrap(), 144);
    let slots: Vec<_> = (0..144).map(|_| cursor.u32().unwrap() as usize).collect();
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

/// Decode an explicit saved-state graph; e.g. `decode_graph(1, 19000, bytes, true)`.
pub(super) fn decode_graph(
    kind: u8,
    version: u32,
    bytes: &[u8],
    kernel: bool,
) -> crate::Result<(Vec<ModelField>, usize)> {
    let state = if kind & 1 == 0 {
        SavedHelperState::Inactive
    } else {
        SavedHelperState::Active
    };
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_saved_helper_state(state)
    .with_kernel(kernel.then(context));
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn complete_flat_patterns_and_tagged_boxes_match_independent_fields_bytes_and_slots() {
    for (kind, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) = decode_graph(kind, version, bytes, true).unwrap();
        assert_eq!(actual_slots, slots);
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
fn flat_patterns_and_boxes_reject_every_prefix_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(kind, version, &bytes[..end], true).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(kind, version, &[bytes, &[0]].concat(), true).is_err());
        assert!(decode_graph(kind, 0x27cb, bytes, true).is_err());
    }
}

#[test]
fn raw_kernel_uses_typed_partition_without_a_suppression_byte_and_requires_context() {
    let mut present = 0;
    for (kind, version, bytes, _, _) in records() {
        let (fields, _) = decode_graph(kind, version, bytes, true).unwrap();
        let frame = fields.iter().find_map(|field| match &field.value {
            ModelFieldValue::Kernel(frame) => Some((field.path.as_ref(), frame)),
            _ => None,
        });
        if let Some((path, frame)) = frame {
            present += 1;
            assert_eq!(path, "root.object.payload.body_a10.frame");
            assert_eq!(
                frame.to_bytes().unwrap(),
                super::kernel_frame_tests::FRAME[1..]
            );
            assert_eq!(frame.record_count(), 1);
            assert_eq!(
                decode_graph(kind, version, bytes, false).unwrap_err().class,
                crate::FailureClass::Missing
            );
        } else {
            assert!(decode_graph(kind, version, bytes, false).is_ok());
        }
    }
    assert_eq!(present, 48);
}

#[test]
fn pair_counts_and_declared_references_reject_malformed_records() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|record| record.0 < 4) {
        let fields = decode_graph(kind, version, bytes, true).unwrap().0;
        for count in [-1i32, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                "root.object.payload.pairs_ac0.count",
                &count.to_le_bytes(),
            );
            assert!(decode_graph(kind, version, &changed, true).is_err());
        }
        for label in [
            "reference_8f8",
            "face_920",
            "reference_910",
            "feature_9f8",
            "list_a38",
        ] {
            let label = format!("root.object.payload.{label}");
            if !fields.iter().any(|field| field.path.as_ref() == label) {
                continue;
            }
            let changed =
                super::manager_context_tests::replace_field(&fields, &label, &2u16.to_le_bytes());
            assert!(
                decode_graph(kind, version, &changed, true).is_err(),
                "{label}"
            );
        }
    }
}

#[test]
fn raw_kernel_lengths_types_and_compression_identifiers_reject_invalid_frames() {
    let (kind, version, bytes, _, _) = records().into_iter().find(|record| record.0 == 2).unwrap();
    let fields = decode_graph(kind, version, bytes, true).unwrap().0;
    let original = super::kernel_frame_tests::FRAME[1..].to_vec();
    for length in [0i32, 28, i32::MAX] {
        let mut frame = original.clone();
        frame[..4].copy_from_slice(&length.to_le_bytes());
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.payload.body_a10.frame",
            &frame,
        );
        assert!(decode_graph(kind, version, &changed, true).is_err());
    }
    for position in [4, 9] {
        let mut frame = original.clone();
        frame[position] ^= 0x7f;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.payload.body_a10.frame",
            &frame,
        );
        assert!(decode_graph(kind, version, &changed, true).is_err());
    }
}
