use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelObjectRole, boundary_references, encode_fields};
use crate::Result;
use crate::archive_codec::{
    ArchiveClass, ArchiveCursor, ArchiveMapIndex, ObjectReader, ObjectRecord,
};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/entity-representation-cases.bin");

pub(super) fn decode(kind: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    if kind == 0 {
        return super::annotation_view_tests::decode(version, bytes);
    }
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    for name in if kind == 1 {
        &["entity"][..]
    } else {
        &["first", "second"][..]
    } {
        reader.scope(name, |reader| {
            boundary_references::entity_id(reader, &mut map)
        })?;
    }
    reader.finish()
}

#[test]
fn entity_representation_variants_and_direct_aliases_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (482, 1144));
}

#[test]
fn entity_representation_prefixes_and_trailing_bytes_cannot_publish_partial_fields() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length]).is_err(),
                "kind {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing).is_err());
    }
}

fn records(name: &str) -> [ObjectRecord; 2] {
    let index = ArchiveMapIndex::nonzero(1).unwrap();
    let class = ArchiveClass {
        name: name.into(),
        schema: 1,
    };
    [
        ObjectRecord::New {
            index,
            class: class.clone(),
        },
        ObjectRecord::Reference { index, class },
    ]
}

#[test]
fn representation_roles_accept_only_documented_variants_for_both_new_objects_and_aliases() {
    let role = ModelObjectRole::Declared(&["moEntityIdRep_c"]);
    for record in records("moDerivedSurfIdRep_c") {
        assert!(super::objects::require_role(&record, role, "representation").is_ok());
        let unrelated = ModelObjectRole::Declared(&["moNodeName_c"]);
        assert!(super::objects::require_role(&record, unrelated, "name").is_err());
    }
    for name in ["moNodeName_c", "moCompRef_c", "moFutureSurfIdRep_c"] {
        for record in records(name) {
            assert!(super::objects::require_role(&record, role, "representation").is_err());
        }
        assert!(super::entity_representations::layout(name).is_none());
    }
    assert!(super::objects::require_role(&ObjectRecord::Null, role, "representation").is_ok());
}

#[test]
fn new_model_assignability_does_not_relax_the_existing_shared_display_role_check() {
    for record in records("moDerivedSurfIdRep_c") {
        let role = ModelObjectRole::Declared(&["moEntityIdRep_c"]);
        assert!(
            crate::archive_codec::require_object_role(&record, role, "display", super::invalid)
                .is_err()
        );
    }
}
