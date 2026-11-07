use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/blends/cases.bin");
type BlendRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<BlendRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"BLN1");
    assert_eq!(cursor.u32().unwrap(), 68);
    let slots: Vec<_> = (0..68).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_blend_and_cut_records_match_independent_fields_bytes_and_slots() {
    let mut primitives = 0;
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
        primitives += oracle.lines().count();
    }
    assert_eq!(primitives, 37370);
}

#[test]
fn blend_records_reject_all_prefixes_extra_bytes_and_old_context() {
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
fn negative_allocation_counts_reject_without_allocating_or_consuming_numeric_columns() {
    for (_, version, bytes, _, _) in records().into_iter().take(12) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.payload.data.declared_count_418",
            &(-1i32).to_le_bytes(),
        );
        let error = decode_graph(version, &changed, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains("blend declared count -1"),
            "{}",
            error.cause
        );
    }
}

#[test]
fn directions_and_dimensions_reject_cached_blend_root_of_the_wrong_class() {
    let record = records()
        .into_iter()
        .find(|record| record.0 == 0 && record.1 == 19000 && record.4 == 13)
        .unwrap();
    let fields = decode_graph(record.1, record.2, None, LIMITS).unwrap().0;
    for label in [
        "data.direction_68",
        "data.direction_70",
        "dimension_c70",
        "dimension_c78",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode_graph(record.1, &changed, None, LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moBlend_c"), "{}", error.cause);
    }
}

#[test]
fn blend_numeric_columns_charge_six_entries_per_row_and_reject_impossible_extents() {
    let bytes = [0u8; 108];
    for maximum in [18, 17] {
        let result = super::direct_collection_tests::decode_limited(
            19000,
            &bytes,
            super::ModelFieldLimits {
                maximum_entries: maximum,
                ..LIMITS
            },
            |reader, _| super::blend_columns::read(reader, 3),
        );
        assert_eq!(result.is_ok(), maximum == 18);
    }
    let result =
        super::direct_collection_tests::decode_collection(19000, &bytes[..107], |reader, _| {
            super::blend_columns::read(reader, 3)
        });
    let error = result.unwrap_err();
    assert!(
        error.cause.contains("blend column count 3"),
        "{}",
        error.cause
    );
}
