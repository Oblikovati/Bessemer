use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::{ModelDecoder, ModelField, ModelObjectRole};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const PACKET: &[u8] = include_bytes!("../fixtures/solid-references/cases.bin");
type SolidRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<SolidRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"SRF1");
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

pub(super) fn decode_graph(
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
fn complete_solid_reference_graphs_match_independent_bytes_fields_and_slots() {
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
fn solid_references_reject_every_truncated_prefix_extra_byte_and_old_context() {
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
fn solid_references_are_admitted_through_the_proven_wrapper_family() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 < 6) {
        let fields = super::direct_collection_tests::decode_kernel(
            version,
            bytes,
            LIMITS,
            Some(super::kernel_frame_tests::context()),
            |reader, map| {
                reader.object(
                    "object",
                    map,
                    ModelObjectRole::Declared(super::model_references::WRAPPER_CLASSES),
                )
            },
        )
        .unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn solid_component_roles_reject_the_cached_array_instead_of_guessing_a_component() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 >= 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].component_reference",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, true) else {
            panic!("suObArray: expected a rejected component-reference role");
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

fn absent_minimum(version: u32) -> usize {
    58 + 4 * usize::from(version >= 0x2b73)
        + 4 * usize::from(version > 0x2faf)
        + 4 * usize::from(version >= 0x3b02)
}

#[test]
fn absent_solid_minima_have_no_body_attributes_or_runtime_fixup_fields() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|row| row.0 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(bytes.len() - 6 - length, absent_minimum(version));
        assert_eq!(slots, 3);
        assert!(
            !fields
                .iter()
                .any(|field| field.path.contains("body_attributes"))
        );
    }
}

#[test]
fn selected_solid_bodies_always_have_direct_attributes_even_when_suppressed() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 < 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let attributes = fields
            .iter()
            .any(|field| field.path.as_ref() == "root.object.body_attributes[0]");
        assert_eq!(attributes, kind % 3 != 0);
        if kind % 3 == 2 {
            assert!(
                matches!(decode_graph(version, bytes, false), Err(error) if error.class == crate::FailureClass::Missing)
            );
        } else {
            assert!(decode_graph(version, bytes, false).is_ok());
        }
    }
}
