use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/revolved-surfaces/cases.bin");
type RevolvedRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<RevolvedRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"RVS1");
    assert_eq!(cursor.u32().unwrap(), 253);
    let slots: Vec<_> = (0..253).map(|_| cursor.u32().unwrap() as usize).collect();
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

fn decode_graph(
    version: u32,
    bytes: &[u8],
    kernel: bool,
) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_kernel(kernel.then(super::kernel_frame_tests::context));
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn complete_revolved_surface_data_and_keeper_graphs_match_independent_bytes_fields_and_slots() {
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
fn revolved_records_reject_every_truncated_prefix_extra_byte_and_old_context() {
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

fn rejected_cached_array(version: u32, bytes: &[u8], path: &str) {
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, &2u16.to_le_bytes());
    let error = decode_graph(version, &changed, true).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("suObArray"));
}

#[test]
fn revolved_pointer_roles_reject_cached_arrays_at_each_declared_reference() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| [2, 6].contains(&row.0))
    {
        for label in ["dimension_08", "dimension_10", "end_spec_20"] {
            rejected_cached_array(version, bytes, &format!("root.object.items[0].{label}"));
        }
        if kind == 2 {
            for label in ["revolve_data_7b0", "axis_7b8"] {
                rejected_cached_array(version, bytes, &format!("root.object.items[2].{label}"));
            }
        }
    }
}

#[test]
fn revolved_explicit_keeper_is_nonnull_and_rejects_the_cached_enclosing_surface() {
    let path = "root.object.reference_geometry.id_keeper";
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 3) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for tag in [0u16, 2] {
            let changed =
                super::manager_context_tests::replace_field(&fields, path, &tag.to_le_bytes());
            assert_eq!(
                decode_graph(version, &changed, true).unwrap_err().class,
                crate::FailureClass::Invalid
            );
        }
    }
}

#[test]
fn revolved_minimum_bodies_do_not_serialize_runtime_keeper_allocation_or_cleanup() {
    use super::direct_collection_tests::decode_collection;
    let feature = decode_collection(19000, &[0; 299], super::revolved_surfaces::read).unwrap();
    assert_eq!(super::encode_fields(&feature, 30000).unwrap(), [0; 299]);
    let data = decode_collection(19000, &[0; 14], super::revolved_surfaces::data).unwrap();
    assert_eq!(data.len(), 5);
    let keeper = decode_collection(19000, &[], |reader, _| {
        super::extruded_surfaces::keeper(reader)
    })
    .unwrap();
    assert!(keeper.is_empty());
}

#[test]
fn revolved_inline_kernel_requires_explicit_schema_and_rejects_invalid_sheet_counts() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 12) {
        let missing = decode_graph(version, bytes, false).unwrap_err();
        assert_eq!(missing.class, crate::FailureClass::Missing);
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for count in [i32::MIN, -1, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                "root.object.surface.sheet_count",
                &count.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, true).is_err());
        }
        assert!(
            fields
                .iter()
                .any(|field| matches!(field.value, super::ModelFieldValue::Kernel(_)))
        );
    }
}

#[test]
fn revolved_axis_keeps_the_documented_exact_line_role_instead_of_guessing_subclasses() {
    let mut bytes = vec![0; 297];
    bytes.extend_from_slice(&1u16.to_le_bytes());
    let error = super::direct_collection_tests::decode_collection(19000, &bytes, |reader, map| {
        map.append_loaded_object(Some(crate::archive_codec::ArchiveClass {
            name: "moTempAxisRef_w".into(),
            schema: 1,
        }))?;
        super::revolved_surfaces::read(reader, map)
    })
    .unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moTempAxisRef_w"));
}
