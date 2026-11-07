use super::{ModelDecoder, ModelField, ModelFieldValue, encode_fields};
use crate::archive_codec::{ObjectReader, ObjectRecord};
use crate::header::ArchiveVersion;
fn decode(bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        super::annotation_view_tests::LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    super::configuration_decals::owned_map(&mut reader, &mut map)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}
const MAPS: &[(&[u8], &str, usize, usize)] = &[
    (
        include_bytes!("../fixtures/configuration-decals/map-0-0.bin"),
        include_str!("../fixtures/configuration-decals/map-0-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-0-1.bin"),
        include_str!("../fixtures/configuration-decals/map-0-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-0-2.bin"),
        include_str!("../fixtures/configuration-decals/map-0-2.tsv"),
        205,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-0-3.bin"),
        include_str!("../fixtures/configuration-decals/map-0-3.tsv"),
        103,
        5,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-0-4.bin"),
        include_str!("../fixtures/configuration-decals/map-0-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-0-5.bin"),
        include_str!("../fixtures/configuration-decals/map-0-5.tsv"),
        207,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-1-0.bin"),
        include_str!("../fixtures/configuration-decals/map-1-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-1-1.bin"),
        include_str!("../fixtures/configuration-decals/map-1-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-1-2.bin"),
        include_str!("../fixtures/configuration-decals/map-1-2.tsv"),
        293,
        17,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-1-3.bin"),
        include_str!("../fixtures/configuration-decals/map-1-3.tsv"),
        194,
        15,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-1-4.bin"),
        include_str!("../fixtures/configuration-decals/map-1-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-1-5.bin"),
        include_str!("../fixtures/configuration-decals/map-1-5.tsv"),
        299,
        17,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-2-0.bin"),
        include_str!("../fixtures/configuration-decals/map-2-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-2-1.bin"),
        include_str!("../fixtures/configuration-decals/map-2-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-2-2.bin"),
        include_str!("../fixtures/configuration-decals/map-2-2.tsv"),
        266,
        15,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-2-3.bin"),
        include_str!("../fixtures/configuration-decals/map-2-3.tsv"),
        164,
        13,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-2-4.bin"),
        include_str!("../fixtures/configuration-decals/map-2-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-2-5.bin"),
        include_str!("../fixtures/configuration-decals/map-2-5.tsv"),
        268,
        15,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-3-0.bin"),
        include_str!("../fixtures/configuration-decals/map-3-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-3-1.bin"),
        include_str!("../fixtures/configuration-decals/map-3-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-3-2.bin"),
        include_str!("../fixtures/configuration-decals/map-3-2.tsv"),
        219,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-3-3.bin"),
        include_str!("../fixtures/configuration-decals/map-3-3.tsv"),
        117,
        5,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-3-4.bin"),
        include_str!("../fixtures/configuration-decals/map-3-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-3-5.bin"),
        include_str!("../fixtures/configuration-decals/map-3-5.tsv"),
        221,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-4-0.bin"),
        include_str!("../fixtures/configuration-decals/map-4-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-4-1.bin"),
        include_str!("../fixtures/configuration-decals/map-4-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-4-2.bin"),
        include_str!("../fixtures/configuration-decals/map-4-2.tsv"),
        213,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-4-3.bin"),
        include_str!("../fixtures/configuration-decals/map-4-3.tsv"),
        111,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-4-4.bin"),
        include_str!("../fixtures/configuration-decals/map-4-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-4-5.bin"),
        include_str!("../fixtures/configuration-decals/map-4-5.tsv"),
        215,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-5-0.bin"),
        include_str!("../fixtures/configuration-decals/map-5-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-5-1.bin"),
        include_str!("../fixtures/configuration-decals/map-5-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-5-2.bin"),
        include_str!("../fixtures/configuration-decals/map-5-2.tsv"),
        213,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-5-3.bin"),
        include_str!("../fixtures/configuration-decals/map-5-3.tsv"),
        111,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-5-4.bin"),
        include_str!("../fixtures/configuration-decals/map-5-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-5-5.bin"),
        include_str!("../fixtures/configuration-decals/map-5-5.tsv"),
        215,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-6-0.bin"),
        include_str!("../fixtures/configuration-decals/map-6-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-6-1.bin"),
        include_str!("../fixtures/configuration-decals/map-6-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-6-2.bin"),
        include_str!("../fixtures/configuration-decals/map-6-2.tsv"),
        206,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-6-3.bin"),
        include_str!("../fixtures/configuration-decals/map-6-3.tsv"),
        104,
        5,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-6-4.bin"),
        include_str!("../fixtures/configuration-decals/map-6-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-6-5.bin"),
        include_str!("../fixtures/configuration-decals/map-6-5.tsv"),
        208,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-7-0.bin"),
        include_str!("../fixtures/configuration-decals/map-7-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-7-1.bin"),
        include_str!("../fixtures/configuration-decals/map-7-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-7-2.bin"),
        include_str!("../fixtures/configuration-decals/map-7-2.tsv"),
        212,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-7-3.bin"),
        include_str!("../fixtures/configuration-decals/map-7-3.tsv"),
        110,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-7-4.bin"),
        include_str!("../fixtures/configuration-decals/map-7-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-7-5.bin"),
        include_str!("../fixtures/configuration-decals/map-7-5.tsv"),
        214,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-8-0.bin"),
        include_str!("../fixtures/configuration-decals/map-8-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-8-1.bin"),
        include_str!("../fixtures/configuration-decals/map-8-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-8-2.bin"),
        include_str!("../fixtures/configuration-decals/map-8-2.tsv"),
        212,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-8-3.bin"),
        include_str!("../fixtures/configuration-decals/map-8-3.tsv"),
        110,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-8-4.bin"),
        include_str!("../fixtures/configuration-decals/map-8-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-8-5.bin"),
        include_str!("../fixtures/configuration-decals/map-8-5.tsv"),
        214,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-9-0.bin"),
        include_str!("../fixtures/configuration-decals/map-9-0.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-9-1.bin"),
        include_str!("../fixtures/configuration-decals/map-9-1.tsv"),
        6,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-9-2.bin"),
        include_str!("../fixtures/configuration-decals/map-9-2.tsv"),
        214,
        9,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-9-3.bin"),
        include_str!("../fixtures/configuration-decals/map-9-3.tsv"),
        112,
        7,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-9-4.bin"),
        include_str!("../fixtures/configuration-decals/map-9-4.tsv"),
        2,
        1,
    ),
    (
        include_bytes!("../fixtures/configuration-decals/map-9-5.bin"),
        include_str!("../fixtures/configuration-decals/map-9-5.tsv"),
        216,
        9,
    ),
];
#[test]
fn configuration_decal_maps_match_independent_oracles_slots_and_reconstruction() {
    for (bytes, oracle, reads, slots) in MAPS {
        let (fields, actual_slots) = decode(bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *reads);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), *bytes);
        assert_eq!(actual_slots, *slots);
    }
}
#[test]
fn configuration_decal_maps_reject_every_prefix_extra_byte_and_impossible_count() {
    for (bytes, _, _, _) in MAPS {
        for end in 0..bytes.len() {
            assert!(decode(&bytes[..end]).is_err());
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(decode(&extra).is_err());
    }
    assert!(decode(&i32::MAX.to_le_bytes()).is_err());
}
#[test]
fn configuration_decal_maps_preserve_empty_negative_counts_and_direct_width() {
    for index in [0, 4] {
        let (fields, slots) = decode(MAPS[index].0).unwrap();
        assert_eq!(MAPS[index].0.len(), 8);
        assert_eq!(slots, 1);
        assert_eq!(fields.len(), 2);
        let expected = if index == 0 {
            [-1, i32::MIN]
        } else {
            [i32::MIN, -1]
        };
        for (field, word) in fields.iter().zip(expected) {
            assert_eq!(field.value, ModelFieldValue::I32(word));
        }
    }
}
#[test]
fn configuration_decal_associations_reject_cached_unit_components_in_decal_roles() {
    let (fields, _) = decode(MAPS[2].0).unwrap();
    let unit = fields
        .iter()
        .find_map(|f| match &f.value {
            ModelFieldValue::ObjectTag(pointer) => match &pointer.record {
                ObjectRecord::New { index, class } if class.name == "moUnitComponent_c" => {
                    Some(index.value())
                }
                _ => None,
            },
            _ => None,
        })
        .unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.owned_6d8.associations[0].objects[0]",
        &(unit as u16).to_le_bytes(),
    );
    let Err(error) = decode(&changed) else {
        panic!("expected declared decal role rejection");
    };
    assert!(
        error.cause.contains("moUnitComponent_c") && error.cause.contains("moDecalProperties_c")
    );
}
