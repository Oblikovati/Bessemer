use super::configuration_manager::{ParsedConfigurationManager, SavedHelperState, parse};
use super::{ModelArchiveLimits, ModelField, encode_fields};
use crate::{Document, FailureClass, header::ArchiveVersion};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

pub(super) fn manager_bytes() -> Vec<u8> {
    Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT"))
        .unwrap()
        .read_stream("Contents/CMgr")
        .unwrap()
}

pub(super) fn decode(bytes: &[u8]) -> crate::Result<ParsedConfigurationManager> {
    parse(bytes, ArchiveVersion::new(19000), LIMITS)
}

pub(super) fn replace_field(fields: &[ModelField], path: &str, replacement: &[u8]) -> Vec<u8> {
    let index = fields
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap();
    let start = encode_fields(&fields[..index], LIMITS.maximum_bytes)
        .unwrap()
        .len();
    let length = encode_fields(&fields[index..=index], LIMITS.maximum_bytes)
        .unwrap()
        .len();
    let mut bytes = encode_fields(fields, LIMITS.maximum_bytes).unwrap();
    bytes.splice(start..start + length, replacement.iter().copied());
    bytes
}

#[test]
fn every_manager_prefix_and_appended_byte_rejects_partial_publication() {
    let bytes = manager_bytes();
    for end in 0..bytes.len() {
        assert!(decode(&bytes[..end]).is_err(), "prefix {end}");
    }
    let mut trailing = bytes;
    trailing.push(0);
    assert!(decode(&trailing).is_err());
}

#[test]
fn saved_helper_state_uses_completed_fields_and_checks_both_allowed_values() {
    let original = decode(&manager_bytes()).unwrap();
    for state in [0_i32, 1] {
        let bytes = replace_field(
            &original.fields,
            "root.manager.persisted_state_650",
            &state.to_le_bytes(),
        );
        let parsed = decode(&bytes).unwrap();
        let expected = if state == 0 {
            SavedHelperState::Inactive
        } else {
            SavedHelperState::Active
        };
        assert_eq!(parsed.helper_state, expected);
    }
    for state in [i32::MIN, -1, 2, i32::MAX] {
        let bytes = replace_field(
            &original.fields,
            "root.manager.persisted_state_650",
            &state.to_le_bytes(),
        );
        let error = decode(&bytes).err().unwrap();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains(&state.to_string()));
    }
}

#[test]
fn manager_context_rejects_unverified_versions_null_roots_and_missing_selected_roots() {
    let bytes = manager_bytes();
    for version in [0, 9000, 18000, 18999, 19001, u32::MAX] {
        assert!(parse(&bytes, ArchiveVersion::new(version), LIMITS).is_err());
    }
    assert!(decode(&[0, 0]).is_err());
    let parsed = decode(&bytes).unwrap();
    let missing = replace_field(&parsed.fields, "root.manager.configurations[0]", &[0, 0]);
    assert!(decode(&missing).is_err());
    let conversion = replace_field(
        &parsed.fields,
        "root.manager.configuration_358.raw_77c",
        &1_i32.to_le_bytes(),
    );
    assert_eq!(
        decode(&conversion).err().unwrap().class,
        FailureClass::Unsupported
    );
}

#[test]
fn singleton_postload_check_rejects_parent_links_and_self_referencing_children() {
    let original = decode(&manager_bytes()).unwrap();
    let field = original
        .fields
        .iter()
        .find(|field| field.path.as_ref() == "root.manager.configuration_360")
        .unwrap();
    let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("configuration tag")
    };
    let super::ModelObjectRecord::Reference { index, .. } = &tag.record else {
        panic!("configuration alias")
    };
    let alias = u16::try_from(index.value()).unwrap().to_le_bytes();
    let parent = replace_field(
        &original.fields,
        "root.manager.configuration_358.parent_698",
        &alias,
    );
    assert_eq!(
        decode(&parent).err().unwrap().class,
        FailureClass::Unsupported
    );
    let mut children = vec![1, 0];
    children.extend_from_slice(&alias);
    let cycle = replace_field(
        &original.fields,
        "root.manager.configuration_358.children_6a0.count.count_word",
        &children,
    );
    assert_eq!(
        decode(&cycle).err().unwrap().class,
        FailureClass::Unsupported
    );
}

#[test]
fn unimplemented_nonempty_branches_and_every_global_budget_fail_without_a_result() {
    let bytes = manager_bytes();
    let original = decode(&bytes).unwrap();
    let nonempty = replace_field(
        &original.fields,
        "root.manager.map_588.count",
        &1_i32.to_le_bytes(),
    );
    assert_eq!(
        decode(&nonempty).err().unwrap().class,
        FailureClass::Unsupported
    );
    for limits in [
        ModelArchiveLimits {
            maximum_bytes: bytes.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_fields: original.fields.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_entries: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_objects: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
    ] {
        assert!(parse(&bytes, ArchiveVersion::new(19000), limits).is_err());
    }
    assert_eq!(decode(&bytes).unwrap().fields, original.fields);
}
