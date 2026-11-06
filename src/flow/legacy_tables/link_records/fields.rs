/// Existing fixed-width link fields; versions, array lengths and counts are not editable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLinkRecordField {
    /// One signed reference; -1 remains legal under bounded context.
    EntryReference {
        /// Existing physical entry ordinal.
        entry: usize,
        /// Reference component zero or one.
        component: usize,
    },
    /// One of seven raw wide words.
    EntryWide {
        /// Existing physical entry ordinal.
        entry: usize,
        /// Existing element index within the selected field.
        index: usize,
    },
    /// Original serialized byte, without applying runtime conversion to storage.
    EntryCode {
        /// Existing physical entry ordinal.
        entry: usize,
    },
    /// Existing byte-counted integer.
    EntryInteger {
        /// Existing physical entry ordinal.
        entry: usize,
        /// Existing element index within the selected field.
        index: usize,
    },
    /// One of eleven raw members; conditional array structure must remain unchanged.
    TailMember {
        /// Existing element index within the selected field.
        index: usize,
    },
    /// Existing version-two/three discarded word.
    Discarded {
        /// Existing element index within the selected field.
        index: usize,
    },
    /// Existing version-five-plus control; conditional array structure must remain unchanged.
    Control,
    /// Existing conditional integer.
    Conditional {
        /// Existing element index within the selected field.
        index: usize,
    },
    /// Existing numeric-tail raw wide word.
    NumericWide {
        /// Existing element index within the selected field.
        index: usize,
    },
    /// Existing stored numeric-tail index; runtime defaults are not serialized fields.
    NumericIndex {
        /// Existing element index within the selected field.
        index: usize,
    },
    /// Existing final element with its version-selected four/eight-byte width.
    FinalValue {
        /// Existing element index within the selected field.
        index: usize,
    },
}

/// Exact selected field width; no numeric coercion or floating-point normalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLinkRecordValue {
    /// Complete signed 32-bit field.
    Signed32(i32),
    /// Complete raw 64-bit pattern.
    WideBits(u64),
    /// Complete original code byte.
    Byte(u8),
}
