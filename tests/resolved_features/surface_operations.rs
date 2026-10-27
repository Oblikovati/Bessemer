use super::lightweight_section_tests::next_case;
use super::solid_reference_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

type SurfaceOperationRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<SurfaceOperationRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/surface-operations/cases.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"SOP1");
    assert_eq!(cursor.u32().unwrap(), 84);
    let slots: Vec<_> = (0..84).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_surface_operations_match_independent_fields_bytes_and_slots() {
    for (_, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) = decode_graph(version, bytes, true).unwrap();
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
fn surface_operations_reject_every_prefix_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], true).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), true).is_err());
        assert!(decode_graph(0x27cb, bytes, true).is_err());
    }
}

#[test]
fn direct_surface_suffix_minima_exclude_chooser_presence_and_allocated_trim_tags() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let cut = [
            "atom_feature_808",
            "raw_818",
            "raw_81c",
            "face_820",
            "surface_810",
            "plane_900",
            "array_8c0",
            "array_8e0",
            "chooser_838",
        ];
        let trim = ["trim_data_800", "chooser_808"];
        let labels: &[&str] = if kind == 0 { &cut } else { &trim };
        let suffix: Vec<_> = fields
            .iter()
            .filter(|field| {
                labels
                    .iter()
                    .any(|label| field.path.starts_with(&format!("root.object.{label}")))
            })
            .cloned()
            .collect();
        assert_eq!(
            super::encode_fields(&suffix, 30000).unwrap().len(),
            if kind == 0 { 24 } else { 30 }
        );
        assert!(
            !suffix
                .iter()
                .any(|field| field.path.ends_with(".presence") || field.path.contains("array_80"))
        );
    }
}

#[test]
fn surface_pointer_and_direct_chooser_roles_reject_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 2 || row.0 == 8)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let cut = [
            "atom_feature_808",
            "face_820",
            "surface_810",
            "plane_900",
            "chooser_838.owner",
        ];
        let trim = [
            "trim_data_800.profile_18",
            "trim_data_800.direction_40",
            "trim_data_800.solid_68",
            "chooser_808.owner",
        ];
        let labels: &[&str] = if kind == 2 { &cut } else { &trim };
        for label in labels {
            let path = format!("root.object.items[0].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, true) else {
                panic!("{label}: expected rejected suObArray role");
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn surface_direct_arrays_and_keeper_reject_impossible_escaped_extents() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 % 6 == 2 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
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
                decode_graph(version, &changed, true).is_err(),
                "{}",
                field.path
            );
        }
    }
}

#[test]
fn selected_surface_and_solid_kernel_bodies_require_explicit_schema_context() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| matches!(row.0, 3 | 5 | 9 | 11))
    {
        assert!(decode_graph(version, bytes, true).is_ok());
        assert!(decode_graph(version, bytes, false).is_err());
    }
}
