use super::trim_edge_identifier_tests::{TrimEdgeRecord, records_with_count};
use super::{ModelFieldValue, ModelObjectRole};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/template-dependencies/cases.bin"),
        b"TDP1",
        312,
    )
}

#[test]
fn complete_template_aliases_and_ids_match_independent_fields_bytes_and_map_slots() {
    for (kind, version, bytes, oracle, slots) in records() {
        let (fields, actual) =
            super::flat_pattern_tests::decode_graph(u8::from(kind / 6 == 1), version, bytes, true)
                .unwrap();
        assert_eq!(actual, slots);
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
fn template_dependencies_reject_every_prefix_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        let state = u8::from(kind / 6 == 1);
        for end in 0..bytes.len() {
            assert!(
                super::flat_pattern_tests::decode_graph(state, version, &bytes[..end], true)
                    .is_err()
            );
        }
        assert!(
            super::flat_pattern_tests::decode_graph(state, version, &[bytes, &[0]].concat(), true)
                .is_err()
        );
        assert!(super::flat_pattern_tests::decode_graph(state, 0x27cb, bytes, true).is_err());
    }
}

#[test]
fn template_flat_suffix_uses_saved_helper_state_instead_of_the_class_name() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 12) {
        let state = u8::from(kind / 6 == 1);
        let fields = super::flat_pattern_tests::decode_graph(state, version, bytes, true)
            .unwrap()
            .0;
        assert_eq!(
            fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.payload.raw_a80"),
            state == 1
        );
        assert!(super::flat_pattern_tests::decode_graph(1 - state, version, bytes, true).is_err());
    }
}

#[test]
fn gusset_representation_suffixes_preserve_three_signed_words() {
    let words = [i32::MIN, i32::MAX, -1, 0, 1, -32769];
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 12) {
        let fields = super::flat_pattern_tests::decode_graph(0, version, bytes, true)
            .unwrap()
            .0;
        for (index, offset) in ["38", "3c", "40"].into_iter().enumerate() {
            let value = fields
                .iter()
                .find(|f| f.path.as_ref() == format!("root.object.unknown_{offset}"))
                .unwrap();
            assert_eq!(
                value.value,
                ModelFieldValue::I32(words[(usize::from(kind % 6) + index) % 6])
            );
        }
    }
}

#[test]
fn fresh_and_cached_gusset_ids_retain_the_declared_representation_base_role() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 12 && r.1 == 19000)
    {
        let fields = super::flat_pattern_tests::decode_graph(0, version, bytes, true)
            .unwrap()
            .0;
        let ModelFieldValue::ObjectTag(tag) = &fields[0].value else {
            panic!("gusset ID lost its object tag")
        };
        super::objects::require_role(
            &tag.record,
            ModelObjectRole::Declared(&["moEntityIdRep_c"]),
            "representation",
        )
        .unwrap();
        let crate::archive_codec::ObjectRecord::New { class, index } = &tag.record else {
            panic!("gusset ID did not allocate its actual class")
        };
        let cached = crate::archive_codec::ObjectRecord::Reference {
            class: class.clone(),
            index: *index,
        };
        super::objects::require_role(
            &cached,
            ModelObjectRole::Declared(&["moEntityIdRep_c"]),
            "representation",
        )
        .unwrap();
        assert!(
            super::objects::require_role(
                &cached,
                ModelObjectRole::Declared(&["moFlatPattern_c"]),
                "flat_pattern"
            )
            .is_err(),
            "kind {kind}"
        );
    }
}
