use super::{Result, SimulationMasterParts, SimulationMasterRow};
use crate::records::{dword, region};
use crate::simulation::{require, unsupported, wire};

const HEADER: [u32; 9] = [384, 70, 81, 0, 70, 0, 0, 0, 70];

pub(super) fn decode(bytes: &[u8]) -> Result<SimulationMasterParts> {
    validate_header(bytes)?;
    Ok(SimulationMasterParts {
        header_padding: bytes[40..388].into(),
        field_tags: field_tags(bytes)?,
        field_table_padding: bytes[1036..1156].into(),
        directory: directory(bytes)?,
        directory_padding: bytes[1436..1540].into(),
        physical_rows: physical_rows(bytes)?.into(),
    })
}

fn validate_header(bytes: &[u8]) -> Result<()> {
    require(
        bytes.len() == 28420,
        "MAS",
        format!(
            "{} bytes; expected complete 28,420-byte observed image",
            bytes.len()
        ),
    )?;
    require(
        dword(bytes, 0)? == 1,
        "MAS transfer",
        format!("count {}; expected 1", dword(bytes, 0)?),
    )?;
    validate_header_words(bytes)
}

fn validate_header_words(bytes: &[u8]) -> Result<()> {
    let header = wire::patterns::<9>(region(bytes, 4, 36)?, "MAS header")?;
    if header != HEADER {
        return Err(unsupported(
            "MAS header",
            format!("{header:?}; expected observed header {HEADER:?}"),
        ));
    }
    Ok(())
}

fn field_tags(bytes: &[u8]) -> Result<[i32; 81]> {
    let mut tags = [0; 81];
    for (index, tag) in tags.iter_mut().enumerate() {
        let offset = 388 + index * 8;
        let width = dword(bytes, offset + 4)?;
        if width != 4 {
            return Err(unsupported(
                "MAS field table",
                format!(
                    "field {} width {width}; expected observed width 4",
                    index + 1
                ),
            ));
        }
        *tag = dword(bytes, offset)? as i32;
    }
    Ok(tags)
}

fn directory(bytes: &[u8]) -> Result<[u8; 70]> {
    let mut directory = [0; 70];
    for (index, record) in directory.iter_mut().enumerate() {
        let value = dword(bytes, 1156 + index * 4)?;
        require(
            (1..=70).contains(&value),
            "MAS directory",
            format!(
                "logical row {} selects record {value}; expected 1..=70",
                index + 1
            ),
        )?;
        *record = value as u8;
    }
    Ok(directory)
}

fn physical_rows(bytes: &[u8]) -> Result<Vec<SimulationMasterRow>> {
    (0..70)
        .map(|index| {
            let offset = 1540 + index * 384;
            Ok(SimulationMasterRow {
                fields: wire::patterns(region(bytes, offset, 324)?, "MAS row fields")?,
                padding: wire::fixed(region(bytes, offset + 324, 60)?, "MAS row padding")?,
            })
        })
        .collect()
}

pub(super) fn validate(parts: &SimulationMasterParts) -> Result<()> {
    for (name, actual, expected) in [
        ("header padding", parts.header_padding.len(), 348),
        ("field-table padding", parts.field_table_padding.len(), 120),
        ("directory padding", parts.directory_padding.len(), 104),
        ("physical rows", parts.physical_rows.len(), 70),
    ] {
        require(
            actual == expected,
            "MAS parts",
            format!("{name} length {actual}; expected {expected}"),
        )?;
    }
    validate_directory(&parts.directory)
}

fn validate_directory(directory: &[u8; 70]) -> Result<()> {
    let mut seen = [false; 70];
    for (logical, record) in directory.iter().enumerate() {
        require(
            (1..=70).contains(record),
            "MAS directory",
            format!(
                "logical row {} record {record}; expected 1..=70",
                logical + 1
            ),
        )?;
        require(
            !seen[*record as usize - 1],
            "MAS directory",
            format!("record {record} selected twice; expected permutation 1..=70"),
        )?;
        seen[*record as usize - 1] = true;
    }
    Ok(())
}

pub(super) fn encode(parts: &SimulationMasterParts) -> Vec<u8> {
    let mut output = Vec::with_capacity(28420);
    wire::append_patterns(&mut output, &[1]);
    wire::append_patterns(&mut output, &HEADER);
    output.extend_from_slice(&parts.header_padding);
    for tag in parts.field_tags {
        wire::append_patterns(&mut output, &[tag as u32, 4]);
    }
    output.extend_from_slice(&parts.field_table_padding);
    for record in parts.directory {
        wire::append_patterns(&mut output, &[u32::from(record)]);
    }
    output.extend_from_slice(&parts.directory_padding);
    for row in parts.physical_rows.iter() {
        wire::append_patterns(&mut output, &row.fields);
        output.extend_from_slice(&row.padding);
    }
    output
}
