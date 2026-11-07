use super::*;
pub(super) fn payload<'a>(
    source: &'a FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
) -> Result<&'a [u8]> {
    super::super::super::payload_codec::record_payload(source, id, identifier, OPERATION)
}
pub(super) fn inflate(stored: &[u8], maximum: usize) -> Result<Vec<u8>> {
    super::super::super::payload_codec::inflate_zlib(stored, maximum, OPERATION)
}
pub(super) fn require_zero(identifier: u32, marker: u64) -> Result<()> {
    if marker == 0 {
        return Ok(());
    }
    Err(invalid(
        identifier,
        format!(
            "writer marker {marker:#018x}; expected exactly zero under the strict writer profile"
        ),
    ))
}
pub(super) fn read_index(
    stored: &[u8],
    width: FlowControlLengthWidth,
    maximum: usize,
) -> Result<(Vec<u8>, IndexFooter)> {
    let footer_bytes = width.byte_width() + 4;
    if stored.len() < 8 + footer_bytes {
        return Err(invalid(
            INDEX,
            format!(
                "stored index {} bytes; expected at least eight marker bytes, one zlib member and {footer_bytes} footer bytes",
                stored.len()
            ),
        ));
    }
    require_zero(INDEX, ArchiveCursor::new(stored).u64()?)?;
    let start = stored.len() - footer_bytes;
    let decoded = inflate(&stored[8..start], maximum)?;
    let footer = read_footer(stored, start, width)?;
    Ok((decoded, footer))
}
fn read_footer(stored: &[u8], start: usize, width: FlowControlLengthWidth) -> Result<IndexFooter> {
    let mut cursor = ArchiveCursor::new(&stored[start..]);
    let distance = match width {
        FlowControlLengthWidth::FourBytes => u64::from(cursor.u32()?),
        FlowControlLengthWidth::EightBytes => cursor.u64()?,
    };
    let identifier = cursor.u32()?;
    if identifier != INDEX || distance != stored.len() as u64 {
        return Err(invalid(
            INDEX,
            format!(
                "footer identifier {identifier:#010x}, backward distance {distance}; expected identifier {INDEX:#010x} and distance {} for the exact prefix/member/footer extent",
                stored.len()
            ),
        ));
    }
    cursor.finish()?;
    Ok(IndexFooter { start, distance })
}
pub(super) fn encode(decoded: &[u8]) -> Result<Vec<u8>> {
    super::super::super::payload_codec::encode_zlib(decoded, OPERATION)
}
pub(super) fn wrap_index(compressed: &[u8], width: FlowControlLengthWidth) -> Result<Vec<u8>> {
    let distance = compressed.len().checked_add(12 + width.byte_width()).ok_or_else(|| invalid(INDEX, format!("compressed extent {}; expected room for raw marker and footer within addressable bytes", compressed.len())))?;
    let footer = encode_distance(distance as u64, width)?;
    let mut stored = vec![0; 8];
    stored.extend_from_slice(compressed);
    stored.extend_from_slice(&footer);
    stored.extend_from_slice(&INDEX.to_le_bytes());
    Ok(stored)
}
fn encode_distance(distance: u64, width: FlowControlLengthWidth) -> Result<Vec<u8>> {
    if width == FlowControlLengthWidth::EightBytes {
        return Ok(distance.to_le_bytes().to_vec());
    }
    u32::try_from(distance).map(|value| value.to_le_bytes().to_vec()).map_err(|_| invalid(INDEX, format!("backward distance {distance}; expected a value fitting the declared four-byte unsigned footer")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distance_width_checks_the_exact_four_byte_overflow_without_allocating_a_giant_payload() {
        use FlowControlLengthWidth::*;
        assert_eq!(
            encode_distance(0x78563412, FourBytes).unwrap(),
            [0x12, 0x34, 0x56, 0x78]
        );
        assert_eq!(
            encode_distance(u32::MAX as u64, FourBytes).unwrap(),
            [255; 4]
        );
        assert!(encode_distance(u32::MAX as u64 + 1, FourBytes).is_err());
        assert_eq!(
            encode_distance(u32::MAX as u64 + 1, EightBytes).unwrap(),
            [0, 0, 0, 0, 1, 0, 0, 0]
        );
        assert_eq!(encode_distance(u64::MAX, EightBytes).unwrap(), [255; 8]);
    }
}
