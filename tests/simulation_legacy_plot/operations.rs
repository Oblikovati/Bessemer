use super::*;

pub(super) fn changed_legacy_value(
    original: &SimulationLegacyPlotValue,
) -> SimulationLegacyPlotValue {
    match original {
        SimulationLegacyPlotValue::RawWordBits(bits) => {
            SimulationLegacyPlotValue::RawWordBits(bits ^ 0x80000001)
        }
        SimulationLegacyPlotValue::RawQwordBits(bits) => {
            SimulationLegacyPlotValue::RawQwordBits(bits ^ 0x8000000000000001)
        }
        SimulationLegacyPlotValue::RawBytes(raw) => {
            let mut raw = raw.to_vec();
            raw[0] ^= 0x55;
            SimulationLegacyPlotValue::RawBytes(raw.into())
        }
        SimulationLegacyPlotValue::Text(text) => {
            SimulationLegacyPlotValue::Text(changed_legacy_text(text))
        }
    }
}

fn changed_legacy_text(text: &SimulationLegacyPlotText) -> SimulationLegacyPlotText {
    let mut payload = text.payload().to_vec();
    if text.character_count() == 512 {
        payload.truncate(payload.len() - 2);
    } else {
        payload.extend_from_slice(&[0, 0xd8]);
    }
    SimulationLegacyPlotText::from_payload(&payload, 1024).unwrap()
}

fn literal_edit_bytes(
    plot: &SimulationLegacyPlot,
    ordinal: usize,
    replacement: &SimulationLegacyPlotValue,
) -> Vec<u8> {
    let raw = plot.to_bytes();
    let (start, end) = plot.field_offsets()[ordinal];
    let mut expected = raw[..start].to_vec();
    expected.extend(encoded_legacy_value(replacement).1);
    expected.extend_from_slice(&raw[end..]);
    expected
}

fn check_legacy_history(edit: &SimulationLegacyPlotEdit, ordinal: usize) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (index, relation) in edit.history().iter().enumerate() {
        let source = edit.source().field_id(index).unwrap();
        let mut parents = vec![SimulationLegacyPlotHistoryParent::Source(source)];
        if index == ordinal {
            parents.push(SimulationLegacyPlotHistoryParent::Replacement);
        }
        assert_eq!(relation.parents.as_ref(), parents);
        assert_eq!(relation.result, edit.archive().field_id(index).unwrap());
        assert_eq!(
            relation.relation,
            if index == ordinal {
                SimulationLegacyPlotRelation::Derived
            } else {
                SimulationLegacyPlotRelation::Copied
            }
        );
    }
}

pub(super) fn changed_legacy_edit(
    plot: &SimulationLegacyPlot,
    ordinal: usize,
) -> SimulationLegacyPlotEdit {
    let id = plot.field_id(ordinal).unwrap();
    let replacement = changed_legacy_value(&plot.field(&id).unwrap().value);
    let expected = literal_edit_bytes(plot, ordinal, &replacement);
    let edit = plot.edit_field(&id, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), expected);
    assert_eq!(edit.source().to_bytes(), plot.to_bytes());
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.result(), edit.archive().field_id(ordinal).unwrap());
    assert_eq!(edit.replacement(), &replacement);
    assert_eq!(
        edit.archive().field(&edit.result()).unwrap().value,
        replacement
    );
    assert!(edit.archive().field(&id).is_err());
    check_changed_coverage(&edit);
    check_legacy_history(&edit, ordinal);
    edit
}

fn check_changed_coverage(edit: &SimulationLegacyPlotEdit) {
    assert_eq!(
        edit.coverage().completion,
        SimulationLegacyPlotCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationLegacyPlotCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(edit.coverage().requested_pattern_error_bits, 0);
}

fn project_changed_history(edit: &SimulationLegacyPlotEdit, output: &mut Vec<u8>) {
    let source: Vec<_> = (0..edit.source().fields().len())
        .map(|index| edit.source().field_id(index).unwrap())
        .collect();
    let result: Vec<_> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    output.extend(project_plot(edit.archive()));
    project_outer_history(output, edit.history(), &source, &result);
}

pub(super) fn project_complete_operation(plot: &SimulationLegacyPlot) -> Vec<u8> {
    let mut output = plot.version().value().to_le_bytes().to_vec();
    output.extend(project_plot(plot));
    project_word(
        &mut output,
        usize::from(plot.named_stream_dispatch_enabled()),
    );
    for ordinal in 0..plot.fields().len() {
        project_changed_history(&changed_legacy_edit(plot, ordinal), &mut output);
    }
    output
}

#[test]
fn every_owned_field_and_all_four_value_kinds_edit_in_isolation_with_full_history() {
    for plot in owned_plots() {
        let before = plot.to_bytes();
        for ordinal in 0..plot.fields().len() {
            changed_legacy_edit(&plot, ordinal);
        }
        assert_eq!(plot.to_bytes(), before);
    }
}
