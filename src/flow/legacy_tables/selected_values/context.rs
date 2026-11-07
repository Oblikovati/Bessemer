use super::*;
const ACCEPTED_CODES: &[u8] = &[
    0, 2, 3, 4, 6, 7, 10, 11, 12, 13, 14, 15, 30, 31, 32, 50, 51, 52, 60, 61, 62, 70, 72, 73, 74,
    76, 77, 80, 82, 83, 84, 88, 90, 91, 92,
];
struct FreshIndices {
    dense: Vec<Option<usize>>,
    ordinals: Vec<usize>,
}
impl FlowSelectionContext {
    /// Derives a fresh map from complete typed inputs; e.g. `let context = FlowSelectionContext::derive(&links, &states, options)?`.
    pub fn derive(
        links: &FlowLinkRecords,
        states: &FlowByteStates,
        options: FlowSelectionContextOptions,
    ) -> Result<Self> {
        validate_inputs(links, states, options.maximum_entries)?;
        let indices = fresh_indices(links, states, options.maximum_selected_values)?;
        Ok(Self {
            links: links.clone(),
            states: states.clone(),
            options,
            dense_indices: indices.dense.into(),
            ordinals: indices.ordinals.into(),
        })
    }
}
fn validate_inputs(links: &FlowLinkRecords, states: &FlowByteStates, maximum: usize) -> Result<()> {
    let count = links.entries().len();
    let state_count = states.stored_count();
    if count > maximum {
        return Err(selected_invalid(format!(
            "link count {count}; expected at most {maximum} actual map entries"
        )));
    }
    if state_count == 0 || state_count == count {
        return Ok(());
    }
    Err(selected_invalid(format!(
        "state count {state_count}, link count {count}; expected an empty state array or one byte per physical link"
    )))
}
fn fresh_indices(
    links: &FlowLinkRecords,
    states: &FlowByteStates,
    maximum: usize,
) -> Result<FreshIndices> {
    let mut dense = vec![None; links.entries().len()];
    let mut ordinals = Vec::new();
    for entry in links.entries() {
        if !is_selected(entry, states) {
            continue;
        }
        if ordinals.len() >= maximum {
            return Err(selected_invalid(format!(
                "selected ordinal {}, previous selected count {}; expected at most {maximum} selected values",
                entry.ordinal(),
                ordinals.len()
            )));
        }
        dense[entry.ordinal()] = Some(ordinals.len());
        ordinals.push(entry.ordinal());
    }
    Ok(FreshIndices { dense, ordinals })
}
fn is_selected(entry: FlowLinkEntry<'_>, states: &FlowByteStates) -> bool {
    states.byte(entry.ordinal()).is_some_and(|flag| flag >= 128)
        && ACCEPTED_CODES.contains(&entry.native_code_byte())
}
