use super::solid_reference_tests::decode_graph;
use super::{ModelField, ModelFieldValue, StoredDouble};

const EMPTY: &[u8] = include_bytes!("../fixtures/mold-parting-line/feature-0-0.bin");
const POPULATED: &[u8] = include_bytes!("../fixtures/mold-parting-line/feature-1-2.bin");
const NEGATIVE: &[u8] = include_bytes!("../fixtures/mold-parting-line/feature-1--2147483648.bin");

fn line_field<'a>(fields: &'a [ModelField], suffix: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|field| field.path.ends_with(suffix))
        .unwrap()
}

#[test]
fn complete_parting_lines_retain_every_byte_map_slot_and_non_offset_flag_order() {
    for (raw, slots) in [(EMPTY, 7), (POPULATED, 21), (NEGATIVE, 21)] {
        let (fields, actual_slots) = decode_graph(19000, raw, true).unwrap();
        assert_eq!(actual_slots, slots);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        assert_line_settings_values(&fields);
    }
}

#[test]
fn parting_line_elements_keep_generic_owner_references_and_shared_direct_list_targets() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    for (suffix, index) in [
        ("data_800.owner_08", 2u16),
        ("list_30[0].reference_10", 2),
        ("list_30[0].reference_18", 14),
        ("list_108[0]", 16),
        ("list_108[1]", 16),
        ("list_188[0]", 16),
    ] {
        let field = line_field(&fields, suffix).clone();
        assert_eq!(
            super::encode_fields(&[field], 2).unwrap(),
            index.to_le_bytes()
        );
    }
}

#[test]
fn parting_line_nonpositive_body_counts_remain_saved_without_consuming_frames() {
    for (raw, expected) in [(EMPTY, 0), (NEGATIVE, i32::MIN)] {
        let fields = decode_graph(19000, raw, true).unwrap().0;
        assert_eq!(
            line_field(&fields, "bodies_af4.count").value,
            ModelFieldValue::I32(expected)
        );
        assert!(
            !fields
                .iter()
                .any(|field| field.path.contains(".bodies_af4["))
        );
        assert_eq!(
            line_field(&fields, "raw_af8").value,
            ModelFieldValue::I32(-11)
        );
    }
}

#[test]
fn parting_line_prefixes_and_trailing_bytes_never_publish_partial_graphs() {
    for raw in [EMPTY, POPULATED, NEGATIVE] {
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

fn assert_line_settings_values(fields: &[ModelField]) {
    for (suffix, value) in [
        ("data_800.raw_1d0", -17),
        ("data_800.raw_1d8", -31),
        ("data_800.raw_1e4", i32::MIN),
        ("data_800.raw_1e0", i32::MAX),
        ("data_800.raw_1dc", -37),
        ("data_800.raw_1e8", 123),
    ] {
        assert_eq!(
            line_field(fields, suffix).value,
            ModelFieldValue::I32(value)
        );
    }
    assert_eq!(
        line_field(fields, "data_800.raw_18").value,
        ModelFieldValue::F64(StoredDouble::from_bits(0x8000000000000000))
    );
}

#[test]
fn parting_line_suppressed_frames_retain_each_noncanonical_byte() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    assert_eq!(
        line_field(&fields, "bodies_af4[0].suppressed").value,
        ModelFieldValue::U8(195)
    );
    assert_eq!(
        line_field(&fields, "bodies_af4[1].suppressed").value,
        ModelFieldValue::U8(201)
    );
}

#[test]
fn parting_line_positive_kernel_frames_require_schema_and_retain_exact_compression() {
    let raw = include_bytes!("../fixtures/mold-parting-line/feature-serialized.bin");
    let (fields, slots) = decode_graph(19000, raw, true).unwrap();
    assert_eq!(slots, 21);
    let frames: Vec<_> = fields
        .iter()
        .filter_map(|field| match &field.value {
            ModelFieldValue::Kernel(frame) => Some(frame),
            _ => None,
        })
        .collect();
    assert_eq!(frames.len(), 2);
    assert!(frames.iter().all(|frame| frame.record_count() == 1));
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    assert_eq!(
        decode_graph(19000, raw, false).unwrap_err().class,
        crate::FailureClass::Missing
    );
}

#[test]
fn parting_line_minimum_modern_version_matches_the_independent_inherited_layout() {
    let raw = include_bytes!("../fixtures/mold-parting-line/feature-minimum.bin");
    let (fields, slots) = decode_graph(0x27cc, raw, true).unwrap();
    assert_eq!(slots, 21);
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    assert!(decode_graph(0x27cb, raw, true).is_err());
}

fn line_in_context(
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
        "line",
        &mut crate::archive_codec::ObjectReader::new(),
        super::ModelObjectRole::Any,
    )?;
    reader.finish()
}

