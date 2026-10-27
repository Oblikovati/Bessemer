use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/weld-corners/cases.bin");
type WeldCornerRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<WeldCornerRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"WCR1");
    assert_eq!(cursor.u32().unwrap(), 120);
    let slots: Vec<_> = (0..120).map(|_| cursor.u32().unwrap() as usize).collect();
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

#[test]
fn complete_weld_corner_graphs_match_independent_fields_bytes_and_slots() {
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
    }
}

#[test]
fn weld_corners_reject_every_prefix_extra_byte_and_old_context() {
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
fn weld_corner_point_roles_reject_the_unrelated_cached_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].payload.point_reference_860",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, None, LIMITS) else {
            panic!("suObArray point: expected a rejected declared point-reference role");
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

fn payload_bytes(fields: &[super::ModelField]) -> Vec<u8> {
    let selected: Vec<_> = fields
        .iter()
        .filter(|field| field.path.starts_with("root.object.payload."))
        .cloned()
        .collect();
    super::encode_fields(&selected, 30000).unwrap()
}

#[test]
fn empty_weld_corner_suffix_and_data_minima_exclude_runtime_only_containers_and_flags() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        if kind == 0 {
            assert_eq!(
                payload_bytes(&fields).len(),
                if version > 0x334d { 30 } else { 26 }
            );
            continue;
        }
        let name_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(
            bytes.len() - 6 - name_length,
            [0, 10, 12, 26][usize::from(kind / 6)]
        );
        assert_eq!(slots, 3);
        assert!(!fields.iter().any(|field| field.path.ends_with("raw_158")));
    }
}

#[test]
fn weld_corner_late_word_is_present_only_after_the_exact_documented_boundary() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 < 6) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let present = fields
            .iter()
            .any(|field| field.path.ends_with("payload.raw_868"));
        assert_eq!(present, version > 0x334d);
        assert!(
            !fields
                .iter()
                .any(|field| field.path.ends_with("loaded_default_868"))
        );
    }
}

#[test]
fn direct_weld_arrays_and_dwords_reject_impossible_escaped_counts() {
    let invalid_count = u32::MAX.to_le_bytes();
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 % 6 == 2 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with("count.count_dword"))
        {
            let changed =
                super::manager_context_tests::replace_field(&fields, &field.path, &invalid_count);
            let result = decode_graph(version, &changed, None, LIMITS);
            assert!(result.is_err(), "{}", field.path);
        }
    }
}
