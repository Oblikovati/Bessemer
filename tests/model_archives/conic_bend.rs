use super::solid_reference_tests::decode_graph;
use super::{ModelField, ModelFieldValue, StoredDouble};
use crate::archive_codec::{ArchiveCursor, ObjectRecord};

const EMPTY: &[u8] = include_bytes!("../fixtures/conic-bend/empty.bin");
const RICH: &[u8] = include_bytes!("../fixtures/conic-bend/rich.bin");
const ALIASES: &[u8] = include_bytes!("../fixtures/conic-bend/aliases.bin");
const CONIC_WORDS: [i32; 6] = [i32::MIN, -1, 0, 1, 255, i32::MAX];
const CONIC_BITS: [u64; 16] = [
    0,
    0x8000000000000000,
    1,
    0x8000000000000001,
    0x7fefffffffffffff,
    0xffefffffffffffff,
    0x7ff0000000000000,
    0xfff0000000000000,
    0x7ff8000000001234,
    0xfff8000000004321,
    0x7ff0000000000001,
    0xfff0000000000001,
    0x0010000000000000,
    0x8010000000000000,
    0x3ff0000000000000,
    0xbff0000000000000,
];
const CONIC_FLOAT_PATHS: [&str; 8] = [
    "point_08[0]",
    "point_08[1]",
    "point_08[2]",
    "vector_20[0]",
    "vector_20[1]",
    "vector_20[2]",
    "raw_38",
    "raw_40",
];

fn conic_records() -> Vec<&'static [u8]> {
    let mut cursor = ArchiveCursor::new(include_bytes!("../fixtures/conic-bend/domains.bin"));
    assert_eq!(cursor.take(4).unwrap(), b"CBN1");
    assert_eq!(cursor.u32().unwrap(), 576);
    let records = (0..576)
        .map(|_| {
            let size = cursor.u32().unwrap() as usize;
            cursor.take(size).unwrap()
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

fn conic_field<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
}

fn conic_tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let ModelFieldValue::ObjectTag(tag) = &conic_field(fields, path).value else {
        panic!()
    };
    &tag.record
}

fn assert_conic_values(fields: &[ModelField], case: usize) {
    for (offset, label) in CONIC_FLOAT_PATHS.iter().enumerate() {
        let expected = CONIC_BITS[(case / 36 + offset) % 16];
        assert_eq!(
            conic_field(fields, &format!("root.object.{label}")).value,
            ModelFieldValue::F64(StoredDouble::from_bits(expected))
        );
    }
    assert_eq!(
        conic_field(fields, "root.object.raw_48").value,
        ModelFieldValue::I32(CONIC_WORDS[(case / 6) % 6])
    );
    assert_eq!(
        conic_field(fields, "root.object.raw_49").value,
        ModelFieldValue::I32(CONIC_WORDS[case % 6])
    );
}

#[test]
fn every_conic_numeric_combination_has_exactly_seventy_two_body_bytes_and_three_map_slots() {
    for (case, raw) in conic_records().into_iter().enumerate() {
        let (fields, slots) = decode_graph(19000, raw, false).unwrap();
        assert_eq!(slots, 3);
        assert_conic_values(&fields, case);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
        assert_eq!(super::encode_fields(&fields[1..], 72).unwrap().len(), 72);
    }
}

#[test]
fn noncanonical_signed_byte_backed_words_are_retained_without_boolean_conversion() {
    for raw in conic_records() {
        let fields = decode_graph(19000, raw, false).unwrap().0;
        let tail = super::encode_fields(&fields[9..], 8).unwrap();
        assert_eq!(tail, &raw[raw.len() - 8..]);
        assert_eq!(fields.len(), 11);
    }
}

#[test]
fn conic_schema_one_and_modern_version_contexts_are_explicit() {
    let raw = conic_records()[0];
    for version in [0x27cc, 19000, u32::MAX] {
        let fields = decode_graph(version, raw, false).unwrap().0;
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    }
    assert!(decode_graph(0x27cb, raw, false).is_err());
    for schema in [0u16, 2, 7, u16::MAX] {
        let mut changed = raw.to_vec();
        changed[2..4].copy_from_slice(&schema.to_le_bytes());
        let error = decode_graph(19000, &changed, false).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains(&schema.to_string()));
    }
}

