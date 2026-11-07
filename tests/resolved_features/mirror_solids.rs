use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

type MirrorSolidRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<MirrorSolidRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/mirror-solids/cases.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"MSL1");
    assert_eq!(cursor.u32().unwrap(), 198);
    let slots: Vec<_> = (0..198).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_mirror_solid_graphs_match_independent_fields_bytes_and_slots() {
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
fn mirror_solids_reject_every_prefix_extra_byte_and_old_context() {
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
fn mirror_solid_suffix_minima_and_gates_match_native_layout() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let suffix: Vec<_> = fields
            .iter()
            .filter(|field| field.path.starts_with("root.object.payload."))
            .cloned()
            .collect();
        let expected = 30
            + 2 * usize::from(version > 0x2f86)
            + 4 * usize::from(version > 0x2f93)
            + 3 * usize::from(version > 0x3745)
            + 4 * usize::from(version > 0x3760);
        assert_eq!(
            super::encode_fields(&suffix, 30000).unwrap().len(),
            expected
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("plane_818.presence")),
            version > 0x3760
        );
    }
}

#[test]
fn mirror_solid_declared_roles_reject_an_unrelated_cached_container() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 2) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        for label in ["face_800", "surface_9a0", "matrix_helper_9c8"] {
            let path = format!("root.object.items[0].payload.{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, None, LIMITS) else {
                panic!("{label}: expected rejected suObArray role");
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn fixed_width_mirror_indices_and_unsigned_array_counts_are_bounded() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 % 6 == 0 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let path = if kind == 0 {
            "root.object.payload.words_8c8.count"
        } else if kind == 12 {
            "root.object.count"
        } else {
            continue;
        };
        for count in [u32::MAX, i32::MAX as u32] {
            let changed =
                super::manager_context_tests::replace_field(&fields, path, &count.to_le_bytes());
            assert!(
                decode_graph(version, &changed, None, LIMITS).is_err(),
                "{kind}/{count}"
            );
        }
    }
}
