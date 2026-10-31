use super::{
    EgrLayout, Result, SimulationEgrLimits, SimulationEgrLoadedField, allocation, header, require,
    wire,
};

pub(super) fn decode(bytes: &[u8], limits: SimulationEgrLimits) -> Result<EgrLayout> {
    let header = header::decode_header(bytes, limits)?;
    require(
        bytes.len() == header.expected_bytes,
        "EGR endpoint",
        format!(
            "{} bytes; expected exactly {} from loaded capacity and allocated counts",
            bytes.len(),
            header.expected_bytes
        ),
    )?;
    let fields = loaded_fields(bytes, header.field_count)?;
    let directory = decode_directory(bytes, &header);
    let free_records = allocation::validate_partition(bytes, &header, &directory)?;
    Ok(EgrLayout {
        header,
        fields,
        directory,
        free_records,
    })
}

fn loaded_fields(bytes: &[u8], count: usize) -> Result<Vec<SimulationEgrLoadedField>> {
    let mut fields = Vec::new();
    let mut start = 0;
    for descriptor in bytes[128..128 + count * 8].as_chunks::<8>().0 {
        let [tag, width] =
            wire::patterns::<2>(descriptor, "EGR loaded descriptor")?.map(|pattern| pattern as i32);
        let field = loaded_field(tag, width, start)?;
        start = field.row_end;
        fields.push(field);
    }
    Ok(fields)
}

fn decode_directory(bytes: &[u8], header: &super::EgrHeader) -> Vec<i32> {
    let start = header.directory_start;
    bytes[start..start + header.capacity * 4]
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| i32::from_le_bytes(*word))
        .collect()
}

fn loaded_field(tag: i32, width: i32, start: usize) -> Result<SimulationEgrLoadedField> {
    require(
        width > 0 && width as u64 + start as u64 <= 128,
        "EGR loaded width",
        format!("{width} at row offset {start}; expected positive width ending within 128 bytes"),
    )?;
    Ok(SimulationEgrLoadedField {
        tag,
        width: width as usize,
        row_offset: start,
        row_end: start + width as usize,
    })
}
