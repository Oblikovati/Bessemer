use super::solid_reference_tests::decode_graph;

const PARENT: &[u8] = include_bytes!("../fixtures/sheet-material-ranges/parent.bin");
const RANGES: &[u8] = include_bytes!("../fixtures/sheet-material-ranges/ranges.bin");
const EMPTY: &[u8] = include_bytes!("../fixtures/sheet-material-ranges/empty.bin");

#[test]
fn material_ranges_preserve_literal_direct_records_and_all_method_octets() {
    for version in [0x27cc, 0x2bae, 19000, u32::MAX] {
        let (fields, slots) = decode_graph(version, PARENT, true).unwrap();
        assert_eq!(slots, 3);
        assert_eq!(super::encode_fields(&fields, PARENT.len()).unwrap(), PARENT);
        assert_literal_ranges(&fields);
    }
}

#[test]
fn empty_material_is_complete_and_every_parent_prefix_is_rejected() {
    let fields = decode_graph(19000, EMPTY, true).unwrap().0;
    assert_eq!(super::encode_fields(&fields, EMPTY.len()).unwrap(), EMPTY);
    for raw in [PARENT, EMPTY] {
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
fn material_range_count_bits_are_zero_extended_and_bounded_before_records() {
    let offset = EMPTY.len() - 4;
    for count in [1u32, i32::MAX as u32, 0x80000000, u32::MAX] {
        let mut raw = EMPTY.to_vec();
        raw[offset..].copy_from_slice(&count.to_le_bytes());
        let error = decode_graph(19000, &raw, true).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains(&count.to_string()));
        assert!(error.cause.contains("37"));
    }
}

fn assert_literal_ranges(fields: &[super::ModelField]) {
    let labels = [
        "raw_08", "raw_10", "raw_18", "raw_20", "raw_28", "raw_2c", "raw_30",
    ];
    for (index, expected) in RANGES.as_chunks::<37>().0.iter().enumerate() {
        let prefix = format!("root.object.ranges[{index}].");
        let row: Vec<_> = fields
            .iter()
            .filter(|field| field.path.starts_with(&prefix))
            .cloned()
            .collect();
        assert_eq!(row.len(), 7);
        for (field, label) in row.iter().zip(labels) {
            assert_eq!(field.path.as_ref(), format!("{prefix}{label}"));
        }
        assert_eq!(super::encode_fields(&row, 37).unwrap(), expected);
    }
}

fn material_in_context(
    context: super::reader::ModelArchiveContext,
    limits: super::ModelFieldLimits,
) -> crate::Result<Vec<super::ModelField>> {
    let mut reader = super::ModelDecoder::new_in_context(
        PARENT,
        crate::header::ArchiveVersion::new(19000),
        limits,
        context,
    )?;
    reader.object(
        "material",
        &mut crate::archive_codec::ObjectReader::new(),
        super::ModelObjectRole::Any,
    )?;
    reader.finish()
}

#[test]
fn material_parameters_enforce_byte_field_entry_depth_and_object_budgets() {
    use super::reader::ModelArchiveContext::OrdinaryPartExternalXml;
    let limits = super::annotation_view_tests::LIMITS;
    assert!(material_in_context(OrdinaryPartExternalXml, limits).is_ok());
    let mut bounded = [limits; 5];
    bounded[0].maximum_bytes = PARENT.len() - 1;
    bounded[1].maximum_fields = 100;
    bounded[2].maximum_entries = 255;
    bounded[3].maximum_depth = 0;
    bounded[4].maximum_objects = 0;
    for limit in bounded {
        assert!(material_in_context(OrdinaryPartExternalXml, limit).is_err());
    }
}

#[test]
fn material_parameters_reject_unspecified_simulation_and_both_assembly_contexts() {
    use super::reader::ModelArchiveContext;
    for context in [
        ModelArchiveContext::Unspecified,
        ModelArchiveContext::SimulationEmbedded,
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::Assembly),
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::RoutingAssembly),
    ] {
        let error = material_in_context(context, super::annotation_view_tests::LIMITS).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn material_ranges_keep_duplicate_direct_records_without_map_entries() {
    let mut raw = EMPTY.to_vec();
    let offset = raw.len() - 4;
    raw[offset..].copy_from_slice(&2i32.to_le_bytes());
    raw.extend_from_slice(&RANGES[..37]);
    raw.extend_from_slice(&RANGES[..37]);
    let (fields, slots) = decode_graph(19000, &raw, true).unwrap();
    assert_eq!(slots, 3);
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    let rows: Vec<_> = fields
        .iter()
        .filter(|field| field.path.contains(".ranges["))
        .cloned()
        .collect();
    assert_eq!(rows.len(), 14);
    assert_eq!(super::encode_fields(&rows[..7], 37).unwrap(), RANGES[..37]);
    assert_eq!(super::encode_fields(&rows[7..], 37).unwrap(), RANGES[..37]);
}
