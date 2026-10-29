use super::{InspectionProfile, invalid, rijndael};
use crate::Result;
use rijndael::{Block, RoundKeys};

pub(super) fn require_length(length: usize, maximum: usize) -> Result<()> {
    if length == 0 || !length.is_multiple_of(16) || length > maximum {
        return Err(invalid(format!(
            "ciphertext length {length}; expected a positive multiple of 16 <= {maximum}"
        )));
    }
    Ok(())
}

pub(super) fn padded_length(length: usize, maximum: usize) -> Result<usize> {
    let padded = length
        .checked_add(16 - length % 16)
        .filter(|padded| *padded <= maximum);
    padded.ok_or_else(|| {
        invalid(format!(
            "plaintext length {length}; expected PKCS#7 padded length <= {maximum}"
        ))
    })
}

pub(super) fn decode(
    profile: InspectionProfile,
    ciphertext: &[u8],
    maximum: usize,
) -> Result<Vec<u8>> {
    require_length(ciphertext.len(), maximum)?;
    let keys = rijndael::round_keys(profile);
    let mut previous = [0; 16];
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    for block in ciphertext.as_chunks::<16>().0 {
        let decoded = rijndael::decrypt(*block, &keys);
        plaintext.extend(
            decoded
                .iter()
                .zip(previous)
                .map(|(byte, preceding)| byte ^ preceding),
        );
        previous = *block;
    }
    remove_padding(&mut plaintext)?;
    Ok(plaintext)
}

fn remove_padding(plaintext: &mut Vec<u8>) -> Result<()> {
    let padding = usize::from(plaintext.last().copied().unwrap_or(0));
    let start = plaintext.len().checked_sub(padding);
    if !(1..=16).contains(&padding)
        || start.is_none_or(|start| {
            plaintext[start..]
                .iter()
                .any(|byte| usize::from(*byte) != padding)
        })
    {
        return Err(invalid(format!(
            "padding count {padding} in {} decoded bytes; expected 1..=16 identical PKCS#7 bytes",
            plaintext.len()
        )));
    }
    plaintext.truncate(plaintext.len() - padding);
    Ok(())
}

pub(super) fn encode(
    profile: InspectionProfile,
    plaintext: &[u8],
    maximum: usize,
) -> Result<Vec<u8>> {
    let length = padded_length(plaintext.len(), maximum)?;
    let padding = (length - plaintext.len()) as u8;
    let mut ciphertext = plaintext.to_vec();
    ciphertext.resize(length, padding);
    encrypt_blocks(&mut ciphertext, &rijndael::round_keys(profile));
    Ok(ciphertext)
}

fn encrypt_blocks(ciphertext: &mut [u8], keys: &RoundKeys) {
    let mut previous: Block = [0; 16];
    for block in ciphertext.as_chunks_mut::<16>().0 {
        for (byte, preceding) in block.iter_mut().zip(previous) {
            *byte ^= preceding;
        }
        *block = rijndael::encrypt(*block, keys);
        previous = *block;
    }
}
