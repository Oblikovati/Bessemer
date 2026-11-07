use super::InspectionProfile;
use super::tables::{INVERSE, SBOX};

pub(super) type Block = [u8; 16];
pub(super) type RoundKeys = [Block; 11];
const FORWARD_SHIFT: [usize; 16] = [0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11];
const INVERSE_SHIFT: [usize; 16] = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];

pub(super) fn round_keys(profile: InspectionProfile) -> RoundKeys {
    // #92 / clauses 14.8 and 14.15: format constants; Framework ignores the ninth historical byte.
    let current = [1, 2, 2, 3, 3, 5, 5, 5, 4, 4, 5, 5, 3, 3, 2, 1];
    let historical = [1, 2, 2, 3, 3, 5, 5, 4];
    let key = match profile {
        InspectionProfile::Current2020 => current.as_slice(),
        InspectionProfile::Historical2015 => historical.as_slice(),
    };
    expand_key(key)
}

fn expand_key(key: &[u8]) -> RoundKeys {
    let mut expanded = [0_u8; 176];
    expanded[..key.len()].copy_from_slice(key);
    let mut rcon = 1;
    for word in key.len() / 4..44 {
        let mut previous = [0; 4];
        previous.copy_from_slice(&expanded[(word - 1) * 4..word * 4]);
        if word % (key.len() / 4) == 0 {
            previous.rotate_left(1);
            previous = previous.map(|byte| SBOX[usize::from(byte)]);
            previous[0] ^= rcon;
            rcon = multiply(rcon, 2);
        }
        for column in 0..4 {
            expanded[word * 4 + column] =
                expanded[word * 4 + column - key.len()] ^ previous[column];
        }
    }
    std::array::from_fn(|round| std::array::from_fn(|byte| expanded[round * 16 + byte]))
}

pub(super) fn encrypt(mut block: Block, keys: &RoundKeys) -> Block {
    add_key(&mut block, &keys[0]);
    for key in &keys[1..10] {
        block = block.map(|byte| SBOX[usize::from(byte)]);
        block = shift(block, &FORWARD_SHIFT);
        mix(&mut block, [2, 3, 1, 1]);
        add_key(&mut block, key);
    }
    block = shift(block.map(|byte| SBOX[usize::from(byte)]), &FORWARD_SHIFT);
    add_key(&mut block, &keys[10]);
    block
}

pub(super) fn decrypt(mut block: Block, keys: &RoundKeys) -> Block {
    add_key(&mut block, &keys[10]);
    for key in keys[1..10].iter().rev() {
        block = shift(block, &INVERSE_SHIFT).map(|byte| INVERSE[usize::from(byte)]);
        add_key(&mut block, key);
        mix(&mut block, [14, 11, 13, 9]);
    }
    block = shift(block, &INVERSE_SHIFT).map(|byte| INVERSE[usize::from(byte)]);
    add_key(&mut block, &keys[0]);
    block
}

fn add_key(block: &mut Block, key: &Block) {
    for (byte, key_byte) in block.iter_mut().zip(key) {
        *byte ^= key_byte;
    }
}

fn shift(block: Block, positions: &[usize; 16]) -> Block {
    std::array::from_fn(|index| block[positions[index]])
}

fn mix(block: &mut Block, coefficients: [u8; 4]) {
    for column in block.as_chunks_mut::<4>().0 {
        let original = *column;
        for row in 0..4 {
            column[row] = (0..4).fold(0, |sum, offset| {
                sum ^ multiply(original[(row + offset) % 4], coefficients[offset])
            });
        }
    }
}

fn multiply(mut left: u8, mut right: u8) -> u8 {
    let mut product = 0;
    while right != 0 {
        if right & 1 != 0 {
            product ^= left;
        }
        left = (left << 1) ^ if left & 0x80 != 0 { 0x1b } else { 0 };
        right >>= 1;
    }
    product
}
