use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelObjectRole};
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/shell-features/cases.bin");
type ShellRecord = (u32, &'static [u8], &'static str, usize);

fn records() -> Vec<ShellRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"SHF1");
    assert_eq!(cursor.u32().unwrap(), 18);
    let slots: Vec<_> = (0..18).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (_, version, bytes, oracle) = next_case(&mut cursor);
            (version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

fn decode_graph(version: u32, bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut slots = 0;
    let fields = decode_collection(version, bytes, |reader, map| {
        reader.object("object", map, ModelObjectRole::Any)?;
        slots = std::mem::replace(map, crate::archive_codec::ObjectReader::new())
            .into_snapshot()
            .slot_count();
        Ok(())
    })?;
    Ok((fields, slots))
}

#[test]
fn complete_shells_match_independent_primitives_bytes_and_final_object_slots() {
    for (version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
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
fn shells_reject_every_truncated_prefix_trailing_bytes_and_old_contexts() {
    for (version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn shell_pointer_roles_reject_cached_owners_as_unrelated_dependencies() {
    let (version, bytes, _, _) = records()
        .into_iter()
        .find(|(_, bytes, _, _)| {
            decode(19000, bytes).is_ok_and(|fields| {
                fields.iter().any(|field| {
                    field.path.as_ref() == "root.object.payload.raw_818"
                        && matches!(field.value, super::ModelFieldValue::U8(0))
                })
            })
        })
        .unwrap();
    let fields = decode(version, bytes).unwrap();
    for label in [
        "list_820",
        "dimension_810",
        "list_828",
        "list_830",
        "edge_840",
        "solid_body_848",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(version, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moShell_c"), "{}", error.cause);
    }
}