#[test]
fn parting_line_budgets_bound_all_five_resource_axes_before_publication() {
    use super::reader::ModelArchiveContext::OrdinaryPartExternalXml;
    let limits = super::annotation_view_tests::LIMITS;
    assert!(line_in_context(OrdinaryPartExternalXml, limits).is_ok());
    let mut bounded = [limits; 5];
    bounded[0].maximum_bytes = POPULATED.len() - 1;
    bounded[1].maximum_fields = 3;
    bounded[2].maximum_entries = 2;
    bounded[3].maximum_depth = 0;
    bounded[4].maximum_objects = 0;
    for limit in bounded {
        assert!(line_in_context(OrdinaryPartExternalXml, limit).is_err());
    }
}

#[test]
fn parting_line_rejects_unspecified_simulation_and_both_assembly_contexts() {
    use super::reader::ModelArchiveContext;
    for context in [
        ModelArchiveContext::Unspecified,
        ModelArchiveContext::SimulationEmbedded,
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::Assembly),
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::RoutingAssembly),
    ] {
        let error = line_in_context(context, super::annotation_view_tests::LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn parting_line_declared_roles_reject_wrong_cached_settings_owner_direction_body_and_split_types() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    for (suffix, index) in LINE_ROLE_REJECTIONS {
        let raw = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{suffix}"),
            &index.to_le_bytes(),
        );
        let error = decode_graph(19000, &raw, true).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(
            error.cause.contains(suffix.rsplit('.').next().unwrap()),
            "{suffix}: {error}"
        );
    }
}

#[test]
fn parting_line_counts_reject_unbounded_positive_bodies_and_each_direct_list() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    for suffix in [
        "bodies_af4.count",
        "data_800.list_78.count.count_dword",
        "data_800.list_c0.count.count_dword",
        "data_800.list_30.count.count_dword",
        "data_800.list_108.count.count_dword",
        "data_800.list_188.count.count_dword",
    ] {
        let raw = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.{suffix}"),
            &i32::MAX.to_le_bytes(),
        );
        let error = decode_graph(19000, &raw, true).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains(&i32::MAX.to_string()));
    }
}

#[test]
fn parting_line_settings_preserve_signed_flag_boundaries_and_all_ieee_categories() {
    let fields = decode_graph(19000, POPULATED, true).unwrap().0;
    for label in ["1d0", "1d8", "1e4", "1e0", "1dc", "1e8"] {
        let path = format!("root.object.data_800.raw_{label}");
        super::property_domains_3086_tests::assert_signed_property_domain(19000, &fields, &path);
    }
    super::property_domains_3086_tests::assert_stored_double_domain(
        19000,
        &fields,
        "root.object.data_800.raw_18",
    );
}

const LINE_ROLE_REJECTIONS: [(&str, u16); 6] = [
    ("data_800", 2u16),
    ("data_800.owner_08", 8),
    ("data_800.direction_10", 10),
    ("data_800.body_28", 10),
    ("data_800.list_30[0].split_info_30", 14),
    ("data_800.list_30[0].split_info_30.face_08", 10),
];
