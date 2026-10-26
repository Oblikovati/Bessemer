use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelObjectRole};
use crate::archive_codec::{ArchiveClass, ArchiveCursor, ObjectReader};

const PACKET: &[u8] = include_bytes!("../fixtures/sweeps/cases.bin");
type SweepRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<SweepRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"SWF1");
    assert_eq!(cursor.u32().unwrap(), 160);
    let slots: Vec<_> = (0..160).map(|_| cursor.u32().unwrap() as usize).collect();
    let records = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    records
}

fn decode_graph(version: u32, bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut slots = 0;
    let fields = decode_collection(version, bytes, |reader, map| {
        reader.object("object", map, ModelObjectRole::Any)?;
        slots = std::mem::replace(map, ObjectReader::new())
            .into_snapshot()
            .slot_count();
        Ok(())
    })?;
    Ok((fields, slots))
}

fn current_options() -> Vec<ModelField> {
    records()
        .into_iter()
        .filter(|(kind, version, _, _, _)| *kind == 0 && *version == 19000)
        .map(|(_, version, bytes, _, _)| decode(version, bytes).unwrap())
        .find(|fields| fields.iter().any(|field| field.path.ends_with("angle_be0")))
        .unwrap()
}

#[test]
fn complete_sweeps_and_dependencies_match_independent_fields_bytes_and_final_slots() {
    for (_, version, bytes, oracle, expected_slots) in records() {
        let (fields, slots) = decode_graph(version, bytes).unwrap();
        assert_eq!(slots, expected_slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn sweeps_and_dependencies_reject_all_truncated_prefixes_trailing_bytes_and_old_contexts() {
    for (_, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn independent_sweep_option_truth_table_and_data_tail_gate_match() {
    let expected = [
        [false; 6],
        [true, false, true, true, true, false],
        [true, true, true, true, false, true],
        [true; 6],
        [false, false, true, true, true, false],
        [true, false, false, false, false, true],
        [true, true, false, false, true, true],
        [false, false, true, false, false, false],
    ];
    let labels = [
        "dimension_bd0",
        "dimension_bd8",
        "angle_be0",
        "radius_bf0",
        "direction_88",
        "direction_90",
    ];
    for (index, (_, version, bytes, _, _)) in records()
        .into_iter()
        .filter(|(kind, _, _, _, _)| *kind == 0)
        .enumerate()
    {
        let fields = decode(version, bytes).unwrap();
        let has = |suffix: &str| fields.iter().any(|field| field.path.ends_with(suffix));
        for (label, present) in labels.into_iter().zip(expected[index % 8]) {
            assert_eq!(has(label), present, "{index} {label}");
        }
        assert_eq!(has("angle_be8"), expected[index % 8][2]);
        assert_eq!(has("raw_3b0"), version > 0x27d9);
    }
}

#[test]
fn conditional_sweep_pointer_roles_reject_the_enclosing_feature() {
    let fields = current_options();
    for label in [
        "angle_be0",
        "radius_bf0",
        "data.twist_298",
        "data.end_point_2a0",
        "data.direction_88",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2u16.to_le_bytes(),
        );
        let error = decode(19000, &changed).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("moSweep_c"));
    }
}

#[test]
fn undocumented_fresh_twist_bodies_remain_explicit_unsupported_results() {
    let name = b"moTwistDataForBiDirectionalSweep_c";
    let mut header = [
        u16::MAX.to_le_bytes(),
        1u16.to_le_bytes(),
        (name.len() as u16).to_le_bytes(),
    ]
    .concat();
    header.extend_from_slice(name);
    let changed = super::manager_context_tests::replace_field(
        &current_options(),
        "root.object.payload.data.twist_298",
        &header,
    );
    let error = decode(19000, &changed).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("moTwistDataForBiDirectionalSweep_c"));
}

#[test]
fn verified_component_profile_family_is_exact_and_keeps_unrelated_classes_out() {
    for (name, accepted) in [
        ("moCompProfile_c", true),
        ("moCompProfile_c_suffix", false),
        ("moNodeName_c", false),
    ] {
        let result = decode_collection(19000, &[1, 0], |reader, map| {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            reader.object(
                "profile",
                map,
                ModelObjectRole::Declared(super::component_references::FEATURE_CLASSES),
            )
        });
        assert_eq!(result.is_ok(), accepted, "{name}");
    }
}