#[test]
fn conic_domain_graphs_reject_every_proper_prefix_and_appended_byte() {
    for raw in conic_records() {
        for end in 0..raw.len() {
            assert!(
                decode_graph(19000, &raw[..end], false).is_err(),
                "prefix {end}"
            );
        }
        assert!(decode_graph(19000, &[raw, &[0]].concat(), false).is_err());
    }
}

#[test]
fn one_bend_containing_graphs_reconstruct_all_inherited_fields_and_maps() {
    for (raw, slots) in [(EMPTY, 7), (RICH, 19), (ALIASES, 27)] {
        let (fields, actual) = decode_graph(19000, raw, false).unwrap();
        assert_eq!(actual, slots);
        assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
    }
    let raw = include_bytes!("../fixtures/conic-bend/minimum-rich.bin");
    let fields = decode_graph(0x27cc, raw, false).unwrap().0;
    assert_eq!(super::encode_fields(&fields, raw.len()).unwrap(), raw);
}

#[test]
fn separate_one_bends_share_one_conic_definition_and_repeated_root_identity() {
    let fields = decode_graph(19000, ALIASES, false).unwrap().0;
    let first = conic_tag(&fields, "root.object.items[0].conic_ad0");
    let second = conic_tag(&fields, "root.object.items[1].conic_ad0");
    let ObjectRecord::New { index, class } = first else {
        panic!()
    };
    assert_eq!(class.schema, 1);
    assert!(matches!(second, ObjectRecord::Reference { index: other, .. } if index == other));
}

#[test]
fn repeated_roots_and_cached_conics_do_not_duplicate_their_saved_bodies() {
    let fields = decode_graph(19000, ALIASES, false).unwrap().0;
    assert_eq!(
        fields
            .iter()
            .filter(|field| field.path.ends_with("conic_ad0.point_08[0]"))
            .count(),
        1
    );
    assert!(matches!(
        conic_tag(&fields, "root.object.items[2]"),
        ObjectRecord::Reference { .. }
    ));
}

#[test]
fn populated_one_bends_reject_truncation_trailers_and_wrong_cached_conic_roles() {
    for raw in [EMPTY, RICH, ALIASES] {
        for end in 0..raw.len() {
            assert!(
                decode_graph(19000, &raw[..end], false).is_err(),
                "prefix {end}"
            );
        }
        assert!(decode_graph(19000, &[raw, &[0]].concat(), false).is_err());
    }
    let fields = decode_graph(19000, RICH, false).unwrap().0;
    let marker = conic_field(&fields, "root.object.reference_860").clone();
    let bytes = super::encode_fields(&[marker], 2).unwrap();
    let raw = super::manager_context_tests::replace_field(&fields, "root.object.conic_ad0", &bytes);
    let error = decode_graph(19000, &raw, false).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("conic_ad0"));
}

fn conic_in_context(
    raw: &[u8],
    context: super::reader::ModelArchiveContext,
    limits: super::ModelFieldLimits,
) -> crate::Result<Vec<ModelField>> {
    let mut reader = super::ModelDecoder::new_in_context(
        raw,
        crate::header::ArchiveVersion::new(19000),
        limits,
        context,
    )?;
    reader.object(
        "conic",
        &mut crate::archive_codec::ObjectReader::new(),
        super::ModelObjectRole::Any,
    )?;
    reader.finish()
}

#[test]
fn conic_data_rejects_unspecified_simulation_and_assembly_contexts() {
    use super::reader::ModelArchiveContext;
    for context in [
        ModelArchiveContext::Unspecified,
        ModelArchiveContext::SimulationEmbedded,
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::Assembly),
        ModelArchiveContext::OrdinaryAssemblyExternalXml(super::AssemblyRootKind::RoutingAssembly),
    ] {
        let error = conic_in_context(
            conic_records()[0],
            context,
            super::annotation_view_tests::LIMITS,
        )
        .unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn complete_conic_bends_obey_all_five_resource_limits() {
    use super::reader::ModelArchiveContext::OrdinaryPartExternalXml;
    let limits = super::annotation_view_tests::LIMITS;
    assert!(conic_in_context(RICH, OrdinaryPartExternalXml, limits).is_ok());
    let mut bounded = [limits; 5];
    bounded[0].maximum_bytes = RICH.len() - 1;
    bounded[1].maximum_fields = 3;
    bounded[2].maximum_entries = 0;
    bounded[3].maximum_depth = 0;
    bounded[4].maximum_objects = 0;
    for limit in bounded {
        assert!(conic_in_context(RICH, OrdinaryPartExternalXml, limit).is_err());
    }
}
