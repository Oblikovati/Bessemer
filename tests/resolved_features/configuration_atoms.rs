use super::direct_collection_tests::{decode_collection, decode_limited};
use super::{ModelArchiveLimits, ModelField, ModelFieldValue, ModelObjectRecord};
use crate::Result;

const RAW: &[u8] = include_bytes!("../fixtures/resolved-features/linked-atoms.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn decode(bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        super::configuration_atoms::linked_map(reader, map, "atoms")
    })
}

#[test]
fn recursive_configuration_atoms_match_reference_partition_aliases_and_unsigned_extremes() {
    let fields = decode(RAW).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        RAW,
        include_str!("../fixtures/resolved-features/linked-atoms-primitives.tsv"),
        37,
    );
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), RAW);
    assert!(
        fields
            .iter()
            .any(|field| field.value == ModelFieldValue::U32(u32::MAX))
    );
    assert!(
        fields
            .iter()
            .any(|field| field.value == ModelFieldValue::I32(i32::MIN))
    );
    let references: Vec<_> = fields.iter().filter_map(referenced_index).collect();
    assert_eq!(references, [2, 3, 2, 2, 3]);
}

fn referenced_index(field: &ModelField) -> Option<u32> {
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        return None;
    };
    let ModelObjectRecord::Reference { index, .. } = &tag.record else {
        return None;
    };
    Some(index.value())
}

#[test]
fn linked_atom_prefixes_extra_bytes_and_impossible_counts_reject_partial_graphs() {
    for length in 0..RAW.len() {
        assert!(decode(&RAW[..length]).is_err(), "prefix {length}");
    }
    assert!(decode(&[RAW, &[0]].concat()).is_err());
    assert!(decode(&u32::MAX.to_le_bytes()).is_err());
    assert_eq!(
        super::encode_fields(&decode(&[0; 4]).unwrap(), 4).unwrap(),
        [0; 4]
    );
}

#[test]
fn linked_atom_identity_and_class_validation_reject_cross_kind_and_missing_targets() {
    let mut broken = RAW.to_vec();
    // The first self-reference follows the unsigned map count and signed key.
    let reference = 4 + 4 + 6 + "moLinkedAtomIdNode_c".len() + 20 + 8;
    broken[reference..reference + 2].copy_from_slice(&300u16.to_le_bytes());
    assert!(decode(&broken).is_err());
    let mut wrong_class = RAW.to_vec();
    wrong_class[14] = b'X';
    assert!(decode(&wrong_class).is_err());
    assert_eq!(
        super::encode_fields(&decode(RAW).unwrap(), 30000).unwrap(),
        RAW
    );
}

#[test]
fn recursive_atom_graphs_obey_all_operation_budgets() {
    let fields = decode(RAW).unwrap();
    for limits in [
        ModelArchiveLimits {
            maximum_bytes: RAW.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_fields: fields.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_entries: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_objects: 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
    ] {
        assert!(
            decode_limited(19000, RAW, limits, |reader, map| {
                super::configuration_atoms::linked_map(reader, map, "atoms")
            })
            .is_err()
        );
    }
}

#[test]
fn linked_atom_fields_are_owned_and_signed_edits_round_trip_without_mutating_inputs() {
    let mut input = RAW.to_vec();
    let original = decode(&input).unwrap();
    input.fill(0);
    let mut edited = original.clone();
    let field = edited
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.atoms[0].object.raw_08")
        .unwrap();
    field.value = ModelFieldValue::I32(i32::MAX);
    let bytes = super::encode_fields(&edited, 30000).unwrap();
    assert_eq!(decode(&bytes).unwrap(), edited);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), RAW);
}
