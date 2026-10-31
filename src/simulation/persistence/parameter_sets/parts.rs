use super::SimulationPersistText;

/// Explicit allocation/consumption budgets, independent of native retention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationParameterLimits {
    /// Maximum complete records.
    pub maximum_records: usize,
    /// Maximum complete table and per-record value count.
    pub maximum_table_items: usize,
    /// Maximum counted text payload, including its terminator.
    pub maximum_text_bytes: usize,
}

impl Default for SimulationParameterLimits {
    fn default() -> Self {
        Self {
            maximum_records: 10000,
            maximum_table_items: 10000,
            maximum_text_bytes: 0x100000,
        }
    }
}

/// Complete parent words; optional presence must match the explicit version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationParameterHeader {
    /// Five signed words in original order; word zero is ignored below 810026.
    pub words: [i32; 5],
    /// Present only above version 700013.
    pub gate700013: Option<i32>,
    /// Present only above version 700018.
    pub gate700018: Option<i32>,
    /// Unconditional trailing signed word.
    pub tail: i32,
    /// Present only above version 820015.
    pub gate820015: Option<i32>,
    /// Present only above version 830009.
    pub gate830009: Option<i32>,
}

/// Complete record number patterns without floating-point conversion or domain meanings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationParameterRecordNumbers {
    /// First signed four-byte record word.
    pub signed_word: i32,
    /// Two unsigned four-byte record words in wire order.
    pub unsigned_words: [u32; 2],
    /// Present only above version 700012.
    pub conditional_word: Option<i32>,
    /// Complete eight-byte double pattern, present only above version 700013.
    pub conditional_double_bits: Option<u64>,
    /// Every complete eight-byte table value, including entries beyond native retention.
    pub values_bits: Vec<u64>,
}

/// Complete input record with its exact counted text and all number patterns.
#[derive(Debug, Clone)]
pub struct SimulationParameterRecordParts {
    /// Raw persisted text, retaining null/empty/odd/UTF-16 distinctions.
    pub text: SimulationPersistText,
    /// All fixed and table-dependent record values.
    pub numbers: SimulationParameterRecordNumbers,
}

/// Complete construction input; counts derive from matching complete arrays.
#[derive(Debug, Clone)]
pub struct SimulationParameterParts {
    /// Complete parent words and version-gated fields.
    pub header: SimulationParameterHeader,
    /// Entire signed integer table, without the native retention cap.
    pub table: Vec<i32>,
    /// Every complete record in wire order.
    pub records: Vec<SimulationParameterRecordParts>,
}
