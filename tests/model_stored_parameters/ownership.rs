use super::*;

#[test]
fn published_parameter_types_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ModelParameterSource>();
    assert_send_sync::<ModelParameterKind>();
    assert_send_sync::<ModelParameterValue>();
    assert_send_sync::<ModelParameterName>();
    assert_send_sync::<ModelParameterStoredEdit>();
    assert_send_sync::<ModelParameterFieldCopy>();
    assert_send_sync::<ModelParameterDiagnostic>();
    assert_send_sync::<ModelParameterStoredState>();
}

#[test]
fn selectors_reject_nondefinitions_foreign_and_stale_revisions() {
    let (_, source) = fixture(fixtures::ORIGINAL[0][1]);
    let parameter = state(&source);
    for index in 0..source.fields().len() {
        let selected = source.stored_parameter(&source.field_handle(index).unwrap());
        assert_eq!(selected.is_ok(), index == parameter.definition().index());
    }
    let (_, foreign) = fixture(fixtures::ORIGINAL[0][1]);
    let error = foreign
        .stored_parameter(parameter.definition())
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert_rejected_edit(&foreign, &parameter);
    let edit = apply_parameter(&source, &parameter, primary_request(0));
    let stale = edit.archive().stored_parameter(parameter.definition());
    assert!(stale.is_err());
    assert_rejected_edit(edit.archive(), &parameter);
    let original = StoredDouble::from_bits(PRIMARY_BITS[1]);
    assert_eq!(parameter.primary(), &ModelParameterValue::Double(original));
}

fn assert_rejected_edit(
    source: &PartResolvedFeaturesArchive,
    parameter: &ModelParameterStoredState,
) {
    assert!(
        source
            .edit_stored_parameter(parameter, primary_request(0))
            .is_err()
    );
}

#[test]
fn wrong_type_absent_alternate_and_shape_changing_presence_fail_without_source_mutation() {
    for kind in 0..6 {
        let (_, source) = fixture(fixtures::ORIGINAL[kind][0]);
        let parameter = state(&source);
        let wrong = if kind == 3 {
            edited_value(0)
        } else {
            edited_value(3)
        };
        let requests = [
            ModelParameterStoredEdit::Primary(wrong),
            ModelParameterStoredEdit::Alternate(edited_value(kind)),
            ModelParameterStoredEdit::AlternatePresence(1),
        ];
        for request in requests {
            assert!(source.edit_stored_parameter(&parameter, request).is_err());
        }
        assert_eq!(source.to_bytes().unwrap(), fixtures::ORIGINAL[kind][0]);
        assert_eq!(state(&source).primary(), parameter.primary());
    }
}

#[test]
fn eight_workers_match_serial_goldens_and_leave_input_snapshots_unchanged() {
    let sources: Vec<_> = (0..6)
        .map(|kind| fixture(fixtures::ORIGINAL[kind][0]).1)
        .collect();
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_parameter_edits(&sources)))
            .collect();
        for job in jobs {
            job.join().unwrap();
        }
    });
}

fn repeat_parameter_edits(sources: &[PartResolvedFeaturesArchive]) {
    for _ in 0..8 {
        for (kind, source) in sources.iter().enumerate() {
            let parameter = state(source);
            let edit = apply_parameter(source, &parameter, primary_request(kind));
            assert_eq!(edit.archive().to_bytes().unwrap(), fixtures::EDITED[kind]);
            assert_eq!(source.to_bytes().unwrap(), fixtures::ORIGINAL[kind][0]);
        }
    }
}
