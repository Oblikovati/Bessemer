/// Existing pointer-table scalar or raw-wide field; structural counts remain unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPointerTableField {
    /// Version-one discarded count word, including empty groups.
    DiscardedCount {
        /// Existing group zero or one.
        group: usize,
    },
    /// One stored member-triplet component.
    Member {
        /// Existing populated group.
        group: usize,
        /// Component zero through two.
        component: usize,
    },
    /// One otherwise discarded triplet component.
    DiscardedTriplet {
        /// Existing populated group.
        group: usize,
        /// Existing discarded triplet.
        triplet: usize,
        /// Component zero through two.
        component: usize,
    },
    /// One raw word in an existing wide array.
    WideWord {
        /// Existing populated group.
        group: usize,
        /// Array zero through two.
        array: usize,
        /// Existing word index.
        index: usize,
    },
    /// Version-one discarded word immediately before pairs.
    DiscardedPrePairs {
        /// Existing populated group.
        group: usize,
    },
    /// One existing signed pair component, checked against supplied context if present.
    PairIndex {
        /// Existing populated group.
        group: usize,
        /// Existing physical pair index.
        pair: usize,
        /// Component zero or one.
        component: usize,
    },
}

/// Matching raw field width; wide values never pass through floating-point normalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPointerTableValue {
    /// Complete signed 32-bit scalar.
    Signed32(i32),
    /// Complete raw wide-word pattern.
    WideBits(u64),
}
