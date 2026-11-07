use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, encode_fields};
use crate::archive_codec::ArchiveCursor;
use crate::header::ArchiveVersion;
type VisualMapRecord = (u32, u8, &'static [u8], &'static str);
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
fn records() -> Vec<VisualMapRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/configuration-visual-maps/cases.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"DVM1");
    let count = cursor.u32().unwrap();
    let mut records = Vec::new();
    for _ in 0..count {
        records.push(visual_map_record(&mut cursor));
    }
    assert_eq!(cursor.remaining(), 0);
    assert_eq!(count, 54);
    records
}
fn visual_map_record(cursor: &mut ArchiveCursor<'static>) -> VisualMapRecord {
    let version = cursor.u32().unwrap();
    let style = cursor.u8().unwrap();
    let width = cursor.u32().unwrap();
    let size = cursor.u32().unwrap();
    let bytes = cursor.take(width as usize).unwrap();
    let oracle = std::str::from_utf8(cursor.take(size as usize).unwrap()).unwrap();
    (version, style, bytes, oracle)
}
fn selected_visual_values(fields: &[ModelField], suffix: &str) -> Vec<ModelFieldValue> {
    fields
        .iter()
        .filter(|f| f.path.ends_with(suffix))
        .map(|f| f.value.clone())
        .collect()
}
fn decode(version: u32, bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    reader.scope("configuration", super::configuration_visual_maps::read)?;
    reader.finish()
}
#[test]
fn configuration_visual_maps_match_independent_primitives_without_object_tags() {
    for (version, _, bytes, oracle) in records() {
        let fields = decode(version, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        assert!(
            !fields
                .iter()
                .any(|f| matches!(f.value, ModelFieldValue::ObjectTag(_)))
        );
    }
}
#[test]
fn configuration_visual_maps_reject_every_prefix_appended_byte_and_wrong_context() {
    for (version, _, bytes, _) in records() {
        for end in 0..bytes.len() {
            assert!(decode(version, &bytes[..end]).is_err());
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing).is_err());
    }
    let (_, _, bytes, _) = records()[0];
    let mut reader = ModelDecoder::new(bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    assert!(super::configuration_visual_maps::read(&mut reader).is_err());
    assert!(decode(0x27cb, bytes).is_err());
}
#[test]
fn configuration_visual_maps_preserve_negative_counts_duplicate_keys_and_order() {
    for (version, style, bytes, _) in records() {
        let fields = decode(version, bytes).unwrap();
        let count = [-1, 0, 1, 2, i32::MIN, 3][usize::from(style)];
        assert_eq!(fields[0].value, ModelFieldValue::I32(count));
        let keys = selected_visual_values(&fields, ".key");
        let key = if style == 3 { i32::MIN } else { i32::MAX };
        assert_eq!(keys, vec![ModelFieldValue::I32(key); count.max(0) as usize]);
        let colors = selected_visual_values(&fields, ".visual.wire_color");
        assert_eq!(
            colors,
            (0..count.max(0) as u32)
                .map(ModelFieldValue::U32)
                .collect::<Vec<_>>()
        );
    }
}
#[test]
fn configuration_visual_maps_bound_counts_before_reading_direct_entries() {
    for declared in [1_i32, i32::MAX] {
        let bytes = declared.to_le_bytes();
        let Err(error) = decode(19000, &bytes) else {
            panic!("expected count failure");
        };
        assert!(error.cause.contains("count") || error.cause.contains("remaining"));
    }
    assert_eq!(decode(19000, &(-1_i32).to_le_bytes()).unwrap().len(), 1);
}

const MANAGERS: &[(&[u8], &str, usize)] = &[
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-0.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-0.tsv"),
        504,
    ),
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-1.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-1.tsv"),
        504,
    ),
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-2.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-2.tsv"),
        541,
    ),
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-3.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-3.tsv"),
        578,
    ),
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-4.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-4.tsv"),
        504,
    ),
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-5.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-5.tsv"),
        615,
    ),
    (
        include_bytes!("../fixtures/configuration-visual-maps/owned-0-2-edited.bin"),
        include_str!("../fixtures/configuration-visual-maps/owned-0-2-edited.tsv"),
        541,
    ),
];
#[test]
fn configuration_visual_complete_managers_match_oracles_and_reconstruction() {
    for (bytes, oracle, reads) in MANAGERS {
        let parsed = super::manager_context_tests::decode(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&parsed.fields, bytes, oracle, *reads);
        assert_eq!(encode_fields(&parsed.fields, 30000).unwrap(), *bytes);
    }
}
#[test]
fn configuration_visual_manager_edit_matches_the_independent_owned_golden() {
    let mut parsed = super::manager_context_tests::decode(MANAGERS[2].0).unwrap();
    let selected = parsed
        .fields
        .iter_mut()
        .find(|f| f.path.ends_with(".visual_map_b50[0].key"))
        .unwrap();
    selected.value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&parsed.fields, 30000).unwrap(), MANAGERS[6].0);
}
#[test]
fn configuration_visual_managers_reject_every_prefix_and_appended_byte() {
    for (bytes, _, _) in MANAGERS {
        for end in 0..bytes.len() {
            assert!(super::manager_context_tests::decode(&bytes[..end]).is_err());
        }
        let mut changed = bytes.to_vec();
        changed.push(0);
        assert!(super::manager_context_tests::decode(&changed).is_err());
    }
}
