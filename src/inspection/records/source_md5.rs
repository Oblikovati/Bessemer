//! Private RFC1321 checksum for the stored Inspection compatibility field (#195).

// Fixed integer parameters avoid platform-dependent runtime floating-point generation (§3.4).
const MD5_ADDITIONS: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];
const MD5_ROTATIONS: [[u32; 4]; 4] = [
    [7, 12, 17, 22],
    [5, 9, 14, 20],
    [4, 11, 16, 23],
    [6, 10, 15, 21],
];

pub(super) fn compatibility_md5_hex(source: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    md5_message_digest(source)
        .into_iter()
        .flat_map(|byte| {
            [
                char::from(HEX[usize::from(byte >> 4)]),
                char::from(HEX[usize::from(byte & 15)]),
            ]
        })
        .collect()
}

fn md5_message_digest(source: &[u8]) -> [u8; 16] {
    let mut state = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
    let (blocks, remainder) = source.as_chunks::<64>();
    for block in blocks {
        compress_md5_block(&mut state, block);
    }
    let (tail, length) = pad_md5_remainder(remainder, source.len());
    for block in tail[..length].as_chunks::<64>().0 {
        compress_md5_block(&mut state, block);
    }
    encode_md5_state(state)
}

fn pad_md5_remainder(remainder: &[u8], source_length: usize) -> ([u8; 128], usize) {
    let mut tail = [0; 128];
    tail[..remainder.len()].copy_from_slice(remainder);
    tail[remainder.len()] = 0x80;
    let length = if remainder.len() < 56 { 64 } else { 128 };
    tail[length - 8..length].copy_from_slice(&(source_length as u64).wrapping_mul(8).to_le_bytes());
    (tail, length)
}

fn compress_md5_block(state: &mut [u32; 4], block: &[u8; 64]) {
    let words = md5_message_words(block);
    let mut registers = *state;
    for round in 0..64 {
        let [a, b, c, d] = registers;
        let (mix, index) = md5_round_input(b, c, d, round);
        let sum = a
            .wrapping_add(mix)
            .wrapping_add(MD5_ADDITIONS[round])
            .wrapping_add(words[index]);
        let next = b.wrapping_add(sum.rotate_left(MD5_ROTATIONS[round / 16][round % 4]));
        registers = [d, next, b, c];
    }
    for (word, value) in state.iter_mut().zip(registers) {
        *word = word.wrapping_add(value);
    }
}

fn md5_message_words(block: &[u8; 64]) -> [u32; 16] {
    let mut words = [0; 16];
    for (word, bytes) in words.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = u32::from_le_bytes(*bytes);
    }
    words
}

fn md5_round_input(b: u32, c: u32, d: u32, round: usize) -> (u32, usize) {
    match round / 16 {
        0 => ((b & c) | (!b & d), round),
        1 => ((d & b) | (!d & c), (5 * round + 1) % 16),
        2 => (b ^ c ^ d, (3 * round + 5) % 16),
        _ => (c ^ (b | !d), (7 * round) % 16),
    }
}

fn encode_md5_state(state: [u32; 4]) -> [u8; 16] {
    let mut digest = [0; 16];
    for (bytes, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(state) {
        *bytes = word.to_le_bytes();
    }
    digest
}
