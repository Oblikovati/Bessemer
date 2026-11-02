use super::*;

pub(super) fn requests() -> Vec<ModelDimensionStoredEdit> {
    use ModelDimensionStoredEdit::*;
    [EnforceBendAngleRange(false), EnforceBendAngleRange(true)]
        .into_iter()
        .chain((0..8).map(FilletRefPointChangeNumber))
        .chain([i32::MIN, -1, 0, 1, i32::MAX].into_iter().map(RedundantId))
        .collect()
}

pub(super) fn literal_edit(
    raw: &[u8],
    base: usize,
    request: ModelDimensionStoredEdit,
) -> (Vec<u8>, usize) {
    let before = u32::from_le_bytes(raw[base + 16..base + 20].try_into().unwrap());
    let (offset, after) = match request {
        ModelDimensionStoredEdit::EnforceBendAngleRange(flag) => {
            (base + 16, (before & 0xfffffffd) + if flag { 2 } else { 0 })
        }
        ModelDimensionStoredEdit::FilletRefPointChangeNumber(number) => {
            (base + 16, (before & 0xffffffe3) + 4 * u32::from(number))
        }
        ModelDimensionStoredEdit::RedundantId(identifier) => (base + 20, identifier as u32),
    };
    let mut expected = raw.to_vec();
    expected[offset..offset + 4].copy_from_slice(&after.to_le_bytes());
    (expected, usize::from(offset == base + 20))
}

pub(super) fn assert_unselected(before: &[ModelField], after: &[ModelField], selected: usize) {
    assert_eq!(before.len(), after.len());
    for (index, (left, right)) in before.iter().zip(after).enumerate() {
        assert_eq!(left.path, right.path);
        if index != selected {
            assert_eq!(left, right);
        }
    }
}

fn assert_history(
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    selected: usize,
) {
    let changed = source.fields() != edit.archive().fields();
    let handles = [
        source.field_handle(selected).unwrap(),
        edit.archive().field_handle(selected).unwrap(),
    ];
    assert_field_history(
        (edit.completion(), edit.unchanged_fields(), edit.changes()),
        source.fields().len(),
        handles,
        changed,
    );
    assert_eq!(
        edit.archive()
            .resolve_field(&source.field_handle(selected).unwrap())
            .is_err(),
        changed
    );
}

pub(super) fn assert_request(
    source: &PartResolvedFeaturesArchive,
    case: &DimensionCase<'_>,
    ordinal: usize,
    request: ModelDimensionStoredEdit,
) -> bool {
    let handle = dimension_handle(source, ordinal);
    let dimension = source.dimension_state(&handle).unwrap();
    let (expected, relative) = literal_edit(case.raw, case.offsets[ordinal], request);
    let edit = source.edit_dimension_state(&dimension, request).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    assert_unselected(
        source.fields(),
        edit.archive().fields(),
        handle.index() + relative,
    );
    assert_history(source, &edit, handle.index() + relative);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.diagnostics(), source.diagnostics());
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    expected != case.raw
}

#[test]
fn every_stored_role_edit_matches_closed_form_full_images_fields_and_ordered_history() {
    let (configuration, manager) = base_configuration();
    let mut changed = 0;
    let mut unchanged = 0;
    for case in cases() {
        let source = feature_archive(&case, &configuration, &manager);
        for ordinal in 0..2 {
            for request in requests() {
                if assert_request(&source, &case, ordinal, request) {
                    changed += 1;
                } else {
                    unchanged += 1;
                }
            }
        }
    }
    assert_eq!((changed, unchanged), (131648, 31552));
}

pub(super) fn assert_field_history(
    actual: (ModelEditCompletion, usize, &[ModelFieldChange]),
    count: usize,
    handles: [ModelFieldHandle; 2],
    changed: bool,
) {
    let (completion, unchanged, relations) = actual;
    assert_eq!(completion == ModelEditCompletion::Changed, changed);
    assert_eq!(unchanged, count - usize::from(changed));
    assert_eq!(relations.len(), usize::from(changed));
    if !changed {
        return;
    }
    assert_eq!(relations[0].relation(), ModelFieldRelation::Derived);
    assert_eq!(relations[0].parents(), &handles[..1]);
    assert_eq!(relations[0].result(), Some(&handles[1]));
}
