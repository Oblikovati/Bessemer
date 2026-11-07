use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sketch-bends/cases.bin"),
        b"SKB1",
        198,
    )
}

#[test]
fn complete_sketch_bends_match_independent_fields_bytes_and_shared_slots() {
    assert_complete_records(records());
}

#[test]
fn sketch_bends_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn kind_seven_stops_before_consuming_any_derived_owned_payload() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.one_bend.raw_ac8",
        &7_i32.to_le_bytes(),
    );
    for end in [changed.len() - 14, changed.len()] {
        let Err(error) = decode_graph(version, &changed[..end], false) else {
            panic!("expected kind-7 owned-data stop")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("kind 7"));
    }
}

#[test]
fn sketch_bend_curve_role_rejects_the_cached_enclosing_feature() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.curve_de0",
        &2_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, false).is_err());
}

#[test]
fn fresh_and_cached_line_curve_targets_keep_their_complete_concrete_class() {
    for (kind, cached) in [(7, false), (13, true)] {
        let (_, version, bytes, _, _) = records()
            .into_iter()
            .find(|r| r.0 == kind && r.1 == 19000)
            .unwrap();
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let field = fields
            .iter()
            .find(|f| f.path.as_ref() == "root.object.payload.curve_de0")
            .unwrap();
        let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
            panic!("expected complete curve tag")
        };
        let (class, is_cached) = match &tag.record {
            crate::archive_codec::ObjectRecord::New { class, .. } => (class, false),
            crate::archive_codec::ObjectRecord::Reference { class, .. } => (class, true),
            _ => panic!("expected complete fresh or cached line reference"),
        };
        assert_eq!(class.name, "moLineRef_w");
        assert_eq!(is_cached, cached);
    }
}

#[test]
fn complete_sketch_bend_records_satisfy_only_reviewed_parent_roles_for_new_and_cached_tags() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let super::ModelFieldValue::ObjectTag(tag) = &fields[0].value else {
        panic!("expected complete root tag")
    };
    let crate::archive_codec::ObjectRecord::New { index, class, .. } = &tag.record else {
        panic!("expected complete fresh sketch bend")
    };
    let cached = crate::archive_codec::ObjectRecord::Reference {
        index: *index,
        class: class.clone(),
    };
    for record in [&tag.record, &cached] {
        assert!(
            super::objects::require_role(
                record,
                super::ModelObjectRole::Declared(&["moOneBend_c"]),
                "bend"
            )
            .is_ok()
        );
        assert!(
            super::objects::require_role(
                record,
                super::ModelObjectRole::Declared(&["moSheetMetal_c"]),
                "bend"
            )
            .is_err()
        );
    }
}
