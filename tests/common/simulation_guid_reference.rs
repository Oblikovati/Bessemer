use super::{packet::Cursor, reference::LIMITS};
use bessemer::simulation::*;

fn unit_bytes(units: &[u16]) -> Vec<u8> {
    units.iter().flat_map(|unit| unit.to_le_bytes()).collect()
}

pub fn check_guid(raw: &[u8], expected: &[u8]) -> SimulationGuidArchive {
    let archive = SimulationGuidArchive::from_bytes(raw, LIMITS.maximum_guid_stream_bytes).unwrap();
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.word(), 4);
    for field in archive.fields() {
        check_field(&archive, field, &mut cursor);
    }
    assert_eq!(archive.coverage().decoded_bytes, cursor.word() as usize);
    assert_eq!(
        archive.coverage().completion == SimulationGuidStreamCompletion::CompleteFourFields,
        cursor.word() != 0
    );
    assert_eq!(archive.tail(), cursor.span());
    assert!(cursor.0.is_empty());
    assert_eq!(archive.to_bytes(), raw);
    assert_eq!(
        SimulationGuidArchive::from_parts(archive.parts(), LIMITS.maximum_guid_stream_bytes)
            .unwrap()
            .to_bytes(),
        raw
    );
    assert_eq!(
        archive.coverage().mode,
        SimulationGuidCheckMode::ExactFourFramesAndTailPreservation
    );
    assert!(!archive.coverage().transfer_prefix);
    archive
}

fn check_field(
    archive: &SimulationGuidArchive,
    field: &SimulationGuidField,
    cursor: &mut Cursor<'_>,
) {
    assert_eq!(field.index, cursor.word() as usize);
    assert_eq!(field.stream_offset, cursor.word() as usize);
    assert_eq!(field.byte_length, cursor.word());
    assert_eq!(field.stored_units().is_some(), cursor.word() != 0);
    if let Some(units) = field.stored_units() {
        assert_eq!(unit_bytes(units), cursor.span());
        assert_eq!(unit_bytes(units), cursor.span());
        assert_eq!(
            unit_bytes(field.caller_visible_units().unwrap()),
            cursor.span()
        );
    } else {
        assert_eq!(field.caller_visible_units(), None);
    }
    assert_eq!(archive.field_bytes(&field.id).unwrap(), cursor.span());
    assert_eq!(archive.field(&field.id).unwrap(), field);
}

pub fn check_edits(archive: &SimulationGuidArchive, cursor: &mut Cursor<'_>) {
    for (index, field) in archive.fields().iter().enumerate() {
        let edit = archive
            .edit_field(&field.id, Some(&[0x52, 0xd800, 0, 0x5a]))
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), cursor.span());
        for (ordinal, history) in edit.history.iter().enumerate() {
            assert_eq!(history.parent, archive.fields()[ordinal].id);
            assert_eq!(history.result, edit.archive.fields()[ordinal].id);
            assert_eq!(history.replaced, index == ordinal);
        }
        assert_eq!(edit.coverage, edit.archive.coverage());
        assert_eq!(edit.diagnostics, archive.diagnostics());
        assert_eq!(edit.requested_pattern_error_bits, 0);
        assert!(edit.archive.field(&field.id).is_err());
        let revert = edit
            .archive
            .edit_field(&edit.history[index].result, field.stored_units())
            .unwrap();
        assert_eq!(revert.archive.to_bytes(), archive.to_bytes());
        let unchanged = archive.edit_field(&field.id, field.stored_units()).unwrap();
        assert_eq!(unchanged.archive.revision(), archive.revision());
    }
}

fn caller_units(cursor: &mut Cursor<'_>) -> Option<Vec<u16>> {
    if cursor.word() == 0 {
        return None;
    }
    Some(
        cursor
            .span()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|unit| u16::from_le_bytes(*unit))
            .collect(),
    )
}

pub fn writer_cases(cursor: &mut Cursor<'_>) {
    assert_eq!(cursor.word(), 10);
    for _ in 0..10 {
        let strings: [Option<Vec<u16>>; 4] = std::array::from_fn(|_| caller_units(cursor));
        let borrowed = strings.each_ref().map(|units| units.as_deref());
        let projection = SimulationGuidWriterProjection::from_caller_strings(
            borrowed,
            LIMITS.maximum_guid_stream_bytes,
        )
        .unwrap();
        assert!(!projection.native_io_executed());
        assert_eq!(projection.stream().is_some(), cursor.word() != 0);
        if let Some(archive) = projection.stream() {
            assert_eq!(archive.to_bytes(), cursor.span());
            check_guid(&archive.to_bytes(), cursor.span());
        }
    }
}
