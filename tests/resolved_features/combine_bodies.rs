use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

type CombineBodyRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<CombineBodyRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/combine-bodies/cases.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"CMB1");
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

#[test]
fn complete_combine_delete_graphs_match_independent_fields_bytes_and_map_slots() {
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
fn combine_delete_graphs_reject_every_prefix_trailing_byte_and_old_context() {
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
fn combine_delete_minima_and_exact_option_byte_gate_match_native_layout() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 % 6 == 0) {
        let expected = match kind {
            0 => 8 + usize::from(version > 0x3f07),
            6 => 2,
            12 => 6,
            _ => panic!("{kind}: expected empty complete combine/delete kind"),
        };
        let (fields, actual_slots) = decode_graph(version, bytes, None, LIMITS).unwrap();
        assert_eq!(actual_slots, slots);
        let suffix: Vec<_> = fields
            .into_iter()
            .filter(|field| {
                [
                    "body_800",
                    "array_808",
                    "raw_828",
                    "raw_838",
                    "data_800",
                    "array_08",
                    "raw_28",
                ]
                .iter()
                .any(|label| field.path.starts_with(&format!("root.object.{label}")))
            })
            .collect();
        assert_eq!(
            super::encode_fields(&suffix, 30000).unwrap().len(),
            expected
        );
    }
}

#[test]
fn combine_option_bytes_retain_non_boolean_values_at_the_strict_boundary() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 < 6) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let options: Vec<_> = fields
            .iter()
            .filter(|field| field.path.ends_with("raw_838"))
            .collect();
        assert_eq!(options.is_empty(), version <= 0x3f07);
        for option in options {
            assert_eq!(
                option.value,
                super::ModelFieldValue::U8([0, 1, 2, 128, 254, 255][usize::from(kind)])
            );
        }
    }
}

#[test]
fn combine_delete_declared_roles_reject_cached_unrelated_root_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 2 || row.0 == 8)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let label = if kind == 2 { "body_800" } else { "data_800" };
        let path = format!("root.object.items[0].{label}");
        let changed =
            super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
        let Err(error) = decode_graph(version, &changed, None, LIMITS) else {
            panic!("{label}: expected rejected suObArray role");
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

#[test]
fn combine_delete_direct_arrays_reject_impossible_escaped_counts() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 % 6 == 2 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with("count.count_dword"))
        {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &field.path,
                &u32::MAX.to_le_bytes(),
            );
            assert!(
                decode_graph(version, &changed, None, LIMITS).is_err(),
                "{}",
                field.path
            );
        }
    }
}
