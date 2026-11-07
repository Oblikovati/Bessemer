use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::{ModelField, ModelFieldValue, ModelObjectRecord, StoredDouble};
use crate::Result;
use crate::archive_codec::ArchiveClass;

type AttributeFixture = (&'static [u8], &'static str, usize);
const OBJECT: &[u8] = include_bytes!("../fixtures/resolved-features/attribute-object.bin");
const CASES: &[AttributeFixture] = &[
    (
        OBJECT,
        include_str!("../fixtures/resolved-features/attribute-object-primitives.tsv"),
        149,
    ),
    (
        include_bytes!("../fixtures/resolved-features/attribute-definition.bin"),
        include_str!("../fixtures/resolved-features/attribute-definition-primitives.tsv"),
        52,
    ),
    (
        include_bytes!("../fixtures/resolved-features/attribute-base-0.bin"),
        include_str!("../fixtures/resolved-features/attribute-base-0-primitives.tsv"),
        20,
    ),
    (
        include_bytes!("../fixtures/resolved-features/attribute-scalar-0.bin"),
        include_str!("../fixtures/resolved-features/attribute-scalar-0-primitives.tsv"),
        22,
    ),
    (
        include_bytes!("../fixtures/resolved-features/attribute-scalar-255.bin"),
        include_str!("../fixtures/resolved-features/attribute-scalar-255-primitives.tsv"),
        23,
    ),
    (
        include_bytes!("../fixtures/resolved-features/attribute-string-0.bin"),
        include_str!("../fixtures/resolved-features/attribute-string-0-primitives.tsv"),
        26,
    ),
];

#[test]
fn complete_attributes_and_parameter_variants_match_independent_primitive_partitions() {
    for (bytes, rows, count) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, rows, *count);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), *bytes);
    }
    let fields = decode(19000, OBJECT).unwrap();
    assert!(!fields.iter().any(|field| {
        field
            .path
            .as_ref()
            .starts_with("root.object.model_feature.feature.feature_list")
    }));
}

#[test]
fn attribute_and_parameter_prefixes_extra_bytes_and_wrong_second_name_classes_reject() {
    for (bytes, _, _) in CASES {
        for length in 0..bytes.len() {
            assert!(decode(19000, &bytes[..length]).is_err());
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
    }
    let fields = decode(19000, CASES[4].0).unwrap();
    let wrong =
        super::manager_context_tests::replace_field(&fields, "root.object.name_40", &[2, 0]);
    assert!(decode(19000, &wrong).is_err());
}

fn identity(value: &ModelFieldValue) -> u32 {
    let ModelFieldValue::ObjectTag(tag) = value else {
        panic!("expected named pointer")
    };
    match &tag.record {
        ModelObjectRecord::New { index, .. } | ModelObjectRecord::Reference { index, .. } => {
            index.value()
        }
        _ => panic!("expected nonnull pointer"),
    }
}

#[test]
fn scalar_parameters_preserve_negative_values_optional_overrides_and_shared_names() {
    for index in [3, 4] {
        let fields = decode(19000, CASES[index].0).unwrap();
        let first = fields
            .iter()
            .find(|field| field.path.as_ref() == "root.object.node.name")
            .unwrap();
        let second = fields
            .iter()
            .find(|field| field.path.as_ref() == "root.object.name_40")
            .unwrap();
        assert_eq!(identity(&first.value), identity(&second.value));
        assert!(fields.iter().any(|field| field.value
            == ModelFieldValue::F64(StoredDouble::from_bits((-1.75f64).to_bits()))));
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.object.raw_override_58"),
            index == 4
        );
    }
}

fn decode_payload(bytes: &[u8], definition: &str) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        for name in ["moScalerParameter_c", "moCompFace_c", definition] {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
        }
        super::attribute_records::payload(reader, map)
    })
}

#[test]
fn attribute_definition_role_is_specific_while_parameter_aliases_remain_shared() {
    let bytes = [
        vec![0xff, 0xff, 2, 0, 0, 0],
        [1u16, 1, 2, 3, 2]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect(),
        (-123i32).to_le_bytes().to_vec(),
    ]
    .concat();
    let fields = decode_payload(&bytes, "moAttributeDef_c").unwrap();
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    assert!(decode_payload(&bytes, "moAttribute_c").is_err());
    assert!(decode_payload(&bytes, "moAttributeDef_c_suffix").is_err());
    assert!(decode_payload(&[0xff; 6], "moAttributeDef_c").is_err());
}

#[test]
fn attribute_parameter_edit_matches_independent_bytes_and_preserves_originals() {
    let mut input = OBJECT.to_vec();
    let original = decode(19000, &input).unwrap();
    input.fill(0);
    let mut fields = original.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.payload.parameters[0].raw_value_50")
        .unwrap()
        .value = ModelFieldValue::F64(StoredDouble::from_bits((-17.5f64).to_bits()));
    let expected = include_bytes!("../fixtures/resolved-features/attribute-object-edited.bin");
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
    assert_eq!(decode(19000, expected).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), OBJECT);
}
