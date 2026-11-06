use super::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Immutable explicit existing/result map; deterministic order does not model native hashing.
#[derive(Clone, Debug)]
pub struct FlowIdStateMap(Arc<BTreeMap<i32, u8>>);

impl From<BTreeMap<i32, u8>> for FlowIdStateMap {
    fn from(states: BTreeMap<i32, u8>) -> Self {
        Self(Arc::new(states))
    }
}

impl FlowIdStateMap {
    /// Returns the number of distinct keys; e.g. `let count = states.len()`.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Distinguishes a completed empty map; e.g. `let empty = states.is_empty()`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns one complete state byte; e.g. `let state = states.get(-1)`.
    pub fn get(&self, identifier: i32) -> Option<u8> {
        self.0.get(&identifier).copied()
    }
    /// Enumerates keys in a stable total order; e.g. `let entries: Vec<_> = states.entries().collect()`.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = (i32, u8)> + '_ {
        self.0
            .iter()
            .map(|(identifier, state)| (*identifier, *state))
    }
}

/// Assignment relation retaining every wire parent, including repeated and unchanged assignments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowIdStateAssignmentRelation {
    /// The key was absent immediately before this row.
    Created,
    /// The key existed, either in the input or from an earlier row.
    Derived,
}

/// One ordered assignment; its parent record is retained by the projection's source checkpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowIdStateAssignment {
    /// Physical source-row ordinal.
    pub entry_ordinal: usize,
    /// Complete signed key.
    pub identifier: i32,
    /// Value immediately before this assignment, distinguishing absent from zero.
    pub previous: Option<u8>,
    /// Complete assigned byte.
    pub state: u8,
    /// Creation or derivation relative to the preceding map state.
    pub relation: FlowIdStateAssignmentRelation,
}

/// Exact map-projection coverage under the explicit input snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowIdStateProjectionCoverage {
    /// Every ordered source row applied, including duplicate keys.
    pub assignments: usize,
    /// Distinct existing keys before application.
    pub existing_keys: usize,
    /// Distinct result keys.
    pub result_keys: usize,
    /// New keys created by their first assignment.
    pub created_keys: usize,
    /// Assignments derived from an immediately preceding value.
    pub derived_assignments: usize,
    /// Unmentioned existing keys remain unchanged.
    pub unmentioned_existing_entries_preserved: bool,
    /// Exact byte/count error, without approximation.
    pub achieved_error: usize,
}

/// Completed non-destructive assignment projection with both map checkpoints and ordered history.
#[derive(Clone, Debug)]
pub struct FlowIdStateProjection {
    source: FlowIdStates,
    existing: FlowIdStateMap,
    result: FlowIdStateMap,
    assignments: Arc<[FlowIdStateAssignment]>,
    created: usize,
}

impl FlowIdStates {
    /// Applies rows in wire order to an explicit existing snapshot; e.g. `let projection = rows.apply_to(&existing)`.
    pub fn apply_to(&self, existing: &FlowIdStateMap) -> FlowIdStateProjection {
        let mut states = existing.0.as_ref().clone();
        let mut assignments = Vec::with_capacity(self.count);
        let mut created = 0;
        for entry in self.entries() {
            let previous = states.insert(entry.identifier(), entry.state_byte());
            created += usize::from(previous.is_none());
            assignments.push(assignment(entry, previous));
        }
        FlowIdStateProjection {
            source: self.clone(),
            existing: existing.clone(),
            result: states.into(),
            assignments: assignments.into(),
            created,
        }
    }
}

fn assignment(entry: FlowIdStateEntry<'_>, previous: Option<u8>) -> FlowIdStateAssignment {
    FlowIdStateAssignment {
        entry_ordinal: entry.ordinal(),
        identifier: entry.identifier(),
        previous,
        state: entry.state_byte(),
        relation: if previous.is_some() {
            FlowIdStateAssignmentRelation::Derived
        } else {
            FlowIdStateAssignmentRelation::Created
        },
    }
}

impl FlowIdStateProjection {
    /// Borrows the source wire checkpoint and every assignment's parent record; e.g. `let record = projection.source().record()`.
    pub fn source(&self) -> &FlowIdStates {
        &self.source
    }
    /// Borrows the complete explicit input map; e.g. `let before = projection.existing()`.
    pub fn existing(&self) -> &FlowIdStateMap {
        &self.existing
    }
    /// Borrows the completed result map; e.g. `let after = projection.result()`.
    pub fn result(&self) -> &FlowIdStateMap {
        &self.result
    }
    /// Borrows every ordered assignment and its preceding value; e.g. `let history = projection.assignments()`.
    pub fn assignments(&self) -> &[FlowIdStateAssignment] {
        &self.assignments
    }
    /// Reports complete projection counts independently of native state; e.g. `let coverage = projection.coverage()`.
    pub fn coverage(&self) -> FlowIdStateProjectionCoverage {
        FlowIdStateProjectionCoverage {
            assignments: self.assignments.len(),
            existing_keys: self.existing.len(),
            result_keys: self.result.len(),
            created_keys: self.created,
            derived_assignments: self.assignments.len() - self.created,
            unmentioned_existing_entries_preserved: true,
            achieved_error: 0,
        }
    }
    /// Borrows native and identifier-role exclusions; e.g. `let exclusions = projection.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowIdStateDiagnostic] {
        self.source.diagnostics()
    }
}
