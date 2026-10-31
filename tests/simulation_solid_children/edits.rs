use super::*;

fn sample(kind: SimulationSolidChildKind) -> SimulationSolidChildProperty {
    read_original(
        &mut if kind == COMPONENT { EXPECTED } else { FOLDERS },
        kind,
    )
}

fn requested(header: &SimulationSolidChildProperty) -> usize {
    usize::from(header.kind() == COMPONENT)
}

fn assert_parents(edit: &SimulationSolidChildEdit) {
    for (ordinal, row) in edit.history().iter().enumerate() {
        let changed = ordinal == edit.parent().ordinal();
        let mut parents = vec![SimulationSolidChildHistoryParent::Source(
            edit.source().field_id(ordinal).unwrap(),
        )];
        if changed {
            parents.push(SimulationSolidChildHistoryParent::Replacement);
        }
        assert_eq!(row.parents.as_ref(), parents);
        assert_eq!(row.result, edit.archive().field_id(ordinal).unwrap());
        assert_eq!(
            row.relation,
            if changed {
                SimulationSolidChildRelation::Derived
            } else {
                SimulationSolidChildRelation::Copied
            }
        );
    }
}

fn changed(header: &SimulationSolidChildProperty, text: bool) -> SimulationSolidChildEdit {
    let ordinal = if text {
        1 - requested(header)
    } else {
        requested(header)
    };
    let replacement = if text {
        SimulationSolidChildValue::Text(
            SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32).unwrap(),
        )
    } else {
        SimulationSolidChildValue::Signed(12345)
    };
    header
        .edit_field(&header.field_id(ordinal).unwrap(), replacement)
        .unwrap()
}

fn assert_isolated_edit(header: &SimulationSolidChildProperty, text: bool) {
    let original = header.to_bytes();
    let edit = changed(header, text);
    assert_eq!(edit.source().to_bytes(), original);
    assert_eq!(header.to_bytes(), original);
    assert_eq!(
        edit.archive().field(&edit.result()).unwrap().value,
        *edit.replacement()
    );
    assert_parents(&edit);
    let (start, end) = header.field_offsets()[edit.parent().ordinal()];
    let (next_start, next_end) = edit.archive().field_offsets()[edit.result().ordinal()];
    assert_eq!(original[..start], edit.archive().to_bytes()[..next_start]);
    assert_eq!(original[end..], edit.archive().to_bytes()[next_end..]);
    assert_reparsed(edit.archive());
}

