use super::lightweight_section_tests::next_case;
use super::solid_reference_tests::decode_graph;
use crate::archive_codec::{ArchiveCursor, ObjectReader};

type BlendReferenceRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<BlendReferenceRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/blend-reference-surfaces/cases.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"BRS1");
    assert_eq!(cursor.u32().unwrap(), 270);
    let slots: Vec<_> = (0..270).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_component_curves_blend_surfaces_and_keepers_match_independent_bytes_fields_and_slots() {
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
fn blend_reference_graphs_reject_every_prefix_extra_byte_and_old_context() {
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
fn component_curve_body_has_only_versioned_component_fields_and_the_feature_id_time_pair() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 == 0) {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        let expected = 74
            + 4 * usize::from(version > 0x2b60)
            + 4 * usize::from(version > 0x2bb8)
            + 4 * usize::from(version > 0x2f55)
            + 4 * usize::from(version >= 0x3ef6);
        assert_eq!(bytes.len() - 6 - name_length, expected);
        assert_eq!(slots, 3);
    }
}

#[test]
fn blend_reference_surface_keeps_five_raw_bytes_without_the_blend_cut_tail() {
    let flags = [0, 1, 2, 128, 254, 255];
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| (6..12).contains(&row.0))
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let bytes_10: Vec<_> = fields
            .iter()
            .filter(|field| field.path.ends_with("payload.raw_c10"))
            .collect();
        assert!(!bytes_10.is_empty());
        for byte in bytes_10 {
            assert_eq!(
                byte.value,
                super::ModelFieldValue::U8(flags[usize::from(kind - 6)])
            );
        }
        assert!(
            !fields
                .iter()
                .any(|field| field.path.ends_with("dimension_c70")
                    || field.path.ends_with("dimension_c78"))
        );
        if kind == 6 {
            let payload: Vec<_> = fields
                .iter()
                .filter(|field| field.path.starts_with("root.object.payload."))
                .cloned()
                .collect();
            assert_eq!(super::encode_fields(&payload, 30000).unwrap().len(), 131);
        }
    }
}

#[test]
fn ordinary_blend_reference_keepers_are_empty_tagged_objects() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 == 12) {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(bytes.len(), 6 + name_length);
        assert_eq!(slots, 3);
        assert_eq!(decode_graph(version, bytes, true).unwrap().1, 3);
    }
}

fn require_component_feature_role(bytes: &[u8]) -> crate::Result<()> {
    let mut reader = super::ModelDecoder::new_in_context(
        bytes,
        crate::header::ArchiveVersion::new(19000),
        super::annotation_view_tests::LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    reader.object(
        "object",
        &mut ObjectReader::new(),
        super::ModelObjectRole::Declared(super::component_references::FEATURE_CLASSES),
    )?;
    reader.finish().map(|_| ())
}

#[test]
fn exact_component_curve_wrapper_role_does_not_infer_component_feature_ancestry() {
    let (_, _, bytes, _, _) = records()
        .into_iter()
        .find(|row| row.0 == 0 && row.1 == 19000)
        .unwrap();
    let Err(error) = require_component_feature_role(bytes) else {
        panic!("moCompRefCurve_c: expected rejected moCompFeature_c role");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 5) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].component_reference",
            &2u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}

#[test]
fn blend_reference_surface_counts_and_serialized_bodies_keep_strict_boundaries() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 == 8 || row.0 == 10 || row.0 == 11)
    {
        assert!(decode_graph(version, bytes, true).is_ok());
        if kind == 8 {
            let fields = decode_graph(version, bytes, true).unwrap().0;
            let changed = super::manager_context_tests::replace_field(
                &fields,
                "root.object.items[0].payload.data.array_330.count.count_word",
                &u16::MAX.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, true).is_err());
        } else {
            assert!(decode_graph(version, bytes, false).is_err());
        }
    }
}
