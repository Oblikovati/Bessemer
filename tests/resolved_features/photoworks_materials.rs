use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelFieldValue, solid_reference_tests::decode_graph};
use crate::archive_codec::ObjectRecord;
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/photoworks-materials/cases.bin"),
        b"PWM1",
        24,
    )
}
#[test]
fn photoworks_properties_match_independent_primitives_slots_and_reconstruction() {
    assert_complete_records(records());
}
#[test]
fn photoworks_properties_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}
#[test]
fn photoworks_minimum_properties_keep_original_zero_defaults_and_601_bytes() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        assert_eq!(bytes.len(), 601 + 6 + b"moPWMaterialProperties_c".len());
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for (path, value) in [
            ("raw_20", ModelFieldValue::I32(0)),
            ("raw_28", ModelFieldValue::U32(0)),
        ] {
            assert_eq!(
                fields
                    .iter()
                    .find(|f| f.path.ends_with(path))
                    .unwrap()
                    .value,
                value
            );
        }
    }
}
#[test]
fn photoworks_nested_lists_keep_cached_material_self_references_and_unit_aliases() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 3 | 5)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ModelFieldValue::ObjectTag(root) = &fields[0].value else {
            panic!("expected root");
        };
        let ObjectRecord::New { index, .. } = &root.record else {
            panic!("expected new material");
        };
        for list in ["list_08", "list_20"] {
            let path = format!("root.object.{list}.items[1]");
            let ModelFieldValue::ObjectTag(alias) = &fields
                .iter()
                .find(|f| f.path.as_ref() == path)
                .unwrap()
                .value
            else {
                panic!("expected cached pointer");
            };
            assert!(
                matches!(&alias.record, ObjectRecord::Reference{index: found,class} if *found==*index && class.name=="moPWMaterialProperties_c")
            );
        }
    }
}

fn decode_manager(bytes: &[u8]) -> crate::Result<(Vec<super::ModelField>, usize)> {
    let mut reader = super::ModelDecoder::new_in_context(
        bytes,
        crate::header::ArchiveVersion::new(19000),
        super::annotation_view_tests::LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = crate::archive_codec::ObjectReader::new();
    reader.scope("manager", |reader| {
        super::photoworks_materials::manager(reader, &mut map)
    })?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}
const DIRECT_MANAGERS: &[(&[u8], &str, usize, usize)] = &[
    (
        include_bytes!("../fixtures/photoworks-materials/manager-0.bin"),
        include_str!("../fixtures/photoworks-materials/manager-0.tsv"),
        3,
        1,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/manager-1.bin"),
        include_str!("../fixtures/photoworks-materials/manager-1.tsv"),
        111,
        3,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/manager-2.bin"),
        include_str!("../fixtures/photoworks-materials/manager-2.tsv"),
        262,
        9,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/manager-3.bin"),
        include_str!("../fixtures/photoworks-materials/manager-3.tsv"),
        123,
        3,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/manager-4.bin"),
        include_str!("../fixtures/photoworks-materials/manager-4.tsv"),
        4,
        1,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/manager-5.bin"),
        include_str!("../fixtures/photoworks-materials/manager-5.tsv"),
        262,
        7,
    ),
];
#[test]
fn photoworks_direct_managers_match_independent_oracles_and_exact_slots() {
    for (bytes, oracle, reads, slots) in DIRECT_MANAGERS {
        let (fields, actual_slots) = decode_manager(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *reads);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), *bytes);
        assert_eq!(actual_slots, *slots);
    }
    assert_eq!(DIRECT_MANAGERS[0].0.len(), 10);
    assert_eq!(DIRECT_MANAGERS[0].3, 1);
}
#[test]
fn photoworks_direct_managers_reject_every_prefix_extra_byte_null_and_oversized_count() {
    for (bytes, _, _, _) in DIRECT_MANAGERS {
        for end in 0..bytes.len() {
            assert!(decode_manager(&bytes[..end]).is_err());
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(decode_manager(&extra).is_err());
    }
    for suffix in [&[1, 0, 0, 0][..], &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]] {
        let mut invalid = vec![0; 8];
        invalid.extend_from_slice(suffix);
        assert!(decode_manager(&invalid).is_err());
    }
}
const OWNED_MANAGERS: &[(&[u8], &str, usize)] = &[
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-0.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-0.tsv"),
        507,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-1.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-1.tsv"),
        615,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-2.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-2.tsv"),
        766,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-3.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-3.tsv"),
        627,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-4.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-4.tsv"),
        508,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-5.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-5.tsv"),
        766,
    ),
    (
        include_bytes!("../fixtures/photoworks-materials/owned-0-2-edited.bin"),
        include_str!("../fixtures/photoworks-materials/owned-0-2-edited.tsv"),
        766,
    ),
];
#[test]
fn photoworks_complete_managers_match_independent_oracles_and_reconstruction() {
    for (bytes, oracle, reads) in OWNED_MANAGERS {
        let parsed = super::manager_context_tests::decode(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&parsed.fields, bytes, oracle, *reads);
        assert_eq!(super::encode_fields(&parsed.fields, 30000).unwrap(), *bytes);
    }
}
#[test]
fn photoworks_owned_edit_matches_the_independent_material_golden() {
    let mut parsed = super::manager_context_tests::decode(OWNED_MANAGERS[2].0).unwrap();
    let selected = parsed
        .fields
        .iter_mut()
        .find(|f| f.path.ends_with(".optional_data_608.materials[0].raw_20"))
        .unwrap();
    selected.value = ModelFieldValue::I32(12345);
    assert_eq!(
        super::encode_fields(&parsed.fields, 30000).unwrap(),
        OWNED_MANAGERS[6].0
    );
}
#[test]
fn photoworks_owned_managers_reject_every_prefix_and_appended_byte() {
    for (bytes, _, _) in OWNED_MANAGERS {
        for end in 0..bytes.len() {
            assert!(super::manager_context_tests::decode(&bytes[..end]).is_err());
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(super::manager_context_tests::decode(&extra).is_err());
    }
}

#[test]
fn photoworks_nested_lists_reject_fresh_unrelated_classes_before_their_bodies() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let name = b"moNodeName_c";
    let mut replacement = vec![1, 0, 0xff, 0xff, 1, 0];
    replacement.extend_from_slice(&(name.len() as u16).to_le_bytes());
    replacement.extend_from_slice(name);
    replacement.push(0);
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.list_08.count.count_word",
        &replacement,
    );
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("expected fresh class rejection");
    };
    assert!(error.cause.contains("moNodeName_c") && error.cause.contains("moUnitComponent_c"));
}
#[test]
fn photoworks_direct_manager_requires_nonnull_materials_of_the_exact_class() {
    let mut raw = vec![0; 8];
    raw.extend_from_slice(&[1, 0, 0, 0]);
    let Err(error) = decode_manager(&raw) else {
        panic!("expected null rejection");
    };
    assert!(error.cause.contains("nonnull"));
    raw.truncate(10);
    let name = b"moUnitComponent_c";
    raw.extend_from_slice(&[0xff, 0xff, 1, 0]);
    raw.extend_from_slice(&(name.len() as u16).to_le_bytes());
    raw.extend_from_slice(name);
    raw.extend_from_slice(&[0; 3]);
    let Err(error) = decode_manager(&raw) else {
        panic!("expected class rejection");
    };
    assert!(
        error.cause.contains("moUnitComponent_c")
            && error.cause.contains("moPWMaterialProperties_c")
    );
}
