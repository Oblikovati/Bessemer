use super::{Result, invalid};

pub(super) fn patterns<const N: usize>(bytes: &[u8], entity: &str) -> Result<[u32; N]> {
    if bytes.len() != N * 4 {
        return Err(invalid(
            entity,
            format!(
                "{} bytes; expected {N} complete four-byte patterns",
                bytes.len()
            ),
        ));
    }
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| u32::from_le_bytes(*word))
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|words: Vec<u32>| {
            invalid(entity, format!("{} patterns; expected {N}", words.len()))
        })
}

pub(super) fn fixed<const N: usize>(bytes: &[u8], entity: &str) -> Result<[u8; N]> {
    bytes.try_into().map_err(|_| {
        invalid(
            entity,
            format!("{} bytes; expected fixed region of {N} bytes", bytes.len()),
        )
    })
}

pub(super) fn append_patterns(output: &mut Vec<u8>, patterns: &[u32]) {
    for pattern in patterns {
        output.extend_from_slice(&pattern.to_le_bytes());
    }
}
