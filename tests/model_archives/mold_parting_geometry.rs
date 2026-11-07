use super::solid_reference_tests::decode_graph;
use super::{ModelField, ModelFieldValue, StoredDouble};

const EMPTY: &[u8] = include_bytes!("../fixtures/mold-parting-geometry/empty.bin");
const RICH: &[u8] = include_bytes!("../fixtures/mold-parting-geometry/rich.bin");
const CHILD: &[u8] = include_bytes!("../fixtures/mold-parting-geometry/child.bin");
const KERNEL: &[u8] = include_bytes!("../fixtures/mold-parting-geometry/kernel-child.bin");
const BRANCHES: [(&[u8], i32, i32, bool); 4] = [
    (RICH, 2, 2, false),
    (
        include_bytes!("../fixtures/mold-parting-geometry/allocated-skipped.bin"),
        1,
        2,
        false,
    ),
    (
        include_bytes!("../fixtures/mold-parting-geometry/unallocated-selected.bin"),
        2,
        1,
        false,
    ),
    (CHILD, 1, 1, true),
];

fn geometry_field<'a>(fields: &'a [ModelField], suffix: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|field| field.path.ends_with(suffix))
        .unwrap()
}

#[test]
fn geometry_graphs_reconstruct_every_byte_and_expected_map_slot() {
    for (raw, slots) in [(EMPTY, 5), (RICH, 17), (CHILD, 23), (KERNEL, 21)] {
        let (fields, actual_slots) = decode_graph(19000, raw, true).unwrap();
        assert_eq!(actual_slots, slots);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        let owner = geometry_field(&fields, "settings_7d8.owner_08").clone();
        assert_eq!(super::encode_fields(&[owner], 2).unwrap(), [2, 0]);
    }
}

#[test]
fn direct_empty_children_add_fourteen_bytes_without_an_object_map_slot() {
    let raw = include_bytes!("../fixtures/mold-parting-geometry/empty-child.bin");
    let (fields, slots) = decode_graph(19000, raw, true).unwrap();
    assert_eq!(slots, 5);
    assert_eq!(raw.len(), EMPTY.len() + 14);
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    assert_eq!(
        geometry_field(&fields, "child_178.list_10.count.count_word").value,
        ModelFieldValue::U16(0)
    );
}

#[test]
fn component_surface_callers_admit_complete_parting_geometry_and_preserve_its_owner() {
    let raw = include_bytes!("../fixtures/mold-parting-geometry/component-child.bin");
    let (fields, slots) = decode_graph(19000, raw, true).unwrap();
    assert_eq!(slots, 25);
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    let owner = geometry_field(&fields, "settings_7d8.owner_08").clone();
    assert_eq!(super::encode_fields(&[owner], 2).unwrap(), [4, 0]);
    assert_eq!(
        geometry_field(&fields, "root.object.raw_100").value,
        ModelFieldValue::I32(-37)
    );
}

#[test]
fn constructor_method_and_saved_selector_independently_gate_the_direct_child() {
    for (raw, method, selector, has_child) in BRANCHES {
        let fields = decode_graph(19000, raw, true).unwrap().0;
        assert_eq!(
            geometry_field(&fields, "method_7d0").value,
            ModelFieldValue::I32(method)
        );
        assert_eq!(
            geometry_field(&fields, "settings_7d8.selector_170").value,
            ModelFieldValue::I32(selector)
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.contains(".child_178.")),
            has_child
        );
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    }
}

#[test]
fn child_and_connected_arrays_keep_generic_owner_aliases() {
    let fields = decode_graph(19000, CHILD, true).unwrap().0;
    for (suffix, index) in [
        ("settings_7d8.owner_08", 2u16),
        ("edges_7b0[2]", 10),
        ("child_178.list_30[0]", 2),
        ("child_178.connected_80.list_90[0]", 2),
        ("child_178.connected_80.synch_28", 20),
        ("child_178.direction_88", 16),
    ] {
        let field = geometry_field(&fields, suffix).clone();
        assert_eq!(
            super::encode_fields(&[field], 2).unwrap(),
            index.to_le_bytes()
        );
    }
}

