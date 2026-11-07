use super::ModelFieldValue;
use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use crate::archive_codec::{ArchiveCursor, ObjectRecord};

const PACKET: &[u8] = include_bytes!("../fixtures/domes/cases.bin");
type DomeRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<DomeRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"DOF1");
    assert_eq!(cursor.u32().unwrap(), 128);
    let slots: Vec<_> = (0..128).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn dome_records_match_every_independent_primitive_byte_and_final_object_slot() {
    let mut reads = 0;
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        reads += oracle.lines().count();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
    assert_eq!(reads, 42006);
}

#[test]
fn domes_reject_every_truncated_prefix_trailing_byte_and_unsupported_version() {
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
fn all_four_dome_pointers_keep_independent_null_tags_and_signed_extrema() {
    for (mask, version, bytes, _, _) in records() {
        let fields = decode(version, bytes).unwrap();
        for (label, bit) in [
            ("dimension_850", 1),
            ("faces_838", 2),
            ("reference_840", 4),
            ("reference_848", 8),
        ] {
            let field = fields
                .iter()
                .find(|field| field.path.as_ref() == format!("root.object.payload.{label}"))
                .unwrap();
            assert!(
                matches!(&field.value, ModelFieldValue::ObjectTag(tag) if matches!(tag.record, ObjectRecord::Null) == (mask & bit == 0))
            );
        }
        for (label, value) in [
            ("raw_858", i32::MIN),
            ("raw_85c", i32::MAX),
            ("raw_860", -1),
        ] {
            let field = fields
                .iter()
                .find(|field| field.path.as_ref() == format!("root.object.payload.{label}"))
                .unwrap();
            assert_eq!(field.value, ModelFieldValue::I32(value));
        }
    }
}

#[test]
fn dome_declared_pointer_families_reject_a_cached_body_feature() {
    let bytes = records()
        .into_iter()
        .find(|(mask, version, _, _, _)| *mask == 15 && *version == 19000)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    for label in [
        "dimension_850",
        "faces_838",
        "reference_840",
        "reference_848",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(19000, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moDome_c"), "{}", error.cause);
    }
}
