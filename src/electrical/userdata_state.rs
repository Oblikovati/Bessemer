//! Constructor-shaped userdata storage transitions with externally supplied provider effects.
use super::electrical_userdata_id_transition;
use std::sync::Arc;

/// Immutable twenty-slot native storage snapshot, with strings retained as raw UTF-16 units.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataState {
    /// Opaque connection context word; never dereferenced by Bessemer.
    pub context_word: u64,
    /// Original byte retained on null input and rejection; acceptance writes one.
    pub load_flag: u8,
    /// Signed object identifier before or after gate normalization.
    pub object_id: i32,
    /// Signed object number retained when no provider write occurs.
    pub object_no: i32,
    /// Exact twenty native string slots, preserving embedded NUL and unpaired surrogates.
    pub slots: [Arc<[u16]>; 20],
}

/// Known destination effect of a typed provider call, independent of its ignored return value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElectricalUserdataEffect<T> {
    /// The provider is known to have left the destination unchanged.
    Unchanged,
    /// The provider wrote this complete converted value.
    Written(T),
}

/// Explicit effects supplied for one non-null recordset; unexecuted branch effects are ignored.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataReadEffects {
    /// Actual opaque context helper result.
    pub context_word: u64,
    /// Effect on the ID gate's temporary, initialized to minus one.
    pub object_id: ElectricalUserdataEffect<i32>,
    /// Effect on the existing object-number member, used only after acceptance.
    pub object_no: ElectricalUserdataEffect<i32>,
    /// Effects on existing UTF-16 destinations, used only after acceptance.
    pub slots: [ElectricalUserdataEffect<Arc<[u16]>>; 20],
}

/// Executed native reader branch, distinct from SQL row existence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalUserdataReadBranch {
    /// A completed empty query skipped the delegate entirely.
    QueryEof,
    /// Null recordset returns without modifying storage.
    NullRecordset,
    /// Rejected ID clears strings and retains the load flag and object number.
    RejectedId,
    /// Accepted ID sets the load flag and attempts the remaining reads.
    AcceptedId,
}

/// Storage history relation; an assignment may preserve the original value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalUserdataStorageRelation {
    /// No write to this destination.
    Copied,
    /// Explicit assignment from the helper, provider, or initialized ID temporary.
    Assigned,
    /// Negative loaded ID normalized to minus one.
    NormalizedNegativeId,
    /// Native rejection branch emptied this slot.
    Cleared,
}

/// Complete destination history, with before/after values retained in the operation result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataStateHistory {
    /// Context-word relation.
    pub context_word: ElectricalUserdataStorageRelation,
    /// Load-flag relation.
    pub load_flag: ElectricalUserdataStorageRelation,
    /// Object-ID relation.
    pub object_id: ElectricalUserdataStorageRelation,
    /// Object-number relation.
    pub object_no: ElectricalUserdataStorageRelation,
    /// Relations for all twenty string destinations in native order.
    pub slots: [ElectricalUserdataStorageRelation; 20],
}

/// Completed pure projection of supplied effects, without provider or application validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataStateRead {
    /// Retained original immutable storage values.
    pub before: ElectricalUserdataState,
    /// Complete resulting storage values.
    pub after: ElectricalUserdataState,
    /// Executed branch and corresponding field-read coverage.
    pub branch: ElectricalUserdataReadBranch,
    /// Complete field history; unchanged values may still have assignment relations.
    pub history: ElectricalUserdataStateHistory,
    /// Exact destination-effect projection coverage, not provider conversion coverage.
    pub coverage: ElectricalUserdataStateCoverage,
    /// Required exclusions accompanying this complete supplied-effect projection.
    pub diagnostics: [ElectricalUserdataStateDiagnostic; 3],
}

/// Checks not established by projecting caller-supplied destination effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalUserdataStateDiagnostic {
    /// Provider conversion, write failure behavior and context acquisition remain external.
    ProviderEffectsSuppliedByCaller,
    /// Only the documented twenty-slot constructor shape is represented.
    MalformedNativeArraysOutsideScope,
    /// SQL row choice, Electrical application and geometry acceptance were not checked.
    NativeApplicationNotChecked,
}