#[test]
fn connected_dwords_keep_unsigned_high_bits_and_the_scalar_stays_signed() {
    let fields = decode_graph(19000, CHILD, true).unwrap().0;
    assert_eq!(
        geometry_field(&fields, "connected_80.dwords_b0[2]").value,
        ModelFieldValue::U32(u32::MAX)
    );
    assert_eq!(
        geometry_field(&fields, "connected_80.raw_80").value,
        ModelFieldValue::I32(i32::MIN)
    );
}

#[test]
fn raw_scalar_order_preserves_signed_words_and_double_bits() {
    for raw in [EMPTY, CHILD] {
        let fields = decode_graph(19000, raw, true).unwrap().0;
        for (suffix, value) in GEOMETRY_SETTINGS_WORDS {
            assert_eq!(
                geometry_field(&fields, &format!("settings_7d8.{suffix}")).value,
                ModelFieldValue::I32(value)
            );
        }
        assert_eq!(
            geometry_field(&fields, "settings_7d8.raw_88").value,
            ModelFieldValue::F64(StoredDouble::from_bits(0x8000000000000000))
        );
    }
}

const GEOMETRY_SETTINGS_WORDS: [(&str, i32); 6] = [
    ("raw_b4", i32::MIN),
    ("raw_c0", i32::MAX),
    ("raw_38", -7),
    ("raw_b0", 1),
    ("raw_3c", -9),
    ("raw_124", -31),
];

#[test]
fn nonpositive_geometry_vector_counts_remain_saved_without_elements() {
    let fields = decode_graph(19000, EMPTY, true).unwrap().0;
    assert_eq!(
        geometry_field(&fields, "integers_58.count").value,
        ModelFieldValue::I32(i32::MIN)
    );
    assert!(
        !fields
            .iter()
            .any(|field| field.path.contains("integers_58["))
    );
}

#[test]
fn inline_sheets_ignore_backend_markers_and_require_an_explicit_strict_kernel_context() {
    let fields = decode_graph(19000, KERNEL, true).unwrap().0;
    assert_eq!(
        geometry_field(&fields, "surface.archive_backend_marker").value,
        ModelFieldValue::I32(1)
    );
    assert_eq!(
        fields
            .iter()
            .filter(|field| matches!(field.value, ModelFieldValue::Kernel(_)))
            .count(),
        1
    );
    assert_eq!(
        decode_graph(19000, KERNEL, false).unwrap_err().class,
        crate::FailureClass::Missing
    );
}

#[test]
fn suppressed_extra_bodies_retain_noncanonical_bytes_without_an_attribute_list() {
    let fields = decode_graph(19000, KERNEL, true).unwrap().0;
    assert_eq!(
        geometry_field(&fields, "surface.extra_body.suppressed").value,
        ModelFieldValue::U8(201)
    );
    assert!(
        !fields
            .iter()
            .any(|field| field.path.contains("extra_body_attributes"))
    );
}

#[test]
fn minimum_modern_geometry_has_its_own_exact_inherited_gates() {
    let raw = include_bytes!("../fixtures/mold-parting-geometry/minimum-child.bin");
    let (fields, slots) = decode_graph(0x27cc, raw, true).unwrap();
    assert_eq!(slots, 23);
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    assert!(
        !fields
            .iter()
            .any(|field| field.path.contains("word_6e8") || field.path.contains("raw_words_748"))
    );
    assert!(decode_graph(0x27cb, raw, true).is_err());
}

