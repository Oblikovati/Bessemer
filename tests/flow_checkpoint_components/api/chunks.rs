use super::*;
pub(super) fn check(
    actual: FlowCheckpointChunks<'_>,
    expected: &ChunkProjection,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    header(actual, expected, edits);
    counts.chunks += expected.count;
    let mut position = 16;
    for (index, expected) in expected.chunks.iter().enumerate() {
        let chunk = actual.chunk(index).unwrap();
        wire(chunk, expected, position);
        arrays::bytes(
            chunk.encoded_fields(),
            position + 4,
            &expected.encoded,
            edits,
        );
        counts.encoded_bytes += expected.stored;
        position += 4 + expected.stored;
    }
}
fn header(
    actual: FlowCheckpointChunks<'_>,
    expected: &ChunkProjection,
    edits: &mut Vec<EditableField>,
) {
    assert_eq!(actual.version(), expected.version);
    assert_eq!(actual.len(), expected.count);
    assert!(!actual.is_empty());
    check_field(
        actual.declared_raw_bytes(),
        FlowCheckpointValue::Pattern64(expected.declared),
        (8, 16),
        EditClass::Plain,
        true,
        edits,
    );
}
fn wire(actual: FlowCheckpointChunk<'_>, expected: &EncodedChunk, position: usize) {
    assert_eq!(actual.ordinal(), expected.ordinal);
    assert_eq!(actual.stored_bytes(), expected.stored);
    assert_eq!(
        actual.payload_interval(),
        (position, position + 4 + expected.stored)
    );
    assert_eq!(
        actual.encoded_interval(),
        (position + 4, position + 4 + expected.stored)
    );
    assert_eq!(actual.encoded_bytes(), expected.encoded);
}
