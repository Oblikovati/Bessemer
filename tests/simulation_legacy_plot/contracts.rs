use super::*;

#[test]
fn every_noop_preserves_original_revision_and_all_copied_history() {
    for plot in owned_plots() {
        for (ordinal, field) in plot.fields().iter().enumerate() {
            let id = plot.field_id(ordinal).unwrap();
            let edit = plot.edit_field(&id, field.value.clone()).unwrap();
            assert_eq!(
                edit.coverage().completion,
                SimulationLegacyPlotCompletion::Unchanged
            );
            assert_eq!(edit.parent(), edit.result());
            assert_eq!(edit.archive().to_bytes(), plot.to_bytes());
            check_noop_history(&plot, &edit);
        }
    }
}

fn check_noop_history(plot: &SimulationLegacyPlot, edit: &SimulationLegacyPlotEdit) {
    for (index, relation) in edit.history().iter().enumerate() {
        assert_eq!(relation.relation, SimulationLegacyPlotRelation::Copied);
        assert_eq!(relation.result, plot.field_id(index).unwrap());
        assert_eq!(
            relation.parents.as_ref(),
            [SimulationLegacyPlotHistoryParent::Source(
                plot.field_id(index).unwrap()
            )]
        );
    }
}

fn reject_wrong_construction(plot: &SimulationLegacyPlot, fields: Vec<SimulationLegacyPlotField>) {
    assert!(SimulationLegacyPlot::from_fields(fields, plot.version(), LIMITS).is_err());
}

#[test]
fn wrong_paths_counts_kinds_and_foreign_identities_reject_without_changing_input() {
    for plot in owned_plots() {
        let before = plot.to_bytes();
        let mut fields = plot.fields().to_vec();
        fields[0].path = "unknown historical member".into();
        reject_wrong_construction(&plot, fields);
        let mut fields = plot.fields().to_vec();
        fields.pop();
        reject_wrong_construction(&plot, fields);
        let mut fields = plot.fields().to_vec();
        fields.push(fields[0].clone());
        reject_wrong_construction(&plot, fields);
        reject_wrong_kinds(&plot);
        let rebuilt =
            SimulationLegacyPlot::from_fields(plot.fields().to_vec(), plot.version(), LIMITS)
                .unwrap();
        assert!(plot.field(&rebuilt.field_id(0).unwrap()).is_err());
        assert!(plot.field_id(plot.fields().len()).is_err());
        assert_eq!(plot.to_bytes(), before);
    }
}

fn reject_wrong_kinds(plot: &SimulationLegacyPlot) {
    for (ordinal, field) in plot.fields().iter().enumerate() {
        let replacement = match field.value {
            SimulationLegacyPlotValue::RawWordBits(_) => SimulationLegacyPlotValue::RawQwordBits(0),
            _ => SimulationLegacyPlotValue::RawWordBits(0),
        };
        let id = plot.field_id(ordinal).unwrap();
        let error = plot.edit_field(&id, replacement).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert_eq!(error.entity, field.path.as_ref());
        assert!(error.cause.contains("expected"));
        assert_eq!(plot.field(&id).unwrap(), field);
    }
}

#[test]
fn caller_input_and_construction_vectors_remain_isolated_after_mutation() {
    for plot in owned_plots() {
        let mut bytes = plot.to_bytes();
        let read = SimulationLegacyPlot::from_bytes(&bytes, plot.version(), LIMITS).unwrap();
        let mut fields = read.fields().to_vec();
        let built =
            SimulationLegacyPlot::from_fields(fields.clone(), plot.version(), LIMITS).unwrap();
        fields[0].value = changed_legacy_value(&fields[0].value);
        bytes.fill(0);
        assert_eq!(read.to_bytes(), plot.to_bytes());
        assert_eq!(built.to_bytes(), plot.to_bytes());
        assert_ne!(built.fields()[0], fields[0]);
        let original_id = plot.field_id(0).unwrap();
        assert_eq!(plot.field(&original_id).unwrap(), &plot.fields()[0]);
    }
}
