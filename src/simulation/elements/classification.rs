use super::SimulationElementRecord;

/// Stored selector inputs and independently derived numeric classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElementClassification {
    /// Raw family byte at offset 20; not an API enum.
    pub selector_family: u8,
    /// Unsigned count at offset 23; classification permits all byte patterns.
    pub accessor_node_count: u8,
    /// Raw variant byte at offset 24.
    pub selector_variant: u8,
    /// Historical stored word at offset 26, never silently replaced.
    pub stored_code: u16,
    /// Full native selector result, including the all-ones sentinel.
    pub selected_code: u32,
}

impl SimulationElementRecord {
    /// Project exact stored inputs and the independent selector, e.g. `record.classification()`.
    pub fn classification(&self) -> SimulationElementClassification {
        let family = self.bytes[20];
        let count = self.bytes[23];
        let variant = self.bytes[24];
        SimulationElementClassification {
            selector_family: family,
            accessor_node_count: count,
            selector_variant: variant,
            stored_code: u16::from_le_bytes([self.bytes[26], self.bytes[27]]),
            selected_code: select_numeric_code(family, count, variant),
        }
    }
}

impl SimulationElementClassification {
    /// Signed selector view, e.g. `classification.selected_signed()` returns -1 for the sentinel.
    pub fn selected_signed(&self) -> i32 {
        self.selected_code as i32
    }
}

fn select_numeric_code(family: u8, count: u8, variant: u8) -> u32 {
    match (family, count) {
        (4, 4) => 98,
        (4, _) => 92,
        (2, 3) => 4,
        (2, 2) => 8,
        (1, 1) => 37,
        (8, _) => select_family_eight(count, variant),
        _ => u32::MAX,
    }
}

fn select_family_eight(count: u8, variant: u8) -> u32 {
    let row = match count {
        3 => Some([63, 43, 53]),
        4 => Some([64, 44, 54]),
        6 => Some([94, 90, 99]),
        _ => None,
    };
    if let Some(value) = row.and_then(|row| row.get(usize::from(variant)).copied()) {
        return value;
    }
    if count.is_multiple_of(3) && variant > 2 {
        return 2;
    }
    match (count.is_multiple_of(4), variant > 2) {
        (true, true) => 42,
        _ => 93,
    }
}
