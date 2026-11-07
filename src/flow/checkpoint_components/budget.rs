use super::*;
#[derive(Clone, Copy)]
pub(super) enum Counter {
    Rows,
    Indices,
    Slots,
    Containers,
    Wides,
    Chunks,
    Encoded,
}
pub(super) fn admit(
    counts: &mut FlowCheckpointCounts,
    options: FlowCheckpointOptions,
    kind: Counter,
    count: usize,
) -> Result<()> {
    let (used, maximum, label) = counter(counts, options, kind);
    if count > maximum.saturating_sub(*used) {
        return Err(invalid(
            label,
            format!(
                "additional count {count} after {used}; expected an aggregate at most configured bound {maximum}"
            ),
        ));
    }
    *used += count;
    Ok(())
}
fn counter(
    counts: &mut FlowCheckpointCounts,
    options: FlowCheckpointOptions,
    kind: Counter,
) -> (&mut usize, usize, &'static str) {
    let used = match kind {
        Counter::Rows => &mut counts.row_positions,
        Counter::Indices => &mut counts.indices,
        Counter::Slots => &mut counts.slots,
        Counter::Containers => &mut counts.containers,
        Counter::Wides => &mut counts.wide_patterns,
        Counter::Chunks => &mut counts.chunks,
        Counter::Encoded => &mut counts.encoded_bytes,
    };
    let (maximum, label) = limit(options, kind);
    (used, maximum, label)
}
fn limit(options: FlowCheckpointOptions, kind: Counter) -> (usize, &'static str) {
    match kind {
        Counter::Rows => (options.maximum_row_positions, "row positions"),
        Counter::Indices => (options.maximum_indices, "indices"),
        Counter::Slots => (options.maximum_slots, "slots"),
        Counter::Containers => (options.maximum_containers, "containers"),
        Counter::Wides => (options.maximum_wide_patterns, "wide patterns"),
        Counter::Chunks => (options.maximum_chunks, "chunks"),
        Counter::Encoded => (options.maximum_encoded_bytes, "encoded bytes"),
    }
}
