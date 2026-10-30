use super::pair_reference::Cursor;
use bessemer::simulation::*;

pub const LIMITS: SimulationOutLimits = SimulationOutLimits {
    maximum_bytes: 20_000_000,
    maximum_lines: 10_000,
    maximum_fields: 1000,
};

pub fn check(raw: &[u8], expected: &[u8]) -> SimulationOutArchive {
    let archive = SimulationOutArchive::from_bytes(raw, LIMITS).unwrap();
    let mut cursor = Cursor(expected);
    check_lines(&archive, &mut cursor);
    check_fields(&archive, &mut cursor);
    assert_eq!(archive.si_declaration_line().is_some(), cursor.word() != 0);
    assert_eq!(
        archive
            .si_declaration_line()
            .map(|index| index as u32)
            .unwrap_or(u32::MAX),
        cursor.word()
    );
    assert!(cursor.0.is_empty());
    assert_eq!(archive.to_bytes(), raw);
    let lines: Vec<_> = archive
        .lines()
        .iter()
        .map(|line| line.input.clone())
        .collect();
    assert_eq!(
        SimulationOutArchive::from_lines(&lines, LIMITS)
            .unwrap()
            .to_bytes(),
        raw
    );
    archive
}

fn check_lines(archive: &SimulationOutArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(archive.lines().len(), cursor.word() as usize);
    for line in archive.lines() {
        assert_eq!(line.stream_offset, cursor.word() as usize);
        assert_eq!(
            line.field_index
                .map(|index| index as u32)
                .unwrap_or(u32::MAX),
            cursor.word()
        );
        assert_eq!(
            line.legend_field_index
                .map(|index| index as u32)
                .unwrap_or(u32::MAX),
            cursor.word()
        );
        assert_eq!(line.input.content.as_ref(), cursor.span());
        assert_eq!(
            if line.input.crlf {
                b"\r\n".as_slice()
            } else {
                b""
            },
            cursor.span()
        );
        assert_eq!(line.nul_positions.len(), cursor.word() as usize);
        for position in line.nul_positions.iter() {
            assert_eq!(*position, cursor.word() as usize);
        }
    }
}

fn check_fields(archive: &SimulationOutArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(archive.fields().len(), cursor.word() as usize);
    for field in archive.fields() {
        assert_eq!(archive.field(&field.id).unwrap(), field);
        assert_eq!(field.line_index, cursor.word() as usize);
        assert_eq!(field.stream_offset, cursor.word() as usize);
        for actual in [&field.label, &field.symbol, &field.numeric_lexeme] {
            assert_eq!(actual.as_bytes(), cursor.span());
        }
        assert_eq!(field.value.is_negative(), cursor.word() != 0);
        assert_eq!(field.value.digits(), cursor.span());
        assert_eq!(field.value.exponent().to_string().as_bytes(), cursor.span());
        check_legends(field, cursor);
    }
}

fn check_legends(field: &SimulationOutField, cursor: &mut Cursor<'_>) {
    assert_eq!(field.legends.len(), cursor.word() as usize);
    for legend in field.legends.iter() {
        assert_eq!(legend.line_index, cursor.word() as usize);
        assert_eq!(legend.stream_offset, cursor.word() as usize);
        assert_eq!(legend.value.as_bytes(), cursor.span());
        assert_eq!(legend.description.as_bytes(), cursor.span());
    }
}