fn assert_reparsed(header: &SimulationSolidChildProperty) {
    let rebuilt = SimulationSolidChildProperty::from_bytes(
        &header.to_bytes(),
        header.version(),
        header.kind(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.fields(), header.fields());
}

#[test]
fn word_and_raw_text_edits_preserve_surroundings_and_all_history_parents() {
    for kind in [COMPONENT, FOLDER] {
        for text in [false, true] {
            assert_isolated_edit(&sample(kind), text);
        }
    }
}

#[test]
fn every_owned_noop_retains_identity_and_source_bytes() {
    for (mut cursor, kind) in [(EXPECTED, COMPONENT), (FOLDERS, FOLDER)] {
        while !cursor.is_empty() {
            let header = read_original(&mut cursor, kind);
            consume_expected_fields(&mut cursor);
            for (index, field) in header.fields().iter().enumerate() {
                let id = header.field_id(index).unwrap();
                let edit = header.edit_field(&id, field.value.clone()).unwrap();
                assert_eq!(edit.result(), id);
                assert_eq!(edit.archive().to_bytes(), header.to_bytes());
                assert_eq!(
                    edit.coverage().completion,
                    SimulationSolidChildCompletion::Unchanged
                );
                assert!(edit.history().iter().all(|row| row.parents.len() == 1));
            }
        }
    }
}

#[test]
fn foreign_stale_wrong_kind_and_structural_requests_fail_without_mutation() {
    for kind in [COMPONENT, FOLDER] {
        let source = sample(kind);
        let original = source.to_bytes();
        let other = sample(kind);
        let id = source.field_id(requested(&source)).unwrap();
        assert_foreign(&source, &other);
        assert!(changed(&source, false).archive().field(&id).is_err());
        assert!(
            source
                .edit_field(&id, SimulationSolidChildValue::Unsigned(0))
                .is_err()
        );
        assert_fabricated(&source);
        assert_eq!(source.to_bytes(), original);
    }
}

fn assert_foreign(source: &SimulationSolidChildProperty, other: &SimulationSolidChildProperty) {
    let id = other.field_id(0).unwrap();
    assert!(
        source
            .edit_field(&id, SimulationSolidChildValue::Signed(0))
            .is_err()
    );
}

fn assert_fabricated(source: &SimulationSolidChildProperty) {
    let mut fields = source.fields().to_vec();
    fields[0].path = "fabricated".into();
    assert!(
        SimulationSolidChildProperty::from_fields(fields, source.version(), source.kind(), LIMITS)
            .is_err()
    );
}

#[test]
fn external_buffers_and_copied_fields_cannot_mutate_published_results() {
    for kind in [COMPONENT, FOLDER] {
        let source = sample(kind);
        let original = source.to_bytes();
        let mut input = original.clone();
        let header =
            SimulationSolidChildProperty::from_bytes(&input, source.version(), kind, LIMITS)
                .unwrap();
        input.fill(255);
        let mut returned = header.to_bytes();
        returned.fill(255);
        let mut fields = header.fields().to_vec();
        fields.clear();
        assert_eq!(header.to_bytes(), original);
        assert_eq!(header.fields(), source.fields());
    }
}

fn concurrent_result(header: &SimulationSolidChildProperty, text: bool) -> Vec<u8> {
    let edit = changed(header, text);
    assert_parents(&edit);
    assert_eq!(edit.archive().coverage().kind, header.kind());
    for field in edit.archive().fields() {
        if let SimulationSolidChildValue::Text(text) = &field.value {
            assert_eq!(text.source_parent(), edit.archive().to_bytes());
        }
    }
    edit.archive().to_bytes()
}

fn assert_worker(sources: &[SimulationSolidChildProperty; 2], serial: &[Vec<u8>], worker: usize) {
    for turn in 0..32 {
        let index = (worker + turn) % 4;
        assert_eq!(
            concurrent_result(&sources[index / 2], index % 2 == 1),
            serial[index]
        );
    }
}

#[test]
fn mixed_concurrent_edits_match_serial_results_and_keep_inputs_isolated() {
    let sources = [sample(COMPONENT), sample(FOLDER)];
    let serial: Vec<Vec<u8>> = (0..4)
        .map(|index| concurrent_result(&sources[index / 2], index % 2 == 1))
        .collect();
    std::thread::scope(|scope| {
        for worker in 0..8 {
            let sources = &sources;
            let serial = &serial;
            scope.spawn(move || {
                assert_worker(sources, serial, worker);
            });
        }
    });
    assert_eq!(sources[0].to_bytes(), sample(COMPONENT).to_bytes());
    assert_eq!(sources[1].to_bytes(), sample(FOLDER).to_bytes());
}

#[test]
fn every_public_child_type_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationSolidChildProperty>();
    check::<SimulationSolidChildKind>();
    check::<SimulationSolidChildEvidence>();
    check::<SimulationSolidChildLimits>();
    check::<SimulationSolidChildEdit>();
    check::<SimulationSolidChildCoverage>();
    check::<SimulationSolidChildCheckMode>();
    check::<SimulationSolidChildCompletion>();
    check::<SimulationSolidChildDiagnostic>();
    check::<SimulationSolidChildField>();
    check::<SimulationSolidChildValue>();
    check::<SimulationSolidChildFieldId>();
    check::<SimulationSolidChildHistory>();
    check::<SimulationSolidChildHistoryParent>();
    check::<SimulationSolidChildRelation>();
}
