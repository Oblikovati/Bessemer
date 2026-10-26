use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelObjectRole};
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/mirror-patterns/cases.bin");
type MirrorRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<MirrorRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"MPF1");
    assert_eq!(cursor.u32().unwrap(), 28);
    let slots: Vec<_> = (0..28).map(|_| cursor.u32().unwrap() as usize).collect();
    let mut records = Vec::new();
    for expected_slots in slots {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        records.push((kind, version, bytes, oracle, expected_slots));
    }
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
fn complete_mirrors_and_planes_match_independent_fields_bytes_and_map_slots() {
    let mut primitives = 0;
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        let count = oracle.lines().count();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, count);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        primitives += count;
    }
    assert_eq!(primitives, 7724);
}

#[test]
fn mirrors_and_planes_reject_all_truncated_prefixes_trailing_bytes_and_old_contexts() {
    for (_, version, bytes, _, _) in records() {
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
fn mirror_plane_pointer_rejects_the_enclosing_pattern_as_an_unrelated_class() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .unwrap();
    let fields = decode(version, bytes).unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.plane_11a8",
        &2u16.to_le_bytes(),
    );
    let error = decode(version, &changed).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moMirrorPattern_c"));
    assert!(error.cause.contains("moPlaneRef_w"));
}

#[test]
fn complete_plane_references_fill_the_declared_wrapper_and_surface_roles() {
    for classes in [
        super::model_references::WRAPPER_CLASSES,
        super::geometric_references::SURFACE_CLASSES,
    ] {
        for (name, accepted) in [
            ("moPlaneRef_w", true),
            ("moPlaneRef_w_suffix", false),
            ("moNodeName_c", false),
        ] {
            let result = decode_collection(19000, &[1, 0], |reader, map| {
                map.append_loaded_object(Some(crate::archive_codec::ArchiveClass {
                    name: name.into(),
                    schema: 1,
                }))?;
                reader.object("reference", map, ModelObjectRole::Declared(classes))
            });
            assert_eq!(result.is_ok(), accepted, "{name}");
        }
    }
}
