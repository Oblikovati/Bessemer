use super::annotation_view_tests::{LIMITS, decode};
use super::lightweight_section_tests::next_case;
use super::sweep_record_tests::decode_graph;
use super::{ModelDecoder, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/reference-points/cases.bin");
type PointRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<PointRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"RPF1");
    assert_eq!(cursor.u32().unwrap(), 132);
    let slots: Vec<_> = (0..132).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn reference_points_and_all_data_variants_match_independent_primitives_bytes_and_slots() {
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
fn reference_points_reject_all_prefixes_trailing_bytes_and_old_contexts() {
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
fn reference_point_required_data_and_declared_pointer_families_reject_invalid_targets() {
    let bytes = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 6 && *version == 19000)
        .unwrap()
        .2;
    let fields = decode(19000, bytes).unwrap();
    let null = super::manager_context_tests::replace_field(
        &fields,
        "root.object.data_6f8",
        &0u16.to_le_bytes(),
    );
    let error = decode(19000, &null).unwrap_err();
    assert!(error.cause.contains("nonnull"), "{}", error.cause);
    for label in [
        "data_6f8",
        "profile_6f0",
        "surface_reference_700",
        "data_6f8.curve_reference_10",
        "data_6f8.dimension_38",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(19000, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moRefPoint_c"), "{}", error.cause);
    }
}

#[test]
fn empty_point_data_keeps_tagged_map_entries_and_requires_explicit_ordinary_context() {
    for (_, version, bytes, _, slots) in records()
        .into_iter()
        .filter(|(kind, _, _, _, _)| *kind == 7)
    {
        let (fields, actual) = decode_graph(version, bytes).unwrap();
        assert_eq!(actual, slots);
        assert_eq!(actual, 3);
        assert_eq!(fields.len(), 1);
        let mut reader = ModelDecoder::new(bytes, ArchiveVersion::new(version), LIMITS).unwrap();
        let error = reader
            .object("data", &mut ObjectReader::new(), ModelObjectRole::Any)
            .unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}
