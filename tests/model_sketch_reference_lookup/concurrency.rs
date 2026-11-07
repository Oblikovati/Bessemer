use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use crate::visual_workers::assert_eight_visual_workers;
use bessemer::BessemerError;

#[derive(Debug, PartialEq, Eq)]
struct LookupConcurrentOutcome {
    changed: ModelVisualOperation,
    unchanged: ModelVisualOperation,
    foreign: BessemerError,
    stale: BessemerError,
}

fn lookup_concurrent_request(state: &LookupState, kind: usize, change: bool) -> LookupEdit {
    let keys = state.keys();
    let object = keys.configuration_object_key;
    let record = keys.configuration_record_key;
    let select = |value: i32| if change { !value } else { value };
    match kind {
        0 => LookupEdit::ConfigurationObjectKey(select(object)),
        1 => LookupEdit::ConfigurationRecordKey(select(record)),
        2 => LookupEdit::BothKeys {
            configuration_object_key: select(object),
            configuration_record_key: select(record),
        },
        _ => panic!("lookup request kind {kind}; expected 0..3"),
    }
}

fn lookup_concurrent_failures(
    foreign: &PartResolvedFeaturesArchive,
    changed: &PartResolvedFeaturesEdit,
    state: &LookupState,
    request: LookupEdit,
) -> (BessemerError, BessemerError) {
    let leading = &state.source_fields()[0];
    let rejected = foreign
        .edit_sketch_reference_lookup_keys(state, request)
        .unwrap_err();
    let stale = changed
        .archive()
        .edit_sketch_reference_lookup_keys(state, request)
        .unwrap_err();
    assert_eq!(rejected, foreign.resolve_field(leading).unwrap_err());
    assert_eq!(stale, changed.archive().resolve_field(leading).unwrap_err());
    (rejected, stale)
}

fn lookup_concurrent_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &LookupState,
    kind: usize,
) -> LookupConcurrentOutcome {
    let request = lookup_concurrent_request(state, kind, true);
    let changed = source
        .edit_sketch_reference_lookup_keys(state, request)
        .unwrap();
    let no_change = lookup_concurrent_request(state, kind, false);
    let unchanged = source
        .edit_sketch_reference_lookup_keys(state, no_change)
        .unwrap();
    assert_eq!(changed.completion(), ModelEditCompletion::Changed);
    assert_eq!(unchanged.completion(), ModelEditCompletion::Unchanged);
    let (foreign, stale) = lookup_concurrent_failures(foreign, &changed, state, request);
    LookupConcurrentOutcome {
        changed: model_visual_operation(source, &changed),
        unchanged: model_visual_operation(source, &unchanged),
        foreign,
        stale,
    }
}

fn lookup_concurrent_case(raw: &[u8], configuration: &PartConfigurationArchive, manager: &[u8]) {
    let source = lookup_archive(raw, configuration, manager);
    let foreign = lookup_archive(raw, configuration, manager);
    for state in source.sketch_reference_lookup_states().unwrap() {
        for kind in 0..3 {
            let project = || lookup_concurrent_outcome(&source, &foreign, &state, kind);
            let serial = project();
            assert_eight_visual_workers(&serial, project);
        }
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(foreign.to_bytes().unwrap(), raw);
}

#[test]
fn lookup_mixed_operations_match_serial_across_eight_workers() {
    let (configuration, manager) = lookup_context();
    let original = configuration.to_bytes().unwrap();
    for raw in lookup_images() {
        lookup_concurrent_case(raw, &configuration, &manager);
    }
    assert_eq!(configuration.to_bytes().unwrap(), original);
}
