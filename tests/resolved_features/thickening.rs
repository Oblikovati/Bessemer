use super::lightweight_section_tests::next_case;
use super::solid_reference_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

type ThickenRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ThickenRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/thickening/cases.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"THK1");
    assert_eq!(cursor.u32().unwrap(), 126);
    let slots: Vec<_> = (0..126).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_thickening_graphs_match_independent_fields_bytes_and_slots() {
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
fn thickening_graphs_reject_every_prefix_extra_byte_and_old_context() {
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
fn thickening_suffix_minimum_and_inclusive_direction_boundary_match_native_layout() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let labels = [
            "raw_810",
            "raw_818",
            "raw_81c",
            "list_808",
            "raw_820",
            "dimension_800",
            "raw_824",
            "raw_828",
            "direction_830",
        ];
        let suffix: Vec<_> = fields
            .iter()
            .filter(|field| {
                labels.iter().any(|label| {
                    field
                        .path
                        .starts_with(&format!("root.object.items[1].{label}"))
                })
            })
            .cloned()
            .collect();
        assert_eq!(
            super::encode_fields(&suffix, 30000).unwrap().len(),
            32 + 2 * usize::from(version >= 0x2f74)
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("direction_830")),
            version >= 0x2f74
        );
    }
}

#[test]
fn thickening_declared_targets_reject_unrelated_cached_root_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 2) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for label in ["list_808", "dimension_800", "direction_830"] {
            if label == "direction_830" && version < 0x2f74 {
                continue;
            }
            let path = format!("root.object.items[1].{label}");
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
fn component_surface_has_one_signed_suffix_without_a_coordinate_transform() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 >= 12) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let words: Vec<_> = fields
            .iter()
            .filter(|field| field.path.ends_with("raw_100"))
            .collect();
        assert!(!words.is_empty());
        for word in words {
            assert_eq!(word.value, super::ModelFieldValue::I32(i32::MAX));
        }
        assert!(
            !fields
                .iter()
                .any(|field| field.path.ends_with("transform_100"))
        );
    }
}

#[test]
fn constant_surface_serialized_bodies_require_explicit_kernel_context() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| matches!(row.0, 3 | 5 | 9 | 11))
    {
        assert!(decode_graph(version, bytes, true).is_ok());
        assert!(decode_graph(version, bytes, false).is_err());
    }
}

fn declared_constant(bytes: &[u8], classes: &[&str]) -> crate::Result<()> {
    let mut reader = super::ModelDecoder::new_in_context(
        bytes,
        crate::header::ArchiveVersion::new(19000),
        super::annotation_view_tests::LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = crate::archive_codec::ObjectReader::new();
    reader.object(
        "object",
        &mut map,
        super::ModelObjectRole::Declared(classes),
    )?;
    reader.finish().map(|_| ())
}

#[test]
fn constant_surface_is_an_atom_wrapper_and_not_a_surface_wrapper() {
    let (_, _, bytes, _, _) = records()
        .into_iter()
        .find(|row| row.0 == 6 && row.1 == 19000)
        .unwrap();
    assert!(declared_constant(bytes, super::model_references::WRAPPER_CLASSES).is_ok());
    let Err(error) = declared_constant(bytes, super::geometric_references::SURFACE_CLASSES) else {
        panic!("moConstSurfRef_w: expected rejected moSurfaceRef_w role");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}