#[test]
fn geometry_rejects_every_truncated_prefix_trailing_byte_and_old_context() {
    for raw in [EMPTY, RICH, CHILD, KERNEL] {
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

#[test]
fn geometry_and_child_raw_words_cover_the_full_signed_domain() {
    let fields = decode_graph(19000, CHILD, true).unwrap().0;
    for label in [
        "raw_b4", "raw_c0", "raw_38", "raw_b0", "raw_3c", "raw_124", "raw_144",
    ] {
        super::property_domains_3086_tests::assert_signed_property_domain(
            19000,
            &fields,
            &format!("root.object.settings_7d8.{label}"),
        );
    }
    super::property_domains_3086_tests::assert_signed_property_domain(
        19000,
        &fields,
        "root.object.settings_7d8.child_178.connected_80.raw_80",
    );
}

#[test]
fn geometry_double_fields_preserve_every_ieee_category_and_payload() {
    let fields = decode_graph(19000, CHILD, true).unwrap().0;
    for label in ["raw_88", "raw_90", "raw_98", "raw_a0", "raw_b8", "raw_118"] {
        super::property_domains_3086_tests::assert_stored_double_domain(
            19000,
            &fields,
            &format!("root.object.settings_7d8.{label}"),
        );
    }
    for index in 0..10 {
        super::property_domains_3086_tests::assert_stored_double_domain(
            19000,
            &fields,
            &format!("root.object.settings_7d8.raw_c8[{index}]"),
        );
    }
}

#[test]
fn geometry_declared_pointer_roles_reject_loaded_objects_of_the_wrong_class() {
    let fields = decode_graph(19000, CHILD, true).unwrap().0;
    for suffix in GEOMETRY_ROLE_REJECTIONS {
        let path = format!("root.object.settings_7d8.{suffix}");
        let raw = super::manager_context_tests::replace_field(&fields, &path, &10u16.to_le_bytes());
        let error = decode_graph(19000, &raw, true).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains(suffix.rsplit('.').next().unwrap()),
            "{suffix}: {error}"
        );
    }
}

const GEOMETRY_ROLE_REJECTIONS: [&str; 10] = [
    "owner_08",
    "dimension_10",
    "component_78",
    "component_48",
    "direction_a8",
    "child_178.wrapper_70",
    "child_178.synch_78",
    "child_178.connected_80",
    "child_178.direction_88",
    "child_178.connected_80.synch_28",
];

#[test]
fn geometry_positive_counts_are_bounded_before_arrays_can_publish() {
    let fields = decode_graph(19000, CHILD, true).unwrap().0;
    for suffix in GEOMETRY_COUNT_REJECTIONS {
        let path = format!("root.object.{suffix}");
        let raw =
            super::manager_context_tests::replace_field(&fields, &path, &i32::MAX.to_le_bytes());
        assert_eq!(
            decode_graph(19000, &raw, true).unwrap_err().class,
            crate::FailureClass::Invalid
        );
    }
}

const GEOMETRY_COUNT_REJECTIONS: [&str; 9] = [
    "surface.sheet_count",
    "surface.raw_words_748.count",
    "settings_7d8.integers_58.count",
    "edges_7b0.count.count_dword",
    "settings_7d8.edges_18.count.count_dword",
    "settings_7d8.child_178.list_10.count.count_dword",
    "settings_7d8.child_178.list_30.count.count_dword",
    "settings_7d8.child_178.list_50.count.count_dword",
    "settings_7d8.child_178.connected_80.dwords_b0.count.count_dword",
];

fn geometry_in_context(
    context: super::reader::ModelArchiveContext,
    limits: super::ModelFieldLimits,
) -> crate::Result<Vec<ModelField>> {
    let mut reader = super::ModelDecoder::new_in_context(
        CHILD,
        crate::header::ArchiveVersion::new(19000),
        limits,
        context,
    )?;
    reader.object(
        "geometry",
        &mut crate::archive_codec::ObjectReader::new(),
        super::ModelObjectRole::Any,
    )?;
    reader.finish()
}

#[test]
fn geometry_limits_bound_all_five_resource_axes() {
    use super::reader::ModelArchiveContext::OrdinaryPartExternalXml;
    let limits = super::annotation_view_tests::LIMITS;
    assert!(geometry_in_context(OrdinaryPartExternalXml, limits).is_ok());
    let mut bounded = [limits; 5];
    bounded[0].maximum_bytes = CHILD.len() - 1;
    bounded[1].maximum_fields = 3;
    bounded[2].maximum_entries = 2;
    bounded[3].maximum_depth = 0;
    bounded[4].maximum_objects = 0;
    for limit in bounded {
        assert!(geometry_in_context(OrdinaryPartExternalXml, limit).is_err());
    }
}

#[test]
fn geometry_excludes_unspecified_simulation_and_both_assembly_contexts() {
    use super::reader::ModelArchiveContext;
    for context in [
        ModelArchiveContext::Unspecified,
        ModelArchiveContext::SimulationEmbedded,
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::Assembly),
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::RoutingAssembly),
    ] {
        let error = geometry_in_context(context, super::annotation_view_tests::LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}
