use super::lightweight_section_tests::next_case;
use super::solid_reference_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

type ReferenceCurveRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ReferenceCurveRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/reference-curves/cases.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"RCV1");
    assert_eq!(cursor.u32().unwrap(), 60);
    let slots: Vec<_> = (0..60).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_reference_curves_match_independent_fields_bytes_and_slots() {
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
fn reference_curves_reject_every_prefix_extra_byte_and_old_context() {
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
fn reference_curve_array_branches_use_stored_flag_bits_and_fresh_or_cached_counts() {
    for (style, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let prefix = if matches!(style, 2 | 4 | 5) {
            "root.object.items[1].payload"
        } else {
            "root.object.payload"
        };
        let present = |label: &str| {
            fields
                .iter()
                .any(|field| field.path.starts_with(&format!("{prefix}.{label}")))
        };
        assert_eq!(present("dwords_8a8"), matches!(style, 3 | 5));
        assert_eq!(present("objects_8c8"), style == 2);
        assert_eq!(
            present("plane_8e8"),
            matches!(style, 0 | 1 | 4) && version > 0x2b94
        );
        assert_eq!(present("body_count_840"), version > 0x2b87);
        assert_eq!(present("curve_870"), version > 0x3efb);
    }
}

#[test]
fn reference_curve_pointer_roles_reject_a_cached_feature_instead_of_the_declared_target() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 1) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for label in [
            "references_860",
            "plane_8e8",
            "plane_868",
            "direction_850",
            "curve_870",
        ] {
            let path = format!("root.object.payload.{label}");
            if !fields.iter().any(|field| field.path.as_ref() == path) {
                continue;
            }
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, true) else {
                panic!("{label}: expected rejected moRefCurve_c role");
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn reference_curve_fixed_u16_body_counts_and_packed_dword_counts_are_bounded() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 == 2 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].payload.body_count_840",
            &u16::MAX.to_le_bytes(),
        );
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
fn reference_curve_selected_bodies_require_explicit_kernel_context() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| matches!(row.0, 3 | 5)) {
        assert!(decode_graph(version, bytes, true).is_ok());
        assert!(decode_graph(version, bytes, false).is_err());
    }
}

#[test]
fn reference_curve_minimal_suffix_retains_the_u16_and_exact_version_lengths() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let payload: Vec<_> = fields
            .iter()
            .filter(|field| field.path.starts_with("root.object.payload."))
            .cloned()
            .collect();
        let expected = 37
            + 2 * usize::from(version > 0x2b94)
            + 10 * usize::from(version > 0x2b87)
            + 2 * usize::from(version > 0x3efb);
        assert_eq!(
            super::encode_fields(&payload, 30000).unwrap().len(),
            expected
        );
        let word = fields
            .iter()
            .find(|field| field.path.ends_with("payload.raw_858"))
            .unwrap();
        assert_eq!(word.value, super::ModelFieldValue::U16(u16::MAX));
    }
}