/// Coverage of the complete constructor-shaped storage projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataStateCoverage {
    /// Attempted typed Long reads, including the gate temporary.
    pub long_reads: u8,
    /// Attempted typed string reads.
    pub string_reads: u8,
    /// Explicitly emptied slots.
    pub cleared_slots: u8,
    /// Exact supplied-unit disagreement; successful projections always report zero.
    pub achieved_unit_error: usize,
}

impl ElectricalUserdataState {
    /// Projects known provider effects; e.g. `let unchanged = state.project_recordset(None)`.
    pub fn project_recordset(
        &self,
        effects: Option<&ElectricalUserdataReadEffects>,
    ) -> ElectricalUserdataStateRead {
        let mut result = initial_result(self);
        let Some(effects) = effects else {
            return result;
        };
        result.after.context_word = effects.context_word;
        result.history.context_word = ElectricalUserdataStorageRelation::Assigned;
        let loaded = effect_value(&effects.object_id, -1);
        let gate = electrical_userdata_id_transition(self.object_id, loaded);
        result.after.object_id = gate.stored_id;
        result.history.object_id = id_relation(self.object_id, loaded);
        if gate.delegated_success {
            accept_effects(&mut result, effects);
        } else {
            reject_effects(&mut result);
        }
        result
    }
}

fn initial_result(state: &ElectricalUserdataState) -> ElectricalUserdataStateRead {
    ElectricalUserdataStateRead {
        before: state.clone(),
        after: state.clone(),
        branch: ElectricalUserdataReadBranch::NullRecordset,
        history: copied_history(),
        coverage: ElectricalUserdataStateCoverage {
            long_reads: 0,
            string_reads: 0,
            cleared_slots: 0,
            achieved_unit_error: 0,
        },
        diagnostics: [
            ElectricalUserdataStateDiagnostic::ProviderEffectsSuppliedByCaller,
            ElectricalUserdataStateDiagnostic::MalformedNativeArraysOutsideScope,
            ElectricalUserdataStateDiagnostic::NativeApplicationNotChecked,
        ],
    }
}

fn copied_history() -> ElectricalUserdataStateHistory {
    use ElectricalUserdataStorageRelation::Copied;
    ElectricalUserdataStateHistory {
        context_word: Copied,
        load_flag: Copied,
        object_id: Copied,
        object_no: Copied,
        slots: [Copied; 20],
    }
}

fn effect_value<T: Clone>(effect: &ElectricalUserdataEffect<T>, previous: T) -> T {
    match effect {
        ElectricalUserdataEffect::Unchanged => previous,
        ElectricalUserdataEffect::Written(value) => value.clone(),
    }
}

fn effect_relation<T>(effect: &ElectricalUserdataEffect<T>) -> ElectricalUserdataStorageRelation {
    match effect {
        ElectricalUserdataEffect::Unchanged => ElectricalUserdataStorageRelation::Copied,
        ElectricalUserdataEffect::Written(_) => ElectricalUserdataStorageRelation::Assigned,
    }
}

fn id_relation(current: i32, loaded: i32) -> ElectricalUserdataStorageRelation {
    use ElectricalUserdataStorageRelation::*;
    if loaded == 0 && current != 0 {
        return Copied;
    }
    if loaded < 0 {
        return NormalizedNegativeId;
    }
    Assigned
}

fn reject_effects(result: &mut ElectricalUserdataStateRead) {
    result.branch = ElectricalUserdataReadBranch::RejectedId;
    result.after.slots = std::array::from_fn(|_| Arc::from([]));
    result.history.slots = [ElectricalUserdataStorageRelation::Cleared; 20];
    result.coverage.long_reads = 1;
    result.coverage.cleared_slots = 20;
}

fn accept_effects(
    result: &mut ElectricalUserdataStateRead,
    effects: &ElectricalUserdataReadEffects,
) {
    result.branch = ElectricalUserdataReadBranch::AcceptedId;
    result.coverage.long_reads = 2;
    result.coverage.string_reads = 20;
    result.after.load_flag = 1;
    result.history.load_flag = ElectricalUserdataStorageRelation::Assigned;
    result.after.object_no = effect_value(&effects.object_no, result.after.object_no);
    result.history.object_no = effect_relation(&effects.object_no);
    for (index, effect) in effects.slots.iter().enumerate() {
        result.after.slots[index] = effect_value(effect, result.after.slots[index].clone());
        result.history.slots[index] = effect_relation(effect);
    }
}
