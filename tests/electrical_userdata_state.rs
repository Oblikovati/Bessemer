//! Independent storage truth tables for the complete constructor-shaped delegate.
use bessemer::electrical::*;
use std::sync::Arc;

fn source_state(object_id: i32) -> ElectricalUserdataState {
    ElectricalUserdataState {
        context_word: u64::MAX,
        load_flag: 255,
        object_id,
        object_no: -37,
        slots: std::array::from_fn(|index| Arc::from([index as u16, 0, 0xd800])),
    }
}

fn supplied_effects(loaded: i32) -> ElectricalUserdataReadEffects {
    ElectricalUserdataReadEffects {
        context_word: 42,
        object_id: ElectricalUserdataEffect::Written(loaded),
        object_no: ElectricalUserdataEffect::Written(17),
        slots: std::array::from_fn(|index| {
            ElectricalUserdataEffect::Written(Arc::from([0xdc00, index as u16, 0]))
        }),
    }
}

#[test]
fn null_recordset_retains_every_original_unit_and_history_parent() {
    let source = source_state(-9);
    let result = source.project_recordset(None);
    assert_eq!(result.before, source);
    assert_eq!(result.after, source);
    assert_eq!(result.branch, ElectricalUserdataReadBranch::NullRecordset);
    assert_eq!(result.coverage.long_reads, 0);
    assert_eq!(result.coverage.string_reads, 0);
    assert_eq!(result.coverage.cleared_slots, 0);
    assert_eq!(result.coverage.achieved_unit_error, 0);
    assert_eq!(
        result.history.context_word,
        ElectricalUserdataStorageRelation::Copied
    );
    assert_eq!(
        result.history.slots,
        [ElectricalUserdataStorageRelation::Copied; 20]
    );
}

#[test]
fn all_signed_boundary_pairs_match_independent_complete_storage_expectations() {
    let ids = [i32::MIN, -37, -2, -1, 0, 1, 17, i32::MAX - 1, i32::MAX];
    for current in ids {
        for loaded in ids {
            let source = source_state(current);
            let result = source.project_recordset(Some(&supplied_effects(loaded)));
            assert_eq!(result.before, source);
            assert_eq!(result.after.context_word, 42);
            let zero_rejected = loaded == 0 && current != 0;
            let accepted = loaded >= 0 && !zero_rejected;
            let expected_id = if zero_rejected {
                current
            } else {
                loaded.max(-1)
            };
            assert_eq!(result.after.object_id, expected_id);
            assert_branch(&result, accepted);
        }
    }
}

fn assert_branch(result: &ElectricalUserdataStateRead, accepted: bool) {
    assert_eq!(result.after.load_flag, if accepted { 1 } else { 255 });
    assert_eq!(result.after.object_no, if accepted { 17 } else { -37 });
    assert_eq!(result.coverage.long_reads, if accepted { 2 } else { 1 });
    assert_eq!(result.coverage.string_reads, if accepted { 20 } else { 0 });
    assert_eq!(result.coverage.cleared_slots, if accepted { 0 } else { 20 });
    for (index, slot) in result.after.slots.iter().enumerate() {
        let expected = if accepted {
            vec![0xdc00, index as u16, 0]
        } else {
            vec![]
        };
        assert_eq!(slot.as_ref(), expected);
    }
}

#[test]
fn unchanged_provider_destinations_retain_storage_but_gate_temporary_starts_at_minus_one() {
    let source = source_state(7);
    let mut effects = supplied_effects(7);
    effects.object_no = ElectricalUserdataEffect::Unchanged;
    effects.slots = std::array::from_fn(|_| ElectricalUserdataEffect::Unchanged);
    let accepted = source.project_recordset(Some(&effects));
    assert_eq!(accepted.after.slots, source.slots);
    assert_eq!(accepted.after.object_no, -37);
    assert_eq!(
        accepted.history.object_no,
        ElectricalUserdataStorageRelation::Copied
    );
    assert_eq!(
        accepted.history.slots,
        [ElectricalUserdataStorageRelation::Copied; 20]
    );
    effects.object_id = ElectricalUserdataEffect::Unchanged;
    let rejected = source.project_recordset(Some(&effects));
    assert_eq!(rejected.after.object_id, -1);
    assert_eq!(rejected.after.load_flag, 255);
    assert!(rejected.after.slots.iter().all(|slot| slot.is_empty()));
}

#[test]
fn shared_snapshots_isolate_eight_workers_and_subsequent_owned_mutation() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ElectricalUserdataStateRead>();
    assert_send_sync::<ElectricalUserdataReadEffects>();
    let mut source = source_state(0);
    let effects = supplied_effects(0);
    let expected = source.project_recordset(Some(&effects));
    std::thread::scope(|scope| {
        let joins: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| source.project_recordset(Some(&effects))))
            .collect();
        for join in joins {
            assert_eq!(join.join().unwrap(), expected);
        }
    });
    source.slots[0] = Arc::from([99]);
    assert_eq!(expected.before.slots[0].as_ref(), &[0, 0, 0xd800]);
    assert_eq!(expected.after.slots[0].as_ref(), &[0xdc00, 0, 0]);
}

#[test]
fn mixed_slot_effects_and_noncanonical_flags_preserve_exact_destination_history() {
    for flag in [0, 1, 7, 255] {
        let mut source = source_state(19);
        source.load_flag = flag;
        let mut effects = supplied_effects(19);
        for index in (0..20).step_by(2) {
            effects.slots[index] = ElectricalUserdataEffect::Unchanged;
        }
        let result = source.project_recordset(Some(&effects));
        assert_mixed_history(&result, &source);
        effects.object_id = ElectricalUserdataEffect::Written(0);
        let rejected = source.project_recordset(Some(&effects));
        assert_rejected_history(&rejected, flag);
    }
}

fn assert_rejected_history(result: &ElectricalUserdataStateRead, flag: u8) {
    use ElectricalUserdataStorageRelation::{Cleared, Copied};
    assert_eq!(result.branch, ElectricalUserdataReadBranch::RejectedId);
    assert_eq!(result.after.load_flag, flag);
    assert_eq!(result.history.object_id, Copied);
    assert_eq!(result.history.load_flag, Copied);
    assert_eq!(result.history.slots, [Cleared; 20]);
}

fn assert_mixed_history(result: &ElectricalUserdataStateRead, source: &ElectricalUserdataState) {
    use ElectricalUserdataStorageRelation::{Assigned, Copied};
    assert_eq!(result.branch, ElectricalUserdataReadBranch::AcceptedId);
    assert_eq!(result.history.context_word, Assigned);
    assert_eq!(result.history.load_flag, Assigned);
    assert_eq!(result.history.object_id, Assigned);
    assert_eq!(result.history.object_no, Assigned);
    assert_eq!(result.diagnostics.len(), 3);
    for index in 0..20 {
        let expected = if index % 2 == 0 { Copied } else { Assigned };
        assert_eq!(result.history.slots[index], expected);
        if index % 2 == 0 {
            assert_eq!(result.after.slots[index], source.slots[index]);
        }
    }
}
