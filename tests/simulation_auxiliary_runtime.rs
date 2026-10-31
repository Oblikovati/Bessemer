//! Exact raw UTF-16 names and explicit memory-only normalization/allocation projections.
use bessemer::simulation::*;
use std::io::Read;
#[path = "common/auxiliary_runtime_packets.rs"]
mod checks;
#[path = "common/auxiliary_runtime_failures.rs"]
mod failures;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;

#[test]
fn owned_gen_trials_preserve_original_slices_for_both_stored_profiles() {
    let raw = packet::owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SIMO");
    assert_eq!(cursor.word(), 9);
    for _ in 0..9 {
        let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
        let raw = cursor.span();
        let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
        assert_gen_trials(&image, raw);
        for _ in 0..5 {
            cursor.span();
        }
    }
    assert!(cursor.0.is_empty());
}

fn inflate(bytes: &[u8]) -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(bytes)
        .read_to_end(&mut raw)
        .unwrap();
    raw
}

#[test]
fn all_owned_name_buffers_code_unit_stores_and_residual_bytes_match_actual_readers() {
    let raw = inflate(include_bytes!(
        "fixtures/simulation-auxiliary-runtime/names.bin"
    ));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"AUN1");
    assert_eq!(cursor.word(), 16);
    for _ in 0..16 {
        owned_name(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn owned_name(cursor: &mut Cursor<'_>) {
    let raw = cursor.span();
    let row = checks::row(raw);
    assert_eq!(checks::name_bytes(&row), cursor.span());
    assert_eq!(cursor.word(), 12);
    for _ in 0..12 {
        let units = checks::caller_units(cursor);
        let golden = cursor.span();
        let copied = cursor.word() as usize;
        let result = checks::name_store(&row, &units, golden, copied);
        assert_eq!(checks::name_bytes(&result.row), cursor.span());
        assert_eq!(row.to_bytes(), raw);
    }
}

#[test]
fn all_owned_workspaces_plans_bindings_and_composed_histories_match_actual_readers() {
    let raw = inflate(include_bytes!(
        "fixtures/simulation-auxiliary-runtime/workspaces.bin"
    ));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"AUW1");
    assert_eq!(cursor.word(), 112);
    for _ in 0..112 {
        owned_workspace(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn owned_workspace(cursor: &mut Cursor<'_>) {
    let row = checks::row(cursor.span());
    let prior = SimulationAuxiliaryWorkspace::from_bytes(cursor.span()).unwrap();
    let normalized = cursor.span();
    let discarded = if cursor.word() != 0 {
        Some(cursor.span())
    } else {
        None
    };
    let result = checks::normalization(&row, &prior, normalized, discarded);
    assert_eq!(cursor.word(), 4);
    for _ in 0..4 {
        owned_plan(&row, &prior, &result.workspace, cursor);
    }
    assert_eq!(result.workspace.to_bytes(), normalized);
}

fn owned_plan(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    workspace: &SimulationAuxiliaryWorkspace,
    cursor: &mut Cursor<'_>,
) {
    let index = SimulationAuxiliaryLogicalIndex::new(cursor.word()).unwrap();
    if cursor.word() != 0 {
        assert_owned_plan(row, workspace, index, cursor.span());
    } else {
        assert!(row.allocation_plan(index).is_err());
        assert!(workspace.allocation_plan(index).is_err());
    }
    assert_eq!(cursor.word(), 5);
    for _ in 0..5 {
        owned_binding(row, prior, workspace, index, cursor);
    }
}

fn owned_binding(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    workspace: &SimulationAuxiliaryWorkspace,
    index: SimulationAuxiliaryLogicalIndex,
    cursor: &mut Cursor<'_>,
) {
    let present = cursor.word() != 0;
    let pattern = u64::from_le_bytes(cursor.take(8).try_into().unwrap());
    let bits = present.then_some(pattern);
    let original = workspace.to_bytes();
    if cursor.word() == 0 {
        assert!(workspace.project_pointer_binding(index, bits).is_err());
        assert!(row.normalize_and_bind(Some(prior), index, bits).is_err());
    } else {
        let expected = cursor.span();
        let binding = workspace.project_pointer_binding(index, bits).unwrap();
        assert_eq!(binding.workspace.to_bytes(), expected);
        checks::assert_binding_contract(&binding);
        checks::composed(row, prior, index, bits, expected);
    }
    assert_eq!(workspace.to_bytes(), original);
}

#[test]
#[ignore = "requires exported approved local native corpus"]
fn all_native_names_and_original_trial_workspaces_match_actual_readers_without_pointer_use() {
    let raw = std::fs::read("target/simulation-auxiliary-runtime-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"AUXN");
    assert_eq!(cursor.word(), 3);
    let mut exported = b"AUXV".to_vec();
    let mut totals = [0usize; 3];
    for _ in 0..3 {
        native_pair(&mut cursor, &mut exported, &mut totals);
    }
    assert_eq!(totals, [5, 4, 1280]);
    assert!(cursor.0.is_empty());
    std::fs::write(
        "target/simulation-auxiliary-runtime-native-projections.bin",
        exported,
    )
    .unwrap();
}

fn native_pair(cursor: &mut Cursor<'_>, exported: &mut Vec<u8>, totals: &mut [usize; 3]) {
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let raw = cursor.span();
    let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
    let mut expected = Cursor(cursor.span());
    let count = expected.word();
    assert_eq!(count, image.layout().counts[8]);
    checks::append_words(exported, &[count]);
    for index in 0..count as usize {
        native_row(&image, index, &mut expected, exported, totals);
    }
    assert!(image.auxiliary_trial_workspace(count as usize).is_err());
    assert!(image.auxiliary_trial_workspace(usize::MAX).is_err());
    assert!(expected.0.is_empty());
    assert_eq!(image.to_bytes(), raw);
}

fn native_row(
    image: &SimulationGenArchive,
    index: usize,
    cursor: &mut Cursor<'_>,
    output: &mut Vec<u8>,
    totals: &mut [usize; 3],
) {
    let row = image.auxiliary_body_row(index).unwrap();
    let raw = cursor.span();
    assert_eq!(row.to_bytes(), raw);
    checks::append_span(output, &row.to_bytes());
    let name = checks::name_bytes(&row);
    assert_eq!(name, cursor.span());
    checks::append_span(output, &name);
    let prefix = row.name_view().bounded_units().to_vec();
    let replay = checks::name_store(&row, &prefix, cursor.span(), prefix.len());
    checks::append_span(output, &replay.row.to_bytes());
    let prior = native_trial(image, index, cursor, output);
    native_normalization(&row, &prior, index, cursor, output);
    totals[0] += 1;
    totals[1] += usize::from(row.profile().stride() == 284);
    totals[2] += row.name_view().name_buffer.len();
}

fn native_normalization(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    index: usize,
    cursor: &mut Cursor<'_>,
    output: &mut Vec<u8>,
) {
    let expected = cursor.span();
    let discarded = if cursor.word() != 0 {
        Some(cursor.span())
    } else {
        None
    };
    let normalization = checks::normalization(row, prior, expected, discarded);
    checks::append_span(output, &normalization.workspace.to_bytes());
    append_discarded(output, normalization.legacy_word_discarded);
    let logical = SimulationAuxiliaryLogicalIndex::new(index as u32 + 1).unwrap();
    native_plan(&normalization.workspace, logical, cursor, output);
    assert_eq!(cursor.word(), 1);
    let expected = cursor.span();
    let composed = checks::composed(row, prior, logical, Some(0), expected);
    checks::append_words(output, &[1]);
    checks::append_span(output, &composed.binding.workspace.to_bytes());
}

fn assert_gen_trials(image: &SimulationGenArchive, raw: &[u8]) {
    let count = image.layout().counts[8] as usize;
    for index in 0..count {
        let trial = image.auxiliary_trial_workspace(index).unwrap();
        let start = image.layout().blocks[7].0 + index * 288;
        assert_eq!(trial.to_bytes(), &raw[start..start + 288]);
        assert_eq!(trial.trial_source_offset(), Some(start));
        assert_eq!(trial.gen_parent(), Some((image.revision(), index)));
    }
    assert!(image.auxiliary_trial_workspace(count).is_err());
}

fn assert_owned_plan(
    row: &SimulationAuxiliaryBodyRow,
    workspace: &SimulationAuxiliaryWorkspace,
    index: SimulationAuxiliaryLogicalIndex,
    expected: &[u8],
) {
    let stored = row.allocation_plan(index).unwrap();
    let normalized = workspace.allocation_plan(index).unwrap();
    assert_eq!(checks::plan_bytes(&stored), expected);
    assert_eq!(checks::plan_bytes(&normalized), expected);
    let parent = SimulationAuxiliaryPlanParent::StoredRow(row.revision().clone());
    assert_eq!(stored.parent, parent);
    let parent = SimulationAuxiliaryPlanParent::Workspace(workspace.revision().clone());
    assert_eq!(normalized.parent, parent);
}

fn native_trial(
    image: &SimulationGenArchive,
    index: usize,
    cursor: &mut Cursor<'_>,
    output: &mut Vec<u8>,
) -> SimulationAuxiliaryWorkspace {
    let offset = cursor.word() as usize;
    let prior = image.auxiliary_trial_workspace(index).unwrap();
    assert_eq!(prior.trial_source_offset(), Some(offset));
    assert_eq!(prior.gen_parent(), Some((image.revision(), index)));
    assert_eq!(prior.to_bytes(), cursor.span());
    checks::append_words(output, &[prior.trial_source_offset().unwrap() as u32]);
    checks::append_span(output, &prior.to_bytes());
    prior
}

fn append_discarded(output: &mut Vec<u8>, discarded: Option<[u8; 4]>) {
    checks::append_words(output, &[u32::from(discarded.is_some())]);
    if let Some(word) = discarded {
        checks::append_span(output, &word);
    }
}

fn native_plan(
    workspace: &SimulationAuxiliaryWorkspace,
    index: SimulationAuxiliaryLogicalIndex,
    cursor: &mut Cursor<'_>,
    output: &mut Vec<u8>,
) {
    let plan = workspace.allocation_plan(index).unwrap();
    assert_eq!(checks::plan_bytes(&plan), cursor.span());
    assert_eq!(plan.allocation_bytes, Some(0));
    assert!(plan.allocation_call_required);
    checks::append_span(output, &checks::plan_bytes(&plan));
}
