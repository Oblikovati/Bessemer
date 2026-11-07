use super::lightweight_section_tests::next_case;
use super::solid_reference_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

type OffsetSurfaceRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<OffsetSurfaceRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/offset-surfaces/cases.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"OFS1");
    assert_eq!(cursor.u32().unwrap(), 133);
    let slots: Vec<_> = (0..133).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_offset_surfaces_data_and_keepers_match_independent_fields_bytes_and_slots() {
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
fn offset_surfaces_reject_every_prefix_extra_byte_and_old_context() {
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
fn offset_surface_data_is_required_and_rejects_cached_self_as_wrong_class() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for pointer in [0u16, 2u16] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                "root.object.offset_data_7c8",
                &pointer.to_le_bytes(),
            );
            let Err(error) = decode_graph(version, &changed, true) else {
                panic!("{pointer}: expected nonnull moOffsetSurfData_c");
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains("offset_data_7c8"));
        }
    }
}

#[test]
fn native_composed_offset_minima_follow_both_inherited_version_boundaries() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 == 18) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let body: Vec<_> = fields
            .iter()
            .filter(|field| {
                field
                    .path
                    .starts_with("root.object.items[1].reference_geometry.")
                    || field.path.starts_with("root.object.items[1].surface.")
                    || field.path.as_ref() == "root.object.items[1].offset_data_7c8"
            })
            .cloned()
            .collect();
        assert_eq!(
            super::encode_fields(&body, 30000).unwrap().len(),
            289 + 4 * usize::from(version >= 0x2bcd) + 4 * usize::from(version > 0x2f85)
        );
        assert_eq!(slots, 7);
    }
}

#[test]
fn offset_nullable_array_presence_retains_signed_values_and_empty_framing() {
    let selectors = [0, 1, -1, i32::MIN, i32::MAX, 1];
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| (6..12).contains(&row.0))
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let style = usize::from(kind - 6);
        let prefix = if style >= 2 {
            "root.object.items[1]"
        } else {
            "root.object"
        };
        let path = format!("{prefix}.array_08.presence");
        assert_eq!(
            fields
                .iter()
                .find(|field| field.path.as_ref() == path)
                .unwrap()
                .value,
            super::ModelFieldValue::I32(selectors[style])
        );
        if style < 2 {
            let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
            assert_eq!(bytes.len() - 6 - name_length, 10 + 2 * style);
        }
    }
}

#[test]
fn ordinary_offset_keepers_have_zero_body_bytes_and_normal_tagged_slots() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 == 12) {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(bytes.len(), 6 + name_length);
        assert_eq!(slots, 3);
        assert_eq!(decode_graph(version, bytes, true).unwrap().1, 3);
    }
}

#[test]
fn offset_dimensions_keepers_and_array_extents_reject_invalid_targets_and_counts() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| (row.0 == 8 || row.0 == 5) && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let path = if kind == 8 {
            "root.object.items[1].dimension_10".to_owned()
        } else {
            fields
                .iter()
                .find(|field| field.path.ends_with(".id_keeper"))
                .unwrap()
                .path
                .to_string()
        };
        let changed =
            super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
        assert!(decode_graph(version, &changed, true).is_err());
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
fn offset_serialized_surface_and_face_bodies_require_explicit_kernel_context() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| matches!(row.0, 4 | 5 | 11))
    {
        assert!(decode_graph(version, bytes, true).is_ok());
        assert!(decode_graph(version, bytes, false).is_err());
    }
}
