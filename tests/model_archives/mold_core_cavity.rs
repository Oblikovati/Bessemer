use super::solid_reference_tests::decode_graph;
use super::{ModelField, ModelFieldValue};

const EMPTY: &[u8] = include_bytes!("../fixtures/mold-core-cavity/feature-0.bin");
const POPULATED: &[u8] = include_bytes!("../fixtures/mold-core-cavity/feature-1.bin");

const MINIMUM_EMPTY: &[u8] = include_bytes!("../fixtures/mold-core-cavity/feature-minimum-0.bin");
const MINIMUM_EMPTY_EDITED: &[u8] =
    include_bytes!("../fixtures/mold-core-cavity/feature-minimum-0-edited.bin");
const MINIMUM_POPULATED: &[u8] =
    include_bytes!("../fixtures/mold-core-cavity/feature-minimum-1.bin");
const MINIMUM_POPULATED_EDITED: &[u8] =
    include_bytes!("../fixtures/mold-core-cavity/feature-minimum-1-edited.bin");

fn mold_field<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
}

#[test]
fn complete_mold_features_preserve_the_owner_direct_lists_and_native_scalar_order() {
    for (raw, slots) in [(EMPTY, 7), (POPULATED, 19)] {
        let (fields, actual_slots) = decode_graph(19000, raw, true).unwrap();
        assert_eq!(actual_slots, slots);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        let owner = mold_field(&fields, "root.object.data_800.owner_08").clone();
        assert_eq!(super::encode_fields(&[owner], 2).unwrap(), [2, 0]);
        let word = mold_field(&fields, "root.object.data_800.raw_30");
        assert_eq!(word.value, ModelFieldValue::I32(-7));
        let setback = mold_field(&fields, "root.object.data_800.raw_34");
        assert_eq!(setback.value, ModelFieldValue::I32(i32::MIN));
    }
}

#[test]
fn populated_mold_direct_lists_share_one_component_without_outer_tags() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    for offset in ["70", "b8", "100"] {
        let prefix = format!("root.object.data_800.list_{offset}");
        let alias = mold_field(&fields, &format!("{prefix}[2]")).clone();
        assert_eq!(super::encode_fields(&[alias], 2).unwrap(), [10, 0]);
        let null = mold_field(&fields, &format!("{prefix}[1]")).clone();
        assert_eq!(super::encode_fields(&[null], 2).unwrap(), [0, 0]);
        let count = mold_field(&fields, &format!("{prefix}.count.count_dword"));
        assert_eq!(count.value, ModelFieldValue::U32(3));
    }
}

#[test]
fn mold_setback_edits_match_independent_literal_graphs_and_fresh_reversals() {
    for (raw, edited) in mold_graph_cases() {
        let original = decode_graph(19000, raw, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &original,
            "root.object.data_800.raw_34",
            &12345i32.to_le_bytes(),
        );
        assert_eq!(changed, edited);
        let fields = decode_graph(19000, edited, true).unwrap().0;
        let reverse = super::manager_context_tests::replace_field(
            &fields,
            "root.object.data_800.raw_34",
            &i32::MIN.to_le_bytes(),
        );
        assert_eq!(reverse, raw);
        assert_eq!(super::encode_fields(&original, raw.len()).unwrap(), raw);
    }
}

#[test]
fn mold_features_reject_every_prefix_appended_byte_and_old_context() {
    for raw in [EMPTY, POPULATED] {
        for end in 0..raw.len() {
            assert!(
                decode_graph(19000, &raw[..end], true).is_err(),
                "prefix {end}"
            );
        }
        assert!(decode_graph(19000, &[raw, &[0]].concat(), true).is_err());
        assert!(decode_graph(0x27cb, raw, true).is_err());
    }
}

fn mold_graph_cases() -> [(&'static [u8], &'static [u8]); 2] {
    [
        (
            EMPTY,
            include_bytes!("../fixtures/mold-core-cavity/feature-0-edited.bin").as_slice(),
        ),
        (
            POPULATED,
            include_bytes!("../fixtures/mold-core-cavity/feature-1-edited.bin").as_slice(),
        ),
    ]
}

#[test]
fn mold_raw_words_preserve_full_signed_values_without_native_setback_normalization() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    for path in ["root.object.data_800.raw_30", "root.object.data_800.raw_34"] {
        super::property_domains_3086_tests::assert_signed_property_domain(19000, &fields, path);
    }
}

#[test]
fn mold_declared_roles_reject_wrong_cached_owner_profile_data_and_dimension_classes() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    let roles = [
        ("data_800", 2u16),
        ("data_800.owner_08", 8),
        ("data_800.profile_50", 2),
        ("data_800.dimension_58", 2),
        ("data_800.dimension_68", 16),
    ];
    for (suffix, index) in roles {
        let path = format!("root.object.{suffix}");
        let raw = super::manager_context_tests::replace_field(&fields, &path, &index.to_le_bytes());
        let error = decode_graph(19000, &raw, true).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains(suffix.rsplit('.').next().unwrap()));
    }
}

fn mold_in_context(
    context: super::reader::ModelArchiveContext,
    limits: super::ModelFieldLimits,
) -> crate::Result<Vec<ModelField>> {
    let mut reader = super::ModelDecoder::new_in_context(
        POPULATED,
        crate::header::ArchiveVersion::new(19000),
        limits,
        context,
    )?;
    reader.object(
        "mold",
        &mut crate::archive_codec::ObjectReader::new(),
        super::ModelObjectRole::Any,
    )?;
    reader.finish()
}

#[test]
fn mold_enforces_byte_field_entry_depth_and_object_budgets_before_publication() {
    use super::reader::ModelArchiveContext::OrdinaryPartExternalXml;
    let limits = super::annotation_view_tests::LIMITS;
    assert!(mold_in_context(OrdinaryPartExternalXml, limits).is_ok());
    let mut bounded = [limits; 5];
    bounded[0].maximum_bytes = POPULATED.len() - 1;
    bounded[1].maximum_fields = 3;
    bounded[2].maximum_entries = 2;
    bounded[3].maximum_depth = 0;
    bounded[4].maximum_objects = 0;
    for limit in bounded {
        assert!(mold_in_context(OrdinaryPartExternalXml, limit).is_err());
    }
}

#[test]
fn mold_rejects_unspecified_simulation_and_both_assembly_contexts() {
    use super::reader::ModelArchiveContext;
    for context in [
        ModelArchiveContext::Unspecified,
        ModelArchiveContext::SimulationEmbedded,
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::Assembly),
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::RoutingAssembly),
    ] {
        let error = mold_in_context(context, super::annotation_view_tests::LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn mold_minimum_modern_version_matches_literal_inherited_layouts() {
    for (raw, edited, slots) in [
        (MINIMUM_EMPTY, MINIMUM_EMPTY_EDITED, 7),
        (MINIMUM_POPULATED, MINIMUM_POPULATED_EDITED, 19),
    ] {
        let (fields, actual_slots) = decode_graph(0x27cc, raw, true).unwrap();
        assert_eq!(actual_slots, slots);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.data_800.raw_34",
            &12345i32.to_le_bytes(),
        );
        assert_eq!(changed, edited);
        assert!(decode_graph(0x27cb, raw, true).is_err());
    }
}
