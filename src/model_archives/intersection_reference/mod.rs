//! Stored intersection-reference roles without native matching or topology resolution (#184).
mod admission;
mod archive_ops;
mod projection;
#[cfg(test)]
#[path = "../../../tests/model_intersection_reference/projection_unit.rs"]
mod projection_tests;

use super::ModelFieldHandle;
use std::sync::Arc;

/// Exact stored words in one physical ordinary intersection-edge reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelIntersectionReferenceStoredWords {
    /// Uninterpreted state word; no enum or active-state restriction is imposed.
    pub raw_state: u16,
    /// Raw word20; a complete loaded coedge-ordinal role remains unproven.
    pub raw_word_20: i32,
    /// Signed stored span-count state, including uncommitted and noncanonical values.
    pub span_count_state: i32,
    /// Signed stored backward-offset state, without traversal or normalization.
    pub backward_offset_state: i32,
}

/// Isolated stored-role replacements; native state transitions and matching do not run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelIntersectionReferenceStoredEdit {
    /// Replace only raw24, preserving all other words and references.
    SpanCountState(i32),
    /// Replace only raw28, preserving all other words and references.
    BackwardOffsetState(i32),
    /// Replace raw24/raw28 together through one complete archive operation.
    SpanAndBackwardStates {
        /// Complete signed replacement for raw24.
        span_count_state: i32,
        /// Complete signed replacement for raw28.
        backward_offset_state: i32,
    },
}

/// Immutable words and physical field identities owned by one complete model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelIntersectionReferenceStoredState {
    words: ModelIntersectionReferenceStoredWords,
    source_fields: [ModelFieldHandle; 4],
    path: Arc<str>,
}

impl ModelIntersectionReferenceStoredState {
    /// Copies every raw word without resolving topology; e.g. `let raw = reference.words();`.
    pub fn words(&self) -> ModelIntersectionReferenceStoredWords {
        self.words
    }

    /// Borrows state/raw20/span/backward identities in physical order; e.g. `source.resolve_field(&reference.source_fields()[2])?`.
    pub fn source_fields(&self) -> &[ModelFieldHandle; 4] {
        &self.source_fields
    }

    fn replacements(&self, edit: ModelIntersectionReferenceStoredEdit) -> [i32; 2] {
        let words = self.words;
        match edit {
            ModelIntersectionReferenceStoredEdit::SpanCountState(value) => {
                [value, words.backward_offset_state]
            }
            ModelIntersectionReferenceStoredEdit::BackwardOffsetState(value) => {
                [words.span_count_state, value]
            }
            ModelIntersectionReferenceStoredEdit::SpanAndBackwardStates {
                span_count_state,
                backward_offset_state,
            } => [span_count_state, backward_offset_state],
        }
    }
}
