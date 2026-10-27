use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelObjectRole};
use crate::FailureClass::{Invalid, Missing, Unsupported};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/helices/cases.bin");
const TOO_MANY_VALUES: [u8; 4] = u32::MAX.to_le_bytes();
const UNSUPPORTED_WIDTH: [u8; 4] = 4u32.to_le_bytes();
type HelixRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<HelixRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"HLX1");
    assert_eq!(cursor.u32().unwrap(), 243);
    let slots: Vec<_> = (0..243).map(|_| cursor.u32().unwrap() as usize).collect();
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
fn complete_helix_and_wire_identifier_graphs_match_independent_bytes_fields_and_slots() {
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
fn helix_records_reject_every_truncated_prefix_extra_byte_and_old_context() {
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

fn field_bits(fields: &[ModelField], path: &str) -> u64 {
    let field = fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let super::ModelFieldValue::F64(value) = field.value else {
        panic!("{path}: expected a stored double");
    };
    value.bits()
}

#[test]
fn helix_strict_threshold_uses_an_independent_integer_order_oracle_including_nan_and_infinity() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let magnitude =
            field_bits(&fields, "root.object.payload.data_788.raw_28") & 0x7fff_ffff_ffff_ffff;
        let expected = magnitude > 0x3eb0_c6f7_a0b5_ed8d && magnitude <= 0x7ff0_0000_0000_0000;
        let present = fields
            .iter()
            .any(|field| field.path.as_ref() == "root.object.payload.dimension_7a8");
        assert_eq!(present, expected, "{magnitude:#018x}");
    }
}

fn rejected_field(
    version: u32,
    bytes: &[u8],
    path: &str,
    replacement: &[u8],
    class: crate::FailureClass,
) {
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, replacement);
    let Err(error) = decode_graph(version, &changed, true) else {
        panic!("{path}: expected {class:?}, got complete reconstruction");
    };
    assert_eq!(error.class, class, "{path}");
}

#[test]
fn helix_dynamic_arrays_and_table_counts_reject_unsupported_widths_and_impossible_extents() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 == 2 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for field in &fields {
            if field.path.contains(".data_788.variable_pitch_60.")
                && field.path.ends_with("element_width")
            {
                rejected_field(version, bytes, &field.path, &UNSUPPORTED_WIDTH, Unsupported);
            }
            if field.path.ends_with(".count") && field.path.starts_with("root.object.payload.") {
                rejected_field(version, bytes, &field.path, &TOO_MANY_VALUES, Invalid);
            }
        }
    }
}

fn is_dimension_pointer(field: &ModelField) -> bool {
    matches!(field.value, super::ModelFieldValue::ObjectTag(_))
        && declared_dimension_path(&field.path)
}

fn declared_dimension_path(path: &str) -> bool {
    let Some(relative) = path.strip_prefix("root.object.items[0].payload.") else {
        return false;
    };
    if matches!(
        relative,
        "dimension_790" | "dimension_798" | "dimension_7a0" | "dimension_7a8" | "dimension_7b0"
    ) {
        return true;
    }
    let Some(table) = relative.strip_prefix("dimension_table_7c0.") else {
        return false;
    };
    table == "dimension_60" || (table.starts_with("rows[") && !table.contains('.'))
}

#[test]
fn helix_dimension_roles_reject_cached_arrays_in_main_optional_table_and_terminal_slots() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 == 1 && row.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for field in fields.iter().filter(|field| is_dimension_pointer(field)) {
            rejected_field(version, bytes, &field.path, &2u16.to_le_bytes(), Invalid);
        }
    }
}

#[test]
fn wire_identifiers_keep_declared_representation_and_external_reference_roles() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 5) {
        for suffix in [
            "source_entity.representation",
            "source_entity.representation.external_feature_reference",
        ] {
            let path = format!("root.object.items[1].{suffix}");
            rejected_field(version, bytes, &path, &2u16.to_le_bytes(), Invalid);
        }
    }
}

#[test]
fn minimum_helix_and_wire_identifiers_add_no_direct_data_list_or_runtime_allocation_tags() {
    let mut minimum = [0; 400];
    minimum[279] = 255;
    let fields = super::direct_collection_tests::decode_collection(
        19000,
        &minimum,
        super::helix_records::read,
    )
    .unwrap();
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), minimum);
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 == 3) {
        assert_eq!(slots, 3);
        let name_length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(bytes.len() - 6 - name_length, 8);
        assert_eq!(decode_graph(version, bytes, true).unwrap().0.len(), 4);
    }
}

#[test]
fn serialized_helix_wires_require_explicit_kernel_context_and_suppressed_wires_do_not() {
    let mut present = 0;
    let mut suppressed = 0;
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 2) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        if fields
            .iter()
            .any(|field| matches!(field.value, super::ModelFieldValue::Kernel(_)))
        {
            assert!(
                matches!(decode_graph(version, bytes, false), Err(error) if error.class == Missing)
            );
            present += 1;
        } else {
            assert!(decode_graph(version, bytes, false).is_ok());
            suppressed += 1;
        }
    }
    assert!(present > 0 && suppressed > 0);
}
